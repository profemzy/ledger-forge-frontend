# LedgerForge Development Guide

**Last Updated:** October 7, 2025
**Current Phase:** Phase 3 - Advanced Features (🚀 IN PROGRESS)
**Status:** Production-ready with Bill Management & CSV Import complete

---

## 🎯 Quick Start

```bash
# Clone and setup
git clone <repository-url>
cd ledger-forge
cp .env.example .env

# Setup database
createuser ledger_user -P
createdb ledger_forge -O ledger_user
sqlx migrate run

# Start server
cargo run

# View API docs
open http://localhost:3000/swagger-ui/
```

---

## 📊 Current Status

### ✅ **Phase 1: Foundation (100% Complete)**
- Database schema with 16 tables
- JWT authentication system
- Chart of Accounts API (7 endpoints)
- Transaction Engine API (5 endpoints)
- Double-entry accounting validation

### ✅ **Phase 2: Core Features (100% COMPLETE!)**
- ✅ Contact Management API (8 endpoints) - Customers, Vendors, Employees
- ✅ Invoice Management API (6 endpoints) - Complete CRUD with line items
- ✅ Financial Reporting API (4 endpoints) - Trial Balance, P&L, Balance Sheet, AR Aging
- ✅ Payment Processing API (6 endpoints) - Customer & Vendor payments
- ✅ Comprehensive seed data with 40+ transactions
- ✅ Redis caching layer - 60% query reduction
- ✅ Performance optimizations
- ✅ 160+ comprehensive tests with validation

### 🚀 **Phase 3: Advanced Features (IN PROGRESS)**
- ✅ **Bill Management API (7 endpoints)** - Accounts Payable ✨
- ✅ **CSV Import System (2 endpoints)** - Chart of Accounts ✨ NEW!
- [ ] Bank Reconciliation (next)
- [ ] Advanced Financial Reports

### 📈 **Live API Endpoints: 50 total**
- Health & Status (1)
- Authentication (4)
- Chart of Accounts (7)
- Transactions (5)
- **Contacts (8)** ✅
- **Invoices (6)** ✅
- **Payments (6)** ✅
- **Bills (7)** ✅
- **Import (2)** ✅ 🎉 NEW!
- **Financial Reporting (4)** ✅

### 🧪 **Test Coverage: 177+ tests (~93%)**
- Auth: 19 tests
- Accounts: 12 tests
- Transactions: 15 tests
- **Contacts: 20 tests** ✅
- **Invoice Service & API: 20+ tests** ✅
- **Payment Processing: 10 tests** ✅
- **Bill Management: 6 tests** ✅
- **CSV Import: 11 tests** ✅ 🎉 NEW!
- **Financial Reporting: 39+ tests** ✅
  - Integration tests: 8 scenarios
  - Unit tests: 15+ tests
  - Data validation tests: 6+ tests
- Cache: 10 tests
- Database: 7 tests

---

## 🏗️ Architecture

### Technology Stack
- **Backend:** Axum 0.8.6 + Tokio 1.47
- **Database:** PostgreSQL + SQLx 0.8
- **Cache:** Redis 6+
- **Auth:** JWT + Argon2
- **Docs:** Swagger UI (OpenAPI 3.0)

### Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── models/              # Data models (User, Account, Transaction, Contact)
├── services/            # Business logic layer
├── handlers/            # API request handlers
├── routes/              # API route definitions
├── middleware/          # JWT authentication
└── utils/               # Error handling & helpers
```

### Database Schema (16 Tables)
**Core Tables:**
- `users` - Authentication & roles
- `companies` - Multi-tenancy
- `chart_of_accounts` - Account hierarchy
- `contacts` - Customers/Vendors/Employees
- `transactions` - Journal entries
- `transaction_line_items` - Double-entry lines

**QuickBooks Compatible:**
- `invoices`, `invoice_line_items` - Customer invoicing
- `bills`, `bill_line_items` - Vendor bills (AP)
- `payments`, `payment_applications` - Customer payments (AR)
- `bill_payments`, `bill_payment_applications` - Vendor payments
- `items` - Products/Services catalog

---

## 🔧 Development

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test auth_service_test        # Auth tests (19)
cargo test --test account_service_test     # Account tests (12)
cargo test --test transaction_service_test # Transaction tests (15)
cargo test --test contact_service_test     # Contact tests (20)
cargo test --test invoice_api_test          # Invoice tests (20+)
cargo test --test payment_service_test      # Payment tests (10)
cargo test --test bill_service_test         # Bill tests (6)
cargo test --test import_service_test       # Import tests (11) 🎉 NEW!
cargo test --test cache_test               # Cache tests (10)

# Run with output
cargo test -- --nocapture

# Run database tests serially
cargo test -- --test-threads=1
```

### Code Quality
```bash
# Check compilation
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Database Management
```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Clear and seed database
cargo run --bin clear
cargo run --bin seed
```

---

## 📚 API Documentation

### Access Points
- **Swagger UI:** http://localhost:3000/swagger-ui/
- **OpenAPI Spec:** http://localhost:3000/api-docs/openapi.json

### Authentication
All endpoints (except `/health` and auth endpoints) require JWT token:

```bash
# Get token
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"SecurePassword123"}'

# Use token
curl http://localhost:3000/api/v1/accounts \
  -H 'Authorization: Bearer <your-token>'
```

### Key Endpoints

#### Accounts (7 endpoints)
```bash
# List accounts
GET /api/v1/accounts

# Create account
POST /api/v1/accounts

# Get account balance
GET /api/v1/accounts/{id}/balance
```

#### Transactions (5 endpoints)
```bash
# Create transaction (double-entry)
POST /api/v1/transactions

# Update status
PUT /api/v1/transactions/{id}/status
```

#### Contacts (8 endpoints) ✅
```bash
# List contacts
GET /api/v1/contacts

# Get customers only
GET /api/v1/contacts/customers

# Create contact
POST /api/v1/contacts
```

#### Invoices (6 endpoints) ✅
```bash
# Create invoice with line items
POST /api/v1/invoices

# Get invoice details
GET /api/v1/invoices/{id}

# Update invoice status
PUT /api/v1/invoices/{id}/status

# Get overdue invoices
GET /api/v1/invoices/overdue
```

#### Payments (6 endpoints) ✅
```bash
# Create customer payment
POST /api/v1/payments

# List payments
GET /api/v1/payments

# Apply payment to invoices
PUT /api/v1/payments/{id}/apply

# Get unapplied payments
GET /api/v1/payments/unapplied

# Create vendor bill payment
POST /api/v1/bill-payments
```

#### Bills (7 endpoints) ✅ 🎉 NEW!
```bash
# Create bill with line items
POST /api/v1/bills

# List bills
GET /api/v1/bills

# Get bill details
GET /api/v1/bills/{id}

# Update bill status
PUT /api/v1/bills/{id}/status

# Delete bill
DELETE /api/v1/bills/{id}

# Get overdue bills
GET /api/v1/bills/overdue

# Get vendor bills
GET /api/v1/vendors/{id}/bills
```

#### Import (2 endpoints) ✅ 🎉 NEW!
```bash
# Import Chart of Accounts from CSV
POST /api/v1/import/accounts

# Download CSV template
GET /api/v1/import/accounts/template

# Example: Import accounts
curl -X POST http://localhost:3000/api/v1/import/accounts \
  -H 'Content-Type: text/csv' \
  -H 'Authorization: Bearer <token>' \
  --data-binary @chart_of_accounts.csv

# Example: Download template
curl http://localhost:3000/api/v1/import/accounts/template \
  -H 'Authorization: Bearer <token>' \
  -o template.csv
```

**CSV Template Format:**
```csv
code,name,account_type,parent_code,description
1000,Cash,Asset,,Primary cash account
1010,Checking Account,Asset,1000,Business checking
2000,Accounts Payable,Liability,,Vendor payables
```

**Import Features:**
- Hierarchical account support (parent-child)
- Two-pass import strategy
- Detailed error reporting with row numbers
- Flexible account type parsing (case-insensitive)
- Duplicate code prevention

---

## 🚀 Next Development Steps

### ✅ Phase 2: COMPLETE! 🎉
All core accounting features implemented:
- ✅ Contact Management
- ✅ Invoice Management
- ✅ Payment Processing
- ✅ Financial Reporting
- ✅ 160+ comprehensive tests
- ✅ 41 API endpoints live

### 🚀 Phase 3: Advanced Features (IN PROGRESS)
- ✅ **Bill Management** - COMPLETE! ✨
  - 7 API endpoints for Accounts Payable
  - Full CRUD with line items
  - Status workflow and validation
  - 6 comprehensive tests

- ✅ **CSV Import for Chart of Accounts** - COMPLETE! 🎉
  - 2 API endpoints (import + template)
  - Hierarchical account import
  - Two-pass import strategy
  - Detailed error reporting
  - 11 comprehensive tests

1. **Bank Reconciliation** (Next Priority)
   - Bank statement import
   - Automated transaction matching
   - Reconciliation workflow

2. **QuickBooks Import Tools** (Expand CSV Import)
   - Import contacts, invoices, bills
   - Data mapping and validation
   - Migration utilities

2. **Advanced Financial Reports**
   - Cash flow statements
   - Budget vs Actual reports
   - Custom report builder
   - Multi-period comparisons

3. **Bank Reconciliation**
   - Bank statement import
   - Automated matching
   - Reconciliation workflow

4. **Data Reconciliation**
   - Account reconciliation tools
   - Variance analysis
   - Audit trails

### Phase 4: Advanced Features
- Multi-currency support
- WebAssembly frontend (Leptos/Dioxus)
- Advanced analytics
- Mobile app

---

## 🔒 Security Features

- ✅ **Argon2 password hashing** - Industry standard 2025
- ✅ **JWT authentication** - Access tokens (1hr) + Refresh tokens (7days)
- ✅ **SQL injection prevention** - Prepared statements via SQLx
- ✅ **Input validation** - Comprehensive validation with validator crate
- ✅ **Type safety** - Rust compile-time guarantees
- ✅ **CORS configuration** - Proper cross-origin handling

---

## 📊 Performance Metrics

- **Server startup:** ~2 seconds
- **Database connection:** ~20ms
- **Redis connection:** ~5ms
- **Cache hit response:** ~1-2ms (80-90% faster)
- **Query reduction:** ~60% for repeated lookups
- **Account hierarchy (cached):** ~2-3ms vs 10-15ms uncached

---

## 🛠️ Configuration

### Environment Variables
```bash
# Database
DATABASE_URL=postgresql://user:pass@host:port/db

# Redis
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-secret-key-change-in-production

# Server
PORT=3000
```

### Database Connection
The system uses PostgreSQL with these features:
- Connection pooling (max 5 connections)
- UUID primary keys
- rust_decimal for financial precision
- Double-entry constraints at database level

---

## 🧪 Testing Strategy

### Test Pyramid
- **70% Unit Tests** - Service layer business logic
- **25% Integration Tests** - API endpoint testing
- **5% E2E Tests** - Full workflow testing

### Coverage Areas
- ✅ Authentication service (19 tests)
- ✅ Account service (12 tests)
- ✅ Transaction service (15 tests)
- ✅ Contact service (20 tests)
- ✅ Cache service (10 tests)
- ✅ Database schema (7 tests)

### Test Database
Tests use isolated database environment:
- Automatic test database creation
- Transaction rollback for isolation
- Seed data for consistent test state

---

## 📝 Development Guidelines

### Code Standards
- Use Rust 2024 edition
- Follow clippy lints
- Comprehensive error handling
- Full test coverage for new features
- Update API docs for new endpoints

### Git Workflow
1. Create feature branch from main
2. Implement feature with tests
3. Update documentation
4. Submit PR for review
5. Merge to main after approval

### When to Update This Guide
- After completing each major milestone
- When adding new API endpoints
- When technology stack changes
- When deployment process changes

---

## 🔗 Quick Links

- **Main README:** ../README.md
- **Development Details:** DEVELOPMENT.md
- **Deployment Instructions:** DEPLOYMENT.md
- **API Documentation:** http://localhost:3000/swagger-ui/

---

## 📞 Support

For questions about:
- **Current development status** → This guide
- **Architecture decisions** → DEVELOPMENT.md
- **Setup/deployment issues** → DEPLOYMENT.md
- **API usage** → Swagger UI

---

*Last Updated: October 7, 2025*
*Phase 3 in progress: Bill Management & CSV Import complete! Bank Reconciliation next.*
*This guide is the single source of truth for LedgerForge development status.*