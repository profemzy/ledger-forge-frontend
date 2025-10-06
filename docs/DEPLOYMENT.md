# Deployment & Setup Guide

**Last Updated:** October 5, 2025
**Status:** Production-ready setup

This guide covers database setup, Redis configuration, seeding, and deployment instructions for LedgerForge.

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.90+ (edition 2024)
- PostgreSQL 14+
- Redis 6+ (for caching)
- SQLx CLI

### 1. Project Setup
```bash
# Clone repository
git clone <repository-url>
cd ledger-forge

# Copy environment configuration
cp .env.example .env
# Edit .env with your database credentials

# Install dependencies
cargo build
```

### 2. Database Setup

#### PostgreSQL Configuration
**Network PostgreSQL Server:**
- **Host:** 10.27.27.66
- **Port:** 34155
- **Database:** ledger_forge
- **User:** infotitans

**Local Development:**
```bash
# Create database user
createuser ledger_user -P

# Create database
createdb ledger_forge -O ledger_user

# Set environment variable
export DATABASE_URL="postgresql://ledger_user:password@localhost:5432/ledger_forge"
```

#### Database Migrations
```bash
# Run migrations (automatic on server start)
sqlx migrate run

# Check migration status
sqlx migrate info

# Create new migration
sqlx migrate add <migration_name>

# Revert last migration
sqlx migrate revert
```

### 3. Redis Setup (Caching)
```bash
# Install Redis (macOS)
brew install redis

# Start Redis server
redis-server

# Verify connection
redis-cli ping

# Set Redis URL in .env
REDIS_URL=redis://localhost:6379
```

### 4. Start Server
```bash
# Development mode
cargo run

# Production mode
cargo run --release

# Server starts on http://localhost:3000 (or PORT from .env)
```

---

## 🗄️ Database Schema

### Current Tables (16 total)
```sql
-- Core Tables
users                      -- Authentication & roles
companies                  -- Multi-tenancy support
chart_of_accounts         -- Account hierarchy
contacts                   -- Customers/Vendors/Employees
transactions              -- Journal entries
transaction_line_items    -- Double-entry lines

-- QuickBooks Compatible Tables
invoices                  -- Customer invoicing
invoice_line_items        -- Invoice details
bills                     -- Vendor bills (AP)
bill_line_items          -- Bill details
payments                  -- Customer payments (AR)
payment_applications      -- Payment-to-invoice links
bill_payments            -- Vendor payments
bill_payment_applications -- Payment-to-bill links
items                     -- Products/Services catalog
```

### Key Features
- **UUID Primary Keys** - Security and distributed-ready
- **Double-entry Constraints** - Database-level validation
- **Foreign Key Relationships** - Data integrity
- **Timestamps** - Audit trails with `created_at`/`updated_at`
- **Soft Deletes** - `is_active` flags for data retention

---

## 🔧 Configuration

### Environment Variables
```bash
# Database Configuration
DATABASE_URL=postgresql://user:password@host:port/database

# Redis Configuration
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-secret-key-change-in-production

# Server Configuration
PORT=3000
RUST_LOG=debug

# Development
DATABASE_URL="postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge"
JWT_SECRET="development-secret-change-in-production"
REDIS_URL="redis://localhost:6379"
PORT="3000"
```

### Default Configuration
- **Server Port:** 3000
- **Database Connections:** 5 max pool size
- **JWT Access Token:** 1 hour expiry
- **JWT Refresh Token:** 7 days expiry
- **Cache TTL:** 10 minutes (accounts), 30 minutes (hierarchies)

---

## 🌱 Database Seeding

### Clear Database
```bash
# Remove all data (keeps schema)
cargo run --bin clear
```

### Seed Database
```bash
# Populate with sample data
cargo run --bin seed
```

### Seed Data Includes
- **Admin User:** username `admin`, password `admin123`
- **Sample Chart of Accounts:** Standard accounting structure
- **Sample Contacts:** Customers, vendors, employees
- **Sample Transactions:** Double-entry examples

### Custom Seeding
Modify `src/bin/seed.rs` to add your own test data:
```rust
// Add custom accounts
AccountService::create(CreateAccountRequest {
    code: "4000".to_string(),
    name: "Service Revenue".to_string(),
    account_type: AccountType::Revenue,
    parent_id: Some(revenue_account_id),
    // ...
}).await?;
```

---

## 🏥 Health Checks

### Server Health
```bash
curl http://localhost:3000/api/v1/health
```

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "database": "healthy",
  "cache": "healthy"
}
```

### Database Health
```bash
# Test database connection
psql $DATABASE_URL -c "SELECT 1;"

# Check table counts
psql $DATABASE_URL -c "SELECT schemaname,tablename FROM pg_tables WHERE schemaname = 'public';"
```

### Redis Health
```bash
# Test Redis connection
redis-cli -u $REDIS_URL ping

# Check cache keys
redis-cli -u $REDIS_URL keys "*"
```

---

## 🚀 Production Deployment

### Build for Production
```bash
# Optimized build
cargo build --release

# Binary location: ./target/release/ledger-forge
```

### Systemd Service (Linux)
```ini
# /etc/systemd/system/ledger-forge.service
[Unit]
Description=LedgerForge Accounting System
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=ledger
WorkingDirectory=/opt/ledger-forge
ExecStart=/opt/ledger-forge/ledger-forge
Restart=always
RestartSec=10
Environment=DATABASE_URL=postgresql://user:pass@localhost/ledger
Environment=REDIS_URL=redis://localhost:6379
Environment=JWT_SECRET=production-secret-key

[Install]
WantedBy=multi-user.target
```

### Docker Deployment
```dockerfile
# Dockerfile
FROM rust:1.90 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ledger-forge /usr/local/bin/
EXPOSE 3000
CMD ["ledger-forge"]
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'
services:
  ledger-forge:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@postgres:5432/ledger
      - REDIS_URL=redis://redis:6379
      - JWT_SECRET=production-secret-key
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: ledger
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  postgres_data:
```

---

## 🔒 Security Considerations

### Production Security Checklist
- [ ] **Change JWT Secret** - Use strong, randomly generated secret
- [ ] **Environment Variables** - Never commit secrets to git
- [ ] **Database Security** - Use dedicated database user with limited permissions
- [ ] **Redis Security** - Enable Redis AUTH if exposed
- [ ] **HTTPS/TLS** - Use reverse proxy (nginx/caddy) for SSL termination
- [ ] **Firewall** - Restrict database and Redis access
- [ ] **Backups** - Regular database backups
- [ ] **Monitoring** - Log aggregation and monitoring

### Environment Security
```bash
# Generate secure JWT secret
openssl rand -base64 32

# Secure file permissions
chmod 600 .env

# Database user with limited privileges
CREATE ROLE ledger_user WITH LOGIN PASSWORD 'secure_password';
GRANT CONNECT ON DATABASE ledger_forge TO ledger_user;
GRANT USAGE ON SCHEMA public TO ledger_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO ledger_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO ledger_user;
```

---

## 📊 Monitoring & Logging

### Application Logs
```bash
# View logs in development
RUST_LOG=debug cargo run

# Production logs
journalctl -u ledger-forge -f
```

### Performance Monitoring
- **Server Metrics:** Use ` Prometheus + Grafana` for system metrics
- **Database Metrics:** Monitor connection pool, query performance
- **Cache Metrics:** Track Redis hit rates, memory usage
- **API Metrics:** Monitor request rates, response times

### Database Monitoring
```sql
-- Active connections
SELECT count(*) FROM pg_stat_activity;

-- Slow queries
SELECT query, mean_time, calls
FROM pg_stat_statements
ORDER BY mean_time DESC
LIMIT 10;

-- Table sizes
SELECT schemaname, tablename,
       pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

---

## 🔧 Troubleshooting

### Common Issues

#### Database Connection Failed
```bash
# Check PostgreSQL is running
pg_isready -h localhost -p 5432

# Test connection manually
psql $DATABASE_URL -c "SELECT 1;"

# Check .env file
cat .env | grep DATABASE_URL
```

#### Migration Failures
```bash
# Check migration status
sqlx migrate info

# Force rerun migrations (development only)
sqlx migrate run --force
```

#### Redis Connection Issues
```bash
# Check Redis server
redis-cli ping

# Check Redis logs
tail -f /usr/local/var/log/redis.log

# Test connection from application
redis-cli -u $REDIS_URL info server
```

#### Performance Issues
```bash
# Check database connections
SELECT * FROM pg_stat_activity WHERE state = 'active';

# Check cache effectiveness
redis-cli -u $REDIS_URL info stats | grep keyspace

# Monitor memory usage
top -p $(pgrep ledger-forge)
```

---

## 📚 Additional Resources

### Development Tools
- **SQLx CLI:** Database migration management
- **PostgreSQL Client:** `psql` for database queries
- **Redis CLI:** `redis-cli` for cache operations
- **API Testing:** Swagger UI at `/swagger-ui`

### Documentation
- **Development Guide:** GUIDE.md - Current status & features
- **Architecture:** DEVELOPMENT.md - Design decisions
- **API Documentation:** http://localhost:3000/swagger-ui/

---

*Last Updated: October 5, 2025*
*For questions, see the Development Guide or check the API documentation.*