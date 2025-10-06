use axum::{
    extract::State,
    routing::{delete, get, post, put},
    Json, Router,
};
use sqlx::PgPool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers,
    services::{AuthService, AccountService, TransactionService, ContactService, InvoiceService, PaymentService, CacheService, ReportingService},
    utils::HealthResponse
};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_service: AuthService,
    pub account_service: AccountService,
    pub transaction_service: TransactionService,
    pub contact_service: ContactService,
    pub invoice_service: InvoiceService,
    pub payment_service: PaymentService,
    pub reporting_service: ReportingService,
    pub cache_service: CacheService,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    // Check Redis connectivity
    let cache_status = match state.cache_service.health_check().await {
        Ok(true) => "healthy",
        Ok(false) => "unhealthy",
        Err(_) => "disconnected",
    };

    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status.to_string(),
        cache: cache_status.to_string(),
    })
}

/// Create all application routes
pub fn create_routes(pool: PgPool, auth_service: AuthService, account_service: AccountService, transaction_service: TransactionService, contact_service: ContactService, invoice_service: InvoiceService, payment_service: PaymentService, reporting_service: ReportingService, cache_service: CacheService) -> Router {
    let app_state = AppState {
        pool,
        auth_service,
        account_service,
        transaction_service,
        contact_service,
        invoice_service,
        payment_service,
        reporting_service,
        cache_service,
    };

    // API routes
    let api_routes = Router::new()
        .route("/api/v1/health", get(health_check))
        // Auth routes
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/refresh", post(handlers::refresh_token))
        .route("/api/v1/auth/me", get(handlers::me))
        // Account routes
        .route("/api/v1/accounts", get(handlers::list_accounts))
        .route("/api/v1/accounts", post(handlers::create_account))
        .route("/api/v1/accounts/{id}", get(handlers::get_account))
        .route("/api/v1/accounts/{id}", put(handlers::update_account))
        .route("/api/v1/accounts/{id}", delete(handlers::deactivate_account))
        .route("/api/v1/accounts/{id}/hierarchy", get(handlers::get_account_hierarchy))
        .route("/api/v1/accounts/{id}/balance", get(handlers::transaction::get_account_balance))
        // Transaction routes
        .route("/api/v1/transactions", get(handlers::list_transactions))
        .route("/api/v1/transactions", post(handlers::create_transaction))
        .route("/api/v1/transactions/{id}", get(handlers::get_transaction))
        .route("/api/v1/transactions/{id}/status", put(handlers::update_transaction_status))
        .route("/api/v1/transactions/{id}", delete(handlers::delete_transaction))
        // Contact routes
        .route("/api/v1/contacts", get(handlers::list_contacts))
        .route("/api/v1/contacts", post(handlers::create_contact))
        .route("/api/v1/contacts/customers", get(handlers::get_customers))
        .route("/api/v1/contacts/vendors", get(handlers::get_vendors))
        .route("/api/v1/contacts/employees", get(handlers::get_employees))
        .route("/api/v1/contacts/{id}", get(handlers::get_contact))
        .route("/api/v1/contacts/{id}", put(handlers::update_contact))
        .route("/api/v1/contacts/{id}", delete(handlers::delete_contact))
        // Invoice routes
        .route("/api/v1/invoices", get(handlers::list_invoices))
        .route("/api/v1/invoices", post(handlers::create_invoice))
        .route("/api/v1/invoices/{id}", get(handlers::get_invoice))
        .route("/api/v1/invoices/{id}/status", put(handlers::update_invoice_status))
        .route("/api/v1/invoices/overdue", get(handlers::get_overdue_invoices))
        .route("/api/v1/customers/{id}/invoices", get(handlers::get_customer_invoices))
        .route("/api/v1/invoices/{id}/payments", get(handlers::get_invoice_payments))
        // Payment routes
        .route("/api/v1/payments", get(handlers::list_payments))
        .route("/api/v1/payments", post(handlers::create_payment))
        .route("/api/v1/payments/{id}", get(handlers::get_payment))
        .route("/api/v1/payments/{id}/apply", put(handlers::apply_payment))
        .route("/api/v1/payments/unapplied", get(handlers::get_unapplied_payments))
        .route("/api/v1/bill-payments", post(handlers::create_bill_payment))
        // Reporting routes
        .route("/api/v1/reports/trial-balance", get(handlers::get_trial_balance))
        .route("/api/v1/reports/profit-loss", get(handlers::get_profit_loss))
        .route("/api/v1/reports/balance-sheet", get(handlers::get_balance_sheet))
        .route("/api/v1/reports/ar-aging", get(handlers::get_ar_aging))
        .with_state(app_state);

    // Combine all routes including Swagger UI
    api_routes.merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", crate::docs::ApiDoc::openapi()))
}
