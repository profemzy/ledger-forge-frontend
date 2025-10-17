use crate::api::client;
use crate::types::reporting::{
    TrialBalance, ProfitLossStatement, BalanceSheet, AccountsReceivableAging,
};

pub async fn get_trial_balance(as_of_date: &str) -> Result<TrialBalance, String> {
    let path = format!("/reports/trial-balance?as_of_date={}", as_of_date);
    client::get::<TrialBalance>(&path).await
}

pub async fn get_profit_loss(start_date: &str, end_date: &str) -> Result<ProfitLossStatement, String> {
    let path = format!("/reports/profit-loss?start_date={}&end_date={}", start_date, end_date);
    client::get::<ProfitLossStatement>(&path).await
}

pub async fn get_balance_sheet(as_of_date: &str) -> Result<BalanceSheet, String> {
    let path = format!("/reports/balance-sheet?as_of_date={}", as_of_date);
    client::get::<BalanceSheet>(&path).await
}

pub async fn get_ar_aging(as_of_date: &str) -> Result<AccountsReceivableAging, String> {
    let path = format!("/reports/ar-aging?as_of_date={}", as_of_date);
    client::get::<AccountsReceivableAging>(&path).await
}
