use anyhow::Result;
use dotenvy::dotenv;
use rust_decimal::Decimal;
use sqlx::postgres::PgPoolOptions;
use std::env;

/// Verify QuickBooks data import integrity
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("\n📊 QuickBooks Import Verification");
    println!("==================================\n");

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 1. Record counts
    println!("1️⃣  Record Counts:");
    println!("   ===============");

    let company_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM companies").fetch_one(&pool).await?;
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&pool).await?;
    let account_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chart_of_accounts").fetch_one(&pool).await?;
    let contact_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts").fetch_one(&pool).await?;
    let transaction_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transactions").fetch_one(&pool).await?;
    let line_item_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transaction_line_items").fetch_one(&pool).await?;

    println!("   Companies:              {}", company_count);
    println!("   Users:                  {}", user_count);
    println!("   Chart of Accounts:      {}", account_count);
    println!("   Contacts:               {}", contact_count);
    println!("   Transactions:           {}", transaction_count);
    println!("   Transaction Line Items: {}", line_item_count);

    // 2. Accounts by type
    println!("\n2️⃣  Chart of Accounts by Type:");
    println!("   ===========================");

    let account_types: Vec<(String, i64)> = sqlx::query_as(
        "SELECT account_type, COUNT(*) FROM chart_of_accounts GROUP BY account_type ORDER BY account_type"
    ).fetch_all(&pool).await?;

    for (account_type, count) in account_types {
        println!("   {:<15} {}", account_type, count);
    }

    // 3. Contacts by type
    println!("\n3️⃣  Contacts by Type:");
    println!("   =================");

    let contact_types: Vec<(String, i64)> = sqlx::query_as(
        "SELECT contact_type, COUNT(*) FROM contacts GROUP BY contact_type ORDER BY contact_type"
    ).fetch_all(&pool).await?;

    for (contact_type, count) in contact_types {
        println!("   {:<15} {}", contact_type, count);
    }

    // 4. Transaction status
    println!("\n4️⃣  Transaction Status:");
    println!("   ===================");

    let statuses: Vec<(String, i64)> = sqlx::query_as(
        "SELECT status, COUNT(*) FROM transactions GROUP BY status ORDER BY status"
    ).fetch_all(&pool).await?;

    for (status, count) in statuses {
        println!("   {:<15} {}", status, count);
    }

    // 5. Double-entry validation
    println!("\n5️⃣  Double-Entry Validation:");
    println!("   =========================");

    #[derive(sqlx::FromRow)]
    struct TransactionBalance {
        id: String,
        reference_number: Option<String>,
        total_debits: Decimal,
        total_credits: Decimal,
        difference: Decimal,
    }

    let balances: Vec<TransactionBalance> = sqlx::query_as(
        r#"
        SELECT
            t.id::text,
            t.reference_number,
            COALESCE(SUM(tli.debit_amount), 0) as total_debits,
            COALESCE(SUM(tli.credit_amount), 0) as total_credits,
            COALESCE(SUM(tli.debit_amount) - SUM(tli.credit_amount), 0) as difference
        FROM transactions t
        LEFT JOIN transaction_line_items tli ON t.id = tli.transaction_id
        GROUP BY t.id, t.reference_number
        "#
    ).fetch_all(&pool).await?;

    let unbalanced: Vec<&TransactionBalance> = balances.iter()
        .filter(|b| b.difference.abs() > Decimal::new(1, 2)) // 0.01
        .collect();

    if unbalanced.is_empty() {
        println!("   ✅ PASSED - All {} transactions are balanced!", balances.len());
    } else {
        println!("   ❌ FAILED - {} out of {} transactions are unbalanced!", unbalanced.len(), balances.len());
        println!("\n   Unbalanced transactions:");
        for trans in unbalanced.iter().take(5) {
            println!("   - Ref: {:?}, Debits: {}, Credits: {}, Diff: {}",
                trans.reference_number, trans.total_debits, trans.total_credits, trans.difference);
        }
    }

    // 6. Account balances
    println!("\n6️⃣  Top 10 Account Balances:");
    println!("   ========================");

    #[derive(sqlx::FromRow)]
    struct AccountBalance {
        code: String,
        name: String,
        account_type: String,
        balance: Decimal,
    }

    let account_balances: Vec<AccountBalance> = sqlx::query_as(
        r#"
        WITH account_balances AS (
            SELECT
                a.code,
                a.name,
                a.account_type,
                CASE
                    WHEN a.account_type IN ('Asset', 'Expense')
                    THEN COALESCE(SUM(tli.debit_amount) - SUM(tli.credit_amount), 0)
                    ELSE COALESCE(SUM(tli.credit_amount) - SUM(tli.debit_amount), 0)
                END as balance
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tli ON a.id = tli.account_id
            GROUP BY a.id, a.code, a.name, a.account_type
        )
        SELECT code, name, account_type, ROUND(balance, 2) as balance
        FROM account_balances
        ORDER BY ABS(balance) DESC
        LIMIT 10
        "#
    ).fetch_all(&pool).await?;

    for acc in account_balances {
        println!("   {} - {} ({}): ${}", acc.code, acc.name, acc.account_type, acc.balance);
    }

    // 7. Date range
    println!("\n7️⃣  Transaction Date Range:");
    println!("   =======================");

    let (min_date, max_date): (Option<String>, Option<String>) = sqlx::query_as(
        "SELECT MIN(transaction_date)::text, MAX(transaction_date)::text FROM transactions"
    ).fetch_one(&pool).await?;

    if let (Some(min), Some(max)) = (min_date, max_date) {
        println!("   Earliest: {}", min);
        println!("   Latest:   {}", max);
    }

    // 8. Sample transactions
    println!("\n8️⃣  Sample Transactions (First 5):");
    println!("   ==============================");

    #[derive(sqlx::FromRow)]
    struct SampleTransaction {
        transaction_date: String,
        reference_number: Option<String>,
        description: Option<String>,
        line_items: i64,
    }

    let samples: Vec<SampleTransaction> = sqlx::query_as(
        r#"
        SELECT
            t.transaction_date::text,
            t.reference_number,
            LEFT(t.description, 50) as description,
            COUNT(tli.id) as line_items
        FROM transactions t
        LEFT JOIN transaction_line_items tli ON t.id = tli.transaction_id
        GROUP BY t.id, t.transaction_date, t.reference_number, t.description
        ORDER BY t.transaction_date
        LIMIT 5
        "#
    ).fetch_all(&pool).await?;

    for sample in samples {
        println!("   {} | Ref: {:?} | Items: {} | {}",
            sample.transaction_date,
            sample.reference_number,
            sample.line_items,
            sample.description.unwrap_or_default()
        );
    }

    pool.close().await;

    println!("\n✅ Verification Complete!\n");

    Ok(())
}
