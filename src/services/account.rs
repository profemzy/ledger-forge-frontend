use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::{Account, AccountType, CreateAccountRequest, UpdateAccountRequest};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct AccountService {
    cache: CacheService,
}

impl AccountService {
    pub fn new() -> Self {
        Self {
            cache: CacheService::default(),
        }
    }

    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Create a new account
    pub async fn create_account(&self, pool: &PgPool, req: CreateAccountRequest) -> Result<Account> {
        // Validate request
        req.validate()?;

        // Validate parent account exists if specified
        if let Some(parent_id) = req.parent_account_id {
            self.get_account_by_id(pool, parent_id).await?;
        }

        // Check if account code already exists
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM chart_of_accounts WHERE code = $1"
        )
        .bind(&req.code)
        .fetch_one(pool)
        .await?;

        if existing > 0 {
            return Err(AppError::Conflict(format!("Account with code '{}' already exists", req.code)));
        }

        // Create account in database
        let account = sqlx::query_as::<_, Account>(
            r#"
            INSERT INTO chart_of_accounts
                (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            RETURNING id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.code)
        .bind(&req.name)
        .bind(req.account_type.to_string())
        .bind(req.parent_account_id)
        .bind(true) // is_active defaults to true
        .bind(req.company_id)
        .fetch_one(pool)
        .await?;

        // Invalidate parent's hierarchy cache if this account has a parent
        if let Some(parent_id) = account.parent_account_id {
            let _ = self.cache.invalidate_account_hierarchy(parent_id).await;
        }

        Ok(account)
    }

    /// Get all accounts (optionally filtered by account type or parent)
    pub async fn list_accounts(
        &self,
        pool: &PgPool,
        account_type: Option<AccountType>,
        parent_id: Option<Uuid>,
        include_inactive: bool,
    ) -> Result<Vec<Account>> {
        let mut query = String::from(
            r#"
            SELECT id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            FROM chart_of_accounts
            WHERE 1=1
            "#
        );

        // Add filters
        if !include_inactive {
            query.push_str(" AND is_active = true");
        }

        if account_type.is_some() {
            query.push_str(" AND account_type = $1");
        }

        if parent_id.is_some() {
            if account_type.is_some() {
                query.push_str(" AND parent_account_id = $2");
            } else {
                query.push_str(" AND parent_account_id = $1");
            }
        }

        query.push_str(" ORDER BY code ASC");

        // Build and execute query dynamically
        let mut sql_query = sqlx::query_as::<_, Account>(&query);

        if let Some(acc_type) = account_type {
            sql_query = sql_query.bind(acc_type.to_string());
        }

        if let Some(pid) = parent_id {
            sql_query = sql_query.bind(pid);
        }

        let accounts = sql_query.fetch_all(pool).await?;

        Ok(accounts)
    }

    /// Get account by ID
    pub async fn get_account_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Account> {
        // Try to get from cache first
        if let Ok(Some(cached_json)) = self.cache.get_account(id).await {
            if let Ok(account) = serde_json::from_str::<Account>(&cached_json) {
                tracing::debug!("Cache hit for account {}", id);
                return Ok(account);
            }
        }

        // Cache miss - fetch from database
        tracing::debug!("Cache miss for account {}", id);
        let account = sqlx::query_as::<_, Account>(
            r#"
            SELECT id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            FROM chart_of_accounts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Account with id {} not found", id)))?;

        // Cache the result
        if let Ok(account_json) = serde_json::to_string(&account) {
            let _ = self.cache.set_account(id, &account_json).await;
        }

        Ok(account)
    }

    /// Update account
    pub async fn update_account(
        &self,
        pool: &PgPool,
        id: Uuid,
        req: UpdateAccountRequest,
    ) -> Result<Account> {
        // Validate request
        req.validate()?;

        // Check if account exists
        let existing = self.get_account_by_id(pool, id).await?;

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut bind_count = 1;

        let name = req.name.unwrap_or(existing.name);
        let is_active = req.is_active.unwrap_or(existing.is_active);

        updates.push(format!("name = ${}", bind_count));
        bind_count += 1;

        updates.push(format!("is_active = ${}", bind_count));
        bind_count += 1;

        updates.push("updated_at = NOW()".to_string());

        let query = format!(
            r#"
            UPDATE chart_of_accounts
            SET {}
            WHERE id = ${}
            RETURNING id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            "#,
            updates.join(", "),
            bind_count
        );

        let account = sqlx::query_as::<_, Account>(&query)
            .bind(&name)
            .bind(is_active)
            .bind(id)
            .fetch_one(pool)
            .await?;

        // Invalidate all caches for this account
        let _ = self.cache.invalidate_all_account_caches(id).await;

        // Invalidate parent's hierarchy cache if this account has a parent
        if let Some(parent_id) = account.parent_account_id {
            let _ = self.cache.invalidate_account_hierarchy(parent_id).await;
        }

        Ok(account)
    }

    /// Deactivate account (soft delete)
    pub async fn deactivate_account(&self, pool: &PgPool, id: Uuid) -> Result<Account> {
        // Check if account exists
        self.get_account_by_id(pool, id).await?;

        // Check if account has any transactions
        let transaction_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM transaction_line_items WHERE account_id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        if transaction_count > 0 {
            return Err(AppError::Conflict(
                "Cannot deactivate account with existing transactions. Mark as inactive instead.".to_string()
            ));
        }

        // Deactivate account
        let account = sqlx::query_as::<_, Account>(
            r#"
            UPDATE chart_of_accounts
            SET is_active = false, updated_at = NOW()
            WHERE id = $1
            RETURNING id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        // Invalidate all caches for this account
        let _ = self.cache.invalidate_all_account_caches(id).await;

        // Invalidate parent's hierarchy cache if this account has a parent
        if let Some(parent_id) = account.parent_account_id {
            let _ = self.cache.invalidate_account_hierarchy(parent_id).await;
        }

        Ok(account)
    }

    /// Get account hierarchy (parent and children)
    pub async fn get_account_hierarchy(&self, pool: &PgPool, id: Uuid) -> Result<AccountHierarchy> {
        // Try to get from cache first
        if let Ok(Some(cached_json)) = self.cache.get_account_hierarchy(id).await {
            if let Ok(hierarchy) = serde_json::from_str::<AccountHierarchy>(&cached_json) {
                tracing::debug!("Cache hit for account hierarchy {}", id);
                return Ok(hierarchy);
            }
        }

        // Cache miss - fetch from database
        tracing::debug!("Cache miss for account hierarchy {}", id);
        let account = self.get_account_by_id(pool, id).await?;

        // Get parent if exists
        let parent = if let Some(parent_id) = account.parent_account_id {
            Some(Box::new(self.get_account_by_id(pool, parent_id).await?))
        } else {
            None
        };

        // Get children
        let children = sqlx::query_as::<_, Account>(
            r#"
            SELECT id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at
            FROM chart_of_accounts
            WHERE parent_account_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        let hierarchy = AccountHierarchy {
            account,
            parent,
            children,
        };

        // Cache the result
        if let Ok(hierarchy_json) = serde_json::to_string(&hierarchy) {
            let _ = self.cache.set_account_hierarchy(id, &hierarchy_json).await;
        }

        Ok(hierarchy)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountHierarchy {
    pub account: Account,
    pub parent: Option<Box<Account>>,
    pub children: Vec<Account>,
}

impl Default for AccountService {
    fn default() -> Self {
        Self::new()
    }
}
