use axum::{
    extract::{Query, State},
};
use serde::Deserialize;
use validator::Validate;
use utoipa::ToSchema;

use crate::models::{
    TrialBalance, ProfitLossStatement, BalanceSheet, AccountsReceivableAging,
    DateRangeRequest, DateRequest
};
use crate::routes::AppState;
use crate::utils::{success, ApiResponse, AppError, Result};

/// Query parameters for trial balance
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TrialBalanceQuery {
    #[serde(flatten)]
    pub date: DateRequest,
}

/// Query parameters for profit and loss statement
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProfitLossQuery {
    #[serde(flatten)]
    pub date_range: DateRangeRequest,
}

/// Query parameters for balance sheet
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BalanceSheetQuery {
    #[serde(flatten)]
    pub date: DateRequest,
}

/// Query parameters for accounts receivable aging
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AgingQuery {
    #[serde(flatten)]
    pub date: DateRequest,
}

/// Generate trial balance
#[utoipa::path(
    get,
    path = "/api/v1/reports/trial-balance",
    tag = "reporting",
    params(
        ("as_of_date" = chrono::NaiveDate, Query, description = "As of date for the trial balance")
    ),
    responses(
        (status = 200, description = "Trial balance generated successfully", body = ApiResponse<TrialBalance>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_trial_balance(
    State(state): State<AppState>,
    Query(params): Query<TrialBalanceQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    params.date.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let trial_balance = state.reporting_service
        .generate_trial_balance(&state.pool, params.date)
        .await?;

    Ok(success(trial_balance))
}

/// Generate profit and loss statement
#[utoipa::path(
    get,
    path = "/api/v1/reports/profit-loss",
    tag = "reporting",
    params(
        ("start_date" = chrono::NaiveDate, Query, description = "Start date for the reporting period"),
        ("end_date" = chrono::NaiveDate, Query, description = "End date for the reporting period")
    ),
    responses(
        (status = 200, description = "Profit and loss statement generated successfully", body = ApiResponse<ProfitLossStatement>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_profit_loss(
    State(state): State<AppState>,
    Query(params): Query<ProfitLossQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    params.date_range.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    if params.date_range.start_date > params.date_range.end_date {
        return Err(AppError::ValidationError(
            "Start date cannot be after end date".to_string()
        ));
    }

    let profit_loss = state.reporting_service
        .generate_profit_loss(&state.pool, params.date_range)
        .await?;

    Ok(success(profit_loss))
}

/// Generate balance sheet
#[utoipa::path(
    get,
    path = "/api/v1/reports/balance-sheet",
    tag = "reporting",
    params(
        ("as_of_date" = chrono::NaiveDate, Query, description = "As of date for the balance sheet")
    ),
    responses(
        (status = 200, description = "Balance sheet generated successfully", body = ApiResponse<BalanceSheet>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_balance_sheet(
    State(state): State<AppState>,
    Query(params): Query<BalanceSheetQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    params.date.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let balance_sheet = state.reporting_service
        .generate_balance_sheet(&state.pool, params.date)
        .await?;

    Ok(success(balance_sheet))
}

/// Generate accounts receivable aging report
#[utoipa::path(
    get,
    path = "/api/v1/reports/ar-aging",
    tag = "reporting",
    params(
        ("as_of_date" = chrono::NaiveDate, Query, description = "As of date for the aging report")
    ),
    responses(
        (status = 200, description = "Accounts receivable aging report generated successfully", body = ApiResponse<AccountsReceivableAging>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_ar_aging(
    State(state): State<AppState>,
    Query(params): Query<AgingQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    params.date.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let ar_aging = state.reporting_service
        .generate_ar_aging(&state.pool, params.date)
        .await?;

    Ok(success(ar_aging))
}