use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

use crate::models::{
    Bill, BillLineItem, BillStatus, CreateBillRequest, BillWithLineItems
};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct BillService {
    cache: CacheService,
}

impl BillService {
    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Create a new bill with line items
    pub async fn create_bill(&self, pool: &PgPool, req: CreateBillRequest) -> Result<Bill> {
        // Validate request
        req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Start a transaction for atomic bill creation
        let mut tx = pool.begin().await?;

        // Calculate total amount from line items
        let total_amount: Decimal = req.line_items.iter()
            .map(|item| item.amount)
            .sum();

        // Create bill record
        let bill = sqlx::query_as::<_, Bill>(
            r#"
            INSERT INTO bills
                (id, bill_number, vendor_id, bill_date, due_date, total_amount, balance,
                 status, memo, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
            RETURNING id, quickbooks_id, bill_number, vendor_id, bill_date, due_date,
                     total_amount, balance, status, memo, company_id, transaction_id,
                     created_by, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.bill_number)
        .bind(req.vendor_id)
        .bind(req.bill_date)
        .bind(req.due_date)
        .bind(total_amount)
        .bind(total_amount) // Initial balance equals total
        .bind(BillStatus::Open.to_string())
        .bind(&req.memo)
        .bind(req.company_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create line items
        for line_item in req.line_items {
            sqlx::query(
                r#"
                INSERT INTO bill_line_items
                    (id, bill_id, line_number, description, amount, expense_account_id,
                     billable, customer_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(bill.id)
            .bind(line_item.line_number)
            .bind(&line_item.description)
            .bind(line_item.amount)
            .bind(line_item.expense_account_id)
            .bind(line_item.billable)
            .bind(line_item.customer_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        // Commit transaction
        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate relevant cache entries
        let _ = self.cache.delete_pattern(&format!("vendor:*")).await;
        let _ = self.cache.delete_pattern(&format!("bill:*")).await;

        Ok(bill)
    }

    /// Get bill by ID with line items
    pub async fn get_bill_by_id(&self, pool: &PgPool, bill_id: Uuid) -> Result<Option<BillWithLineItems>> {
        let cache_key = format!("bill:{}", bill_id);

        // Try cache first
        if let Ok(Some(cached_bill)) = self.cache.get::<BillWithLineItems>(&cache_key).await {
            return Ok(Some(cached_bill));
        }

        // Query bill
        let bill = sqlx::query_as::<_, Bill>(
            "SELECT * FROM bills WHERE id = $1"
        )
        .bind(bill_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(bill) = bill {
            // Query line items
            let line_items = sqlx::query_as::<_, BillLineItem>(
                "SELECT * FROM bill_line_items WHERE bill_id = $1 ORDER BY line_number"
            )
            .bind(bill_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            let bill_with_items = BillWithLineItems {
                bill,
                line_items,
            };

            // Cache the result
            self.cache.set_with_ttl(&cache_key, &bill_with_items, 3600).await?;

            Ok(Some(bill_with_items))
        } else {
            Ok(None)
        }
    }

    /// List bills with optional filters
    pub async fn list_bills(
        &self,
        pool: &PgPool,
        vendor_id: Option<Uuid>,
        status: Option<BillStatus>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Bill>> {
        let mut query = String::from("SELECT * FROM bills WHERE 1=1");
        let mut params: Vec<String> = Vec::new();

        if vendor_id.is_some() {
            params.push(format!("vendor_id = '{}'", vendor_id.unwrap()));
        }

        if let Some(s) = status {
            params.push(format!("status = '{}'", s.to_string()));
        }

        if !params.is_empty() {
            query.push_str(&format!(" AND {}", params.join(" AND ")));
        }

        query.push_str(" ORDER BY bill_date DESC, created_at DESC");
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit.unwrap_or(100), offset.unwrap_or(0)));

        let bills = sqlx::query_as::<_, Bill>(&query)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(bills)
    }

    /// Get bills for a specific vendor
    pub async fn get_vendor_bills(&self, pool: &PgPool, vendor_id: Uuid) -> Result<Vec<Bill>> {
        let cache_key = format!("vendor:bills:{}", vendor_id);

        // Try cache first
        if let Ok(Some(cached_bills)) = self.cache.get::<Vec<Bill>>(&cache_key).await {
            return Ok(cached_bills);
        }

        let bills = sqlx::query_as::<_, Bill>(
            r#"
            SELECT * FROM bills
            WHERE vendor_id = $1
            ORDER BY bill_date DESC, created_at DESC
            "#
        )
        .bind(vendor_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Cache the result
        self.cache.set_with_ttl(&cache_key, &bills, 1800).await?;

        Ok(bills)
    }

    /// Update bill status
    pub async fn update_bill_status(
        &self,
        pool: &PgPool,
        bill_id: Uuid,
        new_status: BillStatus,
    ) -> Result<Bill> {
        let bill = sqlx::query_as::<_, Bill>(
            r#"
            UPDATE bills
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, quickbooks_id, bill_number, vendor_id, bill_date, due_date,
                     total_amount, balance, status, memo, company_id, transaction_id,
                     created_by, created_at, updated_at
            "#
        )
        .bind(new_status.to_string())
        .bind(bill_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate cache
        let _ = self.cache.delete_pattern(&format!("bill:*")).await;
        let _ = self.cache.delete_pattern(&format!("vendor:bills:{}", bill.vendor_id)).await;

        Ok(bill)
    }

    /// Delete a bill (only if no payments have been applied)
    pub async fn delete_bill(&self, pool: &PgPool, bill_id: Uuid) -> Result<()> {
        // Check if bill has any payments
        let payment_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM bill_payment_applications WHERE bill_id = $1"
        )
        .bind(bill_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if payment_count > 0 {
            return Err(AppError::ValidationError(
                "Cannot delete bill with payments applied".to_string()
            ));
        }

        // Get vendor_id before deletion for cache invalidation
        let vendor_id: Option<Uuid> = sqlx::query_scalar(
            "SELECT vendor_id FROM bills WHERE id = $1"
        )
        .bind(bill_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Delete bill (line items will be cascade deleted)
        sqlx::query("DELETE FROM bills WHERE id = $1")
            .bind(bill_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate cache
        let _ = self.cache.delete_pattern(&format!("bill:*")).await;
        if let Some(vid) = vendor_id {
            let _ = self.cache.delete_pattern(&format!("vendor:bills:{}", vid)).await;
        }

        Ok(())
    }

    /// Get overdue bills
    pub async fn get_overdue_bills(&self, pool: &PgPool) -> Result<Vec<Bill>> {
        let bills = sqlx::query_as::<_, Bill>(
            r#"
            SELECT * FROM bills
            WHERE status IN ('open', 'partial')
              AND due_date < CURRENT_DATE
            ORDER BY due_date ASC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(bills)
    }
}

impl ToString for BillStatus {
    fn to_string(&self) -> String {
        match self {
            BillStatus::Open => "open".to_string(),
            BillStatus::Paid => "paid".to_string(),
            BillStatus::Partial => "partial".to_string(),
            BillStatus::Void => "void".to_string(),
        }
    }
}