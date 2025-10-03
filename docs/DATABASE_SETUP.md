# Database Setup Guide

**Last Updated:** October 3, 2025

---

## 📊 Database Configuration

### Network PostgreSQL Server

**Connection Details:**
- **Host:** 10.27.27.66
- **Port:** 34155
- **Database:** ledger_forge
- **User:** infotitans
- **Connection String:** `postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge`

---

## 🚀 Quick Start

### 1. Environment Setup

The `.env` file is already configured:

```bash
DATABASE_URL=postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge
```

### 2. Running Migrations

Migrations run automatically on server startup, or run manually:

```bash
# Apply all pending migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### 3. Verify Connection

```bash
# Using psql
psql "postgresql://infotitans:swift1FEMZY14@10.27.27.66:34155/ledger_forge"

# Check tables
\dt

# Check specific table
\d users
```

---

## 📋 Database Schema

### 16 Tables Created

**Core Tables:**
- `users` - User authentication & roles
- `companies` - Multi-tenancy support
- `chart_of_accounts` - Account hierarchy
- `contacts` - Customers, Vendors, Employees
- `transactions` - Journal entry headers
- `transaction_line_items` - Double-entry line items

**QuickBooks Compatible Tables:**
- `invoices` / `invoice_line_items`
- `bills` / `bill_line_items`
- `payments` / `payment_applications`
- `bill_payments` / `bill_payment_applications`
- `items` - Products/Services catalog

### Migrations Applied

1. `20251003175025_init_schema.sql` - Core accounting tables
2. `20251003180129_quickbooks_compatibility.sql` - QB-specific enhancements

---

## 🔍 Common Tasks

### View All Tables
```sql
SELECT tablename FROM pg_tables
WHERE schemaname = 'public'
ORDER BY tablename;
```

### Check Migration Status
```sql
SELECT * FROM _sqlx_migrations
ORDER BY version;
```

### View Users
```sql
SELECT id, username, email, role, created_at
FROM users
ORDER BY created_at DESC;
```

---

## 🐛 Troubleshooting

### Connection Issues

**Problem:** Cannot connect to database

**Solution:**
1. Check network connectivity to 10.27.27.66
2. Verify port 34155 is accessible
3. Ensure DATABASE_URL in .env is correct

### Migration Issues

**Problem:** Migration fails

**Solution:**
```bash
# Check current migration status
sqlx migrate info

# Force re-run migrations
sqlx migrate run --force
```

---

## 📚 More Information

For complete database schema details, see [PROJECT_STATUS.md](PROJECT_STATUS.md)
