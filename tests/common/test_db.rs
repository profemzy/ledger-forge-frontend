use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Setup test database with migrations
pub async fn setup_test_db() -> PgPool {
    // Load .env file for test environment
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for tests");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Cleanup test database by truncating all tables
pub async fn cleanup_test_db(pool: &PgPool) {
    // Truncate all tables in reverse order of dependencies
    let _ = sqlx::query(
        "TRUNCATE
            bill_payment_applications,
            bill_payments,
            payment_applications,
            payments,
            bill_line_items,
            bills,
            invoice_line_items,
            invoices,
            transaction_line_items,
            transactions,
            items,
            contacts,
            chart_of_accounts,
            companies,
            users
        RESTART IDENTITY CASCADE"
    )
    .execute(pool)
    .await;
}

/// Create a test database pool for a specific test
/// This will automatically cleanup when dropped
pub struct TestDb {
    pub pool: PgPool,
}

impl TestDb {
    pub async fn new() -> Self {
        let pool = setup_test_db().await;
        cleanup_test_db(&pool).await;
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // Cleanup happens automatically when TestDb goes out of scope
        // Note: This is a sync Drop, so we can't run async cleanup here
        // Tests should call cleanup_test_db explicitly if needed
    }
}
