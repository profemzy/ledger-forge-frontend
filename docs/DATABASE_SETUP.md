# Database Setup Guide

## Network Database Server Configuration

**Server:** 10.27.27.66:5432
**Username:** infotitans
**Password:** swift1FEMZY14
**Database:** ledger_forge

## Current Status

✅ **CONNECTED & OPERATIONAL**

Database successfully created and migrations applied!
- Server: 10.27.27.66:**34155** (non-standard port)
- Database: ledger_forge
- Tables: 16 (all created successfully)
- Migrations: 2 applied

## Required PostgreSQL Server Configuration

The database administrator needs to add the following entry to `/etc/postgresql/*/main/pg_hba.conf` (or equivalent):

### Option 1: Allow specific IP
```
# LedgerForge Development Machine
host    all             infotitans      10.27.27.204/32         md5
```

### Option 2: Allow subnet (if multiple machines need access)
```
# LedgerForge Development Network
host    all             infotitans      10.27.27.0/24           md5
```

### Steps for Database Administrator:

1. Edit pg_hba.conf:
   ```bash
   sudo nano /etc/postgresql/16/main/pg_hba.conf
   # Or wherever your PostgreSQL config is located
   ```

2. Add the appropriate entry from above

3. Reload PostgreSQL configuration:
   ```bash
   sudo systemctl reload postgresql
   # OR
   sudo pg_ctlcluster 16 main reload
   ```

4. Verify the configuration:
   ```bash
   sudo -u postgres psql -c "SELECT * FROM pg_hba_file_rules;"
   ```

## Database Creation (After Connection is Fixed)

Once the connection is established, create the database:

```bash
# Create the database
sqlx database create

# Run migrations
sqlx migrate run
```

Or manually:
```bash
psql "postgresql://infotitans:swift1FEMZY14@10.27.27.66:5432/postgres" -c "CREATE DATABASE ledger_forge;"
```

## Alternative: Local Development

If the network database is not immediately available, you can use a local PostgreSQL instance:

### 1. Install PostgreSQL locally:
```bash
brew install postgresql@16  # macOS
# or
sudo apt install postgresql-16  # Ubuntu/Debian
```

### 2. Start PostgreSQL:
```bash
brew services start postgresql@16  # macOS
# or
sudo systemctl start postgresql  # Linux
```

### 3. Create local database:
```bash
createdb ledger_forge
```

### 4. Update .env to use local database:
```
DATABASE_URL=postgresql://localhost:5432/ledger_forge
```

### 5. Run migrations:
```bash
sqlx migrate run
```

## Environment Variables

Current configuration in `.env`:
```
DATABASE_URL=postgresql://infotitans:swift1FEMZY14@10.27.27.66:5432/ledger_forge
```

## Troubleshooting

### Test Connection
```bash
psql "postgresql://infotitans:swift1FEMZY14@10.27.27.66:5432/postgres"
```

### Check if PostgreSQL is listening
```bash
telnet 10.27.27.66 5432
# OR
nc -zv 10.27.27.66 5432
```

### Common Issues

1. **No pg_hba.conf entry** - Add IP to pg_hba.conf (see above)
2. **Connection timeout** - Check firewall rules
3. **Wrong password** - Verify credentials
4. **Database doesn't exist** - Create it first (see above)
5. **SSL required/not supported** - Adjust sslmode parameter

## SQLx Configuration

SQLx needs the DATABASE_URL to be available at compile time for query checking. Make sure `.env` is properly configured before running:

```bash
cargo sqlx prepare  # Generate offline query data
cargo build         # Build the project
```

## Next Steps

1. ✅ Configure pg_hba.conf on database server
2. ⏳ Test connection
3. ⏳ Create ledger_forge database
4. ⏳ Run migrations
5. ⏳ Verify schema creation

---

**Last Updated:** October 3, 2025
**Status:** Waiting for database server configuration
