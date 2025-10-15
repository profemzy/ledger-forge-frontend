-- Verification Script for QuickBooks Data Import
-- This script validates the imported data

\echo '=== Data Import Verification Report ==='
\echo ''

-- 1. Count records in each table
\echo '1. Record Counts:'
\echo '   ==============='
SELECT 'Companies' as table_name, COUNT(*) as count FROM companies
UNION ALL
SELECT 'Users', COUNT(*) FROM users
UNION ALL
SELECT 'Chart of Accounts', COUNT(*) FROM chart_of_accounts
UNION ALL
SELECT 'Contacts', COUNT(*) FROM contacts
UNION ALL
SELECT 'Transactions', COUNT(*) FROM transactions
UNION ALL
SELECT 'Transaction Line Items', COUNT(*) FROM transaction_line_items
ORDER BY table_name;

\echo ''
\echo '2. Chart of Accounts by Type:'
\echo '   ==========================='
SELECT account_type, COUNT(*) as count
FROM chart_of_accounts
GROUP BY account_type
ORDER BY account_type;

\echo ''
\echo '3. Contacts by Type:'
\echo '   ================='
SELECT contact_type, COUNT(*) as count
FROM contacts
GROUP BY contact_type
ORDER BY contact_type;

\echo ''
\echo '4. Transaction Status Distribution:'
\echo '   ================================='
SELECT status, COUNT(*) as count
FROM transactions
GROUP BY status
ORDER BY status;

\echo ''
\echo '5. Double-Entry Validation (All transactions should have balanced debits and credits):'
\echo '   ==================================================================================='
WITH transaction_totals AS (
    SELECT
        t.id,
        t.reference_number,
        SUM(tli.debit_amount) as total_debits,
        SUM(tli.credit_amount) as total_credits,
        SUM(tli.debit_amount) - SUM(tli.credit_amount) as difference
    FROM transactions t
    LEFT JOIN transaction_line_items tli ON t.id = tli.transaction_id
    GROUP BY t.id, t.reference_number
)
SELECT
    CASE
        WHEN COUNT(CASE WHEN ABS(difference) > 0.01 THEN 1 END) = 0 THEN '✅ PASSED'
        ELSE '❌ FAILED'
    END as validation_status,
    COUNT(*) as total_transactions,
    COUNT(CASE WHEN ABS(difference) > 0.01 THEN 1 END) as unbalanced_transactions
FROM transaction_totals;

\echo ''
\echo '6. Unbalanced Transactions (if any):'
\echo '   =================================='
WITH transaction_totals AS (
    SELECT
        t.id,
        t.reference_number,
        t.description,
        SUM(tli.debit_amount) as total_debits,
        SUM(tli.credit_amount) as total_credits,
        SUM(tli.debit_amount) - SUM(tli.credit_amount) as difference
    FROM transactions t
    LEFT JOIN transaction_line_items tli ON t.id = tli.transaction_id
    GROUP BY t.id, t.reference_number, t.description
)
SELECT
    reference_number,
    description,
    total_debits,
    total_credits,
    difference
FROM transaction_totals
WHERE ABS(difference) > 0.01
LIMIT 10;

\echo ''
\echo '7. Account Balance Summary (Top 10 accounts by balance):'
\echo '   ======================================================='
WITH account_balances AS (
    SELECT
        a.code,
        a.name,
        a.account_type,
        SUM(tli.debit_amount) as total_debits,
        SUM(tli.credit_amount) as total_credits,
        CASE
            WHEN a.account_type IN ('Asset', 'Expense')
            THEN SUM(tli.debit_amount) - SUM(tli.credit_amount)
            ELSE SUM(tli.credit_amount) - SUM(tli.debit_amount)
        END as balance
    FROM chart_of_accounts a
    LEFT JOIN transaction_line_items tli ON a.id = tli.account_id
    GROUP BY a.id, a.code, a.name, a.account_type
)
SELECT
    code,
    name,
    account_type,
    ROUND(balance, 2) as balance
FROM account_balances
ORDER BY ABS(balance) DESC
LIMIT 10;

\echo ''
\echo '8. Transaction Date Range:'
\echo '   ======================='
SELECT
    MIN(transaction_date) as earliest_transaction,
    MAX(transaction_date) as latest_transaction,
    MAX(transaction_date) - MIN(transaction_date) as date_range_days
FROM transactions;

\echo ''
\echo '9. Sample Transactions (First 5):'
\echo '   ==============================='
SELECT
    t.transaction_date,
    t.reference_number,
    LEFT(t.description, 50) as description,
    COUNT(tli.id) as line_items
FROM transactions t
LEFT JOIN transaction_line_items tli ON t.id = tli.transaction_id
GROUP BY t.id, t.transaction_date, t.reference_number, t.description
ORDER BY t.transaction_date
LIMIT 5;

\echo ''
\echo '=== Verification Complete ==='
