use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Partial,
    Overdue,
    Void,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InvoiceWithLineItems {
    #[serde(flatten)]
    pub invoice: Invoice,
    pub line_items: Vec<InvoiceLineItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateInvoiceLineItemRequest {
    pub line_number: i32,
    pub item_description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_percent: Option<Decimal>,
    pub tax_code: Option<String>,
    pub revenue_account_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateInvoiceRequest {
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub invoice_date: NaiveDate,
    pub due_date: NaiveDate,
    pub ship_date: Option<NaiveDate>,
    pub customer_memo: Option<String>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub company_id: Option<Uuid>,
    pub line_items: Vec<CreateInvoiceLineItemRequest>,
}

