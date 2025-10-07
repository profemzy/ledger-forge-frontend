use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct TrialBalanceEntry {
    pub account_id: sqlx::types::Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TrialBalance {
    pub as_of_date: NaiveDate,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub is_balanced: bool,
    pub entries: Vec<TrialBalanceEntry>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct ProfitLossEntry {
    pub account_id: sqlx::types::Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ProfitLossStatement {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_revenue: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
    pub revenue_entries: Vec<ProfitLossEntry>,
    pub expense_entries: Vec<ProfitLossEntry>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct BalanceSheetEntry {
    pub account_id: sqlx::types::Uuid,
    pub account_code: String,
    pub account_name: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct BalanceSheet {
    pub as_of_date: NaiveDate,
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
    pub total_equity: Decimal,
    pub asset_entries: Vec<BalanceSheetEntry>,
    pub liability_entries: Vec<BalanceSheetEntry>,
    pub equity_entries: Vec<BalanceSheetEntry>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct AgingBucket {
    pub customer_id: sqlx::types::Uuid,
    pub customer_name: String,
    pub current: Decimal,
    pub days_1_30: Decimal,
    pub days_31_60: Decimal,
    pub days_61_90: Decimal,
    pub days_91_plus: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AccountsReceivableAging {
    pub as_of_date: NaiveDate,
    pub total_outstanding: Decimal,
    pub buckets: Vec<AgingBucket>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct DateRangeRequest {
    #[schema(example = "2023-01-01")]
    pub start_date: NaiveDate,

    #[schema(example = "2023-12-31")]
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct DateRequest {
    #[schema(example = "2023-12-31")]
    pub as_of_date: NaiveDate,
}