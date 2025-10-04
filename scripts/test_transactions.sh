#!/bin/bash

# Get auth token
TOKEN=$(curl -s -X POST http://localhost:3000/api/v1/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"testuser","password":"TestPassword123"}' | jq -r '.data.access_token')

echo "=== Testing Transaction API Endpoints ==="
echo ""

# Create Cash account
echo "1. Creating Cash account..."
CASH_RESPONSE=$(curl -s -X POST http://localhost:3000/api/v1/accounts \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code":"1100","name":"Petty Cash","account_type":"Asset"}')
CASH_ID=$(echo $CASH_RESPONSE | jq -r '.data.id')
echo "Cash Account ID: $CASH_ID"
echo ""

# Create Revenue account
echo "2. Creating Revenue account..."
REVENUE_RESPONSE=$(curl -s -X POST http://localhost:3000/api/v1/accounts \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"code":"4100","name":"Service Revenue","account_type":"Revenue"}')
REVENUE_ID=$(echo $REVENUE_RESPONSE | jq -r '.data.id')
echo "Revenue Account ID: $REVENUE_ID"
echo ""

# Create a transaction
echo "3. Creating a transaction (Cash debit $500, Revenue credit $500)..."
TRANSACTION=$(curl -s -X POST http://localhost:3000/api/v1/transactions \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d "{
    \"transaction_date\": \"2025-10-04\",
    \"description\": \"Service rendered to customer\",
    \"reference_number\": \"INV-001\",
    \"journal_type\": \"Sales\",
    \"line_items\": [
      {
        \"account_id\": \"$CASH_ID\",
        \"description\": \"Cash received\",
        \"debit_amount\": \"500.00\"
      },
      {
        \"account_id\": \"$REVENUE_ID\",
        \"description\": \"Service revenue\",
        \"credit_amount\": \"500.00\"
      }
    ]
  }")
echo "$TRANSACTION" | jq '.'
TRANSACTION_ID=$(echo $TRANSACTION | jq -r '.data.transaction.id')
echo ""

# List transactions
echo "4. Listing all transactions..."
curl -s -X GET "http://localhost:3000/api/v1/transactions" \
  -H "Authorization: Bearer $TOKEN" | jq '.data | length'
echo ""

# Get transaction by ID
echo "5. Getting transaction by ID..."
curl -s -X GET "http://localhost:3000/api/v1/transactions/$TRANSACTION_ID" \
  -H "Authorization: Bearer $TOKEN" | jq '.data.transaction | {id, status, description}'
echo ""

# Update transaction status to Posted
echo "6. Posting the transaction..."
curl -s -X PUT "http://localhost:3000/api/v1/transactions/$TRANSACTION_ID/status" \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"status": "posted"}' | jq '.data | {id, status}'
echo ""

# Get account balance
echo "7. Getting Cash account balance (should be $500.00)..."
curl -s -X GET "http://localhost:3000/api/v1/accounts/$CASH_ID/balance" \
  -H "Authorization: Bearer $TOKEN" | jq '.data'
echo ""

echo "8. Getting Revenue account balance (should be -$500.00)..."
curl -s -X GET "http://localhost:3000/api/v1/accounts/$REVENUE_ID/balance" \
  -H "Authorization: Bearer $TOKEN" | jq '.data'
echo ""

# Try to delete a posted transaction (should fail)
echo "9. Trying to delete posted transaction (should fail)..."
curl -s -X DELETE "http://localhost:3000/api/v1/transactions/$TRANSACTION_ID" \
  -H "Authorization: Bearer $TOKEN" | jq '.'
echo ""

echo "=== Testing Complete ==="
