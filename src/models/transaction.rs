#![allow(dead_code)]
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "PascalCase")]
pub enum JournalType {
    General,
    Sales,
    #[serde(rename = "Cash Receipts")]
    #[sqlx(rename = "Cash Receipts")]
    CashReceipts,
    Purchases,
}

impl std::fmt::Display for JournalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalType::General => write!(f, "General"),
            JournalType::Sales => write!(f, "Sales"),
            JournalType::CashReceipts => write!(f, "Cash Receipts"),
            JournalType::Purchases => write!(f, "Purchases"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TransactionStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "void")]
    Void,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Draft => write!(f, "draft"),
            TransactionStatus::Posted => write!(f, "posted"),
            TransactionStatus::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateTransactionRequest {
    #[schema(example = "2025-10-04")]
    pub transaction_date: NaiveDate,
    #[schema(example = "Service rendered to customer")]
    pub description: Option<String>,
    #[schema(example = "INV-001")]
    pub reference_number: Option<String>,
    pub contact_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub journal_type: Option<JournalType>,

    #[validate(length(min = 2))]
    #[validate(custom(function = "validate_balanced_entry"))]
    pub line_items: Vec<CreateLineItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateLineItemRequest {
    pub account_id: Uuid,
    #[schema(example = "Cash received")]
    pub description: Option<String>,

    #[schema(example = 500.00)]
    pub debit_amount: Option<Decimal>,

    #[schema(example = 500.00)]
    pub credit_amount: Option<Decimal>,
}

// Validation function for balanced double-entry
fn validate_balanced_entry(line_items: &[CreateLineItemRequest]) -> Result<(), validator::ValidationError> {
    let total_debits: Decimal = line_items
        .iter()
        .filter_map(|item| item.debit_amount)
        .sum();

    let total_credits: Decimal = line_items
        .iter()
        .filter_map(|item| item.credit_amount)
        .sum();

    if total_debits != total_credits {
        return Err(validator::ValidationError::new(
            "unbalanced_entry"
        ));
    }

    // Ensure each line item has either debit or credit, not both
    for item in line_items {
        let has_debit = item.debit_amount.unwrap_or(Decimal::ZERO) > Decimal::ZERO;
        let has_credit = item.credit_amount.unwrap_or(Decimal::ZERO) > Decimal::ZERO;

        if has_debit && has_credit {
            return Err(validator::ValidationError::new(
                "both_debit_and_credit"
            ));
        }

        if !has_debit && !has_credit {
            return Err(validator::ValidationError::new(
                "neither_debit_nor_credit"
            ));
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionWithLineItems {
    #[serde(flatten)]
    pub transaction: Transaction,
    pub line_items: Vec<TransactionLineItem>,
}
