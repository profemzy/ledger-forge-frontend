#![allow(dead_code)]
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Invoice {
    pub id: Uuid,
    pub quickbooks_id: Option<String>,
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub ship_date: Option<NaiveDate>,
    pub tracking_number: Option<String>,
    pub total_amount: Decimal,
    pub balance: Decimal,
    pub status: InvoiceStatus,
    pub customer_memo: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub company_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum InvoiceStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "void")]
    Void,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct InvoiceLineItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub line_number: i32,
    pub item_description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub amount: Decimal,
    pub discount_percent: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub tax_code: Option<String>,
    pub revenue_account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateInvoiceRequest {
    #[validate(length(min = 1))]
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub ship_date: Option<NaiveDate>,
    pub customer_memo: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub company_id: Option<Uuid>,

    #[validate(length(min = 1))]
    pub line_items: Vec<CreateInvoiceLineItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInvoiceLineItemRequest {
    pub line_number: i32,
    pub item_description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_percent: Option<Decimal>,
    pub tax_code: Option<String>,
    pub revenue_account_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct InvoiceWithLineItems {
    #[serde(flatten)]
    pub invoice: Invoice,
    pub line_items: Vec<InvoiceLineItem>,
}
