use ledger_forge::models::{Account, AccountType, CreateAccountRequest, UpdateAccountRequest};
use ledger_forge::services::AccountService;
use sqlx::PgPool;
use uuid::Uuid;

mod common;
use common::{cleanup_test_db, setup_test_db};

#[tokio::test]
#[serial_test::serial]
async fn test_create_account_success() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();
    let req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let result = service.create_account(&pool, req).await;

    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.code, "1000");
    assert_eq!(account.name, "Cash");
    assert_eq!(account.account_type, AccountType::Asset);
    assert!(account.is_active);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_account_duplicate_code() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create first account
    let req1 = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    service.create_account(&pool, req1).await.unwrap();

    // Try to create duplicate
    let req2 = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash 2".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let result = service.create_account(&pool, req2).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_account_with_parent() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create parent account
    let parent_req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let parent = service.create_account(&pool, parent_req).await.unwrap();

    // Create child account
    let child_req = CreateAccountRequest {
        code: "1100".to_string(),
        name: "Current Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(parent.id),
        company_id: None,
    };

    let result = service.create_account(&pool, child_req).await;
    assert!(result.is_ok());
    let child = result.unwrap();
    assert_eq!(child.parent_account_id, Some(parent.id));

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_create_account_invalid_parent() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    let req = CreateAccountRequest {
        code: "1100".to_string(),
        name: "Current Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(Uuid::new_v4()), // Non-existent parent
        company_id: None,
    };

    let result = service.create_account(&pool, req).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_accounts() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create multiple accounts
    let accounts = vec![
        ("1000", "Cash", AccountType::Asset),
        ("2000", "Accounts Payable", AccountType::Liability),
        ("3000", "Equity", AccountType::Equity),
    ];

    for (code, name, acc_type) in accounts {
        let req = CreateAccountRequest {
            code: code.to_string(),
            name: name.to_string(),
            account_type: acc_type,
            parent_account_id: None,
            company_id: None,
        };
        service.create_account(&pool, req).await.unwrap();
    }

    // List all accounts
    let result = service.list_accounts(&pool, None, None, false).await;
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.len(), 3);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_accounts_by_type() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create accounts of different types
    let req1 = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    service.create_account(&pool, req1).await.unwrap();

    let req2 = CreateAccountRequest {
        code: "2000".to_string(),
        name: "Accounts Payable".to_string(),
        account_type: AccountType::Liability,
        parent_account_id: None,
        company_id: None,
    };
    service.create_account(&pool, req2).await.unwrap();

    // Filter by Asset type
    let result = service.list_accounts(&pool, Some(AccountType::Asset), None, false).await;
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].account_type, AccountType::Asset);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_by_id() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    let req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let created = service.create_account(&pool, req).await.unwrap();

    // Get by ID
    let result = service.get_account_by_id(&pool, created.id).await;
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.id, created.id);
    assert_eq!(account.code, "1000");

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_not_found() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    let result = service.get_account_by_id(&pool, Uuid::new_v4()).await;
    assert!(result.is_err());

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_update_account() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    let req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let created = service.create_account(&pool, req).await.unwrap();

    // Update account
    let update_req = UpdateAccountRequest {
        name: Some("Cash on Hand".to_string()),
        is_active: None,
    };

    let result = service.update_account(&pool, created.id, update_req).await;
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.name, "Cash on Hand");
    assert_eq!(updated.code, "1000"); // Code should not change

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_deactivate_account() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    let req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let created = service.create_account(&pool, req).await.unwrap();

    // Deactivate account
    let result = service.deactivate_account(&pool, created.id).await;
    assert!(result.is_ok());
    let deactivated = result.unwrap();
    assert!(!deactivated.is_active);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_hierarchy() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create parent
    let parent_req = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let parent = service.create_account(&pool, parent_req).await.unwrap();

    // Create children
    let child1_req = CreateAccountRequest {
        code: "1100".to_string(),
        name: "Current Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(parent.id),
        company_id: None,
    };
    service.create_account(&pool, child1_req).await.unwrap();

    let child2_req = CreateAccountRequest {
        code: "1200".to_string(),
        name: "Fixed Assets".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(parent.id),
        company_id: None,
    };
    service.create_account(&pool, child2_req).await.unwrap();

    // Get hierarchy
    let result = service.get_account_hierarchy(&pool, parent.id).await;
    assert!(result.is_ok());
    let hierarchy = result.unwrap();
    assert_eq!(hierarchy.account.id, parent.id);
    assert!(hierarchy.parent.is_none());
    assert_eq!(hierarchy.children.len(), 2);

    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_list_accounts_include_inactive() {
    let pool = setup_test_db().await;
    cleanup_test_db(&pool).await;

    let service = AccountService::new();

    // Create active account
    let req1 = CreateAccountRequest {
        code: "1000".to_string(),
        name: "Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let account1 = service.create_account(&pool, req1).await.unwrap();

    // Create and deactivate another account
    let req2 = CreateAccountRequest {
        code: "1100".to_string(),
        name: "Petty Cash".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };
    let account2 = service.create_account(&pool, req2).await.unwrap();
    service.deactivate_account(&pool, account2.id).await.unwrap();

    // List only active accounts
    let active_list = service.list_accounts(&pool, None, None, false).await.unwrap();
    assert_eq!(active_list.len(), 1);

    // List including inactive
    let all_list = service.list_accounts(&pool, None, None, true).await.unwrap();
    assert_eq!(all_list.len(), 2);

    cleanup_test_db(&pool).await;
}
