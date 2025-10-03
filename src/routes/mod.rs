use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;

use crate::{handlers, services::AuthService, utils::HealthResponse};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_service: AuthService,
}

/// Health check endpoint
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
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
pub fn create_routes(pool: PgPool, auth_service: AuthService) -> Router {
    let app_state = AppState {
        pool,
        auth_service,
    };

    // All routes under /api/v1
    Router::new()
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/refresh", post(handlers::refresh_token))
        .route("/api/v1/auth/me", get(handlers::me))
        .with_state(app_state)
}
