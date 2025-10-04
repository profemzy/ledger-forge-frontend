# Scripts

This directory contains utility scripts for testing and development.

## Available Scripts

### `test_transactions.sh`
Manual API testing script for transaction endpoints.

**Usage:**
```bash
./scripts/test_transactions.sh
```

**What it does:**
1. Authenticates and gets JWT token
2. Creates test accounts (Cash, Revenue)
3. Creates a sample transaction
4. Posts the transaction
5. Retrieves account balances
6. Tests various endpoints

**Prerequisites:**
- Server must be running on port 3000
- User credentials configured (username: testuser, password: TestPassword123)
- `jq` installed for JSON parsing

## Development Scripts

You can add more scripts here for:
- Database migrations
- Data seeding
- Performance testing
- Deployment automation
- Backup/restore operations
