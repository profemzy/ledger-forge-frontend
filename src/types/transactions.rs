use chrono::{NaiveDate, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Draft,
    Posted,
    Void,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JournalType {
    General,
    Sales,
    #[serde(rename = "Cash Receipts")]
    CashReceipts,
    Purchases,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub reference_number: Option<String>,
    pub contact_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub journal_type: Option<JournalType>,
    pub status: TransactionStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionLineItem {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionWithLineItems {
    #[serde(flatten)]
    pub transaction: Transaction,
    pub line_items: Vec<TransactionLineItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateLineItemRequest {
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateTransactionRequest {
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub reference_number: Option<String>,
    pub contact_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub journal_type: Option<JournalType>,
    pub line_items: Vec<CreateLineItemRequest>,
}

