use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub quickbooks_id: Option<String>,
    pub payment_number: Option<String>,
    pub customer_id: Uuid,
    pub payment_date: NaiveDate,
    pub amount: Decimal,
    pub unapplied_amount: Option<Decimal>,
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub deposit_to_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct PaymentApplication {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub invoice_id: Uuid,
    pub amount_applied: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct BillPayment {
    pub id: Uuid,
    pub quickbooks_id: Option<String>,
    pub payment_number: Option<String>,
    pub vendor_id: Uuid,
    pub payment_date: NaiveDate,
    pub amount: Decimal,
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct BillPaymentApplication {
    pub id: Uuid,
    pub bill_payment_id: Uuid,
    pub bill_id: Uuid,
    pub amount_applied: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    pub payment_number: Option<String>,
    pub customer_id: Uuid,
    pub payment_date: NaiveDate,
    #[validate(range(min = 0))]
    pub amount: Decimal,
    #[validate(length(min = 1))]
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub deposit_to_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub applications: Vec<PaymentApplicationRequest>,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct PaymentApplicationRequest {
    pub invoice_id: Uuid,
    #[validate(range(min = 0))]
    pub amount_applied: Decimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBillPaymentRequest {
    pub payment_number: Option<String>,
    pub vendor_id: Uuid,
    pub payment_date: NaiveDate,
    #[validate(range(min = 0))]
    pub amount: Decimal,
    #[validate(length(min = 1))]
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub applications: Vec<BillPaymentApplicationRequest>,
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct BillPaymentApplicationRequest {
    pub bill_id: Uuid,
    #[validate(range(min = 0))]
    pub amount_applied: Decimal,
}
