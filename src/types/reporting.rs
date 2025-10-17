use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrialBalanceEntry {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TrialBalance {
    pub as_of_date: NaiveDate,
    pub total_debits: Decimal,
    pub total_credits: Decimal,
    pub is_balanced: bool,
    pub entries: Vec<TrialBalanceEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProfitLossEntry {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub amount: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProfitLossStatement {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_revenue: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
    pub revenue_entries: Vec<ProfitLossEntry>,
    pub expense_entries: Vec<ProfitLossEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BalanceSheetEntry {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub amount: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BalanceSheet {
    pub as_of_date: NaiveDate,
    pub total_assets: Decimal,
    pub total_liabilities: Decimal,
    pub total_equity: Decimal,
    pub asset_entries: Vec<BalanceSheetEntry>,
    pub liability_entries: Vec<BalanceSheetEntry>,
    pub equity_entries: Vec<BalanceSheetEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgingBucket {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub current: Decimal,
    pub days_1_30: Decimal,
    pub days_31_60: Decimal,
    pub days_61_90: Decimal,
    pub days_91_plus: Decimal,
    pub total: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountsReceivableAging {
    pub as_of_date: NaiveDate,
    pub total_outstanding: Decimal,
    pub buckets: Vec<AgingBucket>,
}
