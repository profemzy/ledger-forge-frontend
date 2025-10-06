use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

use crate::models::{
    Invoice, InvoiceLineItem, InvoiceStatus, CreateInvoiceRequest,
    CreateInvoiceLineItemRequest, InvoiceWithLineItems
};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct InvoiceService {
    cache: CacheService,
}

impl InvoiceService {
    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Create a new invoice with line items
    pub async fn create_invoice(&self, pool: &PgPool, req: CreateInvoiceRequest) -> Result<InvoiceWithLineItems> {
        // Validate request
        req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Start a transaction for atomic invoice creation
        let mut tx = pool.begin().await?;

        // Calculate total amount
        let total_amount = self.calculate_invoice_total(&req.line_items)?;

        // Create invoice header
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoices
                (id, invoice_number, customer_id, invoice_date, due_date, ship_date,
                 total_amount, balance, status, customer_memo, billing_address,
                 shipping_address, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
            RETURNING id, quickbooks_id, invoice_number, customer_id, invoice_date, due_date,
                     ship_date, tracking_number, total_amount, balance, status, customer_memo,
                     billing_address, shipping_address, company_id, transaction_id,
                     created_by, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.invoice_number)
        .bind(req.customer_id)
        .bind(req.invoice_date)
        .bind(req.due_date)
        .bind(req.ship_date)
        .bind(total_amount)
        .bind(total_amount) // Initially, balance equals total amount
        .bind(InvoiceStatus::Draft.to_string())
        .bind(&req.customer_memo)
        .bind(&req.billing_address)
        .bind(&req.shipping_address)
        .bind(req.company_id)
        .fetch_one(&mut *tx)
        .await?;

        // Create line items
        let mut line_items = Vec::new();
        for (index, line_item_req) in req.line_items.iter().enumerate() {
            let line_amount = self.calculate_line_item_amount(line_item_req)?;

            let line_item = sqlx::query_as::<_, InvoiceLineItem>(
                r#"
                INSERT INTO invoice_line_items
                    (id, invoice_id, line_number, item_description, quantity, unit_price,
                     amount, discount_percent, discount_amount, tax_code, revenue_account_id,
                     created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW())
                RETURNING id, invoice_id, line_number, item_description, quantity, unit_price,
                         amount, discount_percent, discount_amount, tax_code, revenue_account_id,
                         created_at, updated_at
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(invoice.id)
            .bind(index as i32 + 1) // Line numbers start at 1
            .bind(&line_item_req.item_description)
            .bind(line_item_req.quantity)
            .bind(line_item_req.unit_price)
            .bind(line_amount)
            .bind(line_item_req.discount_percent)
            .bind(None::<rust_decimal::Decimal>) // discount_amount calculated in DB
            .bind(&line_item_req.tax_code)
            .bind(line_item_req.revenue_account_id)
            .fetch_one(&mut *tx)
            .await?;

            line_items.push(line_item);
        }

        // Commit transaction
        tx.commit().await?;

        // Invalidate cache
        self.invalidate_invoice_cache(&invoice.id).await;

        Ok(InvoiceWithLineItems {
            invoice,
            line_items,
        })
    }

    /// Get invoice by ID with line items
    pub async fn get_invoice(&self, pool: &PgPool, invoice_id: Uuid) -> Result<Option<InvoiceWithLineItems>> {
        // Check cache first
        let cache_key = format!("invoice:{}", invoice_id);
        if let Ok(Some(cached)) = self.cache.get::<InvoiceWithLineItems>(&cache_key).await {
            return Ok(Some(cached));
        }

        // Get invoice header
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE id = $1"
        )
        .bind(invoice_id)
        .fetch_optional(pool)
        .await?;

        let Some(invoice) = invoice else {
            return Ok(None);
        };

        // Get line items
        let line_items = sqlx::query_as::<_, InvoiceLineItem>(
            "SELECT * FROM invoice_line_items WHERE invoice_id = $1 ORDER BY line_number"
        )
        .bind(invoice_id)
        .fetch_all(pool)
        .await?;

        let invoice_with_items = InvoiceWithLineItems {
            invoice,
            line_items,
        };

        // Cache the result
        let _ = self.cache.set_with_ttl(&cache_key, &invoice_with_items, 600).await; // 10 minutes

        Ok(Some(invoice_with_items))
    }

    /// List invoices with optional filtering
    pub async fn list_invoices(
        &self,
        pool: &PgPool,
        customer_id: Option<Uuid>,
        status: Option<InvoiceStatus>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Invoice>> {
        match (customer_id, status, limit, offset) {
            (Some(customer_id), Some(status), Some(limit), Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 AND status = $2 ORDER BY invoice_date DESC, created_at DESC LIMIT $3 OFFSET $4"
                )
                .bind(customer_id)
                .bind(status.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), Some(status), Some(limit), None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 AND status = $2 ORDER BY invoice_date DESC, created_at DESC LIMIT $3"
                )
                .bind(customer_id)
                .bind(status.to_string())
                .bind(limit)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), Some(status), None, Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 AND status = $2 ORDER BY invoice_date DESC, created_at DESC OFFSET $3"
                )
                .bind(customer_id)
                .bind(status.to_string())
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), Some(status), None, None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 AND status = $2 ORDER BY invoice_date DESC, created_at DESC"
                )
                .bind(customer_id)
                .bind(status.to_string())
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), None, Some(limit), Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 ORDER BY invoice_date DESC, created_at DESC LIMIT $2 OFFSET $3"
                )
                .bind(customer_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), None, Some(limit), None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 ORDER BY invoice_date DESC, created_at DESC LIMIT $2"
                )
                .bind(customer_id)
                .bind(limit)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), None, None, Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 ORDER BY invoice_date DESC, created_at DESC OFFSET $2"
                )
                .bind(customer_id)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (Some(customer_id), None, None, None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE customer_id = $1 ORDER BY invoice_date DESC, created_at DESC"
                )
                .bind(customer_id)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, Some(status), Some(limit), Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE status = $1 ORDER BY invoice_date DESC, created_at DESC LIMIT $2 OFFSET $3"
                )
                .bind(status.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, Some(status), Some(limit), None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE status = $1 ORDER BY invoice_date DESC, created_at DESC LIMIT $2"
                )
                .bind(status.to_string())
                .bind(limit)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, Some(status), None, Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE status = $1 ORDER BY invoice_date DESC, created_at DESC OFFSET $2"
                )
                .bind(status.to_string())
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, Some(status), None, None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices WHERE status = $1 ORDER BY invoice_date DESC, created_at DESC"
                )
                .bind(status.to_string())
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, None, Some(limit), Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices ORDER BY invoice_date DESC, created_at DESC LIMIT $1 OFFSET $2"
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, None, Some(limit), None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices ORDER BY invoice_date DESC, created_at DESC LIMIT $1"
                )
                .bind(limit)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, None, None, Some(offset)) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices ORDER BY invoice_date DESC, created_at DESC OFFSET $1"
                )
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
            (None, None, None, None) => {
                let invoices = sqlx::query_as::<_, Invoice>(
                    "SELECT * FROM invoices ORDER BY invoice_date DESC, created_at DESC"
                )
                .fetch_all(pool)
                .await?;
                Ok(invoices)
            }
        }
    }

    /// Update invoice status
    pub async fn update_invoice_status(
        &self,
        pool: &PgPool,
        invoice_id: Uuid,
        new_status: InvoiceStatus,
    ) -> Result<Invoice> {
        // Validate status transition
        let current_invoice = self.get_invoice_header(pool, invoice_id).await?
            .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

        self.validate_status_transition(&current_invoice.status, &new_status)?;

        // Update status
        let updated_invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, quickbooks_id, invoice_number, customer_id, invoice_date, due_date,
                     ship_date, tracking_number, total_amount, balance, status, customer_memo,
                     billing_address, shipping_address, company_id, transaction_id,
                     created_by, created_at, updated_at
            "#,
        )
        .bind(new_status.to_string())
        .bind(invoice_id)
        .fetch_one(pool)
        .await?;

        // If posting invoice, create transaction (disabled for now - will be implemented in payment processing)
        // TODO: Implement proper transaction creation when posting invoices
        // if new_status == InvoiceStatus::Sent && current_invoice.transaction_id.is_none() {
        //     self.create_invoice_transaction(pool, &updated_invoice).await?;
        // }

        // Invalidate cache
        self.invalidate_invoice_cache(&invoice_id).await;

        Ok(updated_invoice)
    }

    /// Get customer invoices
    pub async fn get_customer_invoices(&self, pool: &PgPool, customer_id: Uuid) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE customer_id = $1 ORDER BY invoice_date DESC, created_at DESC"
        )
        .bind(customer_id)
        .fetch_all(pool)
        .await?;

        Ok(invoices)
    }

    /// Get overdue invoices
    pub async fn get_overdue_invoices(&self, pool: &PgPool) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT * FROM invoices
            WHERE due_date < CURRENT_DATE
            AND status IN ('sent', 'partial')
            AND balance > 0
            ORDER BY due_date ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(invoices)
    }

    // Helper methods

    async fn get_invoice_header(&self, pool: &PgPool, invoice_id: Uuid) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT * FROM invoices WHERE id = $1"
        )
        .bind(invoice_id)
        .fetch_optional(pool)
        .await?;

        Ok(invoice)
    }

    fn calculate_line_item_amount(&self, line_item: &CreateInvoiceLineItemRequest) -> Result<Decimal> {
        let line_total = line_item.quantity * line_item.unit_price;

        let discount_amount = if let Some(discount_percent) = line_item.discount_percent {
            line_total * (discount_percent / Decimal::new(100, 0))
        } else {
            Decimal::ZERO
        };

        Ok(line_total - discount_amount)
    }

    fn calculate_invoice_total(&self, line_items: &[CreateInvoiceLineItemRequest]) -> Result<Decimal> {
        let mut total = Decimal::ZERO;

        for line_item in line_items {
            total += self.calculate_line_item_amount(line_item)?;
        }

        Ok(total)
    }

    fn validate_status_transition(&self, from: &InvoiceStatus, to: &InvoiceStatus) -> Result<()> {
        use InvoiceStatus::*;

        match (from, to) {
            // Allow all transitions from Draft
            (Draft, _) => Ok(()),

            // Allow transitions from Sent
            (Sent, Partial | Paid | Overdue | Void) => Ok(()),

            // Allow transitions from Partial
            (Partial, Paid | Overdue | Void) => Ok(()),

            // Allow transitions from Overdue
            (Overdue, Partial | Paid | Void) => Ok(()),

            // Allow transitions from Paid
            (Paid, Void) => Ok(()),

            // Void is final
            (Void, _) => Err(AppError::ValidationError(
                "Cannot change status from Void".to_string()
            )),

            // Disallow other transitions
            _ => Err(AppError::ValidationError(
                format!("Invalid status transition from {:?} to {:?}", from, to)
            )),
        }
    }

  
    async fn invalidate_invoice_cache(&self, invoice_id: &Uuid) {
        let cache_key = format!("invoice:{}", invoice_id);
        let _ = self.cache.delete(&cache_key).await;
    }
}