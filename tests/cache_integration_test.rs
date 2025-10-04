use ledger_forge::models::{
    Account, AccountType, CreateAccountRequest, CreateTransactionRequest,
    CreateLineItemRequest, TransactionStatus, JournalType
};
use ledger_forge::services::{AccountService, TransactionService, CacheService};
use rust_decimal::Decimal;
use chrono::{Utc, NaiveDate};
use uuid::Uuid;

mod common;
use common::{cleanup_test_db, setup_test_db, setup_test_cache, clear_test_cache, is_redis_available};

#[tokio::test]
#[serial_test::serial]
async fn test_account_balance_caching() {
    // Skip if Redis is not available
    if !is_redis_available().await {
        println!("Skipping cache test - Redis not available");
        return;
    }

    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cache = setup_test_cache().await;
    clear_test_cache(&cache).await;

    // Create test accounts
    let account_service = AccountService::new_with_cache(cache.clone());
    let cash_account = create_test_account(&account_service, &pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&account_service, &pool, "4000", "Sales Revenue", AccountType::Revenue).await;

    let transaction_service = TransactionService::new_with_cache(cache.clone());

    // Initially, account balances should be zero (no posted transactions)
    let cash_balance = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance, Decimal::ZERO);

    let revenue_balance = transaction_service.get_account_balance(&pool, revenue_account.id).await.unwrap();
    assert_eq!(revenue_balance, Decimal::ZERO);

    // Create and post a transaction
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Test transaction".to_string()),
        reference_number: Some("TXN001".to_string()),
        contact_id: None,
        company_id: None,
        journal_type: Some(JournalType::General),
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: Some("Cash received".to_string()),
                debit_amount: Some(Decimal::new(10000, 2)), // $100.00
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: Some("Sales revenue".to_string()),
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)), // $100.00
            },
        ],
    };

    // Create transaction (should be in draft status)
    let transaction = transaction_service.create_transaction(&pool, req, None).await.unwrap();
    assert_eq!(transaction.transaction.status, TransactionStatus::Draft);

    // Even after creating transaction, balances should still be zero (draft doesn't affect balances)
    let cash_balance = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance, Decimal::ZERO);

    // Post the transaction
    transaction_service.update_transaction_status(&pool, transaction.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Now the balance should be updated
    let cash_balance = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance, Decimal::new(10000, 2)); // $100.00

    let revenue_balance = transaction_service.get_account_balance(&pool, revenue_account.id).await.unwrap();
    assert_eq!(revenue_balance, Decimal::new(-10000, 2)); // -$100.00 (credit balance)

    // Test cache hit: second call should be cached
    let cash_balance_cached = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance_cached, Decimal::new(10000, 2));

    clear_test_cache(&cache).await;
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_cache_invalidation_on_transaction_post() {
    // Skip if Redis is not available
    if !is_redis_available().await {
        println!("Skipping cache invalidation test - Redis not available");
        return;
    }

    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cache = setup_test_cache().await;
    clear_test_cache(&cache).await;

    // Create test accounts
    let account_service = AccountService::new_with_cache(cache.clone());
    let cash_account = create_test_account(&account_service, &pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&account_service, &pool, "4000", "Sales Revenue", AccountType::Revenue).await;

    let transaction_service = TransactionService::new_with_cache(cache.clone());

    // Get initial balance (should be cached)
    let initial_balance = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(initial_balance, Decimal::ZERO);

    // Verify it's cached by checking Redis directly
    let cached_balance = cache.get_account_balance(cash_account.id).await.unwrap();
    assert!(cached_balance.is_some());
    assert_eq!(cached_balance.unwrap(), Decimal::ZERO);

    // Create and post a transaction
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Test transaction".to_string()),
        reference_number: Some("TXN002".to_string()),
        contact_id: None,
        company_id: None,
        journal_type: Some(JournalType::General),
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: Some("Cash received".to_string()),
                debit_amount: Some(Decimal::new(5000, 2)), // $50.00
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: Some("Sales revenue".to_string()),
                debit_amount: None,
                credit_amount: Some(Decimal::new(5000, 2)), // $50.00
            },
        ],
    };

    let transaction = transaction_service.create_transaction(&pool, req, None).await.unwrap();

    // Post the transaction (this should invalidate the cache)
    transaction_service.update_transaction_status(&pool, transaction.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Verify cache was invalidated
    let cached_balance = cache.get_account_balance(cash_account.id).await.unwrap();
    assert!(cached_balance.is_none()); // Should be None after invalidation

    // Get fresh balance (should recalculate and cache new value)
    let new_balance = transaction_service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(new_balance, Decimal::new(5000, 2)); // $50.00

    clear_test_cache(&cache).await;
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_cache_health_check() {
    // Skip if Redis is not available
    if !is_redis_available().await {
        println!("Skipping cache health check test - Redis not available");
        return;
    }

    let cache = setup_test_cache().await;

    // Test health check
    let is_healthy = cache.health_check().await.unwrap();
    assert!(is_healthy);
}

// Helper function to create a test account
async fn create_test_account(service: &AccountService, pool: &sqlx::PgPool, code: &str, name: &str, account_type: AccountType) -> Account {
    let req = CreateAccountRequest {
        code: code.to_string(),
        name: name.to_string(),
        account_type,
        parent_account_id: None,
        company_id: None,
    };
    service.create_account(pool, req).await.unwrap()
}