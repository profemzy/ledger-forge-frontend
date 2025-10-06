use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    Payment, CreatePaymentRequest, CreateBillPaymentRequest,
    PaymentApplicationRequest, BillPayment
};
use crate::routes::AppState;
use crate::utils::{created, success, ApiResponse, AppError, Result};

/// Query parameters for listing payments
#[derive(Debug, Deserialize)]
pub struct ListPaymentsQuery {
    #[serde(default)]
    pub customer_id: Option<Uuid>,
    #[serde(default)]
    pub unapplied_only: Option<bool>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

/// Apply payment to invoices request
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ApplyPaymentRequest {
    pub applications: Vec<PaymentApplicationRequest>,
}

/// Create a new customer payment
#[utoipa::path(
    post,
    path = "/api/v1/payments",
    tag = "payments",
    request_body = CreatePaymentRequest,
    responses(
        (status = 201, description = "Payment created successfully", body = ApiResponse<Payment>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Customer not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_payment(
    State(state): State<AppState>,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let payment = state.payment_service
        .create_payment(&state.pool, req)
        .await?;

    Ok(created(payment))
}

/// List payments
#[utoipa::path(
    get,
    path = "/api/v1/payments",
    tag = "payments",
    params(
        ("customer_id" = Option<Uuid>, Query, description = "Filter by customer ID"),
        ("unapplied_only" = Option<bool>, Query, description = "Show only unapplied payments"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results"),
        ("offset" = Option<i64>, Query, description = "Number of results to skip")
    ),
    responses(
        (status = 200, description = "Payments retrieved successfully", body = ApiResponse<Vec<Payment>>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_payments(
    State(state): State<AppState>,
    Query(params): Query<ListPaymentsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    let payments = if let Some(customer_id) = params.customer_id {
        if params.unapplied_only.unwrap_or(false) {
            state.payment_service
                .list_unapplied_payments(&state.pool, Some(customer_id))
                .await?
        } else {
            state.payment_service
                .list_customer_payments(&state.pool, customer_id)
                .await?
        }
    } else {
        if params.unapplied_only.unwrap_or(false) {
            state.payment_service
                .list_unapplied_payments(&state.pool, None)
                .await?
        } else {
            state.payment_service
                .list_payments(&state.pool, params.limit, params.offset)
                .await?
        }
    };

    Ok(success(payments))
}

/// Get payment by ID
#[utoipa::path(
    get,
    path = "/api/v1/payments/{id}",
    tag = "payments",
    params(
        ("id" = Uuid, Path, description = "Payment ID")
    ),
    responses(
        (status = 200, description = "Payment retrieved successfully", body = ApiResponse<Payment>),
        (status = 404, description = "Payment not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_payment(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let payment = state.payment_service
        .get_payment_by_id(&state.pool, payment_id)
        .await?;

    match payment {
        Some(p) => Ok(success(p)),
        None => Err(AppError::NotFound("Payment not found".to_string())),
    }
}

/// Apply payment to invoices
#[utoipa::path(
    put,
    path = "/api/v1/payments/{id}/apply",
    tag = "payments",
    params(
        ("id" = Uuid, Path, description = "Payment ID")
    ),
    request_body = ApplyPaymentRequest,
    responses(
        (status = 200, description = "Payment applied successfully", body = ApiResponse<String>),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Payment or invoice not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn apply_payment(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
    Json(req): Json<ApplyPaymentRequest>,
) -> Result<impl axum::response::IntoResponse> {
    state.payment_service
        .apply_payment_to_invoices(&state.pool, payment_id, req.applications)
        .await?;

    Ok(success("Payment applied successfully".to_string()))
}

/// Get payments for a specific invoice
#[utoipa::path(
    get,
    path = "/api/v1/invoices/{id}/payments",
    tag = "payments",
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    responses(
        (status = 200, description = "Invoice payments retrieved successfully", body = ApiResponse<Vec<Payment>>),
        (status = 404, description = "Invoice not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_invoice_payments(
    State(state): State<AppState>,
    Path(invoice_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    let payments = state.payment_service
        .get_invoice_payments(&state.pool, invoice_id)
        .await?;

    Ok(success(payments))
}

/// Get unapplied payments
#[utoipa::path(
    get,
    path = "/api/v1/payments/unapplied",
    tag = "payments",
    params(
        ("customer_id" = Option<Uuid>, Query, description = "Filter by customer ID")
    ),
    responses(
        (status = 200, description = "Unapplied payments retrieved successfully", body = ApiResponse<Vec<Payment>>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_unapplied_payments(
    State(state): State<AppState>,
    Query(params): Query<ListPaymentsQuery>,
) -> Result<impl axum::response::IntoResponse> {
    let payments = state.payment_service
        .list_unapplied_payments(&state.pool, params.customer_id)
        .await?;

    Ok(success(payments))
}

/// Create a new vendor bill payment
#[utoipa::path(
    post,
    path = "/api/v1/bill-payments",
    tag = "payments",
    request_body = CreateBillPaymentRequest,
    responses(
        (status = 201, description = "Bill payment created successfully", body = ApiResponse<BillPayment>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Vendor not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_bill_payment(
    State(state): State<AppState>,
    Json(req): Json<CreateBillPaymentRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let bill_payment = state.payment_service
        .create_bill_payment(&state.pool, req)
        .await?;

    Ok(created(bill_payment))
}