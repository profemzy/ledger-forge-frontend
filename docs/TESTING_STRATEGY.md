# LedgerForge Testing Strategy

**Version:** 1.0
**Last Updated:** October 3, 2025
**Status:** 🟢 Active

---

## 📋 Executive Summary

This document defines the comprehensive testing strategy for LedgerForge, a mission-critical accounting system where data accuracy and security are paramount. Our testing approach ensures financial integrity, regulatory compliance, and production reliability.

---

## 🎯 Testing Objectives

### Primary Goals
1. **Financial Accuracy** - Ensure all calculations are precise to the penny
2. **Data Integrity** - Prevent invalid states and maintain double-entry accounting rules
3. **Security** - Validate authentication, authorization, and data protection
4. **Reliability** - Catch regressions before they reach production
5. **Performance** - Ensure scalability under load

### Success Metrics
- **Code Coverage:** Minimum 80% for business logic, 60% overall
- **Test Execution Time:** < 30 seconds for unit tests, < 5 minutes for full suite
- **Zero Critical Bugs** in authentication and transaction processing
- **100% Double-Entry Validation** coverage

---

## 🏗️ Testing Pyramid

```
                    /\
                   /  \    E2E Tests (5%)
                  /____\   - Full user workflows
                 /      \  - Real database
                /        \
               /__________\ Integration Tests (25%)
              /            \ - API endpoints
             /              \ - Database queries
            /                \ - Service layer
           /____________________\ Unit Tests (70%)
                                  - Pure functions
                                  - Business logic
                                  - Validation
```

### Test Distribution
- **70% Unit Tests** - Fast, isolated, comprehensive coverage
- **25% Integration Tests** - API + database + services
- **5% E2E Tests** - Critical user workflows

---

## 🧪 Test Categories

### 1. Unit Tests

**Scope:** Individual functions, structs, and pure logic

**What to Test:**
- ✅ Password hashing and verification
- ✅ JWT token generation and validation
- ✅ Input validation rules
- ✅ Error handling and conversions
- ✅ Business logic calculations
- ✅ Model serialization/deserialization

**Example Files:**
- `tests/unit/auth_service_test.rs`
- `tests/unit/validation_test.rs`
- `tests/unit/error_handling_test.rs`

### 2. Integration Tests

**Scope:** Multiple components working together

**What to Test:**
- ✅ API endpoint request/response flows
- ✅ Database CRUD operations
- ✅ Authentication middleware
- ✅ Service + database interactions
- ✅ Error propagation across layers

**Example Files:**
- `tests/integration/auth_api_test.rs`
- `tests/integration/database_test.rs`
- `tests/integration/middleware_test.rs`

### 3. Database Tests

**Scope:** Database schema, migrations, and queries

**What to Test:**
- ✅ Migration scripts execute successfully
- ✅ Constraints and indexes work correctly
- ✅ Double-entry accounting validation at DB level
- ✅ Foreign key relationships
- ✅ Unique constraints
- ✅ SQLx queries compile and return correct types

**Example Files:**
- `tests/database/migrations_test.rs`
- `tests/database/constraints_test.rs`
- `tests/database/queries_test.rs`

### 4. End-to-End Tests

**Scope:** Complete user workflows

**What to Test:**
- ✅ User registration → login → authenticated request flow
- ✅ Create transaction → verify balance → generate report
- ✅ Invoice creation → payment → reconciliation

**Example Files:**
- `tests/e2e/auth_workflow_test.rs`
- `tests/e2e/transaction_workflow_test.rs`

---

## 🛠️ Testing Tools & Dependencies

### Core Testing Framework
```toml
[dev-dependencies]
# Testing framework
tokio-test = "0.4"

# HTTP testing
axum-test = "15"      # For testing Axum handlers
reqwest = "0.12"      # HTTP client for integration tests

# Database testing
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }

# Assertions & utilities
assert-json-diff = "2.0"
fake = "2.9"          # Generate fake data
wiremock = "0.6"      # Mock external services

# Test utilities
serial_test = "3"     # Run tests serially when needed
mockall = "0.13"      # Mocking library
```

### Additional Tools
- **cargo-tarpaulin** - Code coverage
- **cargo-nextest** - Faster test runner
- **insta** - Snapshot testing for API responses

---

## 📁 Test Directory Structure

```
ledger-forge/
├── tests/
│   ├── common/
│   │   ├── mod.rs              # Shared test utilities
│   │   ├── fixtures.rs         # Test data fixtures
│   │   ├── test_db.rs          # Test database setup
│   │   └── assertions.rs       # Custom assertions
│   │
│   ├── unit/
│   │   ├── auth_service_test.rs
│   │   ├── password_test.rs
│   │   ├── jwt_test.rs
│   │   ├── validation_test.rs
│   │   └── error_handling_test.rs
│   │
│   ├── integration/
│   │   ├── auth_api_test.rs
│   │   ├── health_check_test.rs
│   │   ├── middleware_test.rs
│   │   └── database_test.rs
│   │
│   ├── database/
│   │   ├── migrations_test.rs
│   │   ├── constraints_test.rs
│   │   └── queries_test.rs
│   │
│   └── e2e/
│       ├── auth_workflow_test.rs
│       └── user_journey_test.rs
│
└── src/
    └── (each module can have inline tests)
```

---

## 🗄️ Test Database Strategy

### Approach: Isolated Test Database per Test Suite

**Configuration:**
```bash
# .env.test
DATABASE_URL=postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge_test
JWT_SECRET=test-secret-key-do-not-use-in-production
```

**Setup Process:**
1. Create test database with unique name
2. Run all migrations
3. Execute tests
4. Clean up (truncate or drop)

**Implementation:**
```rust
// tests/common/test_db.rs
pub async fn setup_test_db() -> PgPool {
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge_test".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn cleanup_test_db(pool: &PgPool) {
    // Truncate all tables
    sqlx::query("TRUNCATE users, companies, chart_of_accounts CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup test database");
}
```

### Isolation Strategy
- **Per-test isolation:** Use transactions and rollback
- **Parallel execution:** Use separate test databases or table prefixes
- **Cleanup:** Automatic via `Drop` trait or explicit cleanup functions

---

## ✅ Test Coverage Requirements

### Critical Paths (100% Coverage Required)
- ✅ Authentication (password hashing, JWT validation)
- ✅ Transaction posting (double-entry validation)
- ✅ Balance calculations
- ✅ Permission checks
- ✅ SQL migrations

### High Priority (90% Coverage)
- ✅ All service layer functions
- ✅ API handlers
- ✅ Input validation
- ✅ Error handling

### Standard (70% Coverage)
- ✅ Models and DTOs
- ✅ Utility functions
- ✅ Middleware

### Excluded from Coverage
- ❌ Main.rs (server startup)
- ❌ Generated code
- ❌ Simple getters/setters

---

## 🔒 Security Testing

### Authentication & Authorization Tests
```rust
#[tokio::test]
async fn test_weak_password_rejected() { /* ... */ }

#[tokio::test]
async fn test_sql_injection_prevented() { /* ... */ }

#[tokio::test]
async fn test_expired_token_rejected() { /* ... */ }

#[tokio::test]
async fn test_unauthorized_access_blocked() { /* ... */ }
```

### Security Checklist
- [ ] Password strength validation
- [ ] SQL injection prevention
- [ ] XSS prevention in responses
- [ ] JWT token expiry enforcement
- [ ] Token refresh security
- [ ] Rate limiting (when implemented)
- [ ] CORS configuration
- [ ] Sensitive data not logged

---

## 💰 Financial Accuracy Testing

### Double-Entry Validation
```rust
#[tokio::test]
async fn test_transaction_must_balance() {
    // Debit total must equal credit total
    // Test at both DB and application level
}

#[tokio::test]
async fn test_decimal_precision() {
    // Ensure no rounding errors
    // Test with edge cases: 0.01, 999999.99
}
```

### Critical Financial Tests
- [ ] Debits = Credits validation
- [ ] Balance calculations correct to 2 decimals
- [ ] No floating-point errors
- [ ] Transaction atomicity (all-or-nothing)
- [ ] Account balance aggregations
- [ ] Report calculations (P&L, Balance Sheet)

---

## 🚀 CI/CD Integration

### GitHub Actions Workflow
```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: testpass
          POSTGRES_DB: ledger_forge_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        run: cargo test --all-features

      - name: Generate coverage
        run: cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Pre-commit Hooks
```bash
# .git/hooks/pre-commit
#!/bin/bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

---

## 📊 Test Execution

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test file
cargo test --test auth_api_test

# With output
cargo test -- --nocapture

# Run serially (for DB tests)
cargo test -- --test-threads=1

# Fast test runner
cargo nextest run

# With coverage
cargo tarpaulin --out Html
```

### Test Naming Convention
```rust
// Pattern: test_[unit]_[scenario]_[expected_outcome]
#[tokio::test]
async fn test_register_valid_user_returns_201() { }

#[tokio::test]
async fn test_login_invalid_password_returns_401() { }

#[tokio::test]
async fn test_jwt_expired_token_rejected() { }
```

---

## 🐛 Debugging Failed Tests

### Best Practices
1. **Use descriptive test names** - Immediately know what failed
2. **Add debug output** - Use `dbg!()` or `println!()` with `-- --nocapture`
3. **Isolate the failure** - Run single test with `cargo test test_name`
4. **Check test database state** - Query DB directly if needed
5. **Use breakpoints** - Debug with rust-lldb or VS Code

### Common Issues
- **Flaky tests:** Usually due to shared state or timing
- **DB connection errors:** Check test database is running
- **Port conflicts:** Ensure test server uses unique ports
- **Environment variables:** Use `.env.test` for test config

---

## 📈 Test Metrics & Reporting

### Key Metrics to Track
1. **Code Coverage** - Aim for 80%+ on business logic
2. **Test Count** - Target: 200+ tests by MVP
3. **Execution Time** - Keep under 5 minutes
4. **Flaky Test Rate** - Target: 0%
5. **Bug Escape Rate** - Bugs found in production vs tests

### Coverage Goals by Module
| Module | Target Coverage | Current | Status |
|--------|----------------|---------|--------|
| services/auth.rs | 95% | 0% | 🔴 Not Started |
| handlers/auth.rs | 90% | 0% | 🔴 Not Started |
| models/* | 70% | 0% | 🔴 Not Started |
| utils/errors.rs | 85% | 0% | 🔴 Not Started |
| middleware/auth.rs | 90% | 0% | 🔴 Not Started |

---

## 🔄 Testing Workflow

### Development Cycle
1. **Write test first** (TDD when possible)
2. **Implement feature**
3. **Run tests** (`cargo test`)
4. **Check coverage** (`cargo tarpaulin`)
5. **Fix failing tests**
6. **Commit** (tests must pass)

### PR Requirements
- ✅ All tests pass
- ✅ Coverage doesn't decrease
- ✅ New features have tests
- ✅ No compiler warnings
- ✅ Tests run in CI successfully

---

## 📚 Testing Best Practices

### DO ✅
- Write tests before fixing bugs (regression tests)
- Use descriptive test names
- Test edge cases and error paths
- Keep tests isolated and independent
- Use test fixtures for common setup
- Mock external dependencies
- Test both success and failure scenarios

### DON'T ❌
- Share state between tests
- Use production database for tests
- Commit failing tests
- Skip flaky tests (fix them!)
- Test implementation details
- Hardcode test data that should be fixtures
- Leave commented-out tests

---

## 🎯 Implementation Roadmap

### Phase 1: Foundation (This Sprint)
- [x] Create testing strategy document
- [ ] Add test dependencies to Cargo.toml
- [ ] Create test directory structure
- [ ] Set up test database utilities
- [ ] Write unit tests for AuthService
- [ ] Write integration tests for auth endpoints
- [ ] Achieve 80% coverage on auth module

### Phase 2: Core Features (Next Sprint)
- [ ] Tests for Chart of Accounts
- [ ] Tests for Transaction Engine
- [ ] Database constraint tests
- [ ] Double-entry validation tests

### Phase 3: Complete Coverage (Week 3)
- [ ] E2E workflow tests
- [ ] Performance benchmarks
- [ ] Load testing
- [ ] Security penetration tests

### Phase 4: Automation (Week 4)
- [ ] CI/CD pipeline setup
- [ ] Automated coverage reporting
- [ ] Pre-commit hooks
- [ ] Nightly test runs

---

## 🔗 References

### Rust Testing Resources
- [Rust Book - Testing Chapter](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Axum Testing Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [SQLx Testing Guide](https://github.com/launchbadge/sqlx#testing)

### Internal Documentation
- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Current project status
- [README.md](../README.md) - Project overview
- [DATABASE_SETUP.md](DATABASE_SETUP.md) - Database configuration

---

## 📞 Questions & Support

**Testing Lead:** [Your Name]
**Strategy Review:** Weekly
**Next Review:** October 10, 2025

---

*This document is a living strategy and will be updated as we learn and improve our testing practices.*
