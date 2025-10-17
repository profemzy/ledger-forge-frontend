use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BillPayment {
    pub id: Uuid,
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BillPaymentApplicationRequest { pub bill_id: Uuid, pub amount_applied: Decimal }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateBillPaymentRequest {
    pub payment_number: Option<String>,
    pub vendor_id: Uuid,
    pub payment_date: NaiveDate,
    pub amount: Decimal,
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub applications: Vec<BillPaymentApplicationRequest>,
}

