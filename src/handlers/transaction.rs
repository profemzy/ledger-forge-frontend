use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{CreateTransactionRequest, TransactionStatus};
use crate::routes::AppState;
use crate::utils::{created, no_content, success, AppError, Result};

/// Query parameters for listing transactions
#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub company_id: Option<Uuid>,
    #[serde(default)]
    pub limit: Option<i64>,
}

/// Request body for updating transaction status
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

/// List all transactions
pub async fn list_transactions(
    State(state): State<AppState>,
    Query(params): Query<ListTransactionsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Parse status if provided
    let status = if let Some(status_str) = params.status {
        Some(parse_transaction_status(&status_str)?)
    } else {
        None
    };

    let transactions = state
        .transaction_service
        .list_transactions(&state.pool, status, params.company_id, params.limit)
        .await?;

    Ok(success(transactions))
}

/// Create a new transaction
pub async fn create_transaction(
    State(state): State<AppState>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // TODO: Extract user_id from JWT token when auth middleware is fully integrated
    let created_by = None;

    let transaction = state
        .transaction_service
        .create_transaction(&state.pool, req, created_by)
        .await?;

    Ok(created(transaction))
}

/// Get transaction by ID
pub async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let transaction = state
        .transaction_service
        .get_transaction_by_id(&state.pool, id)
        .await?;

    Ok(success(transaction))
}

/// Update transaction status
pub async fn update_transaction_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let new_status = parse_transaction_status(&req.status)?;

    let transaction = state
        .transaction_service
        .update_transaction_status(&state.pool, id, new_status)
        .await?;

    Ok(success(transaction))
}

/// Delete transaction (only drafts)
pub async fn delete_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    state
        .transaction_service
        .delete_transaction(&state.pool, id)
        .await?;

    Ok(no_content())
}

/// Get account balance
pub async fn get_account_balance(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let balance = state
        .transaction_service
        .get_account_balance(&state.pool, account_id)
        .await?;

    Ok(success(serde_json::json!({
        "account_id": account_id,
        "balance": balance,
    })))
}

/// Helper function to parse transaction status string
fn parse_transaction_status(status_str: &str) -> Result<TransactionStatus> {
    match status_str.to_lowercase().as_str() {
        "draft" => Ok(TransactionStatus::Draft),
        "posted" => Ok(TransactionStatus::Posted),
        "void" => Ok(TransactionStatus::Void),
        _ => Err(AppError::BadRequest(format!(
            "Invalid transaction status: {}. Must be one of: draft, posted, void",
            status_str
        ))),
    }
}
