use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::{Contact, ContactType, CreateContactRequest, UpdateContactRequest};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct ContactService {
    cache: CacheService,
}

impl ContactService {
    pub fn new() -> Self {
        Self {
            cache: CacheService::default(),
        }
    }

    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Create a new contact
    pub async fn create_contact(&self, pool: &PgPool, req: CreateContactRequest) -> Result<Contact> {
        // Validate request
        req.validate()?;

        // Create contact in database
        let contact = sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO contacts
                (id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
            RETURNING id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(req.contact_type.to_string())
        .bind(&req.name)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.billing_address)
        .bind(&req.shipping_address)
        .bind(req.company_id)
        .fetch_one(pool)
        .await?;

        Ok(contact)
    }

    /// List all contacts (optionally filtered by contact type)
    pub async fn list_contacts(
        &self,
        pool: &PgPool,
        contact_type: Option<ContactType>,
        company_id: Option<Uuid>,
        limit: Option<i64>,
    ) -> Result<Vec<Contact>> {
        let mut query = String::from(
            r#"
            SELECT id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at
            FROM contacts
            WHERE 1=1
            "#
        );

        // Add filters
        if contact_type.is_some() {
            query.push_str(" AND contact_type = $1");
        }

        if company_id.is_some() {
            if contact_type.is_some() {
                query.push_str(" AND company_id = $2");
            } else {
                query.push_str(" AND company_id = $1");
            }
        }

        query.push_str(" ORDER BY name ASC");

        // Add limit if specified
        if limit.is_some() {
            let param_num = if contact_type.is_some() && company_id.is_some() {
                3
            } else if contact_type.is_some() || company_id.is_some() {
                2
            } else {
                1
            };
            query.push_str(&format!(" LIMIT ${}", param_num));
        }

        // Build and execute query dynamically
        let mut sql_query = sqlx::query_as::<_, Contact>(&query);

        if let Some(ct) = contact_type {
            sql_query = sql_query.bind(ct.to_string());
        }

        if let Some(cid) = company_id {
            sql_query = sql_query.bind(cid);
        }

        if let Some(lim) = limit {
            sql_query = sql_query.bind(lim);
        }

        let contacts = sql_query.fetch_all(pool).await?;

        Ok(contacts)
    }

    /// Get contact by ID
    pub async fn get_contact_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Contact> {
        // Try to get from cache first
        let cache_key = format!("contact:data:{}", id);
        if let Ok(Some(cached_json)) = self.cache.get::<String>(&cache_key).await {
            if let Ok(contact) = serde_json::from_str::<Contact>(&cached_json) {
                tracing::debug!("Cache hit for contact {}", id);
                return Ok(contact);
            }
        }

        // Cache miss - fetch from database
        tracing::debug!("Cache miss for contact {}", id);
        let contact = sqlx::query_as::<_, Contact>(
            r#"
            SELECT id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at
            FROM contacts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Contact with id {} not found", id)))?;

        // Cache the result (10 minute TTL)
        if let Ok(contact_json) = serde_json::to_string(&contact) {
            let _ = self.cache.set_with_ttl(&cache_key, &contact_json, 600).await;
        }

        Ok(contact)
    }

    /// Update contact
    pub async fn update_contact(
        &self,
        pool: &PgPool,
        id: Uuid,
        req: UpdateContactRequest,
    ) -> Result<Contact> {
        // Validate request
        req.validate()?;

        // Check if contact exists
        let existing = self.get_contact_by_id(pool, id).await?;

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut bind_count = 1;

        let name = req.name.unwrap_or(existing.name);
        let email = req.email.or(existing.email);
        let phone = req.phone.or(existing.phone);
        let billing_address = req.billing_address.or(existing.billing_address);
        let shipping_address = req.shipping_address.or(existing.shipping_address);

        updates.push(format!("name = ${}", bind_count));
        bind_count += 1;

        updates.push(format!("email = ${}", bind_count));
        bind_count += 1;

        updates.push(format!("phone = ${}", bind_count));
        bind_count += 1;

        updates.push(format!("billing_address = ${}", bind_count));
        bind_count += 1;

        updates.push(format!("shipping_address = ${}", bind_count));
        bind_count += 1;

        updates.push("updated_at = NOW()".to_string());

        let query = format!(
            r#"
            UPDATE contacts
            SET {}
            WHERE id = ${}
            RETURNING id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at
            "#,
            updates.join(", "),
            bind_count
        );

        let contact = sqlx::query_as::<_, Contact>(&query)
            .bind(&name)
            .bind(&email)
            .bind(&phone)
            .bind(&billing_address)
            .bind(&shipping_address)
            .bind(id)
            .fetch_one(pool)
            .await?;

        // Invalidate cache
        let cache_key = format!("contact:data:{}", id);
        let _ = self.cache.delete(&cache_key).await;

        Ok(contact)
    }

    /// Delete contact
    pub async fn delete_contact(&self, pool: &PgPool, id: Uuid) -> Result<()> {
        // Check if contact exists
        self.get_contact_by_id(pool, id).await?;

        // Check if contact has any transactions
        let transaction_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM transactions WHERE contact_id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        if transaction_count > 0 {
            return Err(AppError::Conflict(
                "Cannot delete contact with existing transactions.".to_string()
            ));
        }

        // Delete contact
        sqlx::query("DELETE FROM contacts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        // Invalidate cache
        let cache_key = format!("contact:data:{}", id);
        let _ = self.cache.delete(&cache_key).await;

        Ok(())
    }

    /// Get contacts by type
    pub async fn get_contacts_by_type(&self, pool: &PgPool, contact_type: ContactType) -> Result<Vec<Contact>> {
        self.list_contacts(pool, Some(contact_type), None, None).await
    }

    /// Get customers (convenience method)
    pub async fn get_customers(&self, pool: &PgPool) -> Result<Vec<Contact>> {
        self.get_contacts_by_type(pool, ContactType::Customer).await
    }

    /// Get vendors (convenience method)
    pub async fn get_vendors(&self, pool: &PgPool) -> Result<Vec<Contact>> {
        self.get_contacts_by_type(pool, ContactType::Vendor).await
    }

    /// Get employees (convenience method)
    pub async fn get_employees(&self, pool: &PgPool) -> Result<Vec<Contact>> {
        self.get_contacts_by_type(pool, ContactType::Employee).await
    }
}

impl Default for ContactService {
    fn default() -> Self {
        Self::new()
    }
}
