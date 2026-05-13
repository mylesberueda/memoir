use std::sync::Arc;

use fred::{clients::Client, interfaces::KeysInterface};
use serde::{Deserialize, Serialize};

use super::{OrgRole, PlanTier, ResolvedPermissions};

/// Cached org membership entry for a user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedOrg {
    pub pid: String,
    pub tier: PlanTier,
    pub role: OrgRole,
    /// Pre-resolved permission matrix (role defaults + per-member overrides).
    /// Absent = legacy cache entry, treat as role defaults.
    #[serde(default)]
    pub permissions: ResolvedPermissions,
}

impl CachedOrg {
    /// Create a CachedOrg with default (role-based) permissions.
    /// Use this for construction sites that don't need explicit permissions.
    pub fn new(pid: impl Into<String>, tier: PlanTier, role: OrgRole) -> Self {
        Self {
            pid: pid.into(),
            tier,
            role,
            permissions: Default::default(),
        }
    }
}

/// Cached user data stored in Redis. One key per user.
///
/// Shape: `{ email: "em@il.io", organizations: [{ pid: "org-1", tier: "pro", role: "member" }] }`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedUserData {
    pub email: String,
    pub organizations: Vec<CachedOrg>,
}

impl CachedUserData {
    /// Find the org entry matching a given pid.
    pub fn org(&self, org_pid: &str) -> Option<&CachedOrg> {
        self.organizations.iter().find(|o| o.pid == org_pid)
    }
}

/// Shared cache for user data, backed by Redis.
///
/// api-service is the single writer (on provisioning, membership changes, plan changes).
/// Other services (rig-service, etc.) are read-only consumers.
///
/// # Key Format
///
/// `{service_key}:user-cache:{user_id}`
///
/// # Example
///
/// ```ignore
/// let cache = UserCache::new(redis_client, "api");
///
/// // Write (api-service only)
/// cache.set("user_123", &CachedUserData {
///     email: "user@example.com".into(),
///     organizations: vec![CachedOrg::new("org_abc", PlanTier::Pro, OrgRole::Owner)],
/// }).await;
///
/// // Read (any service)
/// let data = cache.get("user_123").await; // Some(CachedUserData { ... })
/// ```
#[derive(Clone, Debug)]
pub struct UserCache {
    client: Arc<Client>,
    service_key: &'static str,
}

impl UserCache {
    pub fn new(client: Arc<Client>, service_key: &'static str) -> Self {
        Self { client, service_key }
    }

    /// Read cached user data. Returns `None` on cache miss or deserialization error.
    pub async fn get(&self, user_id: &str) -> Option<CachedUserData> {
        let key = self.key(user_id);
        match self.client.get::<Option<String>, _>(&key).await {
            Ok(Some(json)) => match serde_json::from_str(&json) {
                Ok(data) => Some(data),
                Err(e) => {
                    tracing::warn!(user_id = %user_id, error = %e, "Failed to deserialize user cache");
                    None
                }
            },
            Ok(None) => None,
            Err(e) => {
                tracing::warn!(user_id = %user_id, error = %e, "Failed to read user cache from Redis");
                None
            }
        }
    }

    /// Write cached user data. No TTL — keys are persistent.
    pub async fn set(&self, user_id: &str, data: &CachedUserData) {
        let key = self.key(user_id);
        let json = match serde_json::to_string(data) {
            Ok(j) => j,
            Err(e) => {
                tracing::error!(user_id = %user_id, error = %e, "Failed to serialize user cache");
                return;
            }
        };
        if let Err(e) = self.client.set::<(), _, _>(&key, json, None, None, false).await {
            tracing::error!(user_id = %user_id, error = %e, "Failed to write user cache to Redis");
        }
    }

    /// Delete cached user data (e.g., on user deletion).
    pub async fn delete(&self, user_id: &str) {
        let key = self.key(user_id);
        if let Err(e) = self.client.del::<(), _>(&key).await {
            tracing::error!(user_id = %user_id, error = %e, "Failed to delete user cache from Redis");
        }
    }

    fn key(&self, user_id: &str) -> String {
        format!("{}:user-cache:{}", self.service_key, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_correct_key_format() {
        let key = format!("{}:user-cache:{}", "api", "user_123");
        assert_eq!(key, "api:user-cache:user_123");
    }

    #[test]
    fn should_serialize_cached_user_data() {
        let data = CachedUserData {
            email: "test@example.com".into(),
            organizations: vec![
                CachedOrg::new("org-1", PlanTier::Pro, OrgRole::Owner),
                CachedOrg::new("org-2", PlanTier::Free, OrgRole::Member),
            ],
        };

        let json = serde_json::to_string(&data).unwrap();
        let deserialized: CachedUserData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn should_find_org_by_pid() {
        let data = CachedUserData {
            email: "test@example.com".into(),
            organizations: vec![
                CachedOrg::new("org-1", PlanTier::Pro, OrgRole::Owner),
                CachedOrg::new("org-2", PlanTier::Free, OrgRole::Member),
            ],
        };

        assert_eq!(data.org("org-1").unwrap().tier, PlanTier::Pro);
        assert_eq!(data.org("org-2").unwrap().role, OrgRole::Member);
        assert!(data.org("org-3").is_none());
    }
}

#[cfg(all(test, feature = "integration", feature = "test_utils"))]
mod integration_tests {
    use super::*;
    use common_rs::test_utils::RedisTestContext;
    use test_context::test_context;

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_write_and_read_user_cache(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        let data = CachedUserData {
            email: "test@example.com".into(),
            organizations: vec![CachedOrg::new("org-1", PlanTier::Pro, OrgRole::Owner)],
        };

        cache.set("user_1", &data).await;
        let result = cache.get("user_1").await;

        assert_eq!(result, Some(data));
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_return_none_on_cache_miss(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        let result = cache.get("user_nonexistent").await;

        assert_eq!(result, None);
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_overwrite_on_update(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        let v1 = CachedUserData {
            email: "v1@example.com".into(),
            organizations: vec![CachedOrg::new("org-1", PlanTier::Free, OrgRole::Owner)],
        };

        let v2 = CachedUserData {
            email: "v1@example.com".into(),
            organizations: vec![
                CachedOrg::new("org-1", PlanTier::Pro, OrgRole::Owner),
                CachedOrg::new("org-2", PlanTier::Free, OrgRole::Member),
            ],
        };

        cache.set("user_update", &v1).await;
        assert_eq!(cache.get("user_update").await.unwrap().organizations.len(), 1);

        cache.set("user_update", &v2).await;
        let result = cache.get("user_update").await.unwrap();
        assert_eq!(result.organizations.len(), 2);
        assert_eq!(result.org("org-1").unwrap().tier, PlanTier::Pro);
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_delete_user_cache(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        let data = CachedUserData {
            email: "delete@example.com".into(),
            organizations: vec![CachedOrg::new("org-1", PlanTier::Free, OrgRole::Owner)],
        };

        cache.set("user_delete", &data).await;
        assert!(cache.get("user_delete").await.is_some());

        cache.delete("user_delete").await;
        assert!(cache.get("user_delete").await.is_none());
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_isolate_users(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        let data_a = CachedUserData {
            email: "a@example.com".into(),
            organizations: vec![CachedOrg::new("org-a", PlanTier::Pro, OrgRole::Owner)],
        };

        let data_b = CachedUserData {
            email: "b@example.com".into(),
            organizations: vec![CachedOrg::new("org-b", PlanTier::Free, OrgRole::Member)],
        };

        cache.set("user_a", &data_a).await;
        cache.set("user_b", &data_b).await;

        assert_eq!(cache.get("user_a").await.unwrap().email, "a@example.com");
        assert_eq!(cache.get("user_b").await.unwrap().email, "b@example.com");
    }

    #[test_context(RedisTestContext)]
    #[tokio::test]
    async fn should_return_none_on_corrupt_json(ctx: &mut RedisTestContext) {
        let cache = UserCache::new(ctx.redis.clone(), "test");

        // Write raw invalid JSON directly to the cache key
        let key = format!("test:user-cache:user_corrupt");
        ctx.redis
            .set::<(), _, _>(&key, "not valid json {{{", None, None, false)
            .await
            .unwrap();

        // get() should return None gracefully, not panic
        let result = cache.get("user_corrupt").await;
        assert_eq!(result, None);
    }
}
