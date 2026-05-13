/// Compile-time configuration for a rate limit.
///
/// This struct is designed to be used as a `const` value, ensuring
/// rate limits are baked into the binary at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed in the window
    pub requests: u32,
    /// Window duration in seconds
    pub window_secs: u64,
}

impl RateLimitConfig {
    /// Create a new rate limit configuration.
    ///
    /// This is a const fn, allowing use in static/const contexts.
    pub const fn new(requests: u32, window_secs: u64) -> Self {
        Self {
            requests,
            window_secs,
        }
    }
}

/// Result of a rate limit check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitResult {
    /// Request is allowed
    Allowed {
        /// Number of requests remaining in the current window
        remaining: u32,
        /// Unix timestamp when the current window resets
        reset_at: u64,
    },
    /// Request is rate limited
    Limited {
        /// Seconds until the client should retry
        retry_after_secs: u64,
        /// Unix timestamp when the current window resets
        reset_at: u64,
    },
}

impl RateLimitResult {
    /// Returns true if the request is allowed.
    pub const fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed { .. })
    }

    /// Returns true if the request is rate limited.
    pub const fn is_limited(&self) -> bool {
        matches!(self, Self::Limited { .. })
    }

    /// Returns the reset timestamp for the current window.
    pub const fn reset_at(&self) -> u64 {
        match self {
            Self::Allowed { reset_at, .. } | Self::Limited { reset_at, .. } => *reset_at,
        }
    }

    /// Returns the number of remaining requests, or 0 if limited.
    pub const fn remaining(&self) -> u32 {
        match self {
            Self::Allowed { remaining, .. } => *remaining,
            Self::Limited { .. } => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_config_with_new() {
        let config = RateLimitConfig::new(100, 60);
        assert_eq!(config.requests, 100);
        assert_eq!(config.window_secs, 60);
    }

    #[test]
    fn should_allow_const_config() {
        const CONFIG: RateLimitConfig = RateLimitConfig::new(60, 60);
        assert_eq!(CONFIG.requests, 60);
        assert_eq!(CONFIG.window_secs, 60);
    }

    #[test]
    fn should_report_allowed_correctly() {
        let result = RateLimitResult::Allowed {
            remaining: 50,
            reset_at: 1000,
        };
        assert!(result.is_allowed());
        assert!(!result.is_limited());
        assert_eq!(result.remaining(), 50);
        assert_eq!(result.reset_at(), 1000);
    }

    #[test]
    fn should_report_limited_correctly() {
        let result = RateLimitResult::Limited {
            retry_after_secs: 30,
            reset_at: 1000,
        };
        assert!(!result.is_allowed());
        assert!(result.is_limited());
        assert_eq!(result.remaining(), 0);
        assert_eq!(result.reset_at(), 1000);
    }
}
