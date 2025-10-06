use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::Value;

use ledger_forge::routes::create_routes;
use ledger_forge::services::{AuthService, AccountService, TransactionService, ContactService, InvoiceService, PaymentService, ReportingService, CacheService};
use ledger_forge::models::{DateRequest, DateRangeRequest};

/// Integration tests for financial reporting endpoints
/// These tests validate the complete reporting functionality with realistic data

#[sqlx::test]
async fn test_trial_balance_report() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test trial balance as of specific date
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/v1/reports/trial-balance?as_of_date=2024-12-31"))
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("success").unwrap().as_bool().unwrap());
    let data = json.get("data").unwrap();

    // Verify trial balance structure
    assert!(data.get("as_of_date").is_some());
    assert!(data.get("total_debits").is_some());
    assert!(data.get("total_credits").is_some());
    assert!(data.get("is_balanced").is_some());
    assert!(data.get("entries").unwrap().as_array().unwrap().len() > 0);

    // Verify balance property (debits should equal credits within tolerance)
    let total_debits: f64 = data.get("total_debits").unwrap().as_str().unwrap().parse().unwrap();
    let total_credits: f64 = data.get("total_credits").unwrap().as_str().unwrap().parse().unwrap();
    let is_balanced = data.get("is_balanced").unwrap().as_bool().unwrap();

    assert!((total_debits - total_credits).abs() < 0.01); // Within $0.01 tolerance
    assert!(is_balanced);

    // Verify entry structure
    let entries = data.get("entries").unwrap().as_array().unwrap();
    for entry in entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("account_type").is_some());
        assert!(entry.get("debit").is_some());
        assert!(entry.get("credit").is_some());
        assert!(entry.get("balance").is_some());

        // Verify that only one of debit or credit is non-zero per entry
        let debit: f64 = entry.get("debit").unwrap().as_str().unwrap().parse().unwrap();
        let credit: f64 = entry.get("credit").unwrap().as_str().unwrap().parse().unwrap();
        assert!(debit == 0.0 || credit == 0.0);
    }

    Ok(())
}

#[sqlx::test]
async fn test_profit_loss_statement() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test profit and loss for specific date range
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/v1/reports/profit-loss?start_date=2024-01-01&end_date=2024-12-31"))
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("success").unwrap().as_bool().unwrap());
    let data = json.get("data").unwrap();

    // Verify P&L structure
    assert!(data.get("period_start").is_some());
    assert!(data.get("period_end").is_some());
    assert!(data.get("total_revenue").is_some());
    assert!(data.get("total_expenses").is_some());
    assert!(data.get("net_income").is_some());
    assert!(data.get("revenue_entries").is_some());
    assert!(data.get("expense_entries").is_some());

    // Verify financial logic
    let total_revenue: f64 = data.get("total_revenue").unwrap().as_str().unwrap().parse().unwrap();
    let total_expenses: f64 = data.get("total_expenses").unwrap().as_str().unwrap().parse().unwrap();
    let net_income: f64 = data.get("net_income").unwrap().as_str().unwrap().parse().unwrap();

    assert_eq!(total_revenue - total_expenses, net_income);
    assert!(total_revenue > 0.0); // Should have revenue from test data
    assert!(total_expenses > 0.0); // Should have expenses from test data

    // Verify revenue entries
    let revenue_entries = data.get("revenue_entries").unwrap().as_array().unwrap();
    for entry in revenue_entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("account_type").is_some());
        assert!(entry.get("amount").is_some());

        let account_type = entry.get("account_type").unwrap().as_str().unwrap();
        assert_eq!(account_type, "Revenue");

        let amount: f64 = entry.get("amount").unwrap().as_str().unwrap().parse().unwrap();
        assert!(amount > 0.0);
    }

    // Verify expense entries
    let expense_entries = data.get("expense_entries").unwrap().as_array().unwrap();
    for entry in expense_entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("account_type").is_some());
        assert!(entry.get("amount").is_some());

        let account_type = entry.get("account_type").unwrap().as_str().unwrap();
        assert_eq!(account_type, "Expense");

        let amount: f64 = entry.get("amount").unwrap().as_str().unwrap().parse().unwrap();
        assert!(amount > 0.0);
    }

    Ok(())
}

#[sqlx::test]
async fn test_balance_sheet() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test balance sheet as of specific date
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/v1/reports/balance-sheet?as_of_date=2024-12-31"))
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("success").unwrap().as_bool().unwrap());
    let data = json.get("data").unwrap();

    // Verify balance sheet structure
    assert!(data.get("as_of_date").is_some());
    assert!(data.get("total_assets").is_some());
    assert!(data.get("total_liabilities").is_some());
    assert!(data.get("total_equity").is_some());
    assert!(data.get("asset_entries").is_some());
    assert!(data.get("liability_entries").is_some());
    assert!(data.get("equity_entries").is_some());

    // Verify balance sheet equation: Assets = Liabilities + Equity
    let total_assets: f64 = data.get("total_assets").unwrap().as_str().unwrap().parse().unwrap();
    let total_liabilities: f64 = data.get("total_liabilities").unwrap().as_str().unwrap().parse().unwrap();
    let total_equity: f64 = data.get("total_equity").unwrap().as_str().unwrap().parse().unwrap();

    assert!((total_assets - (total_liabilities + total_equity)).abs() < 0.01); // Within $0.01 tolerance

    // Verify asset entries
    let asset_entries = data.get("asset_entries").unwrap().as_array().unwrap();
    for entry in asset_entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("amount").is_some());

        let amount: f64 = entry.get("amount").unwrap().as_str().unwrap().parse().unwrap();
        assert!(amount > 0.0);
    }

    // Verify liability entries
    let liability_entries = data.get("liability_entries").unwrap().as_array().unwrap();
    for entry in liability_entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("amount").is_some());

        let amount: f64 = entry.get("amount").unwrap().as_str().unwrap().parse().unwrap();
        assert!(amount > 0.0);
    }

    // Verify equity entries
    let equity_entries = data.get("equity_entries").unwrap().as_array().unwrap();
    for entry in equity_entries {
        assert!(entry.get("account_id").is_some());
        assert!(entry.get("account_code").is_some());
        assert!(entry.get("account_name").is_some());
        assert!(entry.get("amount").is_some());

        let amount: f64 = entry.get("amount").unwrap().as_str().unwrap().parse().unwrap();
        assert!(amount > 0.0);
    }

    Ok(())
}

#[sqlx::test]
async fn test_accounts_receivable_aging() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test AR aging as of specific date
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/v1/reports/ar-aging?as_of_date=2024-12-31"))
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert!(json.get("success").unwrap().as_bool().unwrap());
    let data = json.get("data").unwrap();

    // Verify AR aging structure
    assert!(data.get("as_of_date").is_some());
    assert!(data.get("total_outstanding").is_some());
    assert!(data.get("buckets").is_some());

    let total_outstanding: f64 = data.get("total_outstanding").unwrap().as_str().unwrap().parse().unwrap();
    assert!(total_outstanding >= 0.0);

    // Verify aging buckets
    let buckets = data.get("buckets").unwrap().as_array().unwrap();
    let mut calculated_total = 0.0;

    for bucket in buckets {
        assert!(bucket.get("customer_id").is_some());
        assert!(bucket.get("customer_name").is_some());
        assert!(bucket.get("current").is_some());
        assert!(bucket.get("days_1_30").is_some());
        assert!(bucket.get("days_31_60").is_some());
        assert!(bucket.get("days_61_90").is_some());
        assert!(bucket.get("days_91_plus").is_some());
        assert!(bucket.get("total").is_some());

        let current: f64 = bucket.get("current").unwrap().as_str().unwrap().parse().unwrap();
        let days_1_30: f64 = bucket.get("days_1_30").unwrap().as_str().unwrap().parse().unwrap();
        let days_31_60: f64 = bucket.get("days_31_60").unwrap().as_str().unwrap().parse().unwrap();
        let days_61_90: f64 = bucket.get("days_61_90").unwrap().as_str().unwrap().parse().unwrap();
        let days_91_plus: f64 = bucket.get("days_91_plus").unwrap().as_str().unwrap().parse().unwrap();
        let total: f64 = bucket.get("total").unwrap().as_str().unwrap().parse().unwrap();

        // Verify bucket totals
        let calculated_bucket_total = current + days_1_30 + days_31_60 + days_61_90 + days_91_plus;
        assert!((total - calculated_bucket_total).abs() < 0.01);

        calculated_total += total;
    }

    // Verify that sum of all bucket totals equals the reported total outstanding
    assert!((total_outstanding - calculated_total).abs() < 0.01);

    Ok(())
}

#[sqlx::test]
async fn test_trial_balance_date_validation() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test invalid date format
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/trial-balance?as_of_date=invalid-date")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(!json.get("success").unwrap().as_bool().unwrap());
    assert!(json.get("error").is_some());

    Ok(())
}

#[sqlx::test]
async fn test_profit_loss_date_range_validation() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Test start date after end date
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/profit-loss?start_date=2024-12-31&end_date=2024-01-01")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(!json.get("success").unwrap().as_bool().unwrap());
    assert!(json.get("error").is_some());

    Ok(())
}

#[sqlx::test]
async fn test_reporting_with_empty_database() -> sqlx::Result<()> {
    // Create empty database
    let pool = create_empty_test_database().await?;

    // Initialize services for testing
    let auth_service = AuthService::new("test-secret-key".to_string());
    let account_service = AccountService::new_with_cache(CacheService::new(100));
    let transaction_service = TransactionService::new_with_cache(CacheService::new(100));
    let contact_service = ContactService::new_with_cache(CacheService::new(100));
    let invoice_service = InvoiceService::new_with_cache(CacheService::new(100));
    let payment_service = PaymentService::new_with_cache(CacheService::new(100));
    let reporting_service = ReportingService::new_with_cache(CacheService::new(100));
    let cache_service = CacheService::new(100);

    // Create test app
    let app = create_routes(
        pool.clone(),
        auth_service,
        account_service,
        transaction_service,
        contact_service,
        invoice_service,
        payment_service,
        reporting_service,
        cache_service,
    );

    // Test trial balance with no transactions
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/trial-balance?as_of_date=2024-12-31")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("success").unwrap().as_bool().unwrap());
    let data = json.get("data").unwrap();
    let entries = data.get("entries").unwrap().as_array().unwrap();
    assert_eq!(entries.len(), 0); // Should be empty with no transactions

    Ok(())
}

#[sqlx::test]
async fn test_financial_reports_consistency() -> sqlx::Result<()> {
    let (pool, app) = setup_test_database().await?;

    // Generate all reports for the same period
    let trial_balance_request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/trial-balance?as_of_date=2024-12-31")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let profit_loss_request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/profit-loss?start_date=2024-01-01&end_date=2024-12-31")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    let balance_sheet_request = Request::builder()
        .method("GET")
        .uri("/api/v1/reports/balance-sheet?as_of_date=2024-12-31")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .unwrap();

    // Get all reports
    let tb_response = app.clone().oneshot(trial_balance_request).await.unwrap();
    let pl_response = app.clone().oneshot(profit_loss_request).await.unwrap();
    let bs_response = app.oneshot(balance_sheet_request).await.unwrap();

    // All should be successful
    assert_eq!(tb_response.status(), StatusCode::OK);
    assert_eq!(pl_response.status(), StatusCode::OK);
    assert_eq!(bs_response.status(), StatusCode::OK);

    // Parse responses
    let tb_body = axum::body::to_bytes(tb_response.into_body(), usize::MAX).await.unwrap();
    let tb_json: Value = serde_json::from_slice(&tb_body).unwrap();

    let pl_body = axum::body::to_bytes(pl_response.into_body(), usize::MAX).await.unwrap();
    let pl_json: Value = serde_json::from_slice(&pl_body).unwrap();

    let bs_body = axum::body::to_bytes(bs_response.into_body(), usize::MAX).await.unwrap();
    let bs_json: Value = serde_json::from_slice(&bs_body).unwrap();

    // Extract key metrics
    let tb_total_debits: f64 = tb_json.get("data").unwrap().get("total_debits").unwrap().as_str().unwrap().parse().unwrap();
    let tb_total_credits: f64 = tb_json.get("data").unwrap().get("total_credits").unwrap().as_str().unwrap().parse().unwrap();

    let pl_net_income: f64 = pl_json.get("data").unwrap().get("net_income").unwrap().as_str().unwrap().parse().unwrap();

    let bs_total_assets: f64 = bs_json.get("data").unwrap().get("total_assets").unwrap().as_str().unwrap().parse().unwrap();
    let bs_total_liabilities: f64 = bs_json.get("data").unwrap().get("total_liabilities").unwrap().as_str().unwrap().parse().unwrap();
    let bs_total_equity: f64 = bs_json.get("data").unwrap().get("total_equity").unwrap().as_str().unwrap().parse().unwrap();

    // Verify consistency
    assert!((tb_total_debits - tb_total_credits).abs() < 0.01); // Trial balance is balanced
    assert!((bs_total_assets - (bs_total_liabilities + bs_total_equity)).abs() < 0.01); // Balance sheet balances

    Ok(())
}

/// Helper function to set up test database with seed data
async fn setup_test_database() -> sqlx::Result<(sqlx::PgPool, axum::Router)> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run the seed data migration
    let seed_sql = include_str!("../migrations/20241220000000_financial_reporting_seed_data.sql");
    sqlx::query(seed_sql).execute(&pool).await?;

    // Initialize services for testing
    let auth_service = AuthService::new("test-secret-key".to_string());
    let account_service = AccountService::new_with_cache(CacheService::new(100));
    let transaction_service = TransactionService::new_with_cache(CacheService::new(100));
    let contact_service = ContactService::new_with_cache(CacheService::new(100));
    let invoice_service = InvoiceService::new_with_cache(CacheService::new(100));
    let payment_service = PaymentService::new_with_cache(CacheService::new(100));
    let reporting_service = ReportingService::new_with_cache(CacheService::new(100));
    let cache_service = CacheService::new(100);

    // Create test app
    let app = create_routes(
        pool.clone(),
        auth_service,
        account_service,
        transaction_service,
        contact_service,
        invoice_service,
        payment_service,
        reporting_service,
        cache_service,
    );

    Ok((pool, app))
}

/// Helper function to create empty test database
async fn create_empty_test_database() -> sqlx::Result<sqlx::PgPool> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Clear all data but keep schema
    sqlx::query("TRUNCATE TABLE transaction_line_items, transactions, contacts, chart_of_accounts, companies, users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;

    Ok(pool)
}