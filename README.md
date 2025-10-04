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

### 🎉 Latest Achievement (Oct 4, 2025)
**Transaction Engine API - Phase 1 COMPLETE!**
- ✅ Double-entry transaction engine with automatic balance validation
- ✅ Transaction status workflow (draft → posted → void)
- ✅ 6 new transaction endpoints (17 total endpoints live!)
- ✅ Account balance calculation
- ✅ 15 comprehensive unit tests
- ✅ Progress: 80% → 100% ✅

### Progress Checklist
- [x] Project initialization with Axum framework
- [x] PostgreSQL database schema design (16 tables)
- [x] QuickBooks-compatible data model
- [x] Double-entry accounting core
- [x] Rust data models with validation (9 models)
- [x] **JWT authentication (COMPLETE)**
- [x] **API error handling & responses**
- [x] **User registration & login**
- [x] **Axum server with CORS & logging**
- [x] **Chart of Accounts API (COMPLETE)**
- [x] **Transaction Engine API (COMPLETE)** ✅ NEW!

## 🏗️ Technology Stack

### Backend
- **Framework:** Axum 0.8.6
- **Runtime:** Tokio 1.47
- **Database:** PostgreSQL + SQLx 0.8
- **Auth:** JWT (jsonwebtoken) + Argon2
- **Validation:** Validator 0.20
- **API Docs:** utoipa 5 + utoipa-swagger-ui 9

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

5. Run the server:
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
│   └── utils/           # Helper functions
├── migrations/          # Database migrations
├── docs/               # Documentation
├── design.md           # Original design document
└── README.md          # This file
```

## 🎯 Features

### Implemented ✅
- **Database Foundation**
  - Double-entry accounting engine (database-level)
  - QuickBooks-compatible schema (16 tables)
  - Type-safe Rust models (9 models)
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

- **Chart of Accounts** (NEW - Oct 4, 2025) ⭐
  - Complete CRUD operations
  - Account hierarchy (parent-child relationships)
  - Account type filtering (Asset, Liability, Equity, Revenue, Expense)
  - Duplicate code prevention
  - Soft delete with transaction validation
  - 6 fully functional API endpoints

### In Progress 🚧
- Transaction management API
- Role-based access control

### Planned 📋
- Financial reporting (P&L, Balance Sheet)
- Invoice management
- Bill & payment processing
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

### Reports (Planned 📋)
- `GET /api/v1/reports/trial-balance` - Trial balance
- `GET /api/v1/reports/profit-loss` - P&L statement
- `GET /api/v1/reports/balance-sheet` - Balance sheet

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

### Phase 2: Core Features
- [ ] Invoice management
- [ ] Payment processing
- [ ] Expense tracking
- [ ] Contact management

### Phase 3: Migration & Reporting
- [ ] QuickBooks import tools
- [ ] Financial reports
- [ ] Trial balance validation
- [ ] Data reconciliation

### Phase 4: Advanced Features
- [ ] Bank reconciliation
- [ ] Multi-currency
- [ ] Advanced reporting
- [ ] WebAssembly frontend

### Phase 5: Production
- [ ] CI/CD pipeline
- [ ] Production deployment
- [ ] User training
- [ ] Go-live

## 🧪 Testing

**Test Coverage:** 69 tests passing ✅ | ~90% coverage

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test auth_service_test         # Auth tests (19)
cargo test --test account_service_test      # Account tests (12)
cargo test --test transaction_service_test  # Transaction tests (15) ✅ NEW!
cargo test --test migrations_test           # Database tests (7)

# Run with output
cargo test -- --nocapture

# Run database tests serially
cargo test -- --test-threads=1
```

### Test Suites

- **Auth Unit Tests** (19 tests ✅) - Authentication service, password hashing, JWT
- **Account Unit Tests** (12 tests ✅) - Account service, CRUD operations, hierarchy
- **Transaction Unit Tests** (15 tests ✅) - Transaction service, double-entry, status workflow ✅ NEW!
- **Database Tests** (7 tests ✅) - Schema, migrations, constraints, precision
- **Integration Tests** (WIP) - API endpoints, full workflows

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

### Main Documentation
- [Design Document](./design.md) - Complete system design
- [Project Status](./docs/PROJECT_STATUS.md) - Current status & progress (✅ 100% Phase 1)
- [**Phase 1 Complete**](./docs/PHASE1_COMPLETE.md) - Phase 1 completion summary 🎉 NEW!

### Milestone Documentation
- [Phase 1: Database](./docs/archive/PHASE1_DATABASE_MILESTONE.md) - Database foundation
- [Phase 1: Authentication](./docs/archive/PHASE1_AUTH_COMPLETE.md) - Auth API
- [Phase 1: Chart of Accounts](./docs/archive/PHASE1_ACCOUNTS_COMPLETE.md) - Accounts API
- [Testing Strategy](./docs/TESTING_STRATEGY.md) - Testing approach
- [Testing Summary](./docs/TESTING_SUMMARY.md) - Test results

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

*Last Updated: October 4, 2025*
*Latest: **Swagger UI Added!** - Interactive API docs with OpenAPI spec - 69 tests passing!* 🎉
