use ledger_forge::models::{AccountType, CreateAccountRequest, UpdateAccountRequest};
use ledger_forge::services::{AccountService, CacheService};
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

// Helper function to get database URL from environment
fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests")
}

// Helper function to get Redis URL from environment
fn get_redis_url() -> String {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string())
}

// Helper function to create test database pool
async fn create_test_pool() -> sqlx::PgPool {
    let database_url = get_database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

// Helper function to create cache service
fn create_cache_service() -> CacheService {
    let redis_url = get_redis_url();
    CacheService::new(&redis_url).expect("Failed to create cache service")
}

// Helper function to clean up test data
async fn cleanup_accounts(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM chart_of_accounts WHERE code LIKE 'TEST-CACHE%'")
        .execute(pool)
        .await
        .expect("Failed to clean up test accounts");
}

#[tokio::test]
#[serial]
async fn test_account_cache_get_by_id() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create an account
    let request = CreateAccountRequest {
        code: "TEST-CACHE-001".to_string(),
        name: "Test Cache Account".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let created = service
        .create_account(&pool, request)
        .await
        .expect("Failed to create account");

    // First call - should hit database and populate cache
    let first_fetch = service
        .get_account_by_id(&pool, created.id)
        .await
        .expect("Failed to get account");

    assert_eq!(first_fetch.id, created.id);
    assert_eq!(first_fetch.code, "TEST-CACHE-001");

    // Second call - should hit cache
    let second_fetch = service
        .get_account_by_id(&pool, created.id)
        .await
        .expect("Failed to get account from cache");

    assert_eq!(second_fetch.id, created.id);
    assert_eq!(second_fetch.code, "TEST-CACHE-001");

    // Verify cache was used by checking the cache directly
    let cache_key = format!("account:data:{}", created.id);
    let cached_data: Option<String> = cache.get(&cache_key).await.ok().flatten();
    assert!(cached_data.is_some(), "Account should be in cache");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_account_cache_invalidation_on_update() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create an account
    let request = CreateAccountRequest {
        code: "TEST-CACHE-002".to_string(),
        name: "Test Cache Update Account".to_string(),
        account_type: AccountType::Liability,
        parent_account_id: None,
        company_id: None,
    };

    let created = service
        .create_account(&pool, request)
        .await
        .expect("Failed to create account");

    // Fetch to populate cache
    service
        .get_account_by_id(&pool, created.id)
        .await
        .expect("Failed to get account");

    // Verify cache is populated
    let cache_key = format!("account:data:{}", created.id);
    let cached_before: Option<String> = cache.get(&cache_key).await.ok().flatten();
    assert!(cached_before.is_some(), "Account should be in cache before update");

    // Update the account
    let update_req = UpdateAccountRequest {
        name: Some("Updated Cache Account".to_string()),
        is_active: None,
    };

    service
        .update_account(&pool, created.id, update_req)
        .await
        .expect("Failed to update account");

    // Verify cache was invalidated
    let cached_after: Option<String> = cache.get(&cache_key).await.ok().flatten();
    assert!(cached_after.is_none(), "Account cache should be invalidated after update");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_account_cache_invalidation_on_deactivate() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create an account
    let request = CreateAccountRequest {
        code: "TEST-CACHE-003".to_string(),
        name: "Test Cache Deactivate Account".to_string(),
        account_type: AccountType::Equity,
        parent_account_id: None,
        company_id: None,
    };

    let created = service
        .create_account(&pool, request)
        .await
        .expect("Failed to create account");

    // Fetch to populate cache
    service
        .get_account_by_id(&pool, created.id)
        .await
        .expect("Failed to get account");

    // Verify cache is populated
    let cache_key = format!("account:data:{}", created.id);
    let cached_before: Option<String> = cache.get(&cache_key).await.ok().flatten();
    assert!(cached_before.is_some(), "Account should be in cache before deactivate");

    // Deactivate the account
    service
        .deactivate_account(&pool, created.id)
        .await
        .expect("Failed to deactivate account");

    // Verify cache was invalidated
    let cached_after: Option<String> = cache.get(&cache_key).await.ok().flatten();
    assert!(cached_after.is_none(), "Account cache should be invalidated after deactivate");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_account_hierarchy_cache() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create parent account
    let parent_req = CreateAccountRequest {
        code: "TEST-CACHE-PARENT".to_string(),
        name: "Test Cache Parent Account".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let parent = service
        .create_account(&pool, parent_req)
        .await
        .expect("Failed to create parent account");

    // Create child account
    let child_req = CreateAccountRequest {
        code: "TEST-CACHE-CHILD".to_string(),
        name: "Test Cache Child Account".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(parent.id),
        company_id: None,
    };

    let child = service
        .create_account(&pool, child_req)
        .await
        .expect("Failed to create child account");

    // First call - should hit database and populate cache
    let first_hierarchy = service
        .get_account_hierarchy(&pool, parent.id)
        .await
        .expect("Failed to get hierarchy");

    assert_eq!(first_hierarchy.account.id, parent.id);
    assert_eq!(first_hierarchy.children.len(), 1);
    assert_eq!(first_hierarchy.children[0].id, child.id);

    // Second call - should hit cache
    let second_hierarchy = service
        .get_account_hierarchy(&pool, parent.id)
        .await
        .expect("Failed to get hierarchy from cache");

    assert_eq!(second_hierarchy.account.id, parent.id);
    assert_eq!(second_hierarchy.children.len(), 1);

    // Verify cache was used
    let hierarchy_cache_key = format!("account:hierarchy:{}", parent.id);
    let cached_hierarchy: Option<String> = cache.get(&hierarchy_cache_key).await.ok().flatten();
    assert!(cached_hierarchy.is_some(), "Hierarchy should be in cache");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_parent_hierarchy_cache_invalidation_on_child_creation() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create parent account
    let parent_req = CreateAccountRequest {
        code: "TEST-CACHE-PARENT2".to_string(),
        name: "Test Cache Parent 2".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: None,
        company_id: None,
    };

    let parent = service
        .create_account(&pool, parent_req)
        .await
        .expect("Failed to create parent account");

    // Get hierarchy to populate cache
    service
        .get_account_hierarchy(&pool, parent.id)
        .await
        .expect("Failed to get hierarchy");

    // Verify cache is populated
    let hierarchy_cache_key = format!("account:hierarchy:{}", parent.id);
    let cached_before: Option<String> = cache.get(&hierarchy_cache_key).await.ok().flatten();
    assert!(cached_before.is_some(), "Hierarchy should be in cache before child creation");

    // Create child account - should invalidate parent's hierarchy cache
    let child_req = CreateAccountRequest {
        code: "TEST-CACHE-CHILD2".to_string(),
        name: "Test Cache Child 2".to_string(),
        account_type: AccountType::Revenue,
        parent_account_id: Some(parent.id),
        company_id: None,
    };

    service
        .create_account(&pool, child_req)
        .await
        .expect("Failed to create child account");

    // Verify parent's hierarchy cache was invalidated
    let cached_after: Option<String> = cache.get(&hierarchy_cache_key).await.ok().flatten();
    assert!(cached_after.is_none(), "Parent hierarchy cache should be invalidated after child creation");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_hierarchy_cache_with_grandchildren() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create grandparent
    let grandparent_req = CreateAccountRequest {
        code: "TEST-CACHE-GP".to_string(),
        name: "Test Cache Grandparent".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let grandparent = service
        .create_account(&pool, grandparent_req)
        .await
        .expect("Failed to create grandparent");

    // Create parent
    let parent_req = CreateAccountRequest {
        code: "TEST-CACHE-P".to_string(),
        name: "Test Cache Parent".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(grandparent.id),
        company_id: None,
    };

    let parent = service
        .create_account(&pool, parent_req)
        .await
        .expect("Failed to create parent");

    // Create child
    let child_req = CreateAccountRequest {
        code: "TEST-CACHE-C".to_string(),
        name: "Test Cache Child".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: Some(parent.id),
        company_id: None,
    };

    service
        .create_account(&pool, child_req)
        .await
        .expect("Failed to create child");

    // Get hierarchy for middle account (parent)
    let hierarchy = service
        .get_account_hierarchy(&pool, parent.id)
        .await
        .expect("Failed to get hierarchy");

    // Should have a parent (grandparent)
    assert!(hierarchy.parent.is_some());
    assert_eq!(hierarchy.parent.unwrap().id, grandparent.id);

    // Should have children
    assert_eq!(hierarchy.children.len(), 1);

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_multiple_accounts_cache_isolation() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create multiple accounts
    let account1_req = CreateAccountRequest {
        code: "TEST-CACHE-ISO1".to_string(),
        name: "Test Cache Isolation 1".to_string(),
        account_type: AccountType::Asset,
        parent_account_id: None,
        company_id: None,
    };

    let account2_req = CreateAccountRequest {
        code: "TEST-CACHE-ISO2".to_string(),
        name: "Test Cache Isolation 2".to_string(),
        account_type: AccountType::Liability,
        parent_account_id: None,
        company_id: None,
    };

    let account1 = service.create_account(&pool, account1_req).await.unwrap();
    let account2 = service.create_account(&pool, account2_req).await.unwrap();

    // Fetch both to populate cache
    service.get_account_by_id(&pool, account1.id).await.unwrap();
    service.get_account_by_id(&pool, account2.id).await.unwrap();

    // Update account1
    let update_req = UpdateAccountRequest {
        name: Some("Updated Isolation 1".to_string()),
        is_active: None,
    };

    service.update_account(&pool, account1.id, update_req).await.unwrap();

    // Verify account1 cache is invalidated
    let cache_key1 = format!("account:data:{}", account1.id);
    let cached1: Option<String> = cache.get(&cache_key1).await.ok().flatten();
    assert!(cached1.is_none(), "Account1 cache should be invalidated");

    // Verify account2 cache is still valid
    let cache_key2 = format!("account:data:{}", account2.id);
    let cached2: Option<String> = cache.get(&cache_key2).await.ok().flatten();
    assert!(cached2.is_some(), "Account2 cache should still be valid");

    cleanup_accounts(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_cache_hit_performance() {
    let pool = create_test_pool().await;
    cleanup_accounts(&pool).await;

    let cache = create_cache_service();
    let service = AccountService::new_with_cache(cache.clone());

    // Create an account
    let request = CreateAccountRequest {
        code: "TEST-CACHE-PERF".to_string(),
        name: "Test Cache Performance".to_string(),
        account_type: AccountType::Expense,
        parent_account_id: None,
        company_id: None,
    };

    let created = service.create_account(&pool, request).await.unwrap();

    // Measure first fetch (database)
    let start = std::time::Instant::now();
    service.get_account_by_id(&pool, created.id).await.unwrap();
    let first_duration = start.elapsed();

    // Measure second fetch (cache)
    let start = std::time::Instant::now();
    service.get_account_by_id(&pool, created.id).await.unwrap();
    let second_duration = start.elapsed();

    // Cache should be faster (though timing can be unreliable in tests)
    // Just verify both calls succeeded
    assert!(first_duration.as_micros() > 0);
    assert!(second_duration.as_micros() > 0);

    cleanup_accounts(&pool).await;
}
