//! Cache providers for SurrealX

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::error::Result;

/// Cache provider trait
#[async_trait]
pub trait CacheProvider: Send + Sync {
    /// Get a value from cache
    async fn get(&self, key: &str) -> Result<Option<Value>>;

    /// Set a value in cache with optional TTL (seconds)
    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<()>;

    /// Delete a value from cache
    async fn delete(&self, key: &str) -> Result<()>;

    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Clear all cache entries
    async fn clear(&self) -> Result<()>;
}

/// In-memory cache provider using SurrealDB's memory
#[derive(Clone)]
pub struct MemoryCacheProvider {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

struct CacheEntry {
    value: Value,
    expires_at: Option<i64>,
}

impl MemoryCacheProvider {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = chrono::Utc::now().timestamp();
        cache.retain(|_, entry| {
            entry.expires_at.map_or(true, |expires| expires > now)
        });
    }
}

#[async_trait]
impl CacheProvider for MemoryCacheProvider {
    async fn get(&self, key: &str) -> Result<Option<Value>> {
        self.cleanup_expired().await;
        let cache = self.cache.read().await;
        let now = chrono::Utc::now().timestamp();

        Ok(cache.get(key).and_then(|entry| {
            if entry.expires_at.map_or(true, |expires| expires > now) {
                Some(entry.value.clone())
            } else {
                None
            }
        }))
    }

    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<()> {
        let expires_at = ttl.map(|seconds| {
            chrono::Utc::now().timestamp() + seconds as i64
        });

        let mut cache = self.cache.write().await;
        cache.insert(
            key.to_string(),
            CacheEntry {
                value,
                expires_at,
            },
        );

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        Ok(self.get(key).await?.is_some())
    }

    async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }
}

impl Default for MemoryCacheProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Redis cache provider (requires redis-cache feature)
#[cfg(feature = "redis-cache")]
pub struct RedisCacheProvider {
    client: redis::Client,
}

#[cfg(feature = "redis-cache")]
impl RedisCacheProvider {
    pub fn new(url: impl AsRef<str>) -> Result<Self> {
        let client = redis::Client::open(url.as_ref())?;
        Ok(Self { client })
    }

    pub async fn from_client(client: redis::Client) -> Result<Self> {
        Ok(Self { client })
    }
}

#[cfg(feature = "redis-cache")]
#[async_trait]
impl CacheProvider for RedisCacheProvider {
    async fn get(&self, key: &str) -> Result<Option<Value>> {
        use redis::AsyncCommands;

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<()> {
        use redis::AsyncCommands;

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let json = serde_json::to_string(&value)?;

        if let Some(seconds) = ttl {
            conn.set_ex(key, json, seconds).await?;
        } else {
            conn.set(key, json).await?;
        }

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        use redis::AsyncCommands;

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del(key).await?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        use redis::AsyncCommands;

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    async fn clear(&self) -> Result<()> {
        use redis::AsyncCommands;

        let mut conn = self.client.get_multiplexed_async_connection().await?;
        redis::cmd("FLUSHDB").query_async(&mut conn).await?;
        Ok(())
    }
}
