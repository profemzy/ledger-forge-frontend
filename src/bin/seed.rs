use anyhow::Result;
use dotenvy::dotenv;
use ledger_forge::seed::seed_database;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 LedgerForge Database Seeder");
    println!("==============================\n");

    // Safety check: Only allow seeding in development environment
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
    
    if environment.to_lowercase() != "development" {
        eprintln!("❌ ERROR: Database seeding is only allowed in development environment!");
        eprintln!("   Current environment: {}", environment);
        eprintln!("   Set ENVIRONMENT=development in your .env file to enable seeding.");
        std::process::exit(1);
    }

    println!("✅ Environment check passed: {}", environment);

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    println!("📊 Connecting to database...");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Connected successfully!\n");

    // Run seeding
    match seed_database(&pool).await {
        Ok(_) => {
            println!("\n✨ Seeding completed successfully!");
            println!("\n📝 Sample credentials:");
            println!("   Admin:");
            println!("     Username: admin");
            println!("     Password: admin123");
            println!("\n   Accountant:");
            println!("     Username: accountant");
            println!("     Password: accountant123");
            println!("\n🔗 You can now log in to the application with these credentials.\n");
        }
        Err(e) => {
            eprintln!("\n❌ Seeding failed: {}", e);
            std::process::exit(1);
        }
    }

    // Close the pool
    pool.close().await;

    Ok(())
}
