use common_rs::ratelimit::RateLimitConfig;
use platform_rs::ratelimit::TierLimits;

pub(crate) const REDIS_SERVICE_KEY: &str = "rig";

/// User cache key prefix — reads from api-service's cache (api-service is the single writer).
pub(crate) const REDIS_USER_CACHE_KEY: &str = "api";

/// Rate limits for rig-service (AI inference - most expensive).
///
/// AI inference is computationally expensive and involves external API calls,
/// so limits are more restrictive than other services.
pub(crate) const TIER_LIMITS: TierLimits = TierLimits {
    free: RateLimitConfig::new(30, 60),         // 30 req/min
    plus: RateLimitConfig::new(120, 60),        // 120 req/min
    pro: RateLimitConfig::new(300, 60),         // 300 req/min
    enterprise: RateLimitConfig::new(1000, 60), // 1000 req/min
};
