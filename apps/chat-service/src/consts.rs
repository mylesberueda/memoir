use common_rs::ratelimit::RateLimitConfig;
use platform_rs::ratelimit::TierLimits;

pub(crate) const REDIS_SERVICE_KEY: &str = "chat";

/// Rate limits for chat-service (messaging - moderate cost).
///
/// Chat service handles real-time messaging and channel operations.
/// Limits are moderate to allow for active conversations.
pub(crate) const TIER_LIMITS: TierLimits = TierLimits {
    free: RateLimitConfig::new(60, 60),         // 60 req/min
    plus: RateLimitConfig::new(300, 60),        // 300 req/min
    pro: RateLimitConfig::new(600, 60),         // 600 req/min
    enterprise: RateLimitConfig::new(2000, 60), // 2000 req/min
};
