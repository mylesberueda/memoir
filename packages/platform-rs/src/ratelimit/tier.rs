use crate::cache::PlanTier;
use common_rs::ratelimit::RateLimitConfig;

/// Maps subscription tiers to rate limit configurations.
///
/// Each service defines its own `TierLimits` const with appropriate
/// limits for that service's workload.
///
/// # Example
/// ```ignore
/// const TIER_LIMITS: TierLimits = TierLimits {
///     free: RateLimitConfig::new(60, 60),        // 60 req/min
///     plus: RateLimitConfig::new(300, 60),       // 300 req/min
///     pro: RateLimitConfig::new(1000, 60),       // 1000 req/min
///     enterprise: RateLimitConfig::new(5000, 60), // 5000 req/min
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TierLimits {
    pub free: RateLimitConfig,
    pub plus: RateLimitConfig,
    pub pro: RateLimitConfig,
    pub enterprise: RateLimitConfig,
}

impl TierLimits {
    /// Get the rate limit configuration for a tier.
    pub const fn get(&self, tier: PlanTier) -> &RateLimitConfig {
        match tier {
            PlanTier::Free => &self.free,
            PlanTier::Plus => &self.plus,
            PlanTier::Pro => &self.pro,
            PlanTier::Enterprise => &self.enterprise,
        }
    }

    /// Apply a multiplier from `RATE_LIMIT_MULTIPLIER` env var (default: 1).
    ///
    /// Call this at startup to scale all tier limits for dev/test environments.
    /// Example: `RATE_LIMIT_MULTIPLIER=10` makes all tiers 10x more permissive.
    pub fn with_env_multiplier(self) -> Self {
        let multiplier: u32 = std::env::var("RATE_LIMIT_MULTIPLIER")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1)
            .max(1);

        if multiplier == 1 {
            return self;
        }

        tracing::info!(multiplier, "Applying rate limit multiplier from RATE_LIMIT_MULTIPLIER");
        Self {
            free: RateLimitConfig::new(self.free.requests.saturating_mul(multiplier), self.free.window_secs),
            plus: RateLimitConfig::new(self.plus.requests.saturating_mul(multiplier), self.plus.window_secs),
            pro: RateLimitConfig::new(self.pro.requests.saturating_mul(multiplier), self.pro.window_secs),
            enterprise: RateLimitConfig::new(
                self.enterprise.requests.saturating_mul(multiplier),
                self.enterprise.window_secs,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIMITS: TierLimits = TierLimits {
        free: RateLimitConfig::new(60, 60),
        plus: RateLimitConfig::new(300, 60),
        pro: RateLimitConfig::new(1000, 60),
        enterprise: RateLimitConfig::new(5000, 60),
    };

    #[test]
    fn should_get_free_limits() {
        let config = TEST_LIMITS.get(PlanTier::Free);
        assert_eq!(config.requests, 60);
    }

    #[test]
    fn should_get_plus_limits() {
        let config = TEST_LIMITS.get(PlanTier::Plus);
        assert_eq!(config.requests, 300);
    }

    #[test]
    fn should_get_pro_limits() {
        let config = TEST_LIMITS.get(PlanTier::Pro);
        assert_eq!(config.requests, 1000);
    }

    #[test]
    fn should_get_enterprise_limits() {
        let config = TEST_LIMITS.get(PlanTier::Enterprise);
        assert_eq!(config.requests, 5000);
    }
}
