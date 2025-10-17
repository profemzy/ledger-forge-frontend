use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BillStatus { Open, Paid, Partial, Void }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bill {
    pub id: Uuid,
    pub bill_number: Option<String>,
    pub vendor_id: Uuid,
    pub bill_date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
    pub balance: Decimal,
    pub status: BillStatus,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BillLineItem {
    pub id: Uuid,
    pub bill_id: Uuid,
    pub line_number: i32,
    pub description: Option<String>,
    pub amount: Decimal,
    pub expense_account_id: Uuid,
    pub billable: Option<bool>,
    pub customer_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateBillLineItemRequest {
    pub line_number: i32,
    pub description: Option<String>,
    pub amount: Decimal,
    pub expense_account_id: Uuid,
    pub billable: Option<bool>,
    pub customer_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateBillRequest {
    pub bill_number: Option<String>,
    pub vendor_id: Uuid,
    pub bill_date: NaiveDate,
    pub due_date: NaiveDate,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,
    pub line_items: Vec<CreateBillLineItemRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BillWithLineItems {
    #[serde(flatten)]
    pub bill: Bill,
    pub line_items: Vec<BillLineItem>,
}
