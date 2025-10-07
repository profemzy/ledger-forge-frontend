use chrono::NaiveDate;
use rust_decimal::Decimal;

/// Simple validation tests for financial reporting data
/// These tests validate the data integrity and basic reporting functionality

#[sqlx::test]
async fn test_database_seeding_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Clear any existing data
    sqlx::query("TRUNCATE TABLE transaction_line_items, transactions, contacts, chart_of_accounts, companies, users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;

    // Use Rust seeding system instead of SQL migration
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Basic connectivity test
    let account_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chart_of_accounts")
        .fetch_one(&pool)
        .await?;

    println!("✅ Database connectivity verified");
    println!("   Chart of accounts: {} records", account_count);

    // Verify transactions exist
    let transaction_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transactions")
        .fetch_one(&pool)
        .await?;

    println!("   Transactions: {} records", transaction_count);

    // Verify line items exist
    let line_item_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transaction_line_items")
        .fetch_one(&pool)
        .await?;

    println!("   Transaction line items: {} records", line_item_count);

    // Verify contacts exist
    let contact_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts")
        .fetch_one(&pool)
        .await?;

    println!("   Contacts: {} records", contact_count);

    // Should have data from our seed script
    assert!(account_count > 0, "Chart of accounts should have records");
    assert!(transaction_count > 0, "Should have transactions");
    assert!(line_item_count > 0, "Should have transaction line items");
    assert!(contact_count > 0, "Should have contacts");

    Ok(())
}

#[sqlx::test]
async fn test_trial_balance_data_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Use Rust seeding system
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Test trial balance logic directly in SQL
    let result = sqlx::query!(
        r#"
        WITH account_balances AS (
            SELECT
                a.id,
                a.code,
                a.name,
                a.account_type::text as account_type,
                COALESCE(SUM(
                    CASE
                        WHEN a.account_type IN ('Asset', 'Expense') THEN tl.debit_amount - tl.credit_amount
                        ELSE tl.credit_amount - tl.debit_amount
                    END
                ), 0) as balance
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
            LEFT JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date <= $1
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name, a.account_type
            HAVING ABS(COALESCE(SUM(
                CASE
                    WHEN a.account_type IN ('Asset', 'Expense') THEN tl.debit_amount - tl.credit_amount
                    ELSE tl.credit_amount - tl.debit_amount
                END
            ), 0)) > 0.01
        )
        SELECT
            COUNT(*) as entry_count,
            COALESCE(SUM(CASE
                WHEN account_type IN ('Asset', 'Expense') AND balance > 0 THEN balance
                WHEN account_type IN ('Liability', 'Equity', 'Revenue') AND balance < 0 THEN ABS(balance)
                ELSE 0
            END), 0) as total_debits,
            COALESCE(SUM(CASE
                WHEN account_type IN ('Asset', 'Expense') AND balance < 0 THEN ABS(balance)
                WHEN account_type IN ('Liability', 'Equity', 'Revenue') AND balance > 0 THEN balance
                ELSE 0
            END), 0) as total_credits
        FROM account_balances
        "#,
        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    )
    .fetch_one(&pool)
    .await?;

    let entry_count: i64 = result.entry_count.unwrap_or(0);
    let total_debits: Decimal = result.total_debits.unwrap_or_else(|| Decimal::ZERO);
    let total_credits: Decimal = result.total_credits.unwrap_or_else(|| Decimal::ZERO);

    println!("✅ Trial balance data validation");
    println!("   Entry count: {}", entry_count);
    println!("   Total debits: {}", total_debits);
    println!("   Total credits: {}", total_credits);

    // Verify trial balance is balanced
    let difference = total_debits - total_credits;
    println!("   Difference: {} (should be close to 0)", difference);
    assert!(difference.abs() < Decimal::new(1, 2), // Within $0.01
            "Trial balance should be balanced");

    assert!(entry_count > 0, "Should have trial balance entries");
    assert!(total_debits > Decimal::ZERO, "Should have debits");
    assert!(total_credits > Decimal::ZERO, "Should have credits");

    Ok(())
}

#[sqlx::test]
async fn test_profit_loss_data_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Use Rust seeding system
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Test P&L logic directly in SQL
    let result = sqlx::query!(
        r#"
        WITH revenue_entries AS (
            SELECT
                COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0) as total_revenue
            FROM chart_of_accounts a
            INNER JOIN transaction_line_items tl ON a.id = tl.account_id
            INNER JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date BETWEEN $1 AND $2
                AND a.account_type = 'Revenue'
                AND a.is_active = true
                AND t.status = 'posted'
        ),
        expense_entries AS (
            SELECT
                COALESCE(SUM(tl.debit_amount - tl.credit_amount), 0) as total_expenses
            FROM chart_of_accounts a
            INNER JOIN transaction_line_items tl ON a.id = tl.account_id
            INNER JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date BETWEEN $1 AND $2
                AND a.account_type = 'Expense'
                AND a.is_active = true
                AND t.status = 'posted'
        )
        SELECT
            r.total_revenue,
            e.total_expenses,
            (r.total_revenue - e.total_expenses) as net_income
        FROM revenue_entries r, expense_entries e
        "#,
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    )
    .fetch_one(&pool)
    .await?;

    let total_revenue: Decimal = result.total_revenue.unwrap_or_else(|| Decimal::ZERO);
    let total_expenses: Decimal = result.total_expenses.unwrap_or_else(|| Decimal::ZERO);
    let net_income: Decimal = result.net_income.unwrap_or_else(|| Decimal::ZERO);

    println!("✅ Profit & Loss data validation");
    println!("   Total revenue: {}", total_revenue);
    println!("   Total expenses: {}", total_expenses);
    println!("   Net income: {}", net_income);

    // Verify P&L calculations
    let calculated_net_income = total_revenue - total_expenses;
    assert_eq!(net_income, calculated_net_income, "Net income should equal revenue minus expenses");

    // Our seed data should include both revenue and expenses
    assert!(total_revenue > Decimal::ZERO, "Should have revenue from test data");
    assert!(total_expenses > Decimal::ZERO, "Should have expenses from test data");

    Ok(())
}

#[sqlx::test]
async fn test_balance_sheet_data_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Use Rust seeding system
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Test balance sheet logic directly in SQL
    let result = sqlx::query!(
        r#"
        WITH account_balances AS (
            SELECT
                a.account_type,
                COALESCE(SUM(
                    CASE
                        WHEN a.account_type IN ('Asset', 'Expense') THEN tl.debit_amount - tl.credit_amount
                        ELSE tl.credit_amount - tl.debit_amount
                    END
                ), 0) as balance
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
            LEFT JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date <= $1
                AND t.status = 'posted'
                AND a.is_active = true
            GROUP BY a.account_type
        )
        SELECT
            COALESCE(SUM(CASE WHEN account_type = 'Asset' THEN balance ELSE 0 END), 0) as total_assets,
            COALESCE(SUM(CASE WHEN account_type = 'Liability' THEN balance ELSE 0 END), 0) as total_liabilities,
            COALESCE(SUM(CASE WHEN account_type = 'Equity' THEN balance ELSE 0 END), 0) as total_equity
        FROM account_balances
        "#,
        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    )
    .fetch_one(&pool)
    .await?;

    let total_assets: Decimal = result.total_assets.unwrap_or_else(|| Decimal::ZERO);
    let total_liabilities: Decimal = result.total_liabilities.unwrap_or_else(|| Decimal::ZERO);
    let total_equity: Decimal = result.total_equity.unwrap_or_else(|| Decimal::ZERO);

    println!("✅ Balance sheet data validation");
    println!("   Total assets: {}", total_assets);
    println!("   Total liabilities: {}", total_liabilities);
    println!("   Total equity: {}", total_equity);

    // Verify accounting equation: Assets = Liabilities + Equity
    let calculated_equity = total_assets - total_liabilities;
    let equity_difference = total_equity - calculated_equity;
    println!("   Equity difference: {} (should be close to 0)", equity_difference);
    assert!(equity_difference.abs() < Decimal::new(1, 2), // Within $0.01
            "Balance sheet should balance: Assets = Liabilities + Equity");

    // Should have assets from test data
    assert!(total_assets > Decimal::ZERO, "Should have assets from test data");

    Ok(())
}

#[sqlx::test]
async fn test_transaction_double_entry_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Use Rust seeding system
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Verify that all transactions follow double-entry rules
    let result = sqlx::query!(
        r#"
        SELECT
            t.id,
            t.description,
            SUM(tl.debit_amount) as total_debits,
            SUM(tl.credit_amount) as total_credits,
            COUNT(*) as line_item_count
        FROM transactions t
        INNER JOIN transaction_line_items tl ON t.id = tl.transaction_id
        WHERE t.status = 'posted'
        GROUP BY t.id, t.description
        HAVING SUM(tl.debit_amount) != SUM(tl.credit_amount)
        LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await?;

    println!("✅ Double-entry validation");
    println!("   Unbalanced transactions found: {}", result.len());

    // All transactions should be balanced
    assert_eq!(result.len(), 0, "All posted transactions should be balanced (debits = credits)");

    // Test a few individual transactions to ensure they have proper double entries
    let sample_transactions = sqlx::query!(
        r#"
        SELECT
            t.id,
            t.description,
            SUM(tl.debit_amount) as total_debits,
            SUM(tl.credit_amount) as total_credits,
            COUNT(tl.id) as line_item_count
        FROM transactions t
        INNER JOIN transaction_line_items tl ON t.id = tl.transaction_id
        WHERE t.status = 'posted'
        GROUP BY t.id, t.description
        ORDER BY t.created_at
        LIMIT 3
        "#
    )
    .fetch_all(&pool)
    .await?;

    for tx in sample_transactions {
        let debits = tx.total_debits.unwrap_or_else(|| Decimal::ZERO);
        let credits = tx.total_credits.unwrap_or_else(|| Decimal::ZERO);
        let line_count = tx.line_item_count.unwrap_or(0);

        println!("   Transaction {}: {} - Debits: {}, Credits: {}, Lines: {}",
                 tx.id, tx.description.as_deref().unwrap_or("No description"), debits, credits, line_count);

        assert_eq!(debits, credits, "Each transaction should have equal debits and credits");
        assert!(line_count >= 2, "Each transaction should have at least 2 line items");
    }

    Ok(())
}

#[sqlx::test]
async fn test_account_types_validation() -> sqlx::Result<()> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Use Rust seeding system
    ledger_forge::seed::seed_database(&pool).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    // Verify we have all required account types
    let result = sqlx::query!(
        r#"
        SELECT
            account_type,
            COUNT(*) as count,
            STRING_AGG(DISTINCT code, ', ') as sample_codes
        FROM chart_of_accounts
        WHERE is_active = true
        GROUP BY account_type
        ORDER BY account_type
        "#
    )
    .fetch_all(&pool)
    .await?;

    println!("✅ Account types validation");

    let mut has_assets = false;
    let mut has_liabilities = false;
    let mut has_equity = false;
    let mut has_revenue = false;
    let mut has_expenses = false;

    for row in result {
        let account_type = row.account_type;
        let count = row.count.unwrap_or(0);
        let sample_codes = row.sample_codes.unwrap_or_else(|| String::new());

        println!("   {}: {} accounts (e.g., {})", account_type, count, sample_codes);

        match account_type.as_str() {
            "Asset" => has_assets = true,
            "Liability" => has_liabilities = true,
            "Equity" => has_equity = true,
            "Revenue" => has_revenue = true,
            "Expense" => has_expenses = true,
            _ => {}
        }

        assert!(count > 0, "Each account type should have at least one account");
    }

    // Verify we have all required account types
    assert!(has_assets, "Should have Asset accounts");
    assert!(has_liabilities, "Should have Liability accounts");
    assert!(has_equity, "Should have Equity accounts");
    assert!(has_revenue, "Should have Revenue accounts");
    assert!(has_expenses, "Should have Expense accounts");

    println!("   ✅ All required account types present");

    Ok(())
}