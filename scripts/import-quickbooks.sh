#!/bin/bash

# QuickBooks Data Import Script
# This script clears the database and imports QuickBooks data

set -e  # Exit on any error

echo "🚀 QuickBooks Data Import Process"
echo "=================================="
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "   Please copy .env.example to .env and configure it."
    exit 1
fi

# Check if CSV files exist
if [ ! -d data/csv ]; then
    echo "⚠️  CSV files not found. Converting Excel files..."
    source .venv/bin/activate
    python3 scripts/examine_quickbooks_data.py
    echo ""
fi

# Confirm with user
echo "⚠️  WARNING: This will DELETE all existing data and import QuickBooks data!"
echo ""
read -p "Are you sure you want to continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "❌ Import cancelled."
    exit 0
fi

echo ""
echo "Step 1: Clearing existing database..."
echo "======================================"
cargo run --bin clear

echo ""
echo "Step 2: Importing QuickBooks data..."
echo "===================================="
cargo run --bin import-quickbooks

echo ""
echo "✅ Import process completed!"
echo ""
echo "You can now start the server:"
echo "  cargo run"
echo ""
