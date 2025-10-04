use redis::{Client, AsyncCommands};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::utils::{AppError, Result};

/// Cache service for Redis operations
#[derive(Clone)]
pub struct CacheService {
    client: Client,
}

impl CacheService {
    /// Create a new cache service with Redis client
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?;

        Ok(Self { client })
    }

    /// Get a Redis connection
    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))
    }

    /// Set a cache value with TTL
    pub async fn set_with_ttl<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: u64) -> Result<()> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| AppError::InternalError(format!("Serialization failed: {}", e)))?;

        let mut conn = self.get_connection().await?;
        let _: () = conn.set_ex(key, serialized, ttl_seconds)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis SET failed: {}", e)))?;

        Ok(())
    }

    /// Get a cached value
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.get_connection().await?;

        let result: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis GET failed: {}", e)))?;

        match result {
            Some(serialized) => {
                let deserialized: T = serde_json::from_str(&serialized)
                    .map_err(|e| AppError::InternalError(format!("Deserialization failed: {}", e)))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    /// Delete a cache key
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let _: () = conn.del(key)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis DEL failed: {}", e)))?;
        Ok(())
    }

    /// Delete keys matching a pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<u64> {
        let mut conn = self.get_connection().await?;

        // Get all keys matching pattern
        let keys: Vec<String> = conn
            .keys(pattern)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis KEYS failed: {}", e)))?;

        if keys.is_empty() {
            return Ok(0);
        }

        // Delete all matching keys
        let deleted: u64 = conn
            .del(&keys)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis DEL failed: {}", e)))?;

        Ok(deleted)
    }

    /// Check Redis health
    pub async fn health_check(&self) -> Result<bool> {
        let mut conn = self.get_connection().await?;

        // Try a simple SET/GET operation to test connection
        let test_key = "health_check_test";
        let test_value = "ok";

        let _: () = conn.set(test_key, test_value)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis SET failed: {}", e)))?;

        let result: String = conn
            .get(test_key)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis GET failed: {}", e)))?;

        // Clean up test key
        let _: () = conn
            .del(test_key)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis DEL failed: {}", e)))?;

        Ok(result == test_value)
    }

    // === Account Balance Caching ===

    /// Get cached account balance
    pub async fn get_account_balance(&self, account_id: Uuid) -> Result<Option<Decimal>> {
        let key = format!("account:balance:{}", account_id);
        self.get(&key).await
    }

    /// Set cached account balance
    pub async fn set_account_balance(&self, account_id: Uuid, balance: Decimal) -> Result<()> {
        let key = format!("account:balance:{}", account_id);
        // 5 minute TTL for account balances
        self.set_with_ttl(&key, &balance, 300).await
    }

    /// Invalidate account balance
    pub async fn invalidate_account_balance(&self, account_id: Uuid) -> Result<()> {
        let key = format!("account:balance:{}", account_id);
        self.delete(&key).await
    }

    /// Invalidate transaction list caches (simple pattern deletion)
    pub async fn invalidate_transaction_lists(&self) -> Result<()> {
        self.delete_pattern("transactions:list:*").await.map(|_| ())
    }
}

impl Default for CacheService {
    fn default() -> Self {
        // Try to get Redis URL from environment, fallback to localhost
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());

        Self::new(&redis_url)
            .expect("Failed to create cache service with default Redis URL")
    }
}