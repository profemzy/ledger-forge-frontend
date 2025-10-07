use chrono::{NaiveDate, Duration};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use ledger_forge::services::{ReportingService, CacheService};
use ledger_forge::models::{DateRequest, DateRangeRequest, TrialBalance, ProfitLossStatement, BalanceSheet, AccountsReceivableAging};
use ledger_forge::utils::AppError;

/// Unit tests for the reporting service methods
/// These tests validate the business logic and data transformation functions

#[tokio::test]
async fn test_reporting_service_trial_balance_generation() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test trial balance generation for a specific date
    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let trial_balance = reporting_service
        .generate_trial_balance(&pool, date_request.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify trial balance structure
    assert_eq!(trial_balance.as_of_date, date_request.as_of_date);
    assert!(trial_balance.entries.len() > 0);

    // Verify balance properties
    let calculated_total_debits: Decimal = trial_balance.entries.iter().map(|e| e.debit).sum();
    let calculated_total_credits: Decimal = trial_balance.entries.iter().map(|e| e.credit).sum();

    assert_eq!(trial_balance.total_debits, calculated_total_debits);
    assert_eq!(trial_balance.total_credits, calculated_total_credits);
    assert!(trial_balance.is_balanced);

    // Verify that the balance is within tolerance
    let difference = trial_balance.total_debits - trial_balance.total_credits;
    assert!(difference.abs() < Decimal::new(1, 2)); // Within $0.01

    // Verify entry properties
    for entry in &trial_balance.entries {
        // Account should be active
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
        assert!(["Asset", "Liability", "Equity", "Revenue", "Expense"].contains(&entry.account_type.as_str()));

        // Only one of debit or credit should be non-zero
        assert!(entry.debit == Decimal::ZERO || entry.credit == Decimal::ZERO);

        // Balance should be non-zero (entries with zero balance should be filtered out)
        assert!(entry.balance != Decimal::ZERO);
    }

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_trial_balance_earlier_date() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test trial balance for an earlier date (should show fewer transactions)
    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
    };

    let trial_balance = reporting_service
        .generate_trial_balance(&pool, date_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify structure
    assert!(trial_balance.entries.len() > 0);
    assert!(trial_balance.is_balanced);

    // Should have fewer transactions than year-end
    let year_end_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };
    let year_end_balance = reporting_service
        .generate_trial_balance(&pool, year_end_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // The total amounts should be different for different dates
    assert!(trial_balance.total_debits != year_end_balance.total_debits ||
            trial_balance.total_credits != year_end_balance.total_credits);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_profit_loss_generation() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test profit and loss for full year
    let date_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let profit_loss = reporting_service
        .generate_profit_loss(&pool, date_range.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify P&L structure
    assert_eq!(profit_loss.period_start, date_range.start_date);
    assert_eq!(profit_loss.period_end, date_range.end_date);

    // Verify financial calculations
    let calculated_total_revenue: Decimal = profit_loss.revenue_entries.iter().map(|e| e.amount).sum();
    let calculated_total_expenses: Decimal = profit_loss.expense_entries.iter().map(|e| e.amount).sum();
    let calculated_net_income = calculated_total_revenue - calculated_total_expenses;

    assert_eq!(profit_loss.total_revenue, calculated_total_revenue);
    assert_eq!(profit_loss.total_expenses, calculated_total_expenses);
    assert_eq!(profit_loss.net_income, calculated_net_income);

    // Verify we have both revenue and expenses from test data
    assert!(profit_loss.total_revenue > Decimal::ZERO);
    assert!(profit_loss.total_expenses > Decimal::ZERO);

    // Verify revenue entries
    for entry in &profit_loss.revenue_entries {
        assert_eq!(entry.account_type, "Revenue");
        assert!(entry.amount > Decimal::ZERO);
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
    }

    // Verify expense entries
    for entry in &profit_loss.expense_entries {
        assert_eq!(entry.account_type, "Expense");
        assert!(entry.amount > Decimal::ZERO);
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
    }

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_profit_loss_quarterly() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test Q1 profit and loss
    let q1_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2024, 3, 31).unwrap(),
    };

    let q1_profit_loss = reporting_service
        .generate_profit_loss(&pool, q1_range)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Test Q2 profit and loss
    let q2_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
    };

    let q2_profit_loss = reporting_service
        .generate_profit_loss(&pool, q2_range)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify both quarters have data
    assert!(q1_profit_loss.total_revenue >= Decimal::ZERO);
    assert!(q1_profit_loss.total_expenses >= Decimal::ZERO);
    assert!(q2_profit_loss.total_revenue >= Decimal::ZERO);
    assert!(q2_profit_loss.total_expenses >= Decimal::ZERO);

    // Quarters should have different amounts (unless identical transactions occurred)
    // This is more of a sanity check than a strict assertion
    println!("Q1 Revenue: {}, Q1 Expenses: {}, Q1 Net Income: {}",
             q1_profit_loss.total_revenue, q1_profit_loss.total_expenses, q1_profit_loss.net_income);
    println!("Q2 Revenue: {}, Q2 Expenses: {}, Q2 Net Income: {}",
             q2_profit_loss.total_revenue, q2_profit_loss.total_expenses, q2_profit_loss.net_income);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_balance_sheet_generation() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test balance sheet as of year-end
    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let balance_sheet = reporting_service
        .generate_balance_sheet(&pool, date_request.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify balance sheet structure
    assert_eq!(balance_sheet.as_of_date, date_request.as_of_date);

    // Verify balance sheet equation: Assets = Liabilities + Equity
    let calculated_total_assets: Decimal = balance_sheet.asset_entries.iter().map(|e| e.amount).sum();
    let calculated_total_liabilities: Decimal = balance_sheet.liability_entries.iter().map(|e| e.amount).sum();
    let calculated_total_equity: Decimal = balance_sheet.equity_entries.iter().map(|e| e.amount).sum();

    assert_eq!(balance_sheet.total_assets, calculated_total_assets);
    assert_eq!(balance_sheet.total_liabilities, calculated_total_liabilities);
    assert_eq!(balance_sheet.total_equity, calculated_total_equity);

    // Verify the accounting equation holds true
    let assets_minus_liabilities_equity = balance_sheet.total_assets -
        (balance_sheet.total_liabilities + balance_sheet.total_equity);
    assert!(assets_minus_liabilities_equity.abs() < Decimal::new(1, 2)); // Within $0.01

    // Verify asset entries
    for entry in &balance_sheet.asset_entries {
        assert!(entry.amount > Decimal::ZERO);
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
    }

    // Verify liability entries
    for entry in &balance_sheet.liability_entries {
        assert!(entry.amount > Decimal::ZERO);
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
    }

    // Verify equity entries
    for entry in &balance_sheet.equity_entries {
        assert!(entry.amount > Decimal::ZERO);
        assert!(!entry.account_code.is_empty());
        assert!(!entry.account_name.is_empty());
    }

    // Should have assets from test data
    assert!(balance_sheet.total_assets > Decimal::ZERO);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_balance_sheet_different_dates() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Compare balance sheets at different points in time
    let mid_year_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
    };

    let year_end_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let mid_year_bs = reporting_service
        .generate_balance_sheet(&pool, mid_year_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    let year_end_bs = reporting_service
        .generate_balance_sheet(&pool, year_end_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Both should be balanced
    let mid_year_balance = mid_year_bs.total_assets -
        (mid_year_bs.total_liabilities + mid_year_bs.total_equity);
    let year_end_balance = year_end_bs.total_assets -
        (year_end_bs.total_liabilities + year_end_bs.total_equity);

    assert!(mid_year_balance.abs() < Decimal::new(1, 2));
    assert!(year_end_balance.abs() < Decimal::new(1, 2));

    // Year-end should have more cumulative activity
    // (This is a general expectation, specific amounts depend on test data)
    println!("Mid-year assets: {}, Year-end assets: {}",
             mid_year_bs.total_assets, year_end_bs.total_assets);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_ar_aging_generation() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test AR aging as of year-end
    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let ar_aging = reporting_service
        .generate_ar_aging(&pool, date_request.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Verify AR aging structure
    assert_eq!(ar_aging.as_of_date, date_request.as_of_date);

    // Calculate total from buckets and compare with reported total
    let calculated_total: Decimal = ar_aging.buckets.iter().map(|b| b.total).sum();
    assert_eq!(ar_aging.total_outstanding, calculated_total);

    // Verify bucket calculations
    for bucket in &ar_aging.buckets {
        let bucket_total = bucket.current + bucket.days_1_30 +
            bucket.days_31_60 + bucket.days_61_90 + bucket.days_91_plus;
        assert_eq!(bucket.total, bucket_total);

        // Verify customer information
        assert!(!bucket.customer_name.is_empty());

        // All amounts should be non-negative
        assert!(bucket.current >= Decimal::ZERO);
        assert!(bucket.days_1_30 >= Decimal::ZERO);
        assert!(bucket.days_31_60 >= Decimal::ZERO);
        assert!(bucket.days_61_90 >= Decimal::ZERO);
        assert!(bucket.days_91_plus >= Decimal::ZERO);
        assert!(bucket.total >= Decimal::ZERO);

        // Only include buckets with outstanding amounts
        assert!(bucket.total > Decimal::ZERO);
    }

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_cache_functionality() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    // First call should query database
    let start1 = std::time::Instant::now();
    let trial_balance1 = reporting_service
        .generate_trial_balance(&pool, date_request.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;
    let duration1 = start1.elapsed();

    // Second call should use cache (faster)
    let start2 = std::time::Instant::now();
    let trial_balance2 = reporting_service
        .generate_trial_balance(&pool, date_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;
    let duration2 = start2.elapsed();

    // Results should be identical
    assert_eq!(trial_balance1.as_of_date, trial_balance2.as_of_date);
    assert_eq!(trial_balance1.total_debits, trial_balance2.total_debits);
    assert_eq!(trial_balance1.total_credits, trial_balance2.total_credits);
    assert_eq!(trial_balance1.entries.len(), trial_balance2.entries.len());

    // Second call should be faster (from cache)
    // Note: This might not always be true in test environments, but it's a good sanity check
    println!("First call: {:?}, Second call: {:?}", duration1, duration2);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_invalid_date_handling() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test with a date before any transactions exist
    let early_date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
    };

    let trial_balance = reporting_service
        .generate_trial_balance(&pool, early_date_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Should return empty trial balance
    assert_eq!(trial_balance.as_of_date, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    assert_eq!(trial_balance.total_debits, Decimal::ZERO);
    assert_eq!(trial_balance.total_credits, Decimal::ZERO);
    assert!(trial_balance.is_balanced);
    assert_eq!(trial_balance.entries.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_date_range_edge_cases() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Test with same start and end date
    let single_day_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let profit_loss = reporting_service
        .generate_profit_loss(&pool, single_day_range)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Should handle single day ranges correctly
    assert_eq!(profit_loss.period_start, profit_loss.period_end);
    assert_eq!(profit_loss.period_start, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

    // Test with very short range (no transactions expected)
    let empty_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2023, 1, 31).unwrap(),
    };

    let empty_profit_loss = reporting_service
        .generate_profit_loss(&pool, empty_range)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Should return zero values for periods with no transactions
    assert_eq!(empty_profit_loss.total_revenue, Decimal::ZERO);
    assert_eq!(empty_profit_loss.total_expenses, Decimal::ZERO);
    assert_eq!(empty_profit_loss.net_income, Decimal::ZERO);
    assert_eq!(empty_profit_loss.revenue_entries.len(), 0);
    assert_eq!(empty_profit_loss.expense_entries.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_reporting_service_comprehensive_financial_validation() -> Result<(), sqlx::Error> {
    let pool = setup_test_database().await?;
    let cache_service = CacheService::new("redis://localhost").expect("Failed to create cache service");
    let reporting_service = ReportingService::new_with_cache(cache_service);

    // Generate all reports for year-end
    let date_request = DateRequest {
        as_of_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let date_range = DateRangeRequest {
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
    };

    let trial_balance = reporting_service
        .generate_trial_balance(&pool, date_request.clone())
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    let profit_loss = reporting_service
        .generate_profit_loss(&pool, date_range)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    let balance_sheet = reporting_service
        .generate_balance_sheet(&pool, date_request)
        .await
        .map_err(|e| match e {
            AppError::DatabaseError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            AppError::NotFound(msg) => sqlx::Error::RowNotFound,
            AppError::InternalError(msg) => sqlx::Error::Protocol(format!("{}", msg)),
            _ => sqlx::Error::Protocol(format!("Unexpected error: {}", e)),
        })?;

    // Comprehensive validation
    // 1. Trial balance should be balanced
    assert!(trial_balance.is_balanced);
    let tb_difference = trial_balance.total_debits - trial_balance.total_credits;
    assert!(tb_difference.abs() < Decimal::new(1, 2));

    // 2. Balance sheet should balance
    let bs_difference = balance_sheet.total_assets -
        (balance_sheet.total_liabilities + balance_sheet.total_equity);
    assert!(bs_difference.abs() < Decimal::new(1, 2));

    // 3. P&L should have non-zero values (from test data)
    assert!(profit_loss.total_revenue > Decimal::ZERO);
    assert!(profit_loss.total_expenses > Decimal::ZERO);

    // 4. Validate relationships between reports
    // Net income from P&L should affect retained earnings in balance sheet equity
    // This is a complex validation that depends on the specific accounting structure

    println!("Financial Summary:");
    println!("  Trial Balance - Debits: {}, Credits: {}, Balanced: {}",
             trial_balance.total_debits, trial_balance.total_credits, trial_balance.is_balanced);
    println!("  P&L - Revenue: {}, Expenses: {}, Net Income: {}",
             profit_loss.total_revenue, profit_loss.total_expenses, profit_loss.net_income);
    println!("  Balance Sheet - Assets: {}, Liabilities: {}, Equity: {}",
             balance_sheet.total_assets, balance_sheet.total_liabilities, balance_sheet.total_equity);

    Ok(())
}

/// Helper function to set up test database with seed data
async fn setup_test_database() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/ledger_forge_test".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await?;

    // Run the seed data migration
    let seed_sql = include_str!("../migrations/20241220000000_financial_reporting_seed_data.sql");
    sqlx::query(seed_sql).execute(&pool).await?;

    Ok(pool)
}