#![allow(dead_code)]
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct Bill {
    pub id: Uuid,
    pub quickbooks_id: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum BillStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "void")]
    Void,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateBillRequest {
    pub bill_number: Option<String>,
    pub vendor_id: Uuid,
    pub bill_date: NaiveDate,
    pub due_date: NaiveDate,
    pub memo: Option<String>,
    pub company_id: Option<Uuid>,

    #[validate(length(min = 1))]
    pub line_items: Vec<CreateBillLineItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateBillLineItemRequest {
    pub line_number: i32,
    pub description: Option<String>,
    pub amount: Decimal,
    pub expense_account_id: Uuid,
    pub billable: Option<bool>,
    pub customer_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BillWithLineItems {
    #[serde(flatten)]
    pub bill: Bill,
    pub line_items: Vec<BillLineItem>,
}
