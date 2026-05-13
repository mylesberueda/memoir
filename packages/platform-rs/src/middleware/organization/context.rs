use super::OrganizationPid;
use crate::cache::CachedUserData;
use crate::cache::{OrgRole, PlanTier, ResolvedPermissions};
use crate::middleware::BoxFuture;
use crate::middleware::ratelimit::OrgTier;
use tonic::Status;
use tower::{Layer, Service};

/// Org context populated from `CachedUserData`.
///
/// Unlike api-service's DB-backed `OrganizationContext`, this is purely
/// cache-derived — no DB lookups. Suitable for services that trust the
/// cache as the source of truth for membership and tier.
#[derive(Debug, Clone)]
pub struct OrgContext {
    pub pid: String,
    pub role: OrgRole,
    pub tier: PlanTier,
    pub permissions: ResolvedPermissions,
}

/// Middleware that validates org membership from cached user data and
/// populates `OrgContext` + `OrgTier` on request extensions.
///
/// Reads `OrganizationPid` (set by `OrganizationLayer`) and `CachedUserData`
/// (set by `UserCacheMiddleware`). If the user isn't a member of the requested
/// org, returns 403.
///
/// # Middleware Stack Order
///
/// ```text
/// 1. AuthLayer            → extracts User
/// 2. UserCacheMiddleware   → fetches CachedUserData from Redis
/// 3. OrganizationLayer     → extracts OrganizationPid from header
/// 4. OrgContextLayer       → validates membership, inserts OrgContext + OrgTier  ← THIS
/// 5. RateLimitLayer        → reads OrgTier
/// ```
#[derive(Debug, Clone, Default)]
pub struct OrgContextLayer;

impl OrgContextLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for OrgContextLayer {
    type Service = OrgContextMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OrgContextMiddleware { inner }
    }
}

#[derive(Debug, Clone)]
pub struct OrgContextMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for OrgContextMiddleware<S>
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
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // No org header → skip (request is not org-scoped)
            let org_pid = match req.extensions().get::<OrganizationPid>() {
                Some(pid) => pid.0.clone(),
                None => return inner.call(req).await,
            };

            // No cached data → can't validate membership, reject
            let cached = match req.extensions().get::<CachedUserData>() {
                Some(data) => data.clone(),
                None => {
                    return Ok(Status::unauthenticated("User data not available").into_http());
                }
            };

            // Find org in cached memberships
            let org = match cached.org(&org_pid) {
                Some(o) => o.clone(),
                None => {
                    return Ok(Status::permission_denied("Not a member of this organization").into_http());
                }
            };

            req.extensions_mut().insert(OrgTier(org.tier));
            req.extensions_mut().insert(OrgContext {
                pid: org_pid,
                role: org.role,
                tier: org.tier,
                permissions: org.permissions,
            });

            inner.call(req).await
        })
    }
}
