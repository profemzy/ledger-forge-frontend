use serde::Serialize;
use uuid::Uuid;

use crate::api::client;
use crate::types::transactions::{
    Transaction, TransactionWithLineItems, TransactionStatus, CreateTransactionRequest,
};

#[derive(Default, Serialize)]
struct ListQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

fn status_to_str(s: &TransactionStatus) -> &'static str {
    match s {
        TransactionStatus::Draft => "draft",
        TransactionStatus::Posted => "posted",
        TransactionStatus::Void => "void",
    }
}

pub async fn list_transactions(
    status: Option<TransactionStatus>,
    company_id: Option<Uuid>,
    limit: Option<i64>,
) -> Result<Vec<Transaction>, String> {
    let q = ListQuery {
        status: status.as_ref().map(status_to_str),
        company_id,
        limit,
    };
    // Manual query string to avoid external dep
    let mut params = Vec::new();
    if let Some(st) = q.status { params.push(format!("status={}", st)); }
    if let Some(cid) = q.company_id { params.push(format!("company_id={}", cid)); }
    if let Some(l) = q.limit { params.push(format!("limit={}", l)); }
    let qs = if params.is_empty() { String::new() } else { format!("?{}", params.join("&")) };
    let path = format!("/transactions{}", qs);
    client::get::<Vec<Transaction>>(&path).await
}

pub async fn get_transaction(id: Uuid) -> Result<TransactionWithLineItems, String> {
    let path = format!("/transactions/{}", id);
    client::get::<TransactionWithLineItems>(&path).await
}

pub async fn create_transaction(req: &CreateTransactionRequest) -> Result<TransactionWithLineItems, String> {
    client::post::<_, TransactionWithLineItems>("/transactions", req).await
}

#[derive(Serialize)]
struct UpdateStatusBody<'a> { status: &'a str }

pub async fn update_transaction_status(id: Uuid, status: TransactionStatus) -> Result<Transaction, String> {
    let path = format!("/transactions/{}/status", id);
    let body = UpdateStatusBody { status: status_to_str(&status) };
    client::put::<_, Transaction>(&path, &body).await
}

pub async fn delete_transaction(id: Uuid) -> Result<(), String> {
    let path = format!("/transactions/{}", id);
    // We don't need the body, treat any 2xx as success
    let _ignored: serde_json::Value = client::delete(&path).await?;
    Ok(())
}

