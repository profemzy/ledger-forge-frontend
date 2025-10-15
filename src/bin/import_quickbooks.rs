use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use dotenvy::dotenv;
use rust_decimal::Decimal;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::str::FromStr;
use uuid::Uuid;

/// Import QuickBooks data from CSV files into LedgerForge
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("📦 QuickBooks Data Importer");
    println!("============================\n");

    let database_url = env::var("DATABASE_URL")
        .context("DATABASE_URL must be set in .env file")?;

    println!("📊 Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("✅ Connected successfully!\n");

    // Create a default company
    let company_id = create_default_company(&pool).await?;
    println!("✅ Created company: InfoTitans LTD\n");

    // Create a default admin user
    let user_id = create_default_user(&pool).await?;
    println!("✅ Created default admin user\n");

    // Import Chart of Accounts from Trial Balance
    println!("📋 Importing Chart of Accounts...");
    let account_map = import_chart_of_accounts(&pool, company_id).await?;
    println!("✅ Imported {} accounts\n", account_map.len());

    // Import Contacts
    println!("👥 Importing Contacts...");
    let contact_map = import_contacts(&pool, company_id).await?;
    println!("✅ Imported {} contacts\n", contact_map.len());

    // Import Transactions from Journal
    println!("📝 Importing Transactions...");
    let transaction_count = import_journal_transactions(&pool, company_id, user_id, &account_map, &contact_map).await?;
    println!("✅ Imported {} transactions\n", transaction_count);

    pool.close().await;

    println!("\n🎉 Import completed successfully!");
    println!("   - Company: InfoTitans LTD");
    println!("   - Accounts: {}", account_map.len());
    println!("   - Contacts: {}", contact_map.len());
    println!("   - Transactions: {}", transaction_count);
    println!("\n💡 You can now start the server:");
    println!("   cargo run\n");

    Ok(())
}

async fn create_default_company(pool: &sqlx::PgPool) -> Result<Uuid> {
    let company_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO companies (id, name, address, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        ON CONFLICT DO NOTHING
        "#
    )
    .bind(company_id)
    .bind("InfoTitans LTD")
    .bind("Canada")
    .execute(pool)
    .await?;

    Ok(company_id)
}

async fn create_default_user(pool: &sqlx::PgPool) -> Result<Uuid> {
    // Check if admin user already exists
    let existing_user: Option<(Uuid,)> = sqlx::query_as(
        r#"SELECT id FROM users WHERE username = 'admin' LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;

    if let Some((user_id,)) = existing_user {
        println!("   ℹ️  Using existing admin user");
        return Ok(user_id);
    }

    // Create new admin user if doesn't exist
    let user_id = Uuid::new_v4();

    // Use argon2 to hash the password "admin123"
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password("admin123".as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    sqlx::query(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        "#
    )
    .bind(user_id)
    .bind("admin")
    .bind("admin@infotitans.com")
    .bind(password_hash)
    .bind("admin")
    .execute(pool)
    .await?;

    Ok(user_id)
}

async fn import_chart_of_accounts(pool: &sqlx::PgPool, company_id: Uuid) -> Result<HashMap<String, Uuid>> {
    let csv_path = "data/csv/Trial_balance.csv";
    let file = File::open(csv_path)
        .context(format!("Failed to open {}", csv_path))?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut account_map = HashMap::new();
    let mut account_code = 1000;
    let mut found_header = false;

    for result in rdr.records() {
        let record = result?;

        // Skip until we find the header row (row with "Debit" and "Credit")
        if !found_header {
            if record.get(1) == Some("Debit") && record.get(2) == Some("Credit") {
                found_header = true;
            }
            continue;
        }

        // Get account name from first column
        if let Some(account_name) = record.get(0) {
            let trimmed = account_name.trim();

            // Skip empty rows
            if trimmed.is_empty() {
                continue;
            }

            // Determine account type based on name
            let account_type = determine_account_type(trimmed);

            let account_id = Uuid::new_v4();
            let code = format!("{}", account_code);

            sqlx::query(
                r#"
                INSERT INTO chart_of_accounts (id, code, name, account_type, is_active, company_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
                "#
            )
            .bind(account_id)
            .bind(&code)
            .bind(trimmed)
            .bind(account_type)
            .bind(true)
            .bind(company_id)
            .execute(pool)
            .await?;

            account_map.insert(trimmed.to_string(), account_id);
            account_code += 10;

            println!("   ✓ {} ({}) - {}", code, account_type, trimmed);
        }
    }

    Ok(account_map)
}

fn determine_account_type(account_name: &str) -> &str {
    let lower = account_name.to_lowercase();

    // Assets
    if lower.contains("cash") || lower.contains("checking") || lower.contains("savings")
        || lower.contains("receivable") || lower.contains("computer") || lower.contains("inventory")
        || lower.contains("prepaid") || lower.contains("equipment") {
        return "Asset";
    }

    // Liabilities
    if lower.contains("payable") || lower.contains("loan") || lower.contains("credit card")
        || lower.contains("due to") || lower.contains("gst") || lower.contains("hst")
        || lower.contains("tax payable") {
        return "Liability";
    }

    // Equity
    if lower.contains("equity") || lower.contains("capital") || lower.contains("retained")
        || lower.contains("drawing") || lower.contains("owner") {
        return "Equity";
    }

    // Revenue (Income)
    if lower.contains("revenue") || lower.contains("income") || lower.contains("sales")
        || lower.contains("service") || lower.contains("fees earned") {
        return "Revenue";
    }

    // Default to Expense
    "Expense"
}

async fn import_contacts(pool: &sqlx::PgPool, company_id: Uuid) -> Result<HashMap<String, Uuid>> {
    let mut contact_map = HashMap::new();

    // Import Customers
    let customers = parse_contacts("data/csv/Customers.csv", "Customer").await?;
    for contact in customers {
        let contact_id = insert_contact(pool, &contact, company_id).await?;
        contact_map.insert(contact.name.clone(), contact_id);
        println!("   ✓ Customer: {}", contact.name);
    }

    // Import Suppliers (Vendors)
    let suppliers = parse_contacts("data/csv/Suppliers.csv", "Vendor").await?;
    for contact in suppliers {
        let contact_id = insert_contact(pool, &contact, company_id).await?;
        contact_map.insert(contact.name.clone(), contact_id);
        println!("   ✓ Vendor: {}", contact.name);
    }

    // Import Employees
    let employees = parse_contacts("data/csv/Employees.csv", "Employee").await?;
    for contact in employees {
        let contact_id = insert_contact(pool, &contact, company_id).await?;
        contact_map.insert(contact.name.clone(), contact_id);
        println!("   ✓ Employee: {}", contact.name);
    }

    Ok(contact_map)
}

#[derive(Debug)]
struct Contact {
    name: String,
    contact_type: String,
    email: Option<String>,
    phone: Option<String>,
    billing_address: Option<String>,
}

async fn parse_contacts(csv_path: &str, contact_type: &str) -> Result<Vec<Contact>> {
    let file = File::open(csv_path)
        .context(format!("Failed to open {}", csv_path))?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut contacts = Vec::new();
    let mut found_header = false;

    for result in rdr.records() {
        let record = result?;

        // Look for header row
        if !found_header {
            if record.get(1).unwrap_or("").to_lowercase().contains("customer")
                || record.get(1).unwrap_or("").to_lowercase().contains("supplier")
                || record.get(1).unwrap_or("").to_lowercase().contains("employee") {
                found_header = true;
            }
            continue;
        }

        // Get contact name from second column (index 1)
        if let Some(name) = record.get(1) {
            let trimmed = name.trim();

            // Skip empty rows and timestamp rows
            if trimmed.is_empty() || trimmed.to_lowercase().contains("monday") {
                continue;
            }

            let phone = record.get(2).and_then(|s| {
                let t = s.trim();
                if t.is_empty() { None } else { Some(t.to_string()) }
            });

            let email = record.get(3).and_then(|s| {
                let t = s.trim();
                if t.is_empty() { None } else { Some(t.to_string()) }
            });

            let billing_address = record.get(5).and_then(|s| {
                let t = s.trim();
                if t.is_empty() { None } else { Some(t.to_string()) }
            });

            contacts.push(Contact {
                name: trimmed.to_string(),
                contact_type: contact_type.to_string(),
                email,
                phone,
                billing_address,
            });
        }
    }

    Ok(contacts)
}

async fn insert_contact(pool: &sqlx::PgPool, contact: &Contact, company_id: Uuid) -> Result<Uuid> {
    let contact_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO contacts (id, contact_type, name, email, phone, billing_address, company_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        "#
    )
    .bind(contact_id)
    .bind(&contact.contact_type)
    .bind(&contact.name)
    .bind(&contact.email)
    .bind(&contact.phone)
    .bind(&contact.billing_address)
    .bind(company_id)
    .execute(pool)
    .await?;

    Ok(contact_id)
}

async fn import_journal_transactions(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    user_id: Uuid,
    account_map: &HashMap<String, Uuid>,
    _contact_map: &HashMap<String, Uuid>,
) -> Result<usize> {
    let csv_path = "data/csv/Journal.csv";
    let file = File::open(csv_path)
        .context(format!("Failed to open {}", csv_path))?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut found_header = false;
    let mut current_transaction: Option<TransactionData> = None;
    let mut transaction_count = 0;

    for result in rdr.records() {
        let record = result?;

        // Look for header row
        if !found_header {
            if record.get(1) == Some("Date") {
                found_header = true;
            }
            continue;
        }

        // Check if this is a new transaction (has a date in column 1)
        if let Some(date_str) = record.get(1) {
            let date_trimmed = date_str.trim();

            // If we have a current transaction and this row has a date, save the current transaction
            if !date_trimmed.is_empty() && current_transaction.is_some() {
                if let Some(trans) = current_transaction.take() {
                    if trans.is_balanced() {
                        save_transaction(pool, &trans, company_id, user_id, account_map).await?;
                        transaction_count += 1;
                        if transaction_count % 10 == 0 {
                            println!("   ✓ Imported {} transactions...", transaction_count);
                        }
                    }
                }
            }

            // Start a new transaction if this row has a date
            if !date_trimmed.is_empty() && date_trimmed != "Date" {
                let transaction_date = parse_date(date_trimmed)?;
                let transaction_type = record.get(2).unwrap_or("").trim().to_string();
                let reference = record.get(3).unwrap_or("").trim().to_string();
                let contact_name = record.get(4).unwrap_or("").trim().to_string();
                let description = record.get(5).unwrap_or("").trim().to_string();

                current_transaction = Some(TransactionData {
                    date: transaction_date,
                    transaction_type,
                    reference,
                    contact_name,
                    description,
                    line_items: Vec::new(),
                });
            }
        }

        // Add line item to current transaction
        if let Some(ref mut trans) = current_transaction {
            if let Some(account_name) = record.get(6) {
                let account_trimmed = account_name.trim();
                if !account_trimmed.is_empty() && account_trimmed != "Account" {
                    let debit = parse_decimal(record.get(7).unwrap_or("0.0"))?;
                    let credit = parse_decimal(record.get(8).unwrap_or("0.0"))?;

                    trans.line_items.push(LineItemData {
                        account_name: account_trimmed.to_string(),
                        debit_amount: debit,
                        credit_amount: credit,
                    });
                }
            }
        }
    }

    // Save the last transaction
    if let Some(trans) = current_transaction {
        if trans.is_balanced() {
            save_transaction(pool, &trans, company_id, user_id, account_map).await?;
            transaction_count += 1;
        }
    }

    Ok(transaction_count)
}

#[derive(Debug)]
struct TransactionData {
    date: NaiveDate,
    #[allow(dead_code)]
    transaction_type: String,
    reference: String,
    contact_name: String,
    description: String,
    line_items: Vec<LineItemData>,
}

#[derive(Debug)]
struct LineItemData {
    account_name: String,
    debit_amount: Decimal,
    credit_amount: Decimal,
}

impl TransactionData {
    fn is_balanced(&self) -> bool {
        let total_debit: Decimal = self.line_items.iter().map(|li| li.debit_amount).sum();
        let total_credit: Decimal = self.line_items.iter().map(|li| li.credit_amount).sum();
        total_debit == total_credit && total_debit > Decimal::ZERO
    }
}

async fn save_transaction(
    pool: &sqlx::PgPool,
    trans: &TransactionData,
    company_id: Uuid,
    user_id: Uuid,
    account_map: &HashMap<String, Uuid>,
) -> Result<()> {
    let transaction_id = Uuid::new_v4();

    // Insert transaction
    sqlx::query(
        r#"
        INSERT INTO transactions (id, transaction_date, description, reference_number, company_id, journal_type, status, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        "#
    )
    .bind(transaction_id)
    .bind(trans.date)
    .bind(format!("{} - {}", trans.description, trans.contact_name))
    .bind(&trans.reference)
    .bind(company_id)
    .bind("General")
    .bind("posted")
    .bind(user_id)
    .execute(pool)
    .await?;

    // Insert line items
    for line_item in &trans.line_items {
        if let Some(&account_id) = account_map.get(&line_item.account_name) {
            let line_id = Uuid::new_v4();
            sqlx::query(
                r#"
                INSERT INTO transaction_line_items (id, transaction_id, account_id, description, debit_amount, credit_amount, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
                "#
            )
            .bind(line_id)
            .bind(transaction_id)
            .bind(account_id)
            .bind(&line_item.account_name)
            .bind(line_item.debit_amount)
            .bind(line_item.credit_amount)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Try DD/MM/YYYY format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
        return Ok(date);
    }

    // Try other formats...
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date);
    }

    anyhow::bail!("Unable to parse date: {}", date_str)
}

fn parse_decimal(value: &str) -> Result<Decimal> {
    let cleaned = value.trim().replace(",", "");
    if cleaned.is_empty() {
        return Ok(Decimal::ZERO);
    }
    Decimal::from_str(&cleaned).context(format!("Failed to parse decimal: {}", value))
}
