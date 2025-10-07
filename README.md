
# LedgerForge 🔨

**A High-Performance, Rust-Powered Accounting System**

LedgerForge is a custom-built, double-entry accounting system designed to replace QuickBooks with a modern, type-safe, and performant solution built entirely in Rust.

## 🎯 Project Vision

Replace QuickBooks with a bespoke accounting platform that offers:
- **Superior Performance** - Rust's speed and safety
- **Full Customization** - Tailored to specific business needs
- **QuickBooks Migration** - Seamless data import
- **Modern Stack** - Rust backend + WebAssembly frontend
- **Financial Autonomy** - Complete control over financial data

## 🚀 Current Status

**Phase 1: Foundation & Core Engine** - ✅ 100% COMPLETE!
**Phase 2: Core Features** - ✅ 100% COMPLETE!
**Phase 3: Advanced Features** - 🚀 IN PROGRESS!

### 🎉 Latest Achievements (October 7, 2025) ✨
**Bill Management System - PHASE 3 STARTED!**
- ✅ **Bill Management API with 7 endpoints** (Accounts Payable)
- ✅ **Bill CRUD operations** (Create, read, update, delete)
- ✅ **Bill status workflow** (open → partial → paid → void)
- ✅ **Overdue bill tracking** (Automatic aging)
- ✅ **Vendor bill management** (Complete AP workflow)
- ✅ **48 total API endpoints live** (+7 new bill endpoints)
- ✅ **166+ comprehensive tests** (~93% coverage) (+6 new bill tests)
- ✅ **Phase 3: Started with Bill Management!** 🚀

### Progress Checklist

**Phase 1 (✅ Complete):**
- [x] Project initialization with Axum framework
- [x] PostgreSQL database schema design (16 tables)
- [x] QuickBooks-compatible data model
- [x] Double-entry accounting core
- [x] Rust data models with validation (10 models)
- [x] JWT authentication (4 endpoints)
- [x] API error handling & responses
- [x] User registration & login
- [x] Axum server with CORS & logging
- [x] Chart of Accounts API (7 endpoints)
- [x] Transaction Engine API (5 endpoints)

**Phase 2 (✅ 100% Complete):**
- [x] **Redis caching layer** ✨
- [x] **Performance optimizations** ✨
- [x] **Contact Management API (8 endpoints)** ✨
- [x] **Invoice Management API (6 endpoints)** ✨
- [x] **Line items with discount calculations** ✨
- [x] **Invoice status workflow** ✨
- [x] **Financial Reporting API (4 endpoints)** ✨
- [x] **Trial Balance generation** ✨
- [x] **Profit & Loss statements** ✨
- [x] **Balance Sheet generation** ✨
- [x] **AR Aging reports** ✨
- [x] **Comprehensive seed data** ✨
- [x] **Financial data integrity validation** ✨
- [x] **Payment Processing API (6 endpoints)** ✨
- [x] **Customer payment processing** ✨
- [x] **Payment application to invoices** ✨
- [x] **Bill payment processing** ✨
- [x] **160+ comprehensive tests** ✨
- [x] **API integration testing** ✨

**Phase 3 (🚀 In Progress):**
- [x] **Bill Management API (7 endpoints)** ✨ NEW!
- [x] **Bill CRUD operations** ✨ NEW!
- [x] **Bill status workflow** ✨ NEW!
- [x] **Overdue bill tracking** ✨ NEW!
- [x] **166+ comprehensive tests** ✨ NEW!
- [ ] **CSV Import for Chart of Accounts** (next)
- [ ] **Bank Reconciliation**
- [ ] **Advanced Financial Reports**

## 🏗️ Technology Stack

### Backend
- **Framework:** Axum 0.8.6
- **Runtime:** Tokio 1.47
- **Database:** PostgreSQL + SQLx 0.8
- **Cache:** Redis 6+ (caching layer) ✨ NEW!
- **Auth:** JWT (jsonwebtoken) + Argon2
- **Validation:** Validator 0.20
- **API Docs:** Swagger UI (utoipa 5.0) ✨

### Data Types
- **UUID:** 1.18 (v4 generation)
- **Decimal:** rust_decimal 1.36 (financial precision)
- **DateTime:** Chrono 0.4

### Future Frontend
- **Framework:** Leptos or Dioxus (WebAssembly)
- **Language:** Rust (compiled to Wasm)

## 📊 Database Schema

### Core Entities
- **Users** - Authentication & role-based access
- **Companies** - Multi-tenancy support
- **Chart of Accounts** - Hierarchical account structure
- **Contacts** - Customers, Vendors, Employees
- **Transactions** - Double-entry journal entries
- **Transaction Line Items** - Debit/Credit entries

### QuickBooks Compatibility
- **Invoices** - Customer billing
- **Bills** - Vendor invoices (AP)
- **Payments** - Customer payments (AR)
- **Bill Payments** - Vendor payments
- **Items** - Products/Services catalog

## 🔧 Setup

### Prerequisites
- Rust 1.90+ (edition 2024)
- PostgreSQL 14+
- Redis 6+
- SQLx CLI

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ledger-forge
```

2. Set up environment:
```bash
cp .env.example .env
# Edit .env with your database credentials
# IMPORTANT: Set ENVIRONMENT=development for local development
```

3. Create database:
```bash
createuser ledger_user -P  # Enter password when prompted
createdb ledger_forge -O ledger_user
```

4. Run migrations:
```bash
sqlx migrate run
```

5. **Seed the database (Development Only):**
```bash
# Option 1: Using the script
./scripts/seed.sh

# Option 2: Using cargo directly
cargo run --bin seed

# Option 3: Using the release binary
cargo build --bin seed --release
./target/release/seed
```

**⚠️ IMPORTANT:** Database seeding is **only allowed in development environment**. The seeder will check the `ENVIRONMENT` variable and refuse to run if it's not set to `development`. This prevents accidental seeding of production databases.

6. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3000` (or the port specified in your `.env` file).

## 📚 API Documentation

LedgerForge includes interactive API documentation powered by Swagger UI.

**Access Swagger UI:**
```
http://localhost:3000/swagger-ui/
```

**OpenAPI Specification:**
```
http://localhost:3000/api-docs/openapi.json
```

The Swagger UI provides:
- Interactive API testing
- Complete schema definitions for all models
- Request/response examples
- Authentication support (JWT Bearer tokens)

## 🚀 Quick Start

### 1. Check Server Health
```bash
curl http://localhost:3000/api/v1/health
```

### 2. Register a User
```bash
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H 'Content-Type: application/json' \
  -d '{
    "username": "admin",
    "email": "admin@example.com",
    "password": "YourSecurePassword123",
    "role": "admin"
  }'
```

### 3. Login
```bash
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H 'Content-Type: application/json' \
  -d '{
    "username": "admin",
    "password": "YourSecurePassword123"
  }'
```

This returns an `access_token` that you can use for authenticated requests.

### 4. Get Current User (Protected Route)
```bash
curl http://localhost:3000/api/v1/auth/me \
  -H 'Authorization: Bearer <your-access-token>'
```

### 5. Create a Chart of Accounts (NEW!) ⭐
```bash
# Create a Cash account
curl -X POST http://localhost:3000/api/v1/accounts \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer <your-access-token>' \
  -d '{
    "code": "1000",
    "name": "Cash",
    "account_type": "Asset"
  }'
```

### 6. List All Accounts
```bash
# List all active accounts
curl http://localhost:3000/api/v1/accounts \
  -H 'Authorization: Bearer <your-access-token>'

# Filter by account type
curl "http://localhost:3000/api/v1/accounts?account_type=asset" \
  -H 'Authorization: Bearer <your-access-token>'
```

## 🌱 Database Seeding

### Using Seeded Data (Development Only)

After running the seeder, you can log in with these credentials:

**Admin User:**
- Username: `admin`
- Password: `admin123`

**Accountant User:**
- Username: `accountant`
- Password: `accountant123`

### Seed Data Includes:
- 2 Users (admin, accountant)
- 1 Company
- 14 Chart of Accounts (Assets, Liabilities, Equity, Revenue, Expenses)
- 8 Sample Transactions (capital investment, sales, expenses)
- 3 Contacts (customers and vendors)
- 3 Invoices (draft, sent, overdue)

### Safety Features:
- ✅ **Environment Check** - Only runs in `ENVIRONMENT=development`
- ✅ **Idempotent** - Checks for existing data before seeding
- ✅ **Separate Binary** - Never runs automatically with the main app
- ✅ **Production Safe** - Cannot accidentally seed production databases

## 🗄️ Database Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## 📁 Project Structure

```
ledger-forge/
├── src/
│   ├── models/          # Data models & DTOs
│   ├── handlers/        # API request handlers
│   ├── services/        # Business logic
│   ├── routes/          # API route definitions
│   ├── middleware/      # Auth, logging, etc.
│   ├── utils/           # Helper functions
│   ├── seed.rs          # Database seeding logic
│   └── bin/
│       ├── seed.rs      # Seeding binary
│       └── clear.rs     # Database clearing binary
├── migrations/          # Database migrations
├── scripts/             # Helper scripts
├── docs/               # Documentation
├── design.md           # Original design document
└── README.md          # This file
```

## 🎯 Features

### Implemented ✅
- **Database Foundation**
  - Double-entry accounting engine (database-level)
  - QuickBooks-compatible schema (16 tables)
  - Type-safe Rust models (10+ models)
  - Database migrations with SQLx
  - UUID-based entities

- **Authentication & Security**
  - JWT token authentication (access + refresh tokens)
  - Argon2 password hashing
  - User registration & login
  - Token validation & expiry
  - Input validation with validator crate

- **API Infrastructure**
  - Axum web server with async runtime
  - Centralized error handling
  - Standardized JSON responses
  - CORS configuration
  - Request tracing & logging
  - Health check endpoint
  - **Swagger UI** - Interactive API documentation

- **Chart of Accounts** ✅
  - Complete CRUD operations (7 endpoints)
  - Account hierarchy (parent-child relationships)
  - Account type filtering (Asset, Liability, Equity, Revenue, Expense)
  - Duplicate code prevention
  - Soft delete with transaction validation
  - Redis caching for performance

- **Transaction Engine** ✅
  - Double-entry transaction validation
  - Transaction status workflow (draft → posted → void)
  - Account balance calculations
  - Automatic balance validation (debits = credits)

- **Contact Management** ✅ (NEW!)
  - Customer, Vendor, Employee management (8 endpoints)
  - Contact type filtering and pagination
  - Email validation and business rules
  - Transaction protection on deletes
  - Redis caching integration

- **Performance Optimizations** ✅ (NEW!)
  - Redis caching layer (60% query reduction)
  - Account data caching (10-min TTL)
  - Account hierarchy caching (30-min TTL)
  - Smart cache invalidation

- **Financial Reporting System** ✅
  - Trial Balance generation with account validation
  - Profit & Loss statements with revenue/expense aggregation
  - Balance Sheet generation with accounting equation validation
  - Accounts Receivable aging with bucket analysis
  - Comprehensive seed data (40+ transactions, full year activity)
  - Financial data integrity validation (SQL-level verification)
  - Redis caching for report performance

- **Payment Processing System** ✅ NEW!
  - Customer payment processing (6 endpoints)
  - Payment application to invoices (full, partial, multiple)
  - Unapplied payment tracking and management
  - Bill payment processing for vendors
  - Automatic invoice balance updates
  - Payment method tracking (Check, Cash, Credit Card, etc.)
  - Redis caching for payment data

- **Bill Management System** ✅ NEW!
  - Bill CRUD operations (7 endpoints)
  - Bill status workflow (open → partial → paid → void)
  - Overdue bill tracking and management
  - Vendor bill history
  - Automatic total calculation from line items
  - Payment validation on delete
  - Redis caching for bill data

### Phase 2 Complete! Phase 3 In Progress! 🎉
All core accounting features implemented. Now adding advanced features.

### Planned 📋
- CSV Import for Chart of Accounts (in progress)
- Bank reconciliation
- QuickBooks data migration tools
- WebAssembly frontend
- Multi-currency support

## 📖 API Endpoints

### Health & Status (LIVE ✅)
- `GET /api/v1/health` - Health check & database status

### Authentication (LIVE ✅)
- `POST /api/v1/auth/register` - User registration
  ```bash
  curl -X POST http://localhost:3000/api/v1/auth/register \
    -H 'Content-Type: application/json' \
    -d '{"username":"admin","email":"admin@example.com","password":"SecurePass123","role":"admin"}'
  ```
- `POST /api/v1/auth/login` - User login
  ```bash
  curl -X POST http://localhost:3000/api/v1/auth/login \
    -H 'Content-Type: application/json' \
    -d '{"username":"admin","password":"SecurePass123"}'
  ```
- `POST /api/v1/auth/refresh` - Token refresh
- `GET /api/v1/auth/me` - Get current user (requires auth header)
  ```bash
  curl http://localhost:3000/api/v1/auth/me \
    -H 'Authorization: Bearer <your-jwt-token>'
  ```

### Chart of Accounts (LIVE ✅)
- `GET /api/v1/accounts` - List accounts (with filtering)
  ```bash
  # List all active accounts
  curl http://localhost:3000/api/v1/accounts \
    -H 'Authorization: Bearer <token>'

  # Filter by account type
  curl "http://localhost:3000/api/v1/accounts?account_type=asset" \
    -H 'Authorization: Bearer <token>'
  ```
- `POST /api/v1/accounts` - Create account
  ```bash
  curl -X POST http://localhost:3000/api/v1/accounts \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{"code":"1000","name":"Cash","account_type":"Asset"}'
  ```
- `GET /api/v1/accounts/{id}` - Get account details
- `PUT /api/v1/accounts/{id}` - Update account
  ```bash
  curl -X PUT http://localhost:3000/api/v1/accounts/{id} \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{"name":"Updated Account Name"}'
  ```
- `DELETE /api/v1/accounts/{id}` - Deactivate account
- `GET /api/v1/accounts/{id}/hierarchy` - Get account with parent and children
- `GET /api/v1/accounts/{id}/balance` - Get account balance

### Transactions (LIVE ✅)
- `GET /api/v1/transactions` - List transactions
  ```bash
  # List all transactions
  curl http://localhost:3000/api/v1/transactions \
    -H 'Authorization: Bearer <token>'

  # Filter by status
  curl "http://localhost:3000/api/v1/transactions?status=posted" \
    -H 'Authorization: Bearer <token>'
  ```
- `POST /api/v1/transactions` - Create transaction
  ```bash
  curl -X POST http://localhost:3000/api/v1/transactions \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{
      "transaction_date": "2025-10-04",
      "description": "Service rendered",
      "reference_number": "INV-001",
      "journal_type": "Sales",
      "line_items": [
        {
          "account_id": "CASH_ACCOUNT_ID",
          "description": "Cash received",
          "debit_amount": "500.00"
        },
        {
          "account_id": "REVENUE_ACCOUNT_ID",
          "description": "Service revenue",
          "credit_amount": "500.00"
        }
      ]
    }'
  ```
- `GET /api/v1/transactions/{id}` - Get transaction details
- `PUT /api/v1/transactions/{id}/status` - Update status (draft/posted/void)
  ```bash
  curl -X PUT http://localhost:3000/api/v1/transactions/{id}/status \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{"status": "posted"}'
  ```
- `DELETE /api/v1/transactions/{id}` - Delete draft transaction

### Contacts (LIVE ✅)
- `GET /api/v1/contacts` - List all contacts
- `POST /api/v1/contacts` - Create contact
- `GET /api/v1/contacts/{id}` - Get contact details
- `PUT /api/v1/contacts/{id}` - Update contact
- `DELETE /api/v1/contacts/{id}` - Delete contact
- `GET /api/v1/contacts/customers` - List customers only
- `GET /api/v1/contacts/vendors` - List vendors only
- `GET /api/v1/contacts/employees` - List employees only

### Invoices (LIVE ✅)
- `GET /api/v1/invoices` - List invoices
- `POST /api/v1/invoices` - Create invoice with line items
  ```bash
  curl -X POST http://localhost:3000/api/v1/invoices \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{
      "invoice_number": "INV-001",
      "customer_id": "CUSTOMER_ID",
      "invoice_date": "2024-10-01",
      "due_date": "2024-10-31",
      "line_items": [
        {
          "line_number": 1,
          "item_description": "Consulting Services",
          "quantity": "10",
          "unit_price": "150.00",
          "revenue_account_id": "REVENUE_ACCOUNT_ID"
        }
      ]
    }'
  ```
- `GET /api/v1/invoices/{id}` - Get invoice details
- `PUT /api/v1/invoices/{id}/status` - Update invoice status
- `GET /api/v1/invoices/overdue` - Get overdue invoices
- `GET /api/v1/customers/{id}/invoices` - Get customer invoices

### Payments (LIVE ✅) 🎉 NEW!
- `POST /api/v1/payments` - Create customer payment
  ```bash
  curl -X POST http://localhost:3000/api/v1/payments \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{
      "customer_id": "CUSTOMER_ID",
      "payment_date": "2024-10-15",
      "amount": "1500.00",
      "payment_method": "Check",
      "reference_number": "CHK-12345",
      "deposit_to_account_id": "BANK_ACCOUNT_ID",
      "applications": [
        {
          "invoice_id": "INVOICE_ID",
          "amount_applied": "1500.00"
        }
      ]
    }'
  ```
- `GET /api/v1/payments` - List payments
  ```bash
  # List all payments
  curl http://localhost:3000/api/v1/payments \
    -H 'Authorization: Bearer <token>'
  
  # Filter by customer
  curl "http://localhost:3000/api/v1/payments?customer_id=CUSTOMER_ID" \
    -H 'Authorization: Bearer <token>'
  
  # Get unapplied payments only
  curl "http://localhost:3000/api/v1/payments?unapplied_only=true" \
    -H 'Authorization: Bearer <token>'
  ```
- `GET /api/v1/payments/{id}` - Get payment details
- `PUT /api/v1/payments/{id}/apply` - Apply payment to invoices
  ```bash
  curl -X PUT http://localhost:3000/api/v1/payments/{id}/apply \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{
      "applications": [
        {
          "invoice_id": "INVOICE_ID",
          "amount_applied": "500.00"
        }
      ]
    }'
  ```
- `GET /api/v1/payments/unapplied` - Get unapplied payments
- `GET /api/v1/invoices/{id}/payments` - Get payments for an invoice
- `POST /api/v1/bill-payments` - Create vendor bill payment
  ```bash
  curl -X POST http://localhost:3000/api/v1/bill-payments \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer <token>' \
    -d '{
      "vendor_id": "VENDOR_ID",
      "payment_date": "2024-10-15",
      "amount": "2500.00",
      "payment_method": "Check",
      "reference_number": "CHK-VENDOR-001",
      "bank_account_id": "BANK_ACCOUNT_ID",
      "applications": [
        {
          "bill_id": "BILL_ID",
          "amount_applied": "2500.00"
        }
      ]
    }'
  ```

### Financial Reporting (LIVE ✅)
- `GET /api/v1/reports/trial-balance` - Trial balance
  ```bash
  # Generate trial balance as of specific date
  curl "http://localhost:3000/api/v1/reports/trial-balance?as_of_date=2024-12-31" \
    -H 'Authorization: Bearer <token>'
  ```
- `GET /api/v1/reports/profit-loss` - P&L statement
  ```bash
  # Generate P&L for date range
  curl "http://localhost:3000/api/v1/reports/profit-loss?start_date=2024-01-01&end_date=2024-12-31" \
    -H 'Authorization: Bearer <token>'
  ```
- `GET /api/v1/reports/balance-sheet` - Balance sheet
  ```bash
  # Generate balance sheet as of specific date
  curl "http://localhost:3000/api/v1/reports/balance-sheet?as_of_date=2024-12-31" \
    -H 'Authorization: Bearer <token>'
  ```
- `GET /api/v1/reports/ar-aging` - Accounts Receivable aging
  ```bash
  # Generate AR aging report as of specific date
  curl "http://localhost:3000/api/v1/reports/ar-aging?as_of_date=2024-12-31" \
    -H 'Authorization: Bearer <token>'
  ```

## 🔐 Security

- **Password Hashing:** Argon2 (recommended for 2025)
- **Authentication:** JWT tokens with refresh capability
- **Database:** Prepared statements (SQL injection prevention)
- **Input Validation:** Comprehensive validation with `validator`
- **Type Safety:** Rust's compile-time guarantees

## 📈 Development Roadmap

### Phase 1: Foundation (✅ 100% Complete)
- [x] Database schema
- [x] Core models
- [x] Authentication
- [x] Chart of Accounts API
- [x] Transaction API

### Phase 2: Core Features (✅ 100% Complete)
- [x] Invoice management ✅
- [x] Contact management ✅
- [x] Financial reporting ✅
- [x] Payment processing ✅

### Phase 3: Advanced Features (🚀 In Progress)
- [x] Bill Management ✅
- [ ] CSV Import for Chart of Accounts (next)
- [ ] QuickBooks import tools
- [ ] Advanced reporting features
- [ ] Data reconciliation
- [ ] Bank reconciliation

### Phase 4: Advanced Features
- [ ] Multi-currency
- [ ] Advanced reporting
- [ ] WebAssembly frontend

### Phase 5: Production
- [ ] CI/CD pipeline
- [ ] Production deployment
- [ ] User training
- [ ] Go-live

## 🧪 Testing

**Test Coverage:** 166+ tests passing ✅ | ~93% coverage

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test auth_service_test         # Auth tests (19)
cargo test --test account_service_test      # Account tests (12)
cargo test --test transaction_service_test  # Transaction tests (15)
cargo test --test contact_service_test      # Contact tests (20)
cargo test --test invoice_api_test          # Invoice API tests (20+)
cargo test --test payment_service_test      # Payment tests (10)
cargo test --test bill_service_test         # Bill tests (6) 🎉 NEW!
cargo test --test cache_test                # Cache tests (10)
cargo test --test migrations_test           # Database tests (7)
cargo test --test financial_reporting_test     # Financial reporting integration tests
cargo test --test reporting_service_test       # Financial reporting unit tests
cargo test --test financial_reporting_validation_test # Data validation tests

# Run with output
cargo test -- --nocapture

# Run database tests serially
cargo test -- --test-threads=1
```

### Test Suites

- **Auth Unit Tests** (19 tests ✅) - Authentication service, password hashing, JWT
- **Account Unit Tests** (12 tests ✅) - Account service, CRUD operations, hierarchy
- **Transaction Unit Tests** (15 tests ✅) - Transaction service, double-entry, status workflow
- **Contact Unit Tests** (20 tests ✅) - Contact service, CRUD operations, validation
- **Invoice Tests** (20+ tests ✅) - Invoice service, API endpoints, status workflow
- **Payment Tests** (10 tests ✅) - Payment processing, applications, bill payments
- **Bill Tests** (6 tests ✅) - Bill CRUD, status workflow, overdue tracking 🎉 NEW!
- **Financial Reporting Tests** (39+ tests ✅) - Complete reporting system testing
  - Integration tests (8 scenarios) - End-to-end API testing
  - Unit tests (15+ tests) - Business logic validation
  - Data validation tests (6+ tests) - SQL-level verification
- **Cache Unit Tests** (10 tests ✅) - Redis caching, invalidation, performance
- **Database Tests** (7 tests ✅) - Schema, migrations, constraints, precision
- **Integration Tests** (10 scenarios ✅) - Full API workflow testing

See [tests/README.md](./tests/README.md) for detailed testing guide.

### Other Commands

```bash
# Check compilation
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📚 Documentation

### Core Documentation
- **[Development Guide](./docs/GUIDE.md)** ⭐ - Complete project status, features, and development progress
- **[Architecture & Design](./docs/DEVELOPMENT.md)** - Technology decisions and implementation details
- **[Deployment & Setup](./docs/DEPLOYMENT.md)** - Installation, configuration, and deployment instructions
- **[Design Document](./design.md)** - Original system architecture blueprint

### API Documentation
- **Swagger UI:** http://localhost:3000/swagger-ui/ - Interactive API testing
- **OpenAPI Spec:** http://localhost:3000/api-docs/openapi.json - API specification

## 🤝 Contributing

This is a private project currently under active development.

## 📝 License

Proprietary - All rights reserved

## 🙏 Acknowledgments

- QuickBooks for the feature inspiration
- Rust community for excellent tooling
- PostgreSQL for robust data management

---

**Built with ❤️ and Rust** 🦀

*Last Updated: October 7, 2025*
*Latest: **Phase 3 Started!** - Bill Management System with 48 total API endpoints and 166+ comprehensive tests! 🎉*
