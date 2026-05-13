//! Rate limiting middleware for gRPC services.
//!
//! This middleware intercepts requests after authentication, extracts user identity
//! and tier information, and enforces rate limits via Redis.
//!
//! # Middleware Stack Order
//!
//! ```text
//! 1. AuthLayer             → Validates JWT, extracts User (id, roles, metadata)
//! 2. OrganizationLayer     → Extracts organization_pid
//! 3. OrgContextLayer       → Validates org membership, looks up tier from Redis, inserts OrgTier
//! 4. UserContextLayer      → Auto-provisions users
//! 5. RateLimitLayer        → Checks rate limits using OrgTier ← THIS MIDDLEWARE
//! 6. Business Logic
//! ```
//!
//! # Fail-Open Behavior
//!
//! This middleware implements fail-open semantics:
//! - Missing `User` (unauthenticated) → allow request
//! - Missing `OrgTier` in extensions → use `Tier::Free` limits
//! - Redis errors → allow request (handled by store)
//!
//! # Example
//!
//! ```ignore
//! use common_rs::middleware::ratelimit::RateLimitLayer;
//! use common_rs::ratelimit::{RedisRateLimitStore, TierLimits, RateLimitConfig};
//!
//! const TIER_LIMITS: TierLimits = TierLimits {
//!     free: RateLimitConfig::new(60, 60),
//!     plus: RateLimitConfig::new(300, 60),
//!     pro: RateLimitConfig::new(1000, 60),
//!     enterprise: RateLimitConfig::new(5000, 60),
//! };
//!
//! let store = Arc::new(RedisRateLimitStore::new(redis_client, "my-service"));
//! let layer = RateLimitLayer::new(store, &TIER_LIMITS);
//! ```

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use http::{HeaderName, HeaderValue};
use tonic::Status;
use tower::{Layer, Service};

use super::BoxFuture;
use super::auth::User;
use crate::cache::PlanTier;
use crate::ratelimit::TierLimits;
use common_rs::ratelimit::{RateLimitResult, RateLimitStore};

/// Org tier extracted from cached user data by the org context middleware.
/// Inserted into request extensions so the rate limiter can read it.
#[derive(Clone, Debug)]
pub struct OrgTier(pub PlanTier);

/// Tracks store failures to suppress log spam.
///
/// Logs a warning on the first failure in each window, then counts
/// suppressed failures and logs a summary when the window rolls over.
struct FailureCounter {
    /// Failure count in the current window.
    count: AtomicU64,
    /// Start of the current window (unix secs).
    window_start: AtomicU64,
}

/// How long (in seconds) before the failure counter resets and re-emits a warning.
const FAILURE_LOG_WINDOW_SECS: u64 = 30;

impl FailureCounter {
    fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
            window_start: AtomicU64::new(0),
        }
    }

    /// Record a failure. Returns `true` if this is the first failure in the
    /// current window (caller should log a warning). Also returns the number
    /// of suppressed failures from the previous window, if the window rolled over.
    fn record(&self) -> (bool, u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before Unix epoch")
            .as_secs();

        let prev_start = self.window_start.load(Ordering::Relaxed);

        if now.saturating_sub(prev_start) >= FAILURE_LOG_WINDOW_SECS {
            // Window expired — try to reset
            if self
                .window_start
                .compare_exchange(prev_start, now, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                let prev_count = self.count.swap(1, Ordering::Relaxed);
                // Subtract 1 because the first failure in the old window was already logged
                let suppressed = prev_count.saturating_sub(1);
                return (true, suppressed);
            }
        }

        // Same window — just increment
        self.count.fetch_add(1, Ordering::Relaxed);
        (false, 0)
    }
}

/// Standard rate limit response headers.
const HEADER_RETRY_AFTER: HeaderName = HeaderName::from_static("retry-after");
const HEADER_RATELIMIT_LIMIT: HeaderName = HeaderName::from_static("x-ratelimit-limit");
const HEADER_RATELIMIT_REMAINING: HeaderName = HeaderName::from_static("x-ratelimit-remaining");
const HEADER_RATELIMIT_RESET: HeaderName = HeaderName::from_static("x-ratelimit-reset");

/// Tower layer that applies rate limiting to incoming requests.
///
/// This layer wraps services with `RateLimitMiddleware`, which checks
/// rate limits before forwarding requests to the inner service.
#[derive(Clone)]
pub struct RateLimitLayer<S> {
    store: Arc<S>,
    tier_limits: &'static TierLimits,
    failure_counter: Arc<FailureCounter>,
}

impl<S> RateLimitLayer<S>
where
    S: RateLimitStore,
{
    /// Create a new rate limit layer.
    ///
    /// # Arguments
    /// * `store` - Rate limit store (e.g., `RedisRateLimitStore`)
    /// * `tier_limits` - Static reference to tier-based rate limit configurations
    pub fn new(store: Arc<S>, tier_limits: &'static TierLimits) -> Self {
        Self {
            store,
            tier_limits,
            failure_counter: Arc::new(FailureCounter::new()),
        }
    }
}

impl<S, Inner> Layer<Inner> for RateLimitLayer<S>
where
    S: RateLimitStore,
{
    type Service = RateLimitMiddleware<S, Inner>;

    fn layer(&self, inner: Inner) -> Self::Service {
        RateLimitMiddleware {
            inner,
            store: self.store.clone(),
            tier_limits: self.tier_limits,
            failure_counter: self.failure_counter.clone(),
        }
    }
}

/// Middleware that enforces rate limits on incoming requests.
///
/// Extracts user identity from request extensions (populated by `AuthLayer`),
/// determines the user's tier, and checks rate limits against the configured store.
#[derive(Clone)]
pub struct RateLimitMiddleware<S, Inner> {
    inner: Inner,
    store: Arc<S>,
    tier_limits: &'static TierLimits,
    failure_counter: Arc<FailureCounter>,
}

impl<S, Inner, ReqBody, ResBody> Service<http::Request<ReqBody>> for RateLimitMiddleware<S, Inner>
where
    S: RateLimitStore + 'static,
    Inner: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    Inner::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default,
{
    type Response = Inner::Response;
    type Error = Inner::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        let store = self.store.clone();
        let tier_limits = self.tier_limits;
        let failure_counter = self.failure_counter.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract User - if missing, fail-open (rate limiting shouldn't break unauthenticated flows)
            let user = match req.extensions().get::<User>() {
                Some(u) => u,
                None => {
                    tracing::debug!("No User in request extensions - skipping rate limit check");
                    return inner.call(req).await;
                }
            };

            let user_id = &user.id;

            // Read tier from OrgTier extension (set by org context middleware via Redis).
            // Falls back to Free if not present (e.g., no org context on this request).
            let tier = req.extensions().get::<OrgTier>().map(|t| t.0).unwrap_or_default();
            let config = tier_limits.get(tier);

            // Check rate limit
            let result = match store.check_and_increment(user_id, config).await {
                Ok(result) => result,
                Err(e) => {
                    let (is_first, suppressed) = failure_counter.record();
                    if is_first {
                        if suppressed > 0 {
                            tracing::warn!(
                                error = %e,
                                suppressed,
                                window_secs = FAILURE_LOG_WINDOW_SECS,
                                "Rate limit store error - allowing requests (suppressed {suppressed} errors in previous window)"
                            );
                        } else {
                            tracing::warn!(error = %e, "Rate limit store error - allowing requests");
                        }
                    }
                    return inner.call(req).await;
                }
            };

            match result {
                RateLimitResult::Allowed { remaining, reset_at } => {
                    tracing::trace!(
                        user_id = %user_id,
                        tier = ?tier,
                        remaining = remaining,
                        reset_at = reset_at,
                        "Request allowed by rate limiter"
                    );

                    // Call inner service and add rate limit headers to response
                    let response = inner.call(req).await?;
                    Ok(add_ratelimit_headers(response, config.requests, remaining, reset_at))
                }
                RateLimitResult::Limited {
                    retry_after_secs,
                    reset_at,
                } => {
                    tracing::info!(
                        user_id = %user_id,
                        tier = ?tier,
                        retry_after_secs = retry_after_secs,
                        reset_at = reset_at,
                        "Request rate limited"
                    );

                    let message = format!("Rate limit exceeded. Try again in {} seconds.", retry_after_secs);

                    let mut response: http::Response<ResBody> = Status::resource_exhausted(message).into_http();

                    // Add rate limit headers to error response
                    add_ratelimit_headers_to_response(
                        response.headers_mut(),
                        config.requests,
                        0,
                        reset_at,
                        Some(retry_after_secs),
                    );

                    Ok(response)
                }
            }
        })
    }
}

/// Add rate limit headers to a successful response.
fn add_ratelimit_headers<B>(
    mut response: http::Response<B>,
    limit: u32,
    remaining: u32,
    reset_at: u64,
) -> http::Response<B> {
    add_ratelimit_headers_to_response(response.headers_mut(), limit, remaining, reset_at, None);
    response
}

/// Add rate limit headers to response headers.
fn add_ratelimit_headers_to_response(
    headers: &mut http::HeaderMap,
    limit: u32,
    remaining: u32,
    reset_at: u64,
    retry_after: Option<u64>,
) {
    // These conversions are infallible for numeric values
    if let Ok(v) = HeaderValue::from_str(&limit.to_string()) {
        headers.insert(HEADER_RATELIMIT_LIMIT.clone(), v);
    }

    if let Ok(v) = HeaderValue::from_str(&remaining.to_string()) {
        headers.insert(HEADER_RATELIMIT_REMAINING.clone(), v);
    }

    if let Ok(v) = HeaderValue::from_str(&reset_at.to_string()) {
        headers.insert(HEADER_RATELIMIT_RESET.clone(), v);
    }

    if let Some(retry) = retry_after
        && let Ok(v) = HeaderValue::from_str(&retry.to_string())
    {
        headers.insert(HEADER_RETRY_AFTER.clone(), v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_rs::ratelimit::{RateLimitConfig, RateLimitError};
    use std::convert::Infallible;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // Mock store for testing
    #[derive(Clone)]
    struct MockStore {
        result: RateLimitResult,
    }

    impl MockStore {
        fn allowed(remaining: u32, reset_at: u64) -> Self {
            Self {
                result: RateLimitResult::Allowed { remaining, reset_at },
            }
        }

        fn limited(retry_after_secs: u64, reset_at: u64) -> Self {
            Self {
                result: RateLimitResult::Limited {
                    retry_after_secs,
                    reset_at,
                },
            }
        }
    }

    impl RateLimitStore for MockStore {
        async fn check_and_increment(
            &self,
            _key: &str,
            _config: &RateLimitConfig,
        ) -> Result<RateLimitResult, RateLimitError> {
            Ok(self.result)
        }
    }

    // Mock inner service
    #[derive(Clone)]
    struct MockService;

    impl Service<http::Request<()>> for MockService {
        type Response = http::Response<()>;
        type Error = Infallible;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: http::Request<()>) -> Self::Future {
            Box::pin(async { Ok(http::Response::new(())) })
        }
    }

    const TEST_LIMITS: TierLimits = TierLimits {
        free: RateLimitConfig::new(60, 60),
        plus: RateLimitConfig::new(300, 60),
        pro: RateLimitConfig::new(1000, 60),
        enterprise: RateLimitConfig::new(5000, 60),
    };

    fn create_request_with_user() -> http::Request<()> {
        let mut req = http::Request::new(());
        req.extensions_mut().insert(User {
            id: "user_123".to_string(),
            email: Some("test@example.com".to_string()),
            name: Some("Test User".to_string()),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        });
        req
    }

    #[tokio::test]
    async fn should_allow_request_under_limit() {
        let store = Arc::new(MockStore::allowed(50, 1000));
        let layer = RateLimitLayer::new(store, &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        let req = create_request_with_user();
        let response = svc.call(req).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(response.headers().get("x-ratelimit-remaining").unwrap(), "50");
        assert_eq!(response.headers().get("x-ratelimit-limit").unwrap(), "60");
        assert_eq!(response.headers().get("x-ratelimit-reset").unwrap(), "1000");
    }

    #[tokio::test]
    async fn should_return_resource_exhausted_when_limited() {
        let store = Arc::new(MockStore::limited(30, 1000));
        let layer = RateLimitLayer::new(store, &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        let req = create_request_with_user();
        let response = svc.call(req).await.unwrap();

        // gRPC uses HTTP 200 with grpc-status header for errors
        // RESOURCE_EXHAUSTED = 8 in gRPC status codes
        assert_eq!(
            response.headers().get("grpc-status").unwrap(),
            "8" // RESOURCE_EXHAUSTED
        );
        assert_eq!(response.headers().get("retry-after").unwrap(), "30");
        assert_eq!(response.headers().get("x-ratelimit-remaining").unwrap(), "0");
    }

    #[tokio::test]
    async fn should_skip_rate_limit_when_user_missing() {
        let store = Arc::new(MockStore::limited(30, 1000));
        let layer = RateLimitLayer::new(store, &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        // Request without User
        let req = http::Request::new(());
        let response = svc.call(req).await.unwrap();

        // Should pass through without rate limiting
        assert_eq!(response.status(), http::StatusCode::OK);
        // No rate limit headers when skipped
        assert!(response.headers().get("x-ratelimit-limit").is_none());
    }

    #[tokio::test]
    async fn should_use_free_tier_when_tier_missing() {
        let store = Arc::new(MockStore::allowed(59, 1000));
        let layer = RateLimitLayer::new(store, &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        // Request with User but no tier set (defaults to Free)
        let req = create_request_with_user();
        let response = svc.call(req).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        // Free tier limit is 60
        assert_eq!(response.headers().get("x-ratelimit-limit").unwrap(), "60");
    }

    fn create_request_with_user_and_org_tier(tier: PlanTier) -> http::Request<()> {
        let mut req = create_request_with_user();
        req.extensions_mut().insert(OrgTier(tier));
        req
    }

    // A mock store that records which config was used, so we can verify
    // that OrgTier influences which tier limits the middleware selects.
    #[derive(Clone)]
    struct ConfigCapturingStore {
        captured_requests: Arc<std::sync::Mutex<Vec<u32>>>,
    }

    impl ConfigCapturingStore {
        fn new() -> Self {
            Self {
                captured_requests: Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }

        fn captured(&self) -> Vec<u32> {
            self.captured_requests.lock().unwrap().clone()
        }
    }

    impl RateLimitStore for ConfigCapturingStore {
        async fn check_and_increment(
            &self,
            _key: &str,
            config: &RateLimitConfig,
        ) -> Result<RateLimitResult, RateLimitError> {
            self.captured_requests.lock().unwrap().push(config.requests);
            Ok(RateLimitResult::Allowed {
                remaining: config.requests - 1,
                reset_at: 1000,
            })
        }
    }

    #[tokio::test]
    async fn should_use_pro_tier_limits_when_org_tier_is_pro() {
        let store = Arc::new(ConfigCapturingStore::new());
        let layer = RateLimitLayer::new(store.clone(), &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        let req = create_request_with_user_and_org_tier(PlanTier::Pro);
        let response = svc.call(req).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        // Pro tier limit is 1000
        assert_eq!(response.headers().get("x-ratelimit-limit").unwrap(), "1000");
        // Verify the store received the pro config
        assert_eq!(store.captured(), vec![1000]);
    }

    #[tokio::test]
    async fn should_use_enterprise_tier_limits_when_org_tier_is_enterprise() {
        let store = Arc::new(ConfigCapturingStore::new());
        let layer = RateLimitLayer::new(store.clone(), &TEST_LIMITS);
        let mut svc = layer.layer(MockService);

        let req = create_request_with_user_and_org_tier(PlanTier::Enterprise);
        let response = svc.call(req).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(response.headers().get("x-ratelimit-limit").unwrap(), "5000");
        assert_eq!(store.captured(), vec![5000]);
    }

    #[test]
    fn should_add_all_ratelimit_headers() {
        let mut headers = http::HeaderMap::new();
        add_ratelimit_headers_to_response(&mut headers, 100, 50, 1700000000, Some(30));

        assert_eq!(headers.get("x-ratelimit-limit").unwrap(), "100");
        assert_eq!(headers.get("x-ratelimit-remaining").unwrap(), "50");
        assert_eq!(headers.get("x-ratelimit-reset").unwrap(), "1700000000");
        assert_eq!(headers.get("retry-after").unwrap(), "30");
    }

    #[test]
    fn should_omit_retry_after_when_not_limited() {
        let mut headers = http::HeaderMap::new();
        add_ratelimit_headers_to_response(&mut headers, 100, 50, 1700000000, None);

        assert!(headers.get("retry-after").is_none());
    }
}
