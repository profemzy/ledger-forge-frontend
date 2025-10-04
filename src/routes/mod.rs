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
    services::{AuthService, AccountService, TransactionService},
    utils::HealthResponse
};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_service: AuthService,
    pub account_service: AccountService,
    pub transaction_service: TransactionService,
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

    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status.to_string(),
    })
}

/// Create all application routes
pub fn create_routes(pool: PgPool, auth_service: AuthService, account_service: AccountService, transaction_service: TransactionService) -> Router {
    let app_state = AppState {
        pool,
        auth_service,
        account_service,
        transaction_service,
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
        .route("/api/v1/accounts/{id}/balance", get(handlers::get_account_balance))
        // Transaction routes
        .route("/api/v1/transactions", get(handlers::list_transactions))
        .route("/api/v1/transactions", post(handlers::create_transaction))
        .route("/api/v1/transactions/{id}", get(handlers::get_transaction))
        .route("/api/v1/transactions/{id}/status", put(handlers::update_transaction_status))
        .route("/api/v1/transactions/{id}", delete(handlers::delete_transaction))
        .with_state(app_state);

    // Combine all routes including Swagger UI
    api_routes.merge(SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", crate::docs::ApiDoc::openapi()))
}
