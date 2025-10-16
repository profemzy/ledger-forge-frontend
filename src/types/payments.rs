use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentApplication {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub invoice_id: Uuid,
    pub amount_applied: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentApplicationRequest {
    pub invoice_id: Uuid,
    pub amount_applied: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentRequest {
    pub payment_number: Option<String>,
    pub customer_id: Uuid,
    pub payment_date: NaiveDate,
    pub amount: Decimal,
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub deposit_to_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub applications: Vec<PaymentApplicationRequest>,
}

