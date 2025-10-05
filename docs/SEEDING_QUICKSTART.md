# 🌱 Database Seeding - Quick Start

## TL;DR

```bash
# Seed the database with sample data
./scripts/seed-dev.sh
```

## What You Get

✅ **2 Users** (admin/admin123, accountant/accountant123)
✅ **1 Company** (Akowe Demo Company)
✅ **14 Accounts** (Assets, Liabilities, Equity, Revenue, Expenses)
✅ **8 Transactions** (Posted and Draft journal entries)
✅ **3 Contacts** (Customers and Vendors)

## Login Credentials

| User | Username | Password | Role |
|------|----------|----------|------|
| Admin | `admin` | `admin123` | Admin |
| Accountant | `accountant` | `accountant123` | Accountant |

## Commands

```bash
# Quick seed (development)
./scripts/seed-dev.sh

# Production build
./scripts/seed.sh

# Direct cargo
cargo run --bin seed
```

## Fresh Start

```bash
# Reset everything
docker-compose down -v
docker-compose up -d
sleep 5
sqlx migrate run
./scripts/seed-dev.sh
```

## Need Help?

See full documentation: [docs/SEEDING.md](docs/SEEDING.md)
