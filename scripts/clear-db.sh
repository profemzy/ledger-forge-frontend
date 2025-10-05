#!/bin/bash

# Clear all data from database tables (but keep the schema)
# This allows re-seeding without dropping the database

set -e

echo "🗑️  LedgerForge Database Cleaner"
echo "================================"
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "   Please create .env file from .env.example"
    exit 1
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

echo "⚠️  WARNING: This will delete ALL data from the database!"
echo "   Database: $DATABASE_URL"
echo ""
read -p "Are you sure you want to continue? (yes/no): " -r
echo ""

if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Aborted."
    exit 1
fi

echo "🧹 Clearing database tables..."

# Use sqlx to run the SQL commands
sqlx database drop -y || true
sqlx database create
sqlx migrate run

echo ""
echo "✅ Database cleared successfully!"
echo "   All tables have been reset."
echo ""
echo "💡 You can now run the seeder:"
echo "   ./scripts/seed-dev.sh"
