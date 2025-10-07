use ledger_forge::services::{ImportService, CacheService};
use ledger_forge::utils::parse_accounts_csv;

mod common;
use common::test_db::TestDb;

#[tokio::test]
async fn test_import_accounts_from_csv() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    let csv_data = r#"code,name,account_type,parent_code,description
1000,Cash,Asset,,Primary cash account
1100,Accounts Receivable,Asset,,Customer receivables
2000,Accounts Payable,Liability,,Vendor payables
3000,Owner's Equity,Equity,,Owner's capital
4000,Sales Revenue,Revenue,,Product sales
5000,Operating Expenses,Expense,,General expenses"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await
        .unwrap();

    // Verify import results
    assert_eq!(result.total_rows, 6);
    assert_eq!(result.successful, 6);
    assert_eq!(result.failed, 0);
    assert!(result.errors.is_empty());
}

#[tokio::test]
async fn test_import_accounts_with_hierarchy() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    // CSV with parent-child relationships
    let csv_data = r#"code,name,account_type,parent_code,description
6000,Operating Expenses,Expense,,Parent expense account
6010,Rent Expense,Expense,6000,Office rent
6020,Utilities,Expense,6000,Electricity and water
6030,Office Supplies,Expense,6000,Stationery and supplies"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await
        .unwrap();

    // Verify all accounts were created including children
    assert_eq!(result.total_rows, 4);
    assert_eq!(result.successful, 4);
    assert_eq!(result.failed, 0);
}

#[tokio::test]
async fn test_import_accounts_with_invalid_type() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    let csv_data = r#"code,name,account_type,parent_code,description
1000,Cash,InvalidType,,Should fail"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await
        .unwrap();

    // Should have 1 failed row
    assert_eq!(result.total_rows, 1);
    assert_eq!(result.successful, 0);
    assert_eq!(result.failed, 1);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].error_message.contains("Invalid account type"));
}

#[tokio::test]
async fn test_import_accounts_with_missing_parent() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    // Child account references non-existent parent
    let csv_data = r#"code,name,account_type,parent_code,description
6010,Rent Expense,Expense,6000,Parent doesn't exist"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await
        .unwrap();

    // Should fail because parent doesn't exist
    assert_eq!(result.total_rows, 1);
    assert_eq!(result.successful, 0);
    assert_eq!(result.failed, 1);
    assert!(result.errors[0].error_message.contains("Parent account"));
}

#[tokio::test]
async fn test_import_accounts_duplicate_code() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    // First import
    let csv_data1 = r#"code,name,account_type,parent_code,description
1000,Cash,Asset,,Cash account"#;

    let result1 = import_service
        .import_accounts_from_csv(pool, csv_data1.as_bytes())
        .await
        .unwrap();

    assert_eq!(result1.successful, 1);

    // Try to import duplicate code
    let csv_data2 = r#"code,name,account_type,parent_code,description
1000,Duplicate Cash,Asset,,Should fail"#;

    let result2 = import_service
        .import_accounts_from_csv(pool, csv_data2.as_bytes())
        .await
        .unwrap();

    // Should fail due to duplicate code
    assert_eq!(result2.successful, 0);
    assert_eq!(result2.failed, 1);
}

#[tokio::test]
async fn test_parse_csv_with_various_account_types() {
    let csv_data = r#"code,name,account_type,parent_code,description
1000,Cash,asset,,Test lowercase
2000,AP,LIABILITY,,Test uppercase
3000,Equity,Equity,,Test mixed case
4000,Sales,revenue,,Test revenue alias
5000,COGS,EXPENSE,,Test expense uppercase"#;

    let records = parse_accounts_csv(csv_data.as_bytes()).unwrap();
    
    assert_eq!(records.len(), 5);
    // Verify all account types are parsed
    assert_eq!(records[0].account_type, "asset");
    assert_eq!(records[1].account_type, "LIABILITY");
    assert_eq!(records[2].account_type, "Equity");
    assert_eq!(records[3].account_type, "revenue");
    assert_eq!(records[4].account_type, "EXPENSE");
}

#[tokio::test]
async fn test_get_csv_template() {
    let template = ImportService::get_accounts_csv_template();
    
    // Verify template structure
    assert!(template.contains("code,name,account_type"));
    assert!(template.contains("1000,Cash,Asset"));
    assert!(template.contains("parent_code"));
    assert!(template.contains("description"));
    
    // Verify it's valid CSV
    let records = parse_accounts_csv(template.as_bytes()).unwrap();
    assert!(records.len() > 0);
}

#[tokio::test]
async fn test_import_empty_csv() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    let csv_data = r#"code,name,account_type,parent_code,description"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await;

    // Should fail with empty CSV
    assert!(result.is_err());
}

#[tokio::test]
async fn test_import_partial_success() {
    let test_db = TestDb::new().await;
    let pool = test_db.pool();
    
    let cache_service = CacheService::new("redis://localhost:6379").unwrap();
    let import_service = ImportService::new(cache_service);

    // Mix of valid and invalid records
    let csv_data = r#"code,name,account_type,parent_code,description
1000,Cash,Asset,,Valid account
2000,AP,InvalidType,,Invalid type
3000,Equity,Equity,,Valid account
4000,Sales,Revenue,,Valid account"#;

    let result = import_service
        .import_accounts_from_csv(pool, csv_data.as_bytes())
        .await
        .unwrap();

    // Should have partial success
    assert_eq!(result.total_rows, 4);
    assert_eq!(result.successful, 3);
    assert_eq!(result.failed, 1);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].code, "2000");
}