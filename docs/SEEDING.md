# Database Seeding

LedgerForge includes a comprehensive database seeding tool to help you quickly set up sample data for development and testing purposes.

## Features

The seeder creates:

- **Users** (2)
  - Admin user with full access
  - Accountant user with accounting privileges

- **Company** (1)
  - Demo company with contact information

- **Chart of Accounts** (14 accounts)
  - **Assets** (4): Cash, Bank Account, Accounts Receivable, Inventory
  - **Liabilities** (2): Accounts Payable, Bank Loan
  - **Equity** (2): Owner's Capital, Retained Earnings
  - **Revenue** (2): Sales Revenue, Service Revenue
  - **Expenses** (4): Rent, Salaries, Utilities, Office Supplies

- **Transactions** (8 journal entries)
  - Initial capital investment
  - Cash sales
  - Services on credit
  - Various expense payments
  - Draft transaction (for testing)

- **Contacts** (3)
  - Sample customers and vendors

## Usage

### Quick Start (Development)

```bash
# From the project root
./scripts/seed-dev.sh
```

### Production Build

```bash
# Build the seeder
./scripts/seed.sh

# Or manually:
cargo build --release --bin seed
./target/release/seed
```

### Direct Cargo Run

```bash
# Run directly with cargo
cargo run --bin seed
```

## Sample Credentials

After seeding, you can log in with these credentials:

### Admin User
- **Username:** `admin`
- **Password:** `admin123`
- **Email:** `admin@akowe.com`
- **Role:** Admin

### Accountant User
- **Username:** `accountant`
- **Password:** `accountant123`
- **Email:** `accountant@akowe.com`
- **Role:** Accountant

## Important Notes

### Safety Features

1. **Existing Data Check**: The seeder checks if the database already contains data before running
2. **Atomic Operations**: All seeding operations run in a single database transaction
3. **Rollback on Error**: If any part of seeding fails, all changes are rolled back

### Re-seeding

To re-seed the database:

```bash
# Reset the database (WARNING: This deletes all data!)
sqlx database drop
sqlx database create
sqlx migrate run

# Then run the seeder
./scripts/seed-dev.sh
```

Or with Docker:

```bash
# Stop and remove the database container
docker-compose down -v

# Start fresh
docker-compose up -d
sleep 5  # Wait for database to be ready

# Run migrations
sqlx migrate run

# Seed the database
./scripts/seed-dev.sh
```

## Sample Data Details

### Chart of Accounts Structure

```
Assets (1000-1999)
├── 1000 - Cash
├── 1010 - Bank Account - Checking
├── 1200 - Accounts Receivable
└── 1300 - Inventory

Liabilities (2000-2999)
├── 2000 - Accounts Payable
└── 2100 - Bank Loan

Equity (3000-3999)
├── 3000 - Owner's Capital
└── 3100 - Retained Earnings

Revenue (4000-4999)
├── 4000 - Sales Revenue
└── 4010 - Service Revenue

Expenses (5000-5999)
├── 5000 - Rent Expense
├── 5010 - Salary Expense
├── 5020 - Utilities Expense
└── 5030 - Office Supplies Expense
```

### Sample Transactions

| Date       | Description                    | Reference | Type      | Status | Amount     |
|------------|--------------------------------|-----------|-----------|--------|------------|
| 2024-10-01 | Initial capital investment     | CAP-001   | General   | Posted | $50,000.00 |
| 2024-10-05 | Cash sale - Product A          | SALE-001  | Sales     | Posted | $2,500.00  |
| 2024-10-07 | Consulting services rendered   | SRV-001   | Sales     | Posted | $3,500.00  |
| 2024-10-10 | Monthly rent payment           | EXP-001   | General   | Posted | $2,000.00  |
| 2024-10-15 | Salary payment - October       | SAL-001   | General   | Posted | $5,000.00  |
| 2024-10-18 | Utilities payment              | UTIL-001  | General   | Posted | $450.00    |
| 2024-10-20 | Office supplies purchase       | SUP-001   | Purchases | Posted | $250.00    |
| 2024-10-25 | Pending sale - Product B       | SALE-002  | Sales     | Draft  | $1,800.00  |

### Sample Contacts

| Name                  | Type     | Email                      | Phone        |
|-----------------------|----------|----------------------------|--------------|
| ABC Corporation       | Customer | contact@abc-corp.com       | +1-555-0101  |
| XYZ Supplies Inc      | Vendor   | sales@xyz-supplies.com     | +1-555-0102  |
| Tech Services Ltd     | Customer | info@techservices.com      | +1-555-0103  |

## Development Workflow

### Typical Development Cycle

```bash
# 1. Start with clean database
docker-compose down -v && docker-compose up -d
sleep 5

# 2. Run migrations
sqlx migrate run

# 3. Seed sample data
./scripts/seed-dev.sh

# 4. Start the server
cargo run
```

### Testing Scenarios

The seeded data is perfect for testing:

- **Authentication**: Test login with different user roles
- **Chart of Accounts**: View account hierarchy and balances
- **Transactions**: Test posting, voiding, and filtering transactions
- **Reports**: Generate trial balance, balance sheet, income statement
- **Contacts**: Test customer and vendor management

## Customization

To customize the seeded data, edit `/src/seed.rs`:

```rust
// Add more accounts
async fn seed_chart_of_accounts(...) {
    // Add your custom accounts here
}

// Add more transactions
async fn seed_transactions(...) {
    // Add your custom transactions here
}

// Add more contacts
async fn seed_contacts(...) {
    // Add your custom contacts here
}
```

## Troubleshooting

### "Database already contains data"

The seeder won't run if data already exists. To reseed:

```bash
# Option 1: Drop and recreate database
sqlx database drop
sqlx database create
sqlx migrate run
./scripts/seed-dev.sh

# Option 2: Manual cleanup (not recommended)
psql $DATABASE_URL -c "DELETE FROM transaction_line_items;"
psql $DATABASE_URL -c "DELETE FROM transactions;"
psql $DATABASE_URL -c "DELETE FROM accounts;"
psql $DATABASE_URL -c "DELETE FROM contacts;"
psql $DATABASE_URL -c "DELETE FROM companies;"
psql $DATABASE_URL -c "DELETE FROM users;"
```

### Permission Errors

Make sure the scripts are executable:

```bash
chmod +x scripts/seed.sh
chmod +x scripts/seed-dev.sh
```

### Connection Errors

Ensure your `.env` file has the correct `DATABASE_URL`:

```env
DATABASE_URL=postgresql://username:password@localhost/ledger_forge
```

## Contributing

When adding new seed data:

1. Keep it realistic and useful for testing
2. Ensure all transactions balance (debits = credits)
3. Use appropriate account codes (1000s for Assets, 2000s for Liabilities, etc.)
4. Add documentation for any new data types
5. Test the seeder after making changes

## License

Part of the LedgerForge project - see main LICENSE file.
