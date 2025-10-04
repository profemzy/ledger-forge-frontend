use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{Account, AccountType, CreateAccountRequest, UpdateAccountRequest};
use crate::routes::AppState;
use crate::utils::{created, no_content, success, ApiResponse, Result};

/// Query parameters for listing accounts
#[derive(Debug, Deserialize)]
pub struct ListAccountsQuery {
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub parent_id: Option<Uuid>,
    #[serde(default)]
    pub include_inactive: Option<bool>,
}

/// List all accounts
#[utoipa::path(
    get,
    path = "/api/v1/accounts",
    tag = "accounts",
    params(
        ("account_type" = Option<String>, Query, description = "Filter by account type (Asset, Liability, Equity, Revenue, Expense)"),
        ("parent_id" = Option<String>, Query, description = "Filter by parent account ID"),
        ("include_inactive" = Option<bool>, Query, description = "Include inactive accounts")
    ),
    responses(
        (status = 200, description = "List of accounts", body = ApiResponse<Vec<Account>>),
        (status = 400, description = "Invalid query parameters")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_accounts(
    State(state): State<AppState>,
    Query(params): Query<ListAccountsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Parse account type if provided
    let account_type = if let Some(type_str) = params.account_type {
        Some(parse_account_type(&type_str)?)
    } else {
        None
    };

    let accounts = state
        .account_service
        .list_accounts(
            &state.pool,
            account_type,
            params.parent_id,
            params.include_inactive.unwrap_or(false),
        )
        .await?;

    Ok(success(accounts))
}

/// Create a new account
#[utoipa::path(
    post,
    path = "/api/v1/accounts",
    tag = "accounts",
    request_body = CreateAccountRequest,
    responses(
        (status = 201, description = "Account created successfully", body = ApiResponse<Account>),
        (status = 400, description = "Invalid request data"),
        (status = 409, description = "Account code already exists")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let account = state.account_service.create_account(&state.pool, req).await?;
    Ok(created(account))
}

/// Get account by ID
#[utoipa::path(
    get,
    path = "/api/v1/accounts/{id}",
    tag = "accounts",
    params(
        ("id" = Uuid, Path, description = "Account ID")
    ),
    responses(
        (status = 200, description = "Account details", body = ApiResponse<Account>),
        (status = 404, description = "Account not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let account = state.account_service.get_account_by_id(&state.pool, id).await?;
    Ok(success(account))
}

/// Update account
#[utoipa::path(
    put,
    path = "/api/v1/accounts/{id}",
    tag = "accounts",
    params(
        ("id" = Uuid, Path, description = "Account ID")
    ),
    request_body = UpdateAccountRequest,
    responses(
        (status = 200, description = "Account updated successfully", body = ApiResponse<Account>),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Account not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let account = state
        .account_service
        .update_account(&state.pool, id, req)
        .await?;
    Ok(success(account))
}

/// Deactivate account (soft delete)
#[utoipa::path(
    delete,
    path = "/api/v1/accounts/{id}",
    tag = "accounts",
    params(
        ("id" = Uuid, Path, description = "Account ID")
    ),
    responses(
        (status = 204, description = "Account deactivated successfully"),
        (status = 400, description = "Cannot deactivate account with transactions"),
        (status = 404, description = "Account not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn deactivate_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    state
        .account_service
        .deactivate_account(&state.pool, id)
        .await?;
    Ok(no_content())
}

/// Get account hierarchy (parent and children)
#[utoipa::path(
    get,
    path = "/api/v1/accounts/{id}/hierarchy",
    tag = "accounts",
    params(
        ("id" = Uuid, Path, description = "Account ID")
    ),
    responses(
        (status = 200, description = "Account hierarchy details"),
        (status = 404, description = "Account not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_account_hierarchy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let hierarchy = state
        .account_service
        .get_account_hierarchy(&state.pool, id)
        .await?;
    Ok(success(hierarchy))
}

/// Helper function to parse account type string
fn parse_account_type(type_str: &str) -> Result<AccountType> {
    match type_str.to_lowercase().as_str() {
        "asset" => Ok(AccountType::Asset),
        "liability" => Ok(AccountType::Liability),
        "equity" => Ok(AccountType::Equity),
        "revenue" => Ok(AccountType::Revenue),
        "expense" => Ok(AccountType::Expense),
        _ => Err(crate::utils::AppError::BadRequest(format!(
            "Invalid account type: {}. Must be one of: Asset, Liability, Equity, Revenue, Expense",
            type_str
        ))),
    }
}
