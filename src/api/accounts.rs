use uuid::Uuid;

use crate::api::client;
use crate::types::accounts::{
    Account, AccountHierarchy, AccountBalance, CreateAccountRequest, UpdateAccountRequest, AccountType,
};

#[derive(Debug, serde::Deserialize)]
struct BalanceResponse {
    account_id: uuid::Uuid,
    balance: AccountBalance,
}

pub async fn list_accounts(
    account_type: Option<AccountType>,
    parent_id: Option<Uuid>,
    include_inactive: Option<bool>,
) -> Result<Vec<Account>, String> {
    let type_str = account_type.map(|t| match t {
        AccountType::Asset => "asset",
        AccountType::Liability => "liability",
        AccountType::Equity => "equity",
        AccountType::Revenue => "revenue",
        AccountType::Expense => "expense",
    });

    let mut params: Vec<String> = Vec::new();
    if let Some(t) = type_str { params.push(format!("account_type={}", t)); }
    if let Some(pid) = parent_id { params.push(format!("parent_id={}", pid)); }
    if include_inactive.unwrap_or(false) { params.push("include_inactive=true".into()); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let path = format!("/accounts{}", qs);
    client::get::<Vec<Account>>(&path).await
}

pub async fn create_account(req: &CreateAccountRequest) -> Result<Account, String> {
    client::post::<_, Account>("/accounts", req).await
}

pub async fn get_account(id: Uuid) -> Result<Account, String> {
    let path = format!("/accounts/{}", id);
    client::get::<Account>(&path).await
}

pub async fn update_account(id: Uuid, req: &UpdateAccountRequest) -> Result<Account, String> {
    let path = format!("/accounts/{}", id);
    client::put::<_, Account>(&path, req).await
}

pub async fn get_account_hierarchy(id: Uuid) -> Result<AccountHierarchy, String> {
    let path = format!("/accounts/{}/hierarchy", id);
    client::get::<AccountHierarchy>(&path).await
}

pub async fn get_account_balance(id: Uuid) -> Result<AccountBalance, String> {
    let path = format!("/accounts/{}/balance", id);
    let resp = client::get::<BalanceResponse>(&path).await?;
    Ok(resp.balance)
}
