#!/bin/bash

# Quick development database seeding script
# Runs the seeder directly with cargo run

set -e

echo "🌱 LedgerForge Database Seeder (Dev Mode)"
echo "=========================================="
echo ""

# Check if .env file exists
if [ ! -f .env ]; then
    echo "❌ Error: .env file not found!"
    echo "   Please create .env file from .env.example"
    exit 1
fi

echo "🚀 Running database seeder..."
echo ""

cargo run --bin seed

echo ""
echo "✅ Done!"
