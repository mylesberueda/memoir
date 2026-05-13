use fred::{
    clients::Client as RedisClient,
    interfaces::{ClientLike, KeysInterface},
    types::config::Config as RedisConfig,
};
use std::sync::Arc;
use test_context::AsyncTestContext;

/// Test context for Redis integration tests.
///
/// Provides a connected Redis client and helper methods for test isolation.
/// Keys created during tests are tracked and cleaned up on teardown.
///
/// # Usage
///
/// ```ignore
/// use test_context::test_context;
/// use common_rs::test_utils::RedisTestContext;
///
/// #[test_context(RedisTestContext)]
/// #[tokio::test]
/// async fn should_do_something(ctx: &mut RedisTestContext) {
///     let key = ctx.unique_key("mytest");
///     // ... test using key ...
/// }
/// ```
pub struct RedisTestContext {
    pub redis: Arc<RedisClient>,
    test_prefix: String,
    created_keys: Vec<String>,
}

impl AsyncTestContext for RedisTestContext {
    async fn setup() -> Self {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set for integration tests");
        let redis_config = RedisConfig::from_url(&redis_url).expect("Invalid REDIS_URL");

        let redis = RedisClient::new(redis_config, None, None, None);
        redis.init().await.expect("Failed to connect to Redis");

        // Generate unique prefix for this test run to avoid collisions
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        Self {
            redis: Arc::new(redis),
            test_prefix: format!("test:{}:", timestamp),
            created_keys: Vec::new(),
        }
    }

    async fn teardown(self) {
        // Clean up all tracked keys
        for key in &self.created_keys {
            let _: Result<(), _> = self.redis.del(key).await;
        }
    }
}

impl RedisTestContext {
    /// Generate a unique key for this test run.
    ///
    /// The key is prefixed with a timestamp-based namespace to avoid
    /// collisions with other test runs. The key is tracked for cleanup.
    pub fn unique_key(&mut self, suffix: &str) -> String {
        let key = format!("{}{}", self.test_prefix, suffix);
        self.created_keys.push(key.clone());
        key
    }

    /// Generate a unique user ID for rate limit testing.
    ///
    /// Returns a user ID that will create unique rate limit keys.
    pub fn unique_user_id(&mut self, suffix: &str) -> String {
        let user_id = format!("{}user_{}", self.test_prefix, suffix);
        // Don't track directly - the rate limit store will create keys with this
        user_id
    }

    /// Track a key pattern for cleanup (e.g., rate limit keys).
    ///
    /// Use this when the store creates keys you want cleaned up.
    pub fn track_key(&mut self, key: String) {
        self.created_keys.push(key);
    }

    /// Track rate limit keys for a user/service combination.
    ///
    /// Rate limit keys follow the pattern: `ratelimit:api:{service}:{user}:window:*`
    /// This tracks a wildcard pattern for cleanup.
    pub fn track_ratelimit_user(&mut self, service: &str, user_id: &str) {
        // We'll clean up by scanning for keys matching this pattern
        // For now, track a marker that teardown can use
        self.created_keys
            .push(format!("ratelimit:api:{}:{}:*", service, user_id));
    }
}
