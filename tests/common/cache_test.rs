use ledger_forge::services::CacheService;
use std::env;

/// Setup Redis cache for testing
pub async fn setup_test_cache() -> CacheService {
    // Load .env file for test environment
    dotenvy::dotenv().ok();

    // Use test Redis URL or fallback to localhost
    let redis_url = env::var("TEST_REDIS_URL")
        .unwrap_or_else(|_| {
            // Try production Redis URL, then fallback
            env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string())
        });

    // Create cache service
    match CacheService::new(&redis_url) {
        Ok(cache) => {
            // Test Redis connectivity
            if let Err(_) = cache.health_check().await {
                panic!("Redis is not available at {}. Please ensure Redis is running for cache tests.", redis_url);
            }
            cache
        }
        Err(e) => {
            panic!("Failed to connect to Redis at {}: {:?}", redis_url, e);
        }
    }
}

/// Clear all test cache data
pub async fn clear_test_cache(cache: &CacheService) {
    // Clear account balance caches
    let _ = cache.delete_pattern("account:balance:*").await;

    // Clear transaction list caches
    let _ = cache.delete_pattern("transactions:list:*").await;

    // Clear any remaining test data
    let _ = cache.delete("health_check_test").await;
}

/// Check if Redis is available for testing
pub async fn is_redis_available() -> bool {
    // Load .env file for test environment
    dotenvy::dotenv().ok();

    let redis_url = env::var("TEST_REDIS_URL")
        .unwrap_or_else(|_| {
            env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string())
        });

    if let Ok(cache) = CacheService::new(&redis_url) {
        cache.health_check().await.unwrap_or(false)
    } else {
        false
    }
}

/// Create a test cache service that works even if Redis is not available
/// This allows tests to run without Redis but with reduced functionality
pub async fn setup_test_cache_fallback() -> CacheService {
    if is_redis_available().await {
        setup_test_cache().await
    } else {
        // Create a mock cache service that doesn't actually cache
        // This allows tests to run without Redis
        CacheService::default()
    }
}