# QuickBooks Import - Quick Reference

## 🚀 One-Command Import

```bash
./scripts/import-quickbooks.sh
```

## 📋 Manual Steps

### 1. Clear Database
```bash
echo "yes" | cargo run --bin clear
```

### 2. Import Data
```bash
cargo run --bin import-quickbooks
```

### 3. Verify Import
```bash
cargo run --bin verify-import
```

## 📊 Expected Results

- ✅ Company: InfoTitans LTD
- ✅ Accounts: 28
- ✅ Contacts: 86 (2 customers, 84 vendors)
- ✅ Transactions: 438 (all balanced)
- ✅ Date Range: Oct 22, 2024 - Oct 2, 2025

## 🔐 Default Login

```bash
Username: admin
Password: admin123
Email: admin@infotitans.com
```

## 🧪 Test After Import

```bash
# Start server
cargo run

# Login (save the token)
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"admin123"}'

# Get accounts
curl http://localhost:3000/api/v1/accounts \
  -H 'Authorization: Bearer YOUR_TOKEN'

# Get trial balance
curl "http://localhost:3000/api/v1/reports/trial-balance?as_of_date=2025-10-02" \
  -H 'Authorization: Bearer YOUR_TOKEN'
```

## 📁 Files Location

- Source Excel: `data/*.xlsx`
- Converted CSV: `data/csv/*.csv`
- Import script: `src/bin/import_quickbooks.rs`
- Verification: `src/bin/verify_import.rs`

## ⚠️ Important

- Import **deletes all existing data**
- All transactions must balance (verified automatically)
- Default password should be changed after first login

---

**Full Documentation:** `docs/QUICKBOOKS_IMPORT.md`
