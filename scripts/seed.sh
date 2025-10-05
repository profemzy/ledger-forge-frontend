#!/bin/bash

# Database Seeding Script for LedgerForge
# This script seeds the database with sample data for development and testing

set -e

echo "🌱 LedgerForge Database Seeder"
echo "=============================="
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "   Please create .env file from .env.example"
    exit 1
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

echo "📦 Building seeder binary..."
cargo build --bin seed --release

echo ""
echo "🚀 Running database seeder..."
echo ""

./target/release/seed

echo ""
echo "✅ Done!"
