use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

use crate::models::{
    Transaction, TransactionLineItem, TransactionStatus, TransactionWithLineItems,
    CreateTransactionRequest,
};
use crate::utils::{AppError, Result};

#[derive(Clone)]
pub struct TransactionService;

impl TransactionService {
    pub fn new() -> Self {
        Self
    }

    /// Create a new transaction with line items
    pub async fn create_transaction(
        &self,
        pool: &PgPool,
        req: CreateTransactionRequest,
        created_by: Option<Uuid>,
    ) -> Result<TransactionWithLineItems> {
        // Validate request (includes double-entry balance validation)
        req.validate()?;

        // Validate all account IDs exist
        for line_item in &req.line_items {
            self.validate_account_exists(pool, line_item.account_id).await?;
        }

        // Start a transaction
        let mut tx = pool.begin().await?;

        // Create transaction header
        let transaction_id = Uuid::new_v4();
        let transaction = sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions
                (id, transaction_date, description, reference_number, contact_id, company_id,
                 journal_type, status, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
            RETURNING id, transaction_date, description, reference_number, contact_id, company_id,
                      journal_type, status, created_by, created_at, updated_at
            "#,
        )
        .bind(transaction_id)
        .bind(req.transaction_date)
        .bind(&req.description)
        .bind(&req.reference_number)
        .bind(req.contact_id)
        .bind(req.company_id)
        .bind(req.journal_type.as_ref().map(|jt| jt.to_string()))
        .bind(TransactionStatus::Draft.to_string())
        .bind(created_by)
        .fetch_one(&mut *tx)
        .await?;

        // Create line items
        let mut line_items = Vec::new();
        for line_item in &req.line_items {
            let debit = line_item.debit_amount.unwrap_or(Decimal::ZERO);
            let credit = line_item.credit_amount.unwrap_or(Decimal::ZERO);

            let item = sqlx::query_as::<_, TransactionLineItem>(
                r#"
                INSERT INTO transaction_line_items
                    (id, transaction_id, account_id, description, debit_amount, credit_amount, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
                RETURNING id, transaction_id, account_id, description, debit_amount, credit_amount, created_at, updated_at
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(transaction_id)
            .bind(line_item.account_id)
            .bind(&line_item.description)
            .bind(debit)
            .bind(credit)
            .fetch_one(&mut *tx)
            .await?;

            line_items.push(item);
        }

        // Commit transaction
        tx.commit().await?;

        Ok(TransactionWithLineItems {
            transaction,
            line_items,
        })
    }

    /// Get transaction by ID with line items
    pub async fn get_transaction_by_id(&self, pool: &PgPool, id: Uuid) -> Result<TransactionWithLineItems> {
        // Get transaction header
        let transaction = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT id, transaction_date, description, reference_number, contact_id, company_id,
                   journal_type, status, created_by, created_at, updated_at
            FROM transactions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Transaction with id {} not found", id)))?;

        // Get line items
        let line_items = sqlx::query_as::<_, TransactionLineItem>(
            r#"
            SELECT id, transaction_id, account_id, description, debit_amount, credit_amount, created_at, updated_at
            FROM transaction_line_items
            WHERE transaction_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        Ok(TransactionWithLineItems {
            transaction,
            line_items,
        })
    }

    /// List transactions with optional filtering
    pub async fn list_transactions(
        &self,
        pool: &PgPool,
        status: Option<TransactionStatus>,
        company_id: Option<Uuid>,
        limit: Option<i64>,
    ) -> Result<Vec<Transaction>> {
        let mut query = String::from(
            r#"
            SELECT id, transaction_date, description, reference_number, contact_id, company_id,
                   journal_type, status, created_by, created_at, updated_at
            FROM transactions
            WHERE 1=1
            "#
        );

        // Add filters
        if status.is_some() {
            query.push_str(" AND status = $1");
        }

        if company_id.is_some() {
            if status.is_some() {
                query.push_str(" AND company_id = $2");
            } else {
                query.push_str(" AND company_id = $1");
            }
        }

        query.push_str(" ORDER BY transaction_date DESC, created_at DESC");

        // Add limit
        if limit.is_some() {
            if status.is_some() && company_id.is_some() {
                query.push_str(" LIMIT $3");
            } else if status.is_some() || company_id.is_some() {
                query.push_str(" LIMIT $2");
            } else {
                query.push_str(" LIMIT $1");
            }
        }

        // Build and execute query
        let mut sql_query = sqlx::query_as::<_, Transaction>(&query);

        if let Some(s) = status {
            sql_query = sql_query.bind(s.to_string());
        }

        if let Some(cid) = company_id {
            sql_query = sql_query.bind(cid);
        }

        if let Some(_lim) = limit {
            sql_query = sql_query.bind(_lim);
        }

        let transactions = sql_query.fetch_all(pool).await?;

        Ok(transactions)
    }

    /// Update transaction status
    pub async fn update_transaction_status(
        &self,
        pool: &PgPool,
        id: Uuid,
        new_status: TransactionStatus,
    ) -> Result<Transaction> {
        // Get current transaction
        let current = self.get_transaction_by_id(pool, id).await?;

        // Validate status transitions
        match (&current.transaction.status, &new_status) {
            (TransactionStatus::Draft, TransactionStatus::Posted) => {
                // Draft -> Posted is allowed
            }
            (TransactionStatus::Posted, TransactionStatus::Void) => {
                // Posted -> Void is allowed
            }
            (TransactionStatus::Draft, TransactionStatus::Void) => {
                // Draft -> Void is allowed
            }
            (TransactionStatus::Void, _) => {
                return Err(AppError::BadRequest("Cannot modify a voided transaction".to_string()));
            }
            (TransactionStatus::Posted, TransactionStatus::Draft) => {
                return Err(AppError::BadRequest("Cannot revert posted transaction to draft".to_string()));
            }
            _ => {
                return Err(AppError::BadRequest(format!(
                    "Invalid status transition from {:?} to {:?}",
                    current.transaction.status, new_status
                )));
            }
        }

        // Update status
        let transaction = sqlx::query_as::<_, Transaction>(
            r#"
            UPDATE transactions
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, transaction_date, description, reference_number, contact_id, company_id,
                      journal_type, status, created_by, created_at, updated_at
            "#,
        )
        .bind(new_status.to_string())
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(transaction)
    }

    /// Delete transaction (only if in draft status)
    pub async fn delete_transaction(&self, pool: &PgPool, id: Uuid) -> Result<()> {
        // Get transaction
        let transaction_with_items = self.get_transaction_by_id(pool, id).await?;

        // Check if transaction is in draft status
        if transaction_with_items.transaction.status != TransactionStatus::Draft {
            return Err(AppError::BadRequest(
                "Can only delete transactions in draft status. Use void for posted transactions.".to_string()
            ));
        }

        // Start a transaction
        let mut tx = pool.begin().await?;

        // Delete line items first
        sqlx::query("DELETE FROM transaction_line_items WHERE transaction_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // Delete transaction
        sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // Commit
        tx.commit().await?;

        Ok(())
    }

    /// Validate that an account exists
    async fn validate_account_exists(&self, pool: &PgPool, account_id: Uuid) -> Result<()> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM chart_of_accounts WHERE id = $1)"
        )
        .bind(account_id)
        .fetch_one(pool)
        .await?;

        if !exists {
            return Err(AppError::NotFound(format!("Account with id {} not found", account_id)));
        }

        Ok(())
    }

    /// Get account balance (sum of all posted transactions)
    pub async fn get_account_balance(&self, pool: &PgPool, account_id: Uuid) -> Result<Decimal> {
        self.validate_account_exists(pool, account_id).await?;

        let balance = sqlx::query_scalar::<_, Option<Decimal>>(
            r#"
            SELECT COALESCE(SUM(debit_amount - credit_amount), 0)
            FROM transaction_line_items tli
            INNER JOIN transactions t ON tli.transaction_id = t.id
            WHERE tli.account_id = $1 AND t.status = 'posted'
            "#,
        )
        .bind(account_id)
        .fetch_one(pool)
        .await?
        .unwrap_or(Decimal::ZERO);

        Ok(balance)
    }
}

impl Default for TransactionService {
    fn default() -> Self {
        Self::new()
    }
}
