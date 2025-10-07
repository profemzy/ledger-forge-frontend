use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::models::{Bill, BillStatus, CreateBillRequest, BillWithLineItems};
use crate::routes::AppState;
use crate::utils::{created, success, ApiResponse, AppError, Result};

/// Query parameters for listing bills
#[derive(Debug, Deserialize)]
pub struct ListBillsQuery {
    #[serde(default)]
    pub vendor_id: Option<Uuid>,
    #[serde(default)]
    pub status: Option<BillStatus>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

/// Update bill status request
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateBillStatusRequest {
    pub status: BillStatus,
}

/// Create a new bill
#[utoipa::path(
    post,
    path = "/api/v1/bills",
    tag = "bills",
    request_body = CreateBillRequest,
    responses(
        (status = 201, description = "Bill created successfully", body = ApiResponse<Bill>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Vendor not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_bill(
    State(state): State<AppState>,
    Json(req): Json<CreateBillRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let bill = state.bill_service
        .create_bill(&state.pool, req)
        .await?;

    Ok(created(bill))
}

/// List bills
#[utoipa::path(
    get,
    path = "/api/v1/bills",
    tag = "bills",
    params(
        ("vendor_id" = Option<Uuid>, Query, description = "Filter by vendor ID"),
        ("status" = Option<BillStatus>, Query, description = "Filter by status"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("offset" = Option<i64>, Query, description = "Number of results to skip")
    ),
    responses(
        (status = 200, description = "Bills retrieved successfully", body = ApiResponse<Vec<Bill>>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_bills(
    State(state): State<AppState>,
    Query(params): Query<ListBillsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    let bills = state.bill_service
        .list_bills(
            &state.pool,
            params.vendor_id,
            params.status,
            params.limit,
            params.offset,
        )
        .await?;

    Ok(success(bills))
}

/// Get bill by ID
#[utoipa::path(
    get,
    path = "/api/v1/bills/{id}",
    tag = "bills",
    params(
        ("id" = Uuid, Path, description = "Bill ID")
    ),
    responses(
        (status = 200, description = "Bill retrieved successfully", body = ApiResponse<BillWithLineItems>),
        (status = 404, description = "Bill not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_bill(
    State(state): State<AppState>,
    Path(bill_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let bill = state.bill_service
        .get_bill_by_id(&state.pool, bill_id)
        .await?;

    match bill {
        Some(b) => Ok(success(b)),
        None => Err(AppError::NotFound("Bill not found".to_string())),
    }
}

/// Update bill status
#[utoipa::path(
    put,
    path = "/api/v1/bills/{id}/status",
    tag = "bills",
    params(
        ("id" = Uuid, Path, description = "Bill ID")
    ),
    request_body = UpdateBillStatusRequest,
    responses(
        (status = 200, description = "Bill status updated successfully", body = ApiResponse<Bill>),
        (status = 400, description = "Invalid status"),
        (status = 404, description = "Bill not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_bill_status(
    State(state): State<AppState>,
    Path(bill_id): Path<Uuid>,
    Json(req): Json<UpdateBillStatusRequest>,
) -> Result<impl axum::response::IntoResponse> {
    let bill = state.bill_service
        .update_bill_status(&state.pool, bill_id, req.status)
        .await?;

    Ok(success(bill))
}

/// Delete a bill
#[utoipa::path(
    delete,
    path = "/api/v1/bills/{id}",
    tag = "bills",
    params(
        ("id" = Uuid, Path, description = "Bill ID")
    ),
    responses(
        (status = 200, description = "Bill deleted successfully", body = ApiResponse<String>),
        (status = 400, description = "Cannot delete bill with payments"),
        (status = 404, description = "Bill not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_bill(
    State(state): State<AppState>,
    Path(bill_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    state.bill_service
        .delete_bill(&state.pool, bill_id)
        .await?;

    Ok(success("Bill deleted successfully".to_string()))
}

/// Get bills for a specific vendor
#[utoipa::path(
    get,
    path = "/api/v1/vendors/{id}/bills",
    tag = "bills",
    params(
        ("id" = Uuid, Path, description = "Vendor ID")
    ),
    responses(
        (status = 200, description = "Vendor bills retrieved successfully", body = ApiResponse<Vec<Bill>>),
        (status = 404, description = "Vendor not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_vendor_bills(
    State(state): State<AppState>,
    Path(vendor_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let bills = state.bill_service
        .get_vendor_bills(&state.pool, vendor_id)
        .await?;

    Ok(success(bills))
}

/// Get overdue bills
#[utoipa::path(
    get,
    path = "/api/v1/bills/overdue",
    tag = "bills",
    responses(
        (status = 200, description = "Overdue bills retrieved successfully", body = ApiResponse<Vec<Bill>>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_overdue_bills(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse> {
    let bills = state.bill_service
        .get_overdue_bills(&state.pool)
        .await?;

    Ok(success(bills))
}