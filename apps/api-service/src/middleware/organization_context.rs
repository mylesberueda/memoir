use crate::{
    AppContext,
    models::{organization_members, organizations},
};
use platform_rs::cache::CachedUserData;
use platform_rs::ext::RequestAuthExt;
use platform_rs::middleware::BoxFuture;
use platform_rs::middleware::ratelimit::OrgTier;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;
use tonic::Status;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub(crate) struct OrganizationContext {
    pub organization_id: i32,     // Internal DB ID
    pub organization_pid: String, // Public ID
    pub user_role: String,        // "owner", "admin", "member"
}

#[derive(Debug, Clone)]
pub(crate) struct OrganizationContextLayer {
    context: Arc<AppContext>,
}

impl OrganizationContextLayer {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }
}

impl<S> Layer<S> for OrganizationContextLayer {
    type Service = OrganizationContextMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OrganizationContextMiddleware {
            inner,
            db: self.context.db.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrganizationContextMiddleware<S> {
    inner: S,
    db: Arc<DatabaseConnection>,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for OrganizationContextMiddleware<S>
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
        let db = self.db.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let org_pid = match req.organization_pid() {
                Some(pid) => pid,
                None => return inner.call(req).await,
            };

            let user_id = match req.user_id() {
                Some(id) => id,
                None => return inner.call(req).await,
            };

            // By the time we get here, if the user or org aren't in the
            // middleware stack, they'll have already left the middleware.
            let org = match organizations::Entity::find()
                .filter(organizations::Column::Pid.eq(&org_pid))
                .one(db.as_ref())
                .await
            {
                Ok(Some(org)) => org,
                Ok(None) => {
                    return Ok(Status::not_found("Organization not found").into_http());
                }
                Err(e) => {
                    return Ok(Status::internal(format!("Database error: {e}")).into_http());
                }
            };

            let membership = match organization_members::Entity::find()
                .filter(organization_members::Column::OrganizationId.eq(org.id))
                .filter(organization_members::Column::UserId.eq(&user_id))
                .one(db.as_ref())
                .await
            {
                Ok(Some(member)) => member,
                Ok(None) => {
                    // Not a member — skip org context but don't block the request.
                    // Handlers that require org context will fail with a clear error
                    // via request.organization_context() returning Err.
                    tracing::debug!(user_id = %user_id, org_pid = %org_pid, "User is not a member of this organization, skipping org context");
                    return inner.call(req).await;
                }
                Err(e) => {
                    return Ok(Status::internal(format!("Database error: {e}")).into_http());
                }
            };

            // Populate OrgTier from cached user data (set by UserCacheMiddleware upstream)
            let tier = req
                .extensions()
                .get::<CachedUserData>()
                .and_then(|data| data.org(&org_pid))
                .map(|o| o.tier)
                .unwrap_or_default();
            req.extensions_mut().insert(OrgTier(tier));

            // Insert validated context
            let context = OrganizationContext {
                organization_id: org.id,
                organization_pid: org_pid,
                user_role: membership.role,
            };

            req.extensions_mut().insert(context);

            inner.call(req).await
        })
    }
}
