use common_rs::ratelimit::RateLimitConfig;
use platform_rs::ratelimit::TierLimits;

pub(crate) const REDIS_SERVICE_KEY: &str = "notification";

/// Rate limits for notification-service (internal + UI - high limits).
///
/// Notification service is called by both internal services (rig-service,
/// chat-service) and UI. Internal calls count against user's quota, so limits
/// are higher to avoid blocking legitimate workflows.
pub(crate) const TIER_LIMITS: TierLimits = TierLimits {
    free: RateLimitConfig::new(200, 60),         // 200 req/min
    plus: RateLimitConfig::new(1000, 60),        // 1000 req/min
    pro: RateLimitConfig::new(2000, 60),         // 2000 req/min
    enterprise: RateLimitConfig::new(10000, 60), // 10000 req/min
};
