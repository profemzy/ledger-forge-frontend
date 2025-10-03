use serial_test::serial;
use sqlx::Row;

mod common;
use common::{setup_test_db, cleanup_test_db};

#[tokio::test]
#[serial]
async fn test_migrations_run_successfully() {
    let pool = setup_test_db().await;

    // If we got here, migrations ran successfully
    // Verify by checking a known table exists
    let result = sqlx::query("SELECT COUNT(*) as count FROM users")
        .fetch_one(&pool)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_users_table_exists() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let result = sqlx::query(
        "SELECT column_name, data_type
         FROM information_schema.columns
         WHERE table_name = 'users'"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(!result.is_empty());

    let columns: Vec<String> = result.iter()
        .map(|row| row.get("column_name"))
        .collect();

    assert!(columns.contains(&"id".to_string()));
    assert!(columns.contains(&"username".to_string()));
    assert!(columns.contains(&"email".to_string()));
    assert!(columns.contains(&"password_hash".to_string()));
    assert!(columns.contains(&"role".to_string()));
    assert!(columns.contains(&"created_at".to_string()));
    assert!(columns.contains(&"updated_at".to_string()));
}

#[tokio::test]
#[serial]
async fn test_all_core_tables_exist() {
    let pool = setup_test_db().await;

    let expected_tables = vec![
        "users",
        "companies",
        "chart_of_accounts",
        "contacts",
        "transactions",
        "transaction_line_items",
        "invoices",
        "invoice_line_items",
        "bills",
        "bill_line_items",
        "payments",
        "payment_applications",
        "bill_payments",
        "bill_payment_applications",
        "items",
    ];

    for table in expected_tables {
        let result = sqlx::query(&format!("SELECT 1 FROM {} LIMIT 0", table))
            .fetch_optional(&pool)
            .await;

        assert!(
            result.is_ok(),
            "Table '{}' should exist",
            table
        );
    }
}

#[tokio::test]
#[serial]
async fn test_users_unique_constraints() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    // Insert first user
    let result1 = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
         VALUES (gen_random_uuid(), 'testuser', 'test@example.com', 'hash', 'viewer', NOW(), NOW())"
    )
    .execute(&pool)
    .await;

    assert!(result1.is_ok());

    // Try to insert duplicate username
    let result2 = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
         VALUES (gen_random_uuid(), 'testuser', 'different@example.com', 'hash', 'viewer', NOW(), NOW())"
    )
    .execute(&pool)
    .await;

    assert!(result2.is_err()); // Should fail due to unique constraint

    cleanup_test_db(&pool).await;

    // Try to insert duplicate email
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
         VALUES (gen_random_uuid(), 'user1', 'test@example.com', 'hash', 'viewer', NOW(), NOW())"
    )
    .execute(&pool)
    .await
    .unwrap();

    let result3 = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
         VALUES (gen_random_uuid(), 'user2', 'test@example.com', 'hash', 'viewer', NOW(), NOW())"
    )
    .execute(&pool)
    .await;

    assert!(result3.is_err()); // Should fail due to unique constraint
}

#[tokio::test]
#[serial]
async fn test_chart_of_accounts_foreign_key() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    // Try to insert account without valid company
    let result = sqlx::query(
        "INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, created_at, updated_at)
         VALUES (gen_random_uuid(), gen_random_uuid(), '1000', 'Test Account', 'Asset', NOW(), NOW())"
    )
    .execute(&pool)
    .await;

    assert!(result.is_err()); // Should fail due to foreign key constraint
}

#[tokio::test]
#[serial]
async fn test_transaction_balance_calculation() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    // Create company
    let company_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO companies (id, name, created_at, updated_at)
         VALUES (gen_random_uuid(), 'Test Company', NOW(), NOW())
         RETURNING id"
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create accounts
    let account1: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, '1000', 'Cash', 'Asset', NOW(), NOW())
         RETURNING id"
    )
    .bind(company_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let account2: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, '3000', 'Revenue', 'Revenue', NOW(), NOW())
         RETURNING id"
    )
    .bind(company_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create transaction
    let transaction_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO transactions (id, company_id, transaction_date, description, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, NOW(), 'Test Transaction', NOW(), NOW())
         RETURNING id"
    )
    .bind(company_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Insert balanced line items
    sqlx::query(
        "INSERT INTO transaction_line_items (id, transaction_id, account_id, debit_amount, credit_amount, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, $2, 100.00, 0, NOW(), NOW())"
    )
    .bind(transaction_id)
    .bind(account1)
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO transaction_line_items (id, transaction_id, account_id, debit_amount, credit_amount, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, $2, 0, 100.00, NOW(), NOW())"
    )
    .bind(transaction_id)
    .bind(account2)
    .execute(&pool)
    .await
    .unwrap();

    // Verify balance
    let balance: (rust_decimal::Decimal, rust_decimal::Decimal) = sqlx::query_as(
        "SELECT
            COALESCE(SUM(debit_amount), 0) as total_debits,
            COALESCE(SUM(credit_amount), 0) as total_credits
         FROM transaction_line_items
         WHERE transaction_id = $1"
    )
    .bind(transaction_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(balance.0, balance.1); // Debits should equal credits
}

#[tokio::test]
#[serial]
async fn test_decimal_precision() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    // Create company and account
    let company_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO companies (id, name, created_at, updated_at)
         VALUES (gen_random_uuid(), 'Test Company', NOW(), NOW())
         RETURNING id"
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let account_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, '1000', 'Cash', 'Asset', NOW(), NOW())
         RETURNING id"
    )
    .bind(company_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let transaction_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO transactions (id, company_id, transaction_date, description, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, NOW(), 'Decimal Test', NOW(), NOW())
         RETURNING id"
    )
    .bind(company_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Test precise decimal amounts
    let test_amount = rust_decimal::Decimal::new(12345, 2); // 123.45

    sqlx::query(
        "INSERT INTO transaction_line_items (id, transaction_id, account_id, debit_amount, credit_amount, created_at, updated_at)
         VALUES (gen_random_uuid(), $1, $2, $3, 0, NOW(), NOW())"
    )
    .bind(transaction_id)
    .bind(account_id)
    .bind(test_amount)
    .execute(&pool)
    .await
    .unwrap();

    // Retrieve and verify
    let stored_amount: rust_decimal::Decimal = sqlx::query_scalar(
        "SELECT debit_amount FROM transaction_line_items WHERE transaction_id = $1"
    )
    .bind(transaction_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(stored_amount, test_amount);
}
