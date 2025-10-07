use axum_test::TestServer;
use serde_json::Value;

mod common;
use common::{setup_test_db, cleanup_test_db, TEST_JWT_SECRET};

use ledger_forge::services::{AuthService, AccountService, TransactionService, ContactService, InvoiceService, PaymentService, ReportingService, CacheService};
use ledger_forge::routes::create_routes;

#[tokio::test]
async fn test_health_endpoint_returns_200() {
    let pool = setup_test_db().await;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let auth_service = AuthService::new(TEST_JWT_SECRET.to_string());
    let account_service = AccountService::new_with_cache(cache_service.clone());
    let transaction_service = TransactionService::new();
    let contact_service = ContactService::new();
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let reporting_service = ReportingService::new_with_cache(cache_service.clone());
    let app = create_routes(pool, auth_service, account_service, transaction_service, contact_service, invoice_service, payment_service, reporting_service, cache_service);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/v1/health").await;

    response.assert_status_ok();

    let json: Value = response.json();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_health_endpoint_checks_database() {
    let pool = setup_test_db().await;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let auth_service = AuthService::new(TEST_JWT_SECRET.to_string());
    let account_service = AccountService::new_with_cache(cache_service.clone());
    let transaction_service = TransactionService::new();
    let contact_service = ContactService::new();
    let invoice_service = InvoiceService::new_with_cache(cache_service.clone());
    let payment_service = PaymentService::new_with_cache(cache_service.clone());
    let reporting_service = ReportingService::new_with_cache(cache_service.clone());
    let app = create_routes(pool, auth_service, account_service, transaction_service, contact_service, invoice_service, payment_service, reporting_service, cache_service);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/v1/health").await;

    let json: Value = response.json();
    assert_eq!(json["database"], "healthy");
}
