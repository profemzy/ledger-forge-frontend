use ledger_forge::models::{
    Account, AccountType, CreateAccountRequest, CreateTransactionRequest,
    CreateLineItemRequest, TransactionStatus, JournalType
};
use ledger_forge::services::{AccountService, TransactionService, CacheService};
use rust_decimal::Decimal;
use chrono::{Utc, NaiveDate};
use uuid::Uuid;

mod common;
use common::{cleanup_test_db, setup_test_db, setup_test_cache_fallback, clear_test_cache};

// Helper function to create a test account
async fn create_test_account(pool: &sqlx::PgPool, cache: &CacheService, code: &str, name: &str, account_type: AccountType) -> Account {
    let service = AccountService::new_with_cache(cache.clone());
    let req = CreateAccountRequest {
        code: code.to_string(),
        name: name.to_string(),
        account_type,
        parent_account_id: None,
        company_id: None,
    };
    service.create_account(pool, req).await.unwrap()
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_transaction_success() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    // Setup cache
    let cache = setup_test_cache_fallback().await;
    clear_test_cache(&cache).await;

    // Create test accounts
    let cash_account = create_test_account(&pool, &cache, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, &cache, "4000", "Sales Revenue", AccountType::Revenue).await;

    let service = TransactionService::new_with_cache(cache.clone());
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

    let result = service.create_transaction(&pool, req, None).await;
    assert!(result.is_ok());

    let transaction = result.unwrap();
    assert_eq!(transaction.transaction.status, TransactionStatus::Draft);
    assert_eq!(transaction.line_items.len(), 2);
    assert_eq!(transaction.transaction.journal_type, Some(JournalType::General));

    clear_test_cache(&cache).await;
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_transaction_unbalanced() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales Revenue", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Unbalanced transaction".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)), // $100.00
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(5000, 2)), // $50.00 - UNBALANCED!
            },
        ],
    };

    let result = service.create_transaction(&pool, req, None).await;
    assert!(result.is_err()); // Should fail validation

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_transaction_both_debit_and_credit() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Invalid transaction".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: Some(Decimal::new(10000, 2)), // Both debit and credit!
            },
        ],
    };

    let result = service.create_transaction(&pool, req, None).await;
    assert!(result.is_err()); // Should fail validation

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_transaction_invalid_account() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = TransactionService::new();
    let fake_account_id = Uuid::new_v4();

    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Transaction with invalid account".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: fake_account_id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: fake_account_id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let result = service.create_transaction(&pool, req, None).await;
    assert!(result.is_err()); // Should fail account validation

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_transaction_by_id() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let create_req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Get test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(5000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(5000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, create_req, None).await.unwrap();
    let transaction_id = created.transaction.id;

    // Get the transaction
    let result = service.get_transaction_by_id(&pool, transaction_id).await;
    assert!(result.is_ok());

    let fetched = result.unwrap();
    assert_eq!(fetched.transaction.id, transaction_id);
    assert_eq!(fetched.line_items.len(), 2);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_transaction_not_found() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = TransactionService::new();
    let fake_id = Uuid::new_v4();

    let result = service.get_transaction_by_id(&pool, fake_id).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_transactions() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();

    // Create multiple transactions
    for i in 1..=3 {
        let req = CreateTransactionRequest {
            transaction_date: Utc::now().date_naive(),
            description: Some(format!("Transaction {}", i)),
            reference_number: None,
            contact_id: None,
            company_id: None,
            journal_type: None,
            line_items: vec![
                CreateLineItemRequest {
                    account_id: cash_account.id,
                    description: None,
                    debit_amount: Some(Decimal::new(1000 * i, 2)),
                    credit_amount: None,
                },
                CreateLineItemRequest {
                    account_id: revenue_account.id,
                    description: None,
                    debit_amount: None,
                    credit_amount: Some(Decimal::new(1000 * i, 2)),
                },
            ],
        };
        service.create_transaction(&pool, req, None).await.unwrap();
    }

    // List all transactions
    let result = service.list_transactions(&pool, None, None, None).await;
    assert!(result.is_ok());
    let transactions = result.unwrap();
    assert_eq!(transactions.len(), 3);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_transactions_with_status_filter() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();

    // Create transaction
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Draft transaction".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };
    service.create_transaction(&pool, req, None).await.unwrap();

    // List draft transactions
    let result = service.list_transactions(&pool, Some(TransactionStatus::Draft), None, None).await;
    assert!(result.is_ok());
    let transactions = result.unwrap();
    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].status, TransactionStatus::Draft);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_transaction_status_draft_to_posted() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Status test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();
    assert_eq!(created.transaction.status, TransactionStatus::Draft);

    // Update status to Posted
    let result = service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Posted).await;
    assert!(result.is_ok());

    let updated = result.unwrap();
    assert_eq!(updated.status, TransactionStatus::Posted);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_transaction_status_posted_to_void() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Void test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();

    // Post the transaction
    service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Void the transaction
    let result = service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Void).await;
    assert!(result.is_ok());

    let voided = result.unwrap();
    assert_eq!(voided.status, TransactionStatus::Void);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_transaction_status_invalid_transition() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Invalid transition test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();

    // Post the transaction
    service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Try to revert to Draft (should fail)
    let result = service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Draft).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_delete_transaction_draft() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Delete test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();

    // Delete the draft transaction
    let result = service.delete_transaction(&pool, created.transaction.id).await;
    assert!(result.is_ok());

    // Verify it's deleted
    let get_result = service.get_transaction_by_id(&pool, created.transaction.id).await;
    assert!(get_result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_delete_transaction_posted_fails() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Delete posted test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();

    // Post the transaction
    service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Try to delete (should fail)
    let result = service.delete_transaction(&pool, created.transaction.id).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_balance() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();

    // Create and post a transaction
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Balance test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)), // $100.00
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)), // $100.00
            },
        ],
    };

    let created = service.create_transaction(&pool, req, None).await.unwrap();

    // Post the transaction
    service.update_transaction_status(&pool, created.transaction.id, TransactionStatus::Posted).await.unwrap();

    // Get cash account balance (debit = positive for asset)
    let cash_balance = service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance, Decimal::new(10000, 2)); // $100.00

    // Get revenue account balance (credit = negative for revenue)
    let revenue_balance = service.get_account_balance(&pool, revenue_account.id).await.unwrap();
    assert_eq!(revenue_balance, Decimal::new(-10000, 2)); // -$100.00

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_balance_draft_not_included() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let cash_account = create_test_account(&pool, "1000", "Cash", AccountType::Asset).await;
    let revenue_account = create_test_account(&pool, "4000", "Sales", AccountType::Revenue).await;

    let service = TransactionService::new();

    // Create a DRAFT transaction
    let req = CreateTransactionRequest {
        transaction_date: Utc::now().date_naive(),
        description: Some("Draft balance test".to_string()),
        reference_number: None,
        contact_id: None,
        company_id: None,
        journal_type: None,
        line_items: vec![
            CreateLineItemRequest {
                account_id: cash_account.id,
                description: None,
                debit_amount: Some(Decimal::new(10000, 2)),
                credit_amount: None,
            },
            CreateLineItemRequest {
                account_id: revenue_account.id,
                description: None,
                debit_amount: None,
                credit_amount: Some(Decimal::new(10000, 2)),
            },
        ],
    };

    service.create_transaction(&pool, req, None).await.unwrap();

    // Balance should be zero because transaction is not posted
    let cash_balance = service.get_account_balance(&pool, cash_account.id).await.unwrap();
    assert_eq!(cash_balance, Decimal::ZERO);

    cleanup_test_db(&pool).await;
}
