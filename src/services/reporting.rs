use sqlx::PgPool;
use rust_decimal::Decimal;

use crate::models::{
    TrialBalance, TrialBalanceEntry, ProfitLossStatement, ProfitLossEntry,
    BalanceSheet, BalanceSheetEntry, AccountsReceivableAging, AgingBucket,
    DateRangeRequest, DateRequest
};
use crate::utils::{AppError, Result};
use crate::services::CacheService;

#[derive(Clone)]
pub struct ReportingService {
    cache: CacheService,
}

impl ReportingService {
    pub fn new_with_cache(cache: CacheService) -> Self {
        Self { cache }
    }

    /// Generate trial balance as of a specific date
    pub async fn generate_trial_balance(&self, pool: &PgPool, req: DateRequest) -> Result<TrialBalance> {
        let cache_key = format!("trial_balance:{}", req.as_of_date);

        // Try cache first
        if let Ok(Some(cached_report)) = self.cache.get::<TrialBalance>(&cache_key).await {
            return Ok(cached_report);
        }

        // Query database for account balances
        let entries = sqlx::query_as::<_, TrialBalanceEntry>(
            r#"
            WITH account_balances AS (
                SELECT
                    a.id,
                    a.code,
                    a.name,
                    a.account_type::text as account_type,
                    COALESCE(SUM(
                        CASE
                            WHEN a.account_type IN ('Asset', 'Expense') THEN tl.debit_amount - tl.credit_amount
                            ELSE tl.credit_amount - tl.debit_amount
                        END
                    ), 0) as balance
                FROM chart_of_accounts a
                LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
                LEFT JOIN transactions t ON tl.transaction_id = t.id
                    AND t.transaction_date <= $1
                    AND a.is_active = true
                GROUP BY a.id, a.code, a.name, a.account_type
                HAVING ABS(COALESCE(SUM(
                    CASE
                        WHEN a.account_type IN ('Asset', 'Expense') THEN tl.debit_amount - tl.credit_amount
                        ELSE tl.credit_amount - tl.debit_amount
                    END
                ), 0)) > 0.01
            )
            SELECT
                id as account_id,
                code as account_code,
                name as account_name,
                account_type,
                CASE
                    WHEN account_type IN ('Asset', 'Expense') AND balance > 0 THEN balance
                    WHEN account_type IN ('Liability', 'Equity', 'Revenue') AND balance < 0 THEN ABS(balance)
                    ELSE 0
                END as debit,
                CASE
                    WHEN account_type IN ('Asset', 'Expense') AND balance < 0 THEN ABS(balance)
                    WHEN account_type IN ('Liability', 'Equity', 'Revenue') AND balance > 0 THEN balance
                    ELSE 0
                END as credit,
                balance
            FROM account_balances
            ORDER BY account_code
            "#
        )
        .bind(req.as_of_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Calculate totals
        let total_debits: Decimal = entries.iter().map(|e| e.debit).sum();
        let total_credits: Decimal = entries.iter().map(|e| e.credit).sum();
        let is_balanced = (total_debits - total_credits).abs() < Decimal::new(1, 2); // Within $0.01

        let trial_balance = TrialBalance {
            as_of_date: req.as_of_date,
            total_debits,
            total_credits,
            is_balanced,
            entries,
        };

        // Cache the result for 1 hour
        self.cache.set_with_ttl(&cache_key, &trial_balance, 3600).await?;

        Ok(trial_balance)
    }

    /// Generate profit and loss statement for a date range
    pub async fn generate_profit_loss(&self, pool: &PgPool, req: DateRangeRequest) -> Result<ProfitLossStatement> {
        let cache_key = format!("profit_loss:{}:{}", req.start_date, req.end_date);

        // Try cache first
        if let Ok(Some(cached_report)) = self.cache.get::<ProfitLossStatement>(&cache_key).await {
            return Ok(cached_report);
        }

        // Query revenue entries
        let revenue_entries = sqlx::query_as::<_, ProfitLossEntry>(
            r#"
            SELECT
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                a.account_type::text as account_type,
                COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0) as amount
            FROM chart_of_accounts a
            INNER JOIN transaction_line_items tl ON a.id = tl.account_id
            INNER JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date BETWEEN $1 AND $2
                AND a.account_type = 'Revenue'
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name, a.account_type
            HAVING ABS(COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0)) > 0.01
            ORDER BY a.code
            "#
        )
        .bind(req.start_date)
        .bind(req.end_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Query expense entries
        let expense_entries = sqlx::query_as::<_, ProfitLossEntry>(
            r#"
            SELECT
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                a.account_type::text as account_type,
                COALESCE(SUM(tl.debit_amount - tl.credit_amount), 0) as amount
            FROM chart_of_accounts a
            INNER JOIN transaction_line_items tl ON a.id = tl.account_id
            INNER JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date BETWEEN $1 AND $2
                AND a.account_type = 'Expense'
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name, a.account_type
            HAVING ABS(COALESCE(SUM(tl.debit_amount - tl.credit_amount), 0)) > 0.01
            ORDER BY a.code
            "#
        )
        .bind(req.start_date)
        .bind(req.end_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Calculate totals
        let total_revenue: Decimal = revenue_entries.iter().map(|e| e.amount).sum();
        let total_expenses: Decimal = expense_entries.iter().map(|e| e.amount).sum();
        let net_income = total_revenue - total_expenses;

        let profit_loss = ProfitLossStatement {
            period_start: req.start_date,
            period_end: req.end_date,
            total_revenue,
            total_expenses,
            net_income,
            revenue_entries,
            expense_entries,
        };

        // Cache the result for 2 hours
        self.cache.set_with_ttl(&cache_key, &profit_loss, 7200).await?;

        Ok(profit_loss)
    }

    /// Generate balance sheet as of a specific date
    pub async fn generate_balance_sheet(&self, pool: &PgPool, req: DateRequest) -> Result<BalanceSheet> {
        let cache_key = format!("balance_sheet:{}", req.as_of_date);

        // Try cache first
        if let Ok(Some(cached_report)) = self.cache.get::<BalanceSheet>(&cache_key).await {
            return Ok(cached_report);
        }

        // Query asset entries
        let asset_entries = sqlx::query_as::<_, BalanceSheetEntry>(
            r#"
            SELECT
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                COALESCE(SUM(tl.debit_amount - tl.credit_amount), 0) as amount
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
            LEFT JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date <= $1
                AND a.account_type = 'Asset'
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name
            HAVING ABS(COALESCE(SUM(tl.debit_amount - tl.credit_amount), 0)) > 0.01
            ORDER BY a.code
            "#
        )
        .bind(req.as_of_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Query liability entries
        let liability_entries = sqlx::query_as::<_, BalanceSheetEntry>(
            r#"
            SELECT
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0) as amount
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
            LEFT JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date <= $1
                AND a.account_type = 'Liability'
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name
            HAVING ABS(COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0)) > 0.01
            ORDER BY a.code
            "#
        )
        .bind(req.as_of_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Query equity entries
        let equity_entries = sqlx::query_as::<_, BalanceSheetEntry>(
            r#"
            SELECT
                a.id as account_id,
                a.code as account_code,
                a.name as account_name,
                COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0) as amount
            FROM chart_of_accounts a
            LEFT JOIN transaction_line_items tl ON a.id = tl.account_id
            LEFT JOIN transactions t ON tl.transaction_id = t.id
                AND t.transaction_date <= $1
                AND a.account_type = 'Equity'
                AND a.is_active = true
                AND t.status = 'posted'
            GROUP BY a.id, a.code, a.name
            HAVING ABS(COALESCE(SUM(tl.credit_amount - tl.debit_amount), 0)) > 0.01
            ORDER BY a.code
            "#
        )
        .bind(req.as_of_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Calculate totals
        let total_assets: Decimal = asset_entries.iter().map(|e| e.amount).sum();
        let total_liabilities: Decimal = liability_entries.iter().map(|e| e.amount).sum();
        let total_equity: Decimal = equity_entries.iter().map(|e| e.amount).sum();

        let balance_sheet = BalanceSheet {
            as_of_date: req.as_of_date,
            total_assets,
            total_liabilities,
            total_equity,
            asset_entries,
            liability_entries,
            equity_entries,
        };

        // Cache the result for 1 hour
        self.cache.set_with_ttl(&cache_key, &balance_sheet, 3600).await?;

        Ok(balance_sheet)
    }

    /// Generate accounts receivable aging report as of a specific date
    pub async fn generate_ar_aging(&self, pool: &PgPool, req: DateRequest) -> Result<AccountsReceivableAging> {
        let cache_key = format!("ar_aging:{}", req.as_of_date);

        // Try cache first
        if let Ok(Some(cached_report)) = self.cache.get::<AccountsReceivableAging>(&cache_key).await {
            return Ok(cached_report);
        }

        // Query aging buckets
        let buckets = sqlx::query_as::<_, AgingBucket>(
            r#"
            WITH invoice_aging AS (
                SELECT
                    c.id as customer_id,
                    c.name as customer_name,
                    i.id as invoice_id,
                    i.invoice_date,
                    i.due_date,
                    i.balance,
                    CASE
                        WHEN $1 - i.due_date < 0 THEN 0
                        WHEN $1 - i.due_date <= 30 THEN 1
                        WHEN $1 - i.due_date <= 60 THEN 2
                        WHEN $1 - i.due_date <= 90 THEN 3
                        ELSE 4
                    END as aging_bucket
                FROM invoices i
                INNER JOIN contacts c ON i.customer_id = c.id
                WHERE i.balance > 0.01
                    AND i.invoice_date <= $1
                    AND i.status != 'paid'
            )
            SELECT
                customer_id,
                customer_name,
                COALESCE(SUM(CASE WHEN aging_bucket = 0 THEN balance ELSE 0 END), 0) as current,
                COALESCE(SUM(CASE WHEN aging_bucket = 1 THEN balance ELSE 0 END), 0) as days_1_30,
                COALESCE(SUM(CASE WHEN aging_bucket = 2 THEN balance ELSE 0 END), 0) as days_31_60,
                COALESCE(SUM(CASE WHEN aging_bucket = 3 THEN balance ELSE 0 END), 0) as days_61_90,
                COALESCE(SUM(CASE WHEN aging_bucket = 4 THEN balance ELSE 0 END), 0) as days_91_plus,
                COALESCE(SUM(balance), 0) as total
            FROM invoice_aging
            GROUP BY customer_id, customer_name
            HAVING COALESCE(SUM(balance), 0) > 0.01
            ORDER BY customer_name
            "#
        )
        .bind(req.as_of_date)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Calculate total outstanding
        let total_outstanding: Decimal = buckets.iter().map(|b| b.total).sum();

        let ar_aging = AccountsReceivableAging {
            as_of_date: req.as_of_date,
            total_outstanding,
            buckets,
        };

        // Cache the result for 1 hour
        self.cache.set_with_ttl(&cache_key, &ar_aging, 3600).await?;

        Ok(ar_aging)
    }
}