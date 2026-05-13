use super::config::{RateLimitConfig, RateLimitResult};

/// Calculate the window number for a given timestamp.
///
/// Window numbers are used to generate Redis keys and determine
/// when counters should reset.
///
/// # Example
/// ```
/// use common_rs::ratelimit::window_number;
///
/// // 60-second windows
/// assert_eq!(window_number(0, 60), 0);
/// assert_eq!(window_number(59, 60), 0);
/// assert_eq!(window_number(60, 60), 1);
/// assert_eq!(window_number(120, 60), 2);
/// ```
#[inline]
pub const fn window_number(timestamp_secs: u64, window_secs: u64) -> u64 {
    timestamp_secs / window_secs
}

/// Calculate the reset timestamp for a given window.
///
/// Returns the Unix timestamp when the window ends (start of next window).
#[inline]
pub const fn window_reset_at(window_num: u64, window_secs: u64) -> u64 {
    (window_num + 1) * window_secs
}

/// Sliding window counter algorithm for rate limiting.
///
/// This is a pure algorithm with no I/O - it takes current state
/// and returns the rate limit decision. The caller is responsible
/// for reading/writing the counter from storage (e.g., Redis).
pub struct SlidingWindowCounter;

impl SlidingWindowCounter {
    /// Check if a request should be allowed based on current counter state.
    ///
    /// # Arguments
    /// * `current_count` - Current number of requests in this window
    /// * `now_secs` - Current Unix timestamp in seconds
    /// * `config` - Rate limit configuration
    ///
    /// # Returns
    /// `RateLimitResult::Allowed` if under limit, `RateLimitResult::Limited` if over.
    ///
    /// # Note
    /// This check happens *before* incrementing the counter. If `Allowed`,
    /// the caller should increment the counter in storage.
    pub fn check(current_count: u64, now_secs: u64, config: &RateLimitConfig) -> RateLimitResult {
        let window_num = window_number(now_secs, config.window_secs);
        let reset_at = window_reset_at(window_num, config.window_secs);

        if current_count < config.requests as u64 {
            // Request allowed - calculate remaining capacity
            let remaining = (config.requests as u64).saturating_sub(current_count + 1) as u32;
            RateLimitResult::Allowed { remaining, reset_at }
        } else {
            // Rate limited - calculate retry time
            let retry_after_secs = reset_at.saturating_sub(now_secs);
            RateLimitResult::Limited {
                retry_after_secs,
                reset_at,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: RateLimitConfig = RateLimitConfig::new(100, 60);

    #[test]
    fn should_calculate_window_number_at_zero() {
        assert_eq!(window_number(0, 60), 0);
    }

    #[test]
    fn should_calculate_window_number_within_first_window() {
        assert_eq!(window_number(59, 60), 0);
    }

    #[test]
    fn should_calculate_window_number_at_boundary() {
        assert_eq!(window_number(60, 60), 1);
    }

    #[test]
    fn should_calculate_window_number_in_second_window() {
        assert_eq!(window_number(119, 60), 1);
    }

    #[test]
    fn should_calculate_window_number_in_third_window() {
        assert_eq!(window_number(120, 60), 2);
    }

    #[test]
    fn should_allow_when_count_is_zero() {
        let result = SlidingWindowCounter::check(0, 30, &CONFIG);
        assert!(result.is_allowed());
        // After this request, 99 remaining
        assert_eq!(result.remaining(), 99);
        assert_eq!(result.reset_at(), 60);
    }

    #[test]
    fn should_allow_when_under_limit() {
        let result = SlidingWindowCounter::check(99, 30, &CONFIG);
        assert!(result.is_allowed());
        // This is the 100th request, 0 remaining after
        assert_eq!(result.remaining(), 0);
        assert_eq!(result.reset_at(), 60);
    }

    #[test]
    fn should_limit_when_at_limit() {
        let result = SlidingWindowCounter::check(100, 30, &CONFIG);
        assert!(result.is_limited());
        assert_eq!(result.remaining(), 0);
        // 30 seconds until window reset
        if let RateLimitResult::Limited {
            retry_after_secs, ..
        } = result
        {
            assert_eq!(retry_after_secs, 30);
        }
    }

    #[test]
    fn should_limit_when_over_limit() {
        let result = SlidingWindowCounter::check(150, 30, &CONFIG);
        assert!(result.is_limited());
    }

    #[test]
    fn should_calculate_correct_reset_time() {
        // At timestamp 90 (in window 1), reset should be at 120
        let result = SlidingWindowCounter::check(0, 90, &CONFIG);
        assert_eq!(result.reset_at(), 120);
    }

    #[test]
    fn should_calculate_retry_after_near_window_end() {
        // At timestamp 55, only 5 seconds until reset
        let result = SlidingWindowCounter::check(100, 55, &CONFIG);
        if let RateLimitResult::Limited {
            retry_after_secs, ..
        } = result
        {
            assert_eq!(retry_after_secs, 5);
        }
    }
}
