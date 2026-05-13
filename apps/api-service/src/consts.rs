use common_rs::ratelimit::RateLimitConfig;
use platform_rs::ratelimit::TierLimits;

pub(crate) const REDIS_SERVICE_KEY: &str = "api";

/// Rate limits for api-service (user/org management - moderate cost).
///
/// API service handles user profiles, organizations, and billing operations.
/// Limits are more generous than AI inference but still bounded.
pub(crate) const TIER_LIMITS: TierLimits = TierLimits {
    free: RateLimitConfig::new(60, 60),         // 60 req/min
    plus: RateLimitConfig::new(200, 60),        // 200 req/min
    pro: RateLimitConfig::new(500, 60),         // 500 req/min
    enterprise: RateLimitConfig::new(2000, 60), // 2000 req/min
};
