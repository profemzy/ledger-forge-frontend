mod docs;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::create_routes;
use crate::services::{AuthService, AccountService, TransactionService, CacheService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ledger_forge=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    // Get JWT secret from environment
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET not set, using default (NOT FOR PRODUCTION!)");
            "your-secret-key-change-this-in-production".to_string()
        });

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("✅ Connected to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    tracing::info!("✅ Database migrations applied");

    // Get Redis URL from environment
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| {
            tracing::info!("REDIS_URL not set, using default localhost:6379");
            "redis://localhost:6379".to_string()
        });

    // Initialize cache service
    let cache_service = CacheService::new(&redis_url)?;
    tracing::info!("✅ Redis cache service initialized");

    // Initialize services with cache
    let auth_service = AuthService::new(jwt_secret);
    let account_service = AccountService::new_with_cache(cache_service.clone());
    let transaction_service = TransactionService::new_with_cache(cache_service.clone());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create application routes
    let app = create_routes(pool, auth_service, account_service, transaction_service, cache_service)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Get server port from environment or use default
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 Server starting on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
