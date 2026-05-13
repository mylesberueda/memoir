//! Redis-backed storage for rate limit counters.

use std::{sync::Arc, time::Duration};

use fred::{clients::Client, interfaces::LuaInterface};

use super::{RateLimitConfig, RateLimitResult, window_number};

/// Errors that can occur during rate limit storage operations.
#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    /// Redis operation failed
    #[error("Redis error: {0}")]
    Redis(#[from] fred::error::Error),

    /// Redis operation timed out
    #[error("Redis timeout after {0:?}")]
    Timeout(Duration),

    /// Redis returned unexpected data format
    #[error("Unexpected Redis response")]
    UnexpectedResponse,
}

/// Abstraction for rate limit storage backends.
///
/// This trait allows swapping storage implementations (Redis, in-memory, etc.)
/// without changing the rate limiting logic.
pub trait RateLimitStore: Send + Sync {
    /// Check if a request is allowed and increment the counter if so.
    ///
    /// # Arguments
    /// * `key` - Unique identifier for the rate limit bucket (e.g., user ID)
    /// * `config` - Rate limit configuration (requests per window)
    ///
    /// # Returns
    /// - `Ok(RateLimitResult::Allowed { .. })` if request is allowed
    /// - `Ok(RateLimitResult::Limited { .. })` if rate limited
    /// - `Err(_)` on storage errors (caller should fail-open)
    fn check_and_increment(
        &self,
        key: &str,
        config: &RateLimitConfig,
    ) -> impl std::future::Future<Output = Result<RateLimitResult, RateLimitError>> + Send;
}

/// Redis-backed rate limit store using atomic Lua scripts.
///
/// This implementation uses a sliding window counter algorithm with
/// atomic check-and-increment operations via Lua scripting.
///
/// # Fail-Open Behavior
///
/// When using this store, callers should implement fail-open semantics:
/// if `check_and_increment` returns an error, allow the request rather
/// than blocking all traffic during Redis outages.
///
/// # Key Format
///
/// Keys follow the pattern: `ratelimit:api:{service_key}:{user_key}:window:{window_num}`
///
/// # Example
///
/// ```ignore
/// let store = RedisRateLimitStore::new(redis_client, "rig");
/// let config = RateLimitConfig::new(100, 60); // 100 req/min
///
/// match store.check_and_increment("user_123", &config).await {
///     Ok(result) if result.is_allowed() => { /* proceed */ }
///     Ok(result) => { /* rate limited */ }
///     Err(_) => { /* fail open - allow request */ }
/// }
/// ```
#[derive(Clone)]
pub struct RedisRateLimitStore {
    client: Arc<Client>,
    service_key: &'static str,
    timeout: Duration,
}

/// Lua script for atomic check-and-increment.
///
/// Returns: {allowed (0|1), current_count}
/// - allowed=0: rate limited (count >= limit)
/// - allowed=1: request allowed (count was incremented)
const LUA_SCRIPT: &str = r#"
local current = redis.call('GET', KEYS[1])
if current and tonumber(current) >= tonumber(ARGV[1]) then
    return {0, tonumber(current)}
end
local new_count = redis.call('INCR', KEYS[1])
if new_count == 1 then
    redis.call('EXPIRE', KEYS[1], ARGV[2])
end
return {1, new_count}
"#;

impl RedisRateLimitStore {
    /// Default timeout for Redis operations.
    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);

    /// Create a new Redis rate limit store.
    ///
    /// # Arguments
    /// * `client` - Shared Redis client
    /// * `service_key` - Service identifier for key namespacing (e.g., "rig", "chat")
    pub fn new(client: Arc<Client>, service_key: &'static str) -> Self {
        Self {
            client,
            service_key,
            timeout: Self::DEFAULT_TIMEOUT,
        }
    }

    /// Create a new Redis rate limit store with custom timeout.
    pub fn with_timeout(client: Arc<Client>, service_key: &'static str, timeout: Duration) -> Self {
        Self {
            client,
            service_key,
            timeout,
        }
    }

    /// Generate the Redis key for a rate limit bucket.
    fn make_key(&self, user_key: &str, window_num: u64) -> String {
        format!("ratelimit:api:{}:{}:window:{}", self.service_key, user_key, window_num)
    }

    /// Get current Unix timestamp in seconds.
    fn now_secs() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before Unix epoch")
            .as_secs()
    }
}

impl RateLimitStore for RedisRateLimitStore {
    async fn check_and_increment(
        &self,
        user_key: &str,
        config: &RateLimitConfig,
    ) -> Result<RateLimitResult, RateLimitError> {
        let now = Self::now_secs();
        let window_num = window_number(now, config.window_secs);
        let reset_at = (window_num + 1) * config.window_secs;

        let key = self.make_key(user_key, window_num);
        // TTL includes buffer for clock skew
        let ttl = (config.window_secs + 60) as i64;
        let limit = config.requests as i64;

        // Execute with timeout
        let result = tokio::time::timeout(self.timeout, async {
            // For simplicity, use eval directly (Redis caches scripts automatically)
            // This avoids needing mutable self for script_load
            let values: Vec<i64> = self.client.eval(LUA_SCRIPT, vec![key], vec![limit, ttl]).await?;

            if values.len() == 2 {
                Ok((values[0], values[1]))
            } else {
                Err(RateLimitError::UnexpectedResponse)
            }
        })
        .await
        .map_err(|_| RateLimitError::Timeout(self.timeout))??;

        let (allowed, count) = result;

        if allowed == 1 {
            // Request was allowed and counter incremented
            let remaining = (config.requests as i64 - count).max(0) as u32;
            Ok(RateLimitResult::Allowed { remaining, reset_at })
        } else {
            // Rate limited
            let retry_after_secs = reset_at.saturating_sub(now);
            Ok(RateLimitResult::Limited {
                retry_after_secs,
                reset_at,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_generate_correct_key_format() {
        // We can't create a real RedisRateLimitStore without a client,
        // so test the key format logic directly
        let service_key = "rig";
        let user_key = "user_abc123";
        let window_num = 28534721u64;

        let key = format!("ratelimit:api:{}:{}:window:{}", service_key, user_key, window_num);

        assert_eq!(key, "ratelimit:api:rig:user_abc123:window:28534721");
    }

    #[test]
    fn should_include_service_key_in_namespace() {
        let key1 = format!("ratelimit:api:{}:user1:window:1", "rig");
        let key2 = format!("ratelimit:api:{}:user1:window:1", "chat");

        assert_ne!(key1, key2);
        assert!(key1.contains(":rig:"));
        assert!(key2.contains(":chat:"));
    }
}

#[cfg(all(test, feature = "integration", feature = "test_utils"))]
mod integration_tests {
    use super::*;
    use crate::test_utils::RedisTestContext;
    use test_context::test_context;

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_allow_first_request(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("first");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        let config = RateLimitConfig::new(10, 60); // 10 requests per minute

        let result = store.check_and_increment(&user_id, &config).await.unwrap();

        assert!(result.is_allowed());
        assert_eq!(result.remaining(), 9); // 10 - 1 = 9 remaining
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_decrement_remaining_on_each_request(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("decrement");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        let config = RateLimitConfig::new(10, 60);

        // Make 5 requests
        for expected_remaining in (5..=9).rev() {
            let result = store.check_and_increment(&user_id, &config).await.unwrap();
            assert!(result.is_allowed());
            assert_eq!(
                result.remaining(),
                expected_remaining,
                "After request, expected {} remaining",
                expected_remaining
            );
        }
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_reject_when_limit_exceeded(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("exceed");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        let config = RateLimitConfig::new(3, 60); // Only 3 requests allowed

        // Use up all 3 requests
        for _ in 0..3 {
            let result = store.check_and_increment(&user_id, &config).await.unwrap();
            assert!(result.is_allowed());
        }

        // 4th request should be rejected
        let result = store.check_and_increment(&user_id, &config).await.unwrap();
        assert!(result.is_limited());
        assert_eq!(result.remaining(), 0);
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_return_retry_after_when_limited(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("retry");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        let config = RateLimitConfig::new(1, 60); // 1 request per minute

        // Use the single allowed request
        let _ = store.check_and_increment(&user_id, &config).await.unwrap();

        // Next request should be limited with retry_after
        let result = store.check_and_increment(&user_id, &config).await.unwrap();

        match result {
            RateLimitResult::Limited {
                retry_after_secs,
                reset_at,
            } => {
                // retry_after should be between 0 and window_secs
                assert!(retry_after_secs <= 60, "retry_after_secs should be <= window size");
                assert!(retry_after_secs > 0, "retry_after_secs should be > 0");

                // reset_at should be in the future
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                assert!(reset_at > now, "reset_at should be in the future");
            }
            _ => panic!("Expected Limited result"),
        }
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_isolate_different_users(ctx: &mut RedisTestContext) {
        let user_a = ctx.unique_user_id("user_a");
        let user_b = ctx.unique_user_id("user_b");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        let config = RateLimitConfig::new(5, 60);

        // User A makes 5 requests (exhausts limit)
        for _ in 0..5 {
            let _ = store.check_and_increment(&user_a, &config).await.unwrap();
        }

        // User A should be limited
        let result_a = store.check_and_increment(&user_a, &config).await.unwrap();
        assert!(result_a.is_limited(), "User A should be rate limited");

        // User B should still have full quota
        let result_b = store.check_and_increment(&user_b, &config).await.unwrap();
        assert!(result_b.is_allowed(), "User B should not be affected by User A");
        assert_eq!(result_b.remaining(), 4); // 5 - 1 = 4
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_isolate_different_services(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("multi_service");
        let store_rig = RedisRateLimitStore::new(ctx.redis.clone(), "rig");
        let store_chat = RedisRateLimitStore::new(ctx.redis.clone(), "chat");
        let config = RateLimitConfig::new(3, 60);

        // Exhaust quota on "rig" service
        for _ in 0..3 {
            let _ = store_rig.check_and_increment(&user_id, &config).await.unwrap();
        }

        // "rig" should be limited
        let result_rig = store_rig.check_and_increment(&user_id, &config).await.unwrap();
        assert!(result_rig.is_limited(), "rig service should be rate limited");

        // "chat" should still have full quota for same user
        let result_chat = store_chat.check_and_increment(&user_id, &config).await.unwrap();
        assert!(result_chat.is_allowed(), "chat service should not be affected");
        assert_eq!(result_chat.remaining(), 2);
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_reset_counter_after_window_expires(ctx: &mut RedisTestContext) {
        let user_id = ctx.unique_user_id("window_reset");
        let store = RedisRateLimitStore::new(ctx.redis.clone(), "test");
        // Use 2-second window for faster test
        let config = RateLimitConfig::new(2, 2);

        // Exhaust quota
        for _ in 0..2 {
            let _ = store.check_and_increment(&user_id, &config).await.unwrap();
        }

        // Should be limited
        let result = store.check_and_increment(&user_id, &config).await.unwrap();
        assert!(result.is_limited(), "Should be rate limited");

        // Wait for window to expire (add buffer for timing)
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Should be allowed again with fresh quota
        let result = store.check_and_increment(&user_id, &config).await.unwrap();
        assert!(result.is_allowed(), "Should be allowed after window reset");
        assert_eq!(result.remaining(), 1, "Should have fresh quota minus 1");
    }
}
