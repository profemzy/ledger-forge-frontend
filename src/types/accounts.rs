use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_account_id: Option<Uuid>,
    pub is_active: bool,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_account_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountHierarchy {
    pub account: Account,
    pub parent: Option<Box<Account>>,
    pub children: Vec<Account>,
}

pub type AccountBalance = Decimal;

