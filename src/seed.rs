use anyhow::Result;
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{AccountType, JournalType, TransactionStatus, UserRole};

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

// Helper macro for decimal values
macro_rules! dec {
    ($val:literal) => {
        Decimal::from_str_exact(stringify!($val)).unwrap()
    };
}

/// Seed the database with sample data for testing and development
pub async fn seed_database(pool: &PgPool) -> Result<()> {
    println!("🌱 Starting database seeding...");

    // Check if data already exists
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count > 0 {
        println!("⚠️  Database already contains data. Skipping seeding.");
        println!("   To reseed, please run migrations:reset first.");
        return Ok(());
    }

    // Start transaction for atomicity
    let mut tx = pool.begin().await?;

    // Seed Users
    let (admin_id, _accountant_id) = seed_users(&mut tx).await?;
    println!("✅ Seeded 2 users");

    // Seed Company (optional)
    let company_id = seed_company(&mut tx).await?;
    println!("✅ Seeded 1 company");

    // Seed Chart of Accounts
    let account_ids = seed_chart_of_accounts(&mut tx, company_id).await?;
    println!("✅ Seeded {} accounts", account_ids.len());

    // Seed Sample Transactions
    let transaction_count = seed_transactions(&mut tx, &account_ids, admin_id).await?;
    println!("✅ Seeded {} transactions", transaction_count);

    // Seed Contacts
    let contact_count = seed_contacts(&mut tx, company_id).await?;
    println!("✅ Seeded {} contacts", contact_count);

    // Commit transaction
    tx.commit().await?;

    println!("🎉 Database seeding completed successfully!");
    Ok(())
}

async fn seed_users(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<(Uuid, Uuid)> {
    let admin_id = Uuid::new_v4();
    let accountant_id = Uuid::new_v4();
    let now = Utc::now();

    // Admin user (password: admin123)
    let admin_password_hash = hash_password("admin123")?;
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(admin_id)
    .bind("admin")
    .bind("admin@akowe.com")
    .bind(admin_password_hash)
    .bind(UserRole::Admin.to_string())
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Accountant user (password: accountant123)
    let accountant_password_hash = hash_password("accountant123")?;
    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(accountant_id)
    .bind("accountant")
    .bind("accountant@akowe.com")
    .bind(accountant_password_hash)
    .bind(UserRole::Accountant.to_string())
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok((admin_id, accountant_id))
}

async fn seed_company(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<Option<Uuid>> {
    let company_id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO companies (id, name, address, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(company_id)
    .bind("Akowe Demo Company")
    .bind("123 Business St, Suite 100, Business City, BC 12345")
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(Some(company_id))
}

async fn seed_chart_of_accounts(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    company_id: Option<Uuid>,
) -> Result<Vec<(Uuid, String, AccountType)>> {
    let mut accounts = Vec::new();
    let now = Utc::now();

    // Asset Accounts
    let cash_id = Uuid::new_v4();
    accounts.push((cash_id, "1000".to_string(), AccountType::Asset));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(cash_id)
    .bind("1000")
    .bind("Cash")
    .bind(AccountType::Asset.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let bank_id = Uuid::new_v4();
    accounts.push((bank_id, "1010".to_string(), AccountType::Asset));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(bank_id)
    .bind("1010")
    .bind("Bank Account - Checking")
    .bind(AccountType::Asset.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let ar_id = Uuid::new_v4();
    accounts.push((ar_id, "1200".to_string(), AccountType::Asset));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(ar_id)
    .bind("1200")
    .bind("Accounts Receivable")
    .bind(AccountType::Asset.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let inventory_id = Uuid::new_v4();
    accounts.push((inventory_id, "1300".to_string(), AccountType::Asset));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(inventory_id)
    .bind("1300")
    .bind("Inventory")
    .bind(AccountType::Asset.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Liability Accounts
    let ap_id = Uuid::new_v4();
    accounts.push((ap_id, "2000".to_string(), AccountType::Liability));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(ap_id)
    .bind("2000")
    .bind("Accounts Payable")
    .bind(AccountType::Liability.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let loan_id = Uuid::new_v4();
    accounts.push((loan_id, "2100".to_string(), AccountType::Liability));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(loan_id)
    .bind("2100")
    .bind("Bank Loan")
    .bind(AccountType::Liability.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Equity Accounts
    let capital_id = Uuid::new_v4();
    accounts.push((capital_id, "3000".to_string(), AccountType::Equity));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(capital_id)
    .bind("3000")
    .bind("Owner's Capital")
    .bind(AccountType::Equity.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let retained_id = Uuid::new_v4();
    accounts.push((retained_id, "3100".to_string(), AccountType::Equity));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(retained_id)
    .bind("3100")
    .bind("Retained Earnings")
    .bind(AccountType::Equity.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Revenue Accounts
    let sales_id = Uuid::new_v4();
    accounts.push((sales_id, "4000".to_string(), AccountType::Revenue));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(sales_id)
    .bind("4000")
    .bind("Sales Revenue")
    .bind(AccountType::Revenue.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let service_revenue_id = Uuid::new_v4();
    accounts.push((service_revenue_id, "4010".to_string(), AccountType::Revenue));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(service_revenue_id)
    .bind("4010")
    .bind("Service Revenue")
    .bind(AccountType::Revenue.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Expense Accounts
    let rent_id = Uuid::new_v4();
    accounts.push((rent_id, "5000".to_string(), AccountType::Expense));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(rent_id)
    .bind("5000")
    .bind("Rent Expense")
    .bind(AccountType::Expense.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let salary_id = Uuid::new_v4();
    accounts.push((salary_id, "5010".to_string(), AccountType::Expense));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(salary_id)
    .bind("5010")
    .bind("Salary Expense")
    .bind(AccountType::Expense.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let utilities_id = Uuid::new_v4();
    accounts.push((utilities_id, "5020".to_string(), AccountType::Expense));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(utilities_id)
    .bind("5020")
    .bind("Utilities Expense")
    .bind(AccountType::Expense.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    let supplies_id = Uuid::new_v4();
    accounts.push((supplies_id, "5030".to_string(), AccountType::Expense));
    sqlx::query(
        r#"
        INSERT INTO chart_of_accounts (id, code, name, account_type, parent_account_id, is_active, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(supplies_id)
    .bind("5030")
    .bind("Office Supplies Expense")
    .bind(AccountType::Expense.to_string())
    .bind(None::<Uuid>)
    .bind(true)
    .bind(company_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(accounts)
}

async fn seed_transactions(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    accounts: &[(Uuid, String, AccountType)],
    user_id: Uuid,
) -> Result<usize> {
    let _now = Utc::now();

    // Find account IDs by type for easier transaction creation
    let cash = accounts.iter().find(|(_, code, _)| code == "1000").unwrap().0;
    let bank = accounts.iter().find(|(_, code, _)| code == "1010").unwrap().0;
    let ar = accounts.iter().find(|(_, code, _)| code == "1200").unwrap().0;
    let capital = accounts.iter().find(|(_, code, _)| code == "3000").unwrap().0;
    let sales = accounts.iter().find(|(_, code, _)| code == "4000").unwrap().0;
    let service_revenue = accounts.iter().find(|(_, code, _)| code == "4010").unwrap().0;
    let rent = accounts.iter().find(|(_, code, _)| code == "5000").unwrap().0;
    let salary = accounts.iter().find(|(_, code, _)| code == "5010").unwrap().0;
    let utilities = accounts.iter().find(|(_, code, _)| code == "5020").unwrap().0;
    let supplies = accounts.iter().find(|(_, code, _)| code == "5030").unwrap().0;

    let mut count = 0;

    // Transaction 1: Initial Capital Investment
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(),
        "Initial capital investment".to_string(),
        "CAP-001".to_string(),
        JournalType::General,
        TransactionStatus::Posted,
        user_id,
        vec![
            (bank, dec!(50000.00), dec!(0.00), "Initial capital deposit"),
            (capital, dec!(0.00), dec!(50000.00), "Owner's capital contribution"),
        ],
    )
    .await?;

    // Transaction 2: Cash Sale
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 5).unwrap(),
        "Cash sale - Product A".to_string(),
        "SALE-001".to_string(),
        JournalType::Sales,
        TransactionStatus::Posted,
        user_id,
        vec![
            (cash, dec!(2500.00), dec!(0.00), "Cash received from sale"),
            (sales, dec!(0.00), dec!(2500.00), "Revenue from product sale"),
        ],
    )
    .await?;

    // Transaction 3: Service Revenue on Credit
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 7).unwrap(),
        "Consulting services rendered".to_string(),
        "SRV-001".to_string(),
        JournalType::Sales,
        TransactionStatus::Posted,
        user_id,
        vec![
            (ar, dec!(3500.00), dec!(0.00), "Service on credit"),
            (service_revenue, dec!(0.00), dec!(3500.00), "Consulting revenue"),
        ],
    )
    .await?;

    // Transaction 4: Paid Rent
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 10).unwrap(),
        "Monthly rent payment".to_string(),
        "EXP-001".to_string(),
        JournalType::General,
        TransactionStatus::Posted,
        user_id,
        vec![
            (rent, dec!(2000.00), dec!(0.00), "October rent"),
            (bank, dec!(0.00), dec!(2000.00), "Payment from bank"),
        ],
    )
    .await?;

    // Transaction 5: Paid Salaries
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        "Salary payment - October".to_string(),
        "SAL-001".to_string(),
        JournalType::General,
        TransactionStatus::Posted,
        user_id,
        vec![
            (salary, dec!(5000.00), dec!(0.00), "Employee salaries"),
            (bank, dec!(0.00), dec!(5000.00), "Payment from bank"),
        ],
    )
    .await?;

    // Transaction 6: Utilities Bill Paid
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 18).unwrap(),
        "Utilities payment".to_string(),
        "UTIL-001".to_string(),
        JournalType::General,
        TransactionStatus::Posted,
        user_id,
        vec![
            (utilities, dec!(450.00), dec!(0.00), "Electricity and water"),
            (bank, dec!(0.00), dec!(450.00), "Payment from bank"),
        ],
    )
    .await?;

    // Transaction 7: Office Supplies Purchase
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 20).unwrap(),
        "Office supplies purchase".to_string(),
        "SUP-001".to_string(),
        JournalType::Purchases,
        TransactionStatus::Posted,
        user_id,
        vec![
            (supplies, dec!(250.00), dec!(0.00), "Stationery and printer paper"),
            (cash, dec!(0.00), dec!(250.00), "Cash payment"),
        ],
    )
    .await?;

    // Transaction 8: Draft Transaction - Future Sale
    count += create_transaction(
        tx,
        NaiveDate::from_ymd_opt(2024, 10, 25).unwrap(),
        "Pending sale - Product B".to_string(),
        "SALE-002".to_string(),
        JournalType::Sales,
        TransactionStatus::Draft,
        user_id,
        vec![
            (cash, dec!(1800.00), dec!(0.00), "Future cash sale"),
            (sales, dec!(0.00), dec!(1800.00), "Revenue pending"),
        ],
    )
    .await?;

    Ok(count)
}

async fn create_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
    description: String,
    reference: String,
    journal_type: JournalType,
    status: TransactionStatus,
    user_id: Uuid,
    line_items: Vec<(Uuid, Decimal, Decimal, &str)>,
) -> Result<usize> {
    let transaction_id = Uuid::new_v4();
    let now = Utc::now();

    // Insert transaction
    sqlx::query(
        r#"
        INSERT INTO transactions (id, transaction_date, description, reference_number, journal_type, status, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(transaction_id)
    .bind(date)
    .bind(description)
    .bind(reference)
    .bind(journal_type.to_string())
    .bind(status.to_string())
    .bind(user_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    // Insert line items
    for (account_id, debit, credit, desc) in line_items {
        let line_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(line_id)
        .bind(transaction_id)
        .bind(account_id)
        .bind(desc)
        .bind(debit)
        .bind(credit)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(1)
}

async fn seed_contacts(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    company_id: Option<Uuid>,
) -> Result<usize> {
    let now = Utc::now();

    let contacts = vec![
        (
            "ABC Corporation",
            "Customer",
            "contact@abc-corp.com",
            "+1-555-0101",
            "123 Customer Ave, Client City, CC 12345",
            "456 Shipping Rd, Client City, CC 12345",
        ),
        (
            "XYZ Supplies Inc",
            "Vendor",
            "sales@xyz-supplies.com",
            "+1-555-0102",
            "789 Vendor Blvd, Supplier Town, ST 67890",
            "789 Vendor Blvd, Supplier Town, ST 67890",
        ),
        (
            "Tech Services Ltd",
            "Customer",
            "info@techservices.com",
            "+1-555-0103",
            "321 Tech Park, Innovation City, IC 11111",
            "321 Tech Park, Innovation City, IC 11111",
        ),
    ];

    let contact_count = contacts.len();

    for (name, contact_type, email, phone, billing_addr, shipping_addr) in contacts {
        let contact_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO contacts (id, contact_type, name, email, phone, billing_address, shipping_address, company_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(contact_id)
        .bind(contact_type)
        .bind(name)
        .bind(email)
        .bind(phone)
        .bind(billing_addr)
        .bind(shipping_addr)
        .bind(company_id)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(contact_count)
}
