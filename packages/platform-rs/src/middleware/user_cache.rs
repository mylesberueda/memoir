use super::BoxFuture;
use super::auth::User;
use crate::cache::UserCache;
use tower::{Layer, Service};

/// Middleware layer that fetches user data from Redis via `UserCache`.
///
/// Does exactly ONE Redis GET per request using the authenticated user's ID.
/// Inserts `CachedUserData` into request extensions for downstream middleware
/// (UserContext, OrganizationContext) to read from — no additional Redis calls needed.
///
/// # Middleware Stack Order
///
/// ```text
/// 1. AuthLayer            → extracts User (id from JWT sub claim)
/// 2. UserCacheMiddleware   → ONE Redis GET, inserts CachedUserData  ← THIS
/// 3. OrganizationLayer     → extracts org pid from header
/// 4. UserContextMiddleware → reads CachedUserData, populates UserContext
/// 5. OrgContextMiddleware  → reads CachedUserData + org pid, populates OrgContext + OrgTier
/// 6. RateLimitLayer        → reads OrgTier
/// ```
///
/// If the user is not in cache (miss), the request continues without `CachedUserData`.
/// Downstream middleware should handle this gracefully (fall back to DB, default to free, etc.).
#[derive(Debug, Clone)]
pub struct UserCacheLayer {
    user_cache: UserCache,
}

impl UserCacheLayer {
    pub fn new(user_cache: UserCache) -> Self {
        Self { user_cache }
    }
}

impl<S> Layer<S> for UserCacheLayer {
    type Service = UserCacheMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        UserCacheMiddleware {
            inner,
            user_cache: self.user_cache.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserCacheMiddleware<S> {
    inner: S,
    user_cache: UserCache,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for UserCacheMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Default + Send + 'static,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let user_cache = self.user_cache.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Read user ID from auth layer's User extension
            if let Some(user) = req.extensions().get::<User>() {
                let user_id = user.id.clone();

                if let Some(cached_data) = user_cache.get(&user_id).await {
                    req.extensions_mut().insert(cached_data);
                }
            }

            inner.call(req).await
        })
    }
}
