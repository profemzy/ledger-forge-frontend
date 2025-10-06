use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    Invoice, InvoiceStatus, CreateInvoiceRequest, InvoiceWithLineItems
};
use crate::routes::AppState;
use crate::utils::{created, success, ApiResponse, AppError, Result};

/// Query parameters for listing invoices
#[derive(Debug, Deserialize)]
pub struct ListInvoicesQuery {
    #[serde(default)]
    pub customer_id: Option<Uuid>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub offset: Option<i64>,
}

/// Update invoice status request
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateInvoiceStatusRequest {
    pub status: String,
}

/// Create a new invoice
#[utoipa::path(
    post,
    path = "/api/v1/invoices",
    tag = "invoices",
    request_body = CreateInvoiceRequest,
    responses(
        (status = 201, description = "Invoice created successfully", body = ApiResponse<InvoiceWithLineItems>),
        (status = 400, description = "Invalid request data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Customer or account not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_invoice(
    State(state): State<AppState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Validate request
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Create invoice
    let invoice = state.invoice_service
        .create_invoice(&state.pool, req)
        .await?;

    Ok(created(invoice))
}

/// List all invoices with optional filtering
#[utoipa::path(
    get,
    path = "/api/v1/invoices",
    tag = "invoices",
    params(
        ("customer_id" = Option<Uuid>, Query, description = "Filter by customer ID"),
        ("status" = Option<String>, Query, description = "Filter by invoice status (draft, sent, paid, partial, overdue, void)"),
        ("limit" = Option<i64>, Query, description = "Limit number of results"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination")
    ),
    responses(
        (status = 200, description = "List of invoices", body = ApiResponse<Vec<Invoice>>),
        (status = 400, description = "Invalid query parameters"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_invoices(
    State(state): State<AppState>,
    Query(params): Query<ListInvoicesQuery>,
) -> Result<impl axum::response::IntoResponse> {
    // Parse status if provided
    let status = if let Some(status_str) = params.status {
        Some(parse_invoice_status(&status_str)?)
    } else {
        None
    };

    // List invoices
    let invoices = state.invoice_service
        .list_invoices(
            &state.pool,
            params.customer_id,
            status,
            params.limit,
            params.offset,
        )
        .await?;

    Ok(success(invoices))
}

/// Get invoice by ID with line items
#[utoipa::path(
    get,
    path = "/api/v1/invoices/{id}",
    tag = "invoices",
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    responses(
        (status = 200, description = "Invoice details with line items", body = ApiResponse<InvoiceWithLineItems>),
        (status = 404, description = "Invoice not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_invoice(
    State(state): State<AppState>,
    Path(invoice_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    // Get invoice
    let invoice = state.invoice_service
        .get_invoice(&state.pool, invoice_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

    Ok(success(invoice))
}

/// Update invoice status
#[utoipa::path(
    put,
    path = "/api/v1/invoices/{id}/status",
    tag = "invoices",
    params(
        ("id" = Uuid, Path, description = "Invoice ID")
    ),
    request_body = UpdateInvoiceStatusRequest,
    responses(
        (status = 200, description = "Invoice status updated", body = ApiResponse<Invoice>),
        (status = 400, description = "Invalid status or status transition"),
        (status = 404, description = "Invoice not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_invoice_status(
    State(state): State<AppState>,
    Path(invoice_id): Path<Uuid>,
    Json(req): Json<UpdateInvoiceStatusRequest>,
) -> Result<impl axum::response::IntoResponse> {
    // Parse status
    let new_status = parse_invoice_status(&req.status)?;

    // Update invoice status
    let invoice = state.invoice_service
        .update_invoice_status(&state.pool, invoice_id, new_status)
        .await?;

    Ok(success(invoice))
}

/// Get customer invoices
#[utoipa::path(
    get,
    path = "/api/v1/customers/{id}/invoices",
    tag = "invoices",
    params(
        ("id" = Uuid, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer invoices", body = ApiResponse<Vec<Invoice>>),
        (status = 404, description = "Customer not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_customer_invoices(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse> {
    // Get customer invoices
    let invoices = state.invoice_service
        .get_customer_invoices(&state.pool, customer_id)
        .await?;

    Ok(success(invoices))
}

/// Get overdue invoices
#[utoipa::path(
    get,
    path = "/api/v1/invoices/overdue",
    tag = "invoices",
    responses(
        (status = 200, description = "Overdue invoices", body = ApiResponse<Vec<Invoice>>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_overdue_invoices(
    State(state): State<AppState>,
) -> Result<impl axum::response::IntoResponse> {
    // Get overdue invoices
    let invoices = state.invoice_service
        .get_overdue_invoices(&state.pool)
        .await?;

    Ok(success(invoices))
}

// Helper functions

fn parse_invoice_status(status_str: &str) -> Result<InvoiceStatus> {
    match status_str.to_lowercase().as_str() {
        "draft" => Ok(InvoiceStatus::Draft),
        "sent" => Ok(InvoiceStatus::Sent),
        "paid" => Ok(InvoiceStatus::Paid),
        "partial" => Ok(InvoiceStatus::Partial),
        "overdue" => Ok(InvoiceStatus::Overdue),
        "void" => Ok(InvoiceStatus::Void),
        _ => Err(AppError::ValidationError(
            format!("Invalid invoice status: {}", status_str)
        )),
    }
}

