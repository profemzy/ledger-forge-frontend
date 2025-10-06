use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

use crate::models::{
    Payment, PaymentApplication, CreatePaymentRequest, PaymentApplicationRequest,
    CreateBillPaymentRequest, BillPayment, BillPaymentApplication
};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct PaymentService {
    cache: CacheService,
}

impl PaymentService {
    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Create a new customer payment with optional invoice applications
    pub async fn create_payment(&self, pool: &PgPool, req: CreatePaymentRequest) -> Result<Payment> {
        // Validate request
        req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Start a transaction for atomic payment creation
        let mut tx = pool.begin().await?;

        // Validate that total applications don't exceed payment amount
        let total_applications: Decimal = req.applications.iter()
            .map(|app| app.amount_applied)
            .sum();

        if total_applications > req.amount {
            return Err(AppError::ValidationError(
                "Total application amount cannot exceed payment amount".to_string()
            ));
        }

        // Create payment record
        let payment = sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payments
                (id, payment_number, customer_id, payment_date, amount, unapplied_amount,
                 payment_method, reference_number, deposit_to_account_id, memo, company_id,
                 created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), NOW())
            RETURNING id, quickbooks_id, payment_number, customer_id, payment_date, amount,
                     unapplied_amount, payment_method, reference_number, deposit_to_account_id,
                     memo, company_id, transaction_id, created_by, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.payment_number)
        .bind(req.customer_id)
        .bind(req.payment_date)
        .bind(req.amount)
        .bind(req.amount - total_applications) // Calculate unapplied amount
        .bind(&req.payment_method)
        .bind(&req.reference_number)
        .bind(req.deposit_to_account_id)
        .bind(&req.memo)
        .bind(req.company_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create payment applications if any
        if !req.applications.is_empty() {
            for application in req.applications {
                // Validate that invoice exists and belongs to the same customer
                let invoice_exists = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM invoices WHERE id = $1 AND customer_id = $2)"
                )
                .bind(application.invoice_id)
                .bind(req.customer_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                if !invoice_exists {
                    return Err(AppError::ValidationError(
                        format!("Invoice {} does not exist or does not belong to customer", application.invoice_id)
                    ));
                }

                // Create payment application
                sqlx::query_as::<_, PaymentApplication>(
                    r#"
                    INSERT INTO payment_applications
                        (id, payment_id, invoice_id, amount_applied, created_at)
                    VALUES ($1, $2, $3, $4, NOW())
                    RETURNING id, payment_id, invoice_id, amount_applied, created_at
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(payment.id)
                .bind(application.invoice_id)
                .bind(application.amount_applied)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                // Update invoice balance
                sqlx::query(
                    r#"
                    UPDATE invoices
                    SET balance = balance - $1,
                        status = CASE
                            WHEN balance - $1 <= 0 THEN 'paid'
                            WHEN balance - $1 < total_amount THEN 'partial'
                            ELSE status
                        END,
                        updated_at = NOW()
                    WHERE id = $2
                    "#
                )
                .bind(application.amount_applied)
                .bind(application.invoice_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            }
        }

        // Commit transaction
        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate relevant cache entries
        let _ = self.cache.delete_pattern(&format!("customer:*")).await;
        let _ = self.cache.delete_pattern(&format!("invoice:*")).await;

        Ok(payment)
    }

    /// Get payment by ID
    pub async fn get_payment_by_id(&self, pool: &PgPool, payment_id: Uuid) -> Result<Option<Payment>> {
        let cache_key = format!("payment:{}", payment_id);

        // Try cache first
        if let Ok(Some(cached_payment)) = self.cache.get::<Payment>(&cache_key).await {
            return Ok(Some(cached_payment));
        }

        // Query database
        let payment = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE id = $1"
        )
        .bind(payment_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Cache the result
        if let Some(ref p) = payment {
            self.cache.set_with_ttl(&cache_key, p, 3600).await?;
        }

        Ok(payment)
    }

    /// List payments for a customer
    pub async fn list_customer_payments(&self, pool: &PgPool, customer_id: Uuid) -> Result<Vec<Payment>> {
        let cache_key = format!("customer:payments:{}", customer_id);

        // Try cache first
        if let Ok(Some(cached_payments)) = self.cache.get::<Vec<Payment>>(&cache_key).await {
            return Ok(cached_payments);
        }

        // Query database
        let payments = sqlx::query_as::<_, Payment>(
            r#"
            SELECT * FROM payments
            WHERE customer_id = $1
            ORDER BY payment_date DESC, created_at DESC
            "#
        )
        .bind(customer_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Cache the result
        self.cache.set_with_ttl(&cache_key, &payments, 1800).await?;

        Ok(payments)
    }

    /// List all payments
    pub async fn list_payments(&self, pool: &PgPool, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            r#"
            SELECT * FROM payments
            ORDER BY payment_date DESC, created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit.unwrap_or(100))
        .bind(offset.unwrap_or(0))
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(payments)
    }

    /// Get unapplied payments (payments with remaining balance)
    pub async fn list_unapplied_payments(&self, pool: &PgPool, customer_id: Option<Uuid>) -> Result<Vec<Payment>> {
        let query = if customer_id.is_some() {
            r#"
            SELECT * FROM payments
            WHERE unapplied_amount > 0 AND customer_id = $1
            ORDER BY payment_date ASC
            "#
        } else {
            r#"
            SELECT * FROM payments
            WHERE unapplied_amount > 0
            ORDER BY payment_date ASC
            "#
        };

        let mut query_builder = sqlx::query_as::<_, Payment>(query);

        if let Some(cid) = customer_id {
            query_builder = query_builder.bind(cid);
        }

        let payments = query_builder
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(payments)
    }

    /// Apply payment to invoices
    pub async fn apply_payment_to_invoices(
        &self,
        pool: &PgPool,
        payment_id: Uuid,
        applications: Vec<PaymentApplicationRequest>
    ) -> Result<()> {
        // Start transaction
        let mut tx = pool.begin().await?;

        // Get payment details
        let payment = sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE id = $1 FOR UPDATE"
        )
        .bind(payment_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Validate total applications don't exceed unapplied amount
        let total_applications: Decimal = applications.iter()
            .map(|app| app.amount_applied)
            .sum();

        if total_applications > payment.unapplied_amount.unwrap_or(Decimal::ZERO) {
            return Err(AppError::ValidationError(
                "Total application amount cannot exceed unapplied amount".to_string()
            ));
        }

        // Process each application
        for application in applications {
            // Validate invoice exists and belongs to the same customer
            let invoice_exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM invoices WHERE id = $1 AND customer_id = $2)"
            )
            .bind(application.invoice_id)
            .bind(payment.customer_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            if !invoice_exists {
                return Err(AppError::ValidationError(
                    format!("Invoice {} does not exist or does not belong to customer", application.invoice_id)
                ));
            }

            // Create payment application
            sqlx::query_as::<_, PaymentApplication>(
                r#"
                INSERT INTO payment_applications
                    (id, payment_id, invoice_id, amount_applied, created_at)
                VALUES ($1, $2, $3, $4, NOW())
                RETURNING id, payment_id, invoice_id, amount_applied, created_at
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(payment_id)
            .bind(application.invoice_id)
            .bind(application.amount_applied)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            // Update invoice balance
            sqlx::query(
                r#"
                UPDATE invoices
                SET balance = balance - $1,
                    status = CASE
                        WHEN balance - $1 <= 0 THEN 'paid'
                        WHEN balance - $1 < total_amount THEN 'partial'
                        ELSE status
                    END,
                    updated_at = NOW()
                WHERE id = $2
                "#
            )
            .bind(application.amount_applied)
            .bind(application.invoice_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        // Update payment unapplied amount
        let new_unapplied = payment.unapplied_amount.unwrap_or(Decimal::ZERO) - total_applications;
        sqlx::query(
            "UPDATE payments SET unapplied_amount = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(new_unapplied)
        .bind(payment_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Commit transaction
        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate cache
        let _ = self.cache.delete_pattern(&format!("payment:*")).await;
        let _ = self.cache.delete_pattern(&format!("customer:payments:{}", payment.customer_id)).await;
        let _ = self.cache.delete_pattern(&format!("invoice:*")).await;

        Ok(())
    }

    /// Get payments applied to a specific invoice
    pub async fn get_invoice_payments(&self, pool: &PgPool, invoice_id: Uuid) -> Result<Vec<Payment>> {
        let payments = sqlx::query_as::<_, Payment>(
            r#"
            SELECT p.* FROM payments p
            INNER JOIN payment_applications pa ON p.id = pa.payment_id
            WHERE pa.invoice_id = $1
            ORDER BY p.payment_date DESC, p.created_at DESC
            "#
        )
        .bind(invoice_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(payments)
    }

    /// Create a new vendor bill payment
    pub async fn create_bill_payment(&self, pool: &PgPool, req: CreateBillPaymentRequest) -> Result<BillPayment> {
        // Validate request
        req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Start a transaction
        let mut tx = pool.begin().await?;

        // Validate that total applications don't exceed payment amount
        let total_applications: Decimal = req.applications.iter()
            .map(|app| app.amount_applied)
            .sum();

        if total_applications > req.amount {
            return Err(AppError::ValidationError(
                "Total application amount cannot exceed payment amount".to_string()
            ));
        }

        // Create bill payment record
        let bill_payment = sqlx::query_as::<_, BillPayment>(
            r#"
            INSERT INTO bill_payments
                (id, payment_number, vendor_id, payment_date, amount, payment_method,
                 reference_number, bank_account_id, memo, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
            RETURNING id, quickbooks_id, payment_number, vendor_id, payment_date, amount,
                     payment_method, reference_number, bank_account_id, memo, company_id,
                     transaction_id, created_by, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&req.payment_number)
        .bind(req.vendor_id)
        .bind(req.payment_date)
        .bind(req.amount)
        .bind(&req.payment_method)
        .bind(&req.reference_number)
        .bind(req.bank_account_id)
        .bind(&req.memo)
        .bind(req.company_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Create bill payment applications if any
        if !req.applications.is_empty() {
            for application in req.applications {
                // Validate that bill exists and belongs to the same vendor
                let bill_exists = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM bills WHERE id = $1 AND vendor_id = $2)"
                )
                .bind(application.bill_id)
                .bind(req.vendor_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                if !bill_exists {
                    return Err(AppError::ValidationError(
                        format!("Bill {} does not exist or does not belong to vendor", application.bill_id)
                    ));
                }

                // Create bill payment application
                sqlx::query_as::<_, BillPaymentApplication>(
                    r#"
                    INSERT INTO bill_payment_applications
                        (id, bill_payment_id, bill_id, amount_applied, created_at)
                    VALUES ($1, $2, $3, $4, NOW())
                    RETURNING id, bill_payment_id, bill_id, amount_applied, created_at
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(bill_payment.id)
                .bind(application.bill_id)
                .bind(application.amount_applied)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                // Update bill balance
                sqlx::query(
                    r#"
                    UPDATE bills
                    SET balance = balance - $1,
                        status = CASE
                            WHEN balance - $1 <= 0 THEN 'paid'
                            WHEN balance - $1 < total_amount THEN 'partial'
                            ELSE status
                        END,
                        updated_at = NOW()
                    WHERE id = $2
                    "#
                )
                .bind(application.amount_applied)
                .bind(application.bill_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            }
        }

        // Commit transaction
        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Invalidate relevant cache entries
        let _ = self.cache.delete_pattern(&format!("vendor:*")).await;
        let _ = self.cache.delete_pattern(&format!("bill:*")).await;

        Ok(bill_payment)
    }
}