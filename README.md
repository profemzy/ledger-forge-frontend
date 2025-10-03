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

**Phase 1: Foundation & API Authentication** - ✅ 60% Complete

- [x] Project initialization with Axum framework
- [x] PostgreSQL database schema design (16 tables)
- [x] QuickBooks-compatible data model
- [x] Double-entry accounting core
- [x] Rust data models with validation (9 models)
- [x] **JWT authentication (COMPLETE)**
- [x] **API error handling & responses**
- [x] **User registration & login**
- [x] **Axum server with CORS & logging**
- [ ] Chart of Accounts API
- [ ] Transaction API endpoints

## 🏗️ Technology Stack

### Backend
- **Framework:** Axum 0.8.6
- **Runtime:** Tokio 1.47
- **Database:** PostgreSQL + SQLx 0.8
- **Auth:** JWT (jsonwebtoken) + Argon2
- **Validation:** Validator 0.20

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

### In Progress 🚧
- Chart of Accounts API
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

### Chart of Accounts (Planned 📋)
- `GET /api/v1/accounts` - List accounts
- `POST /api/v1/accounts` - Create account
- `GET /api/v1/accounts/:id` - Get account details
- `PUT /api/v1/accounts/:id` - Update account
- `DELETE /api/v1/accounts/:id` - Deactivate account

### Transactions (Planned 📋)
- `GET /api/v1/transactions` - List transactions
- `POST /api/v1/transactions` - Create transaction
- `GET /api/v1/transactions/:id` - Get transaction details
- `PUT /api/v1/transactions/:id/status` - Update status (draft/posted/void)

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

### Phase 1: Foundation (Current)
- [x] Database schema
- [x] Core models
- [ ] Authentication
- [ ] Basic API endpoints

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

**Test Coverage:** 26 tests passing ✅ | ~75% coverage

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test auth_service_test
cargo test --test migrations_test

# Run with output
cargo test -- --nocapture

# Run database tests serially
cargo test -- --test-threads=1
```

### Test Suites

- **Unit Tests** (19 tests ✅) - Authentication service, password hashing, JWT
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

- [Design Document](./design.md) - Complete system design
- [Phase 1 Milestone](./docs/PHASE1_DATABASE_MILESTONE.md) - Database completion
- [QuickBooks Migration](./docs/quickbooks-migration-mapping.md) - Migration guide (planned)

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

*Last Updated: October 3, 2025*
