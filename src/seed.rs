use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::UserRole;

// Helper function to hash passwords
fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    Ok(password_hash)
}

/// Seed the database with minimal data (admin user only)
pub async fn seed_database(pool: &PgPool) -> Result<()> {
    println!("🌱 Starting minimal database seeding (admin user only)...");

    // Check if users already exist
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count > 0 {
        println!("⚠️  Users already exist. Skipping seeding.");
        return Ok(());
    }

    // Start transaction
    let mut tx = pool.begin().await?;

    println!("👤 Creating admin user...");

    // Create admin user
    let admin_id = Uuid::new_v4();
    let admin_password_hash = hash_password("admin123")?;

    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        "#,
    )
    .bind(admin_id)
    .bind("admin")
    .bind("admin@akowe.com")
    .bind(admin_password_hash)
    .bind(UserRole::Admin)
    .execute(&mut *tx)
    .await?;

    println!("✅ Admin user created");

    // Commit transaction
    tx.commit().await?;

    println!("✨ Seeding completed!");

    Ok(())
}
