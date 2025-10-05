use anyhow::Result;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    println!("🗑️  Akowe Database Cleaner");
    println!("==========================\n");

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    println!("⚠️  WARNING: This will delete ALL data from the database!");
    println!("   Database: {}\n", mask_password(&database_url));

    print!("Are you sure you want to continue? (yes/no): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "yes" {
        println!("\n❌ Aborted.");
        return Ok(());
    }

    println!("\n📊 Connecting to database...");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Connected successfully!\n");

    println!("🧹 Clearing all data...");

    // Delete data in correct order (respecting foreign keys)
    let tables = vec![
        "transaction_line_items",
        "transactions",
        "invoice_line_items",
        "invoices",
        "bill_line_items",
        "bills",
        "payments",
        "item_variants",
        "items",
        "contacts",
        "chart_of_accounts",
        "companies",
        "users",
    ];

    for table in tables {
        match sqlx::query(&format!("DELETE FROM {}", table))
            .execute(&pool)
            .await
        {
            Ok(result) => {
                let rows = result.rows_affected();
                if rows > 0 {
                    println!("   ✓ Deleted {} rows from {}", rows, table);
                }
            }
            Err(e) => {
                // Table might not exist, that's okay
                println!("   ⚠ Skipped {} ({})", table, e);
            }
        }
    }

    // Reset sequences if needed
    println!("\n🔄 Resetting sequences...");
    // PostgreSQL will auto-reset sequences when tables are empty

    pool.close().await;

    println!("\n✅ Database cleared successfully!");
    println!("   All data has been removed.\n");
    println!("💡 You can now run the seeder:");
    println!("   cargo run --bin seed");
    println!("   or");
    println!("   ./scripts/seed-dev.sh\n");

    Ok(())
}

fn mask_password(url: &str) -> String {
    // Mask password in connection string for display
    if let Some(at_pos) = url.rfind('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range(colon_pos + 1..at_pos, "****");
            return masked;
        }
    }
    url.to_string()
}
