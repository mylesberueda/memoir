#![allow(unused)] // TODO(_): Leave until fully implemented

use platform_rs::middleware::BoxFuture;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait};
use std::sync::Arc;
use tonic::Status;
use tower::{Layer, Service};

use crate::{
    AppContext,
    middleware::RequestExt,
    models::{OrganizationRole, organization_members, organizations, users},
};
use platform_rs::cache::{CachedOrg, CachedUserData, PlanTier, UserCache};
use platform_rs::ext::RequestAuthExt;

#[derive(Debug, Clone)]
pub(crate) struct UserContext {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub(crate) struct UserContextLayer {
    context: Arc<AppContext>,
}

impl UserContextLayer {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }
}

impl<S> Layer<S> for UserContextLayer {
    type Service = UserContextMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        UserContextMiddleware {
            inner,
            db: self.context.db.clone(),
            user_cache: UserCache::new(self.context.redis.clone(), crate::REDIS_SERVICE_KEY),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UserContextMiddleware<S> {
    inner: S,
    db: Arc<DatabaseConnection>,
    user_cache: UserCache,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for UserContextMiddleware<S>
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
        let user_cache = self.user_cache.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let auth_user = match req.user() {
                Some(u) => u.clone(),
                None => {
                    tracing::debug!("No user in request");
                    return Ok(Status::unauthenticated("Authentication required").into_http());
                }
            };

            let user_id = auth_user.id.clone();
            tracing::debug!(user_id = %user_id, "Looking up user in database");

            let user = match users::Entity::find_by_id(&user_id).one(db.as_ref()).await {
                Ok(Some(m)) => m,
                Ok(None) => {
                    // Auto-provision user from token claims
                    tracing::debug!(user_id = %user_id, "User not found, auto-provisioning from token");

                    let email = match auth_user.email.clone() {
                        Some(e) => e,
                        None => {
                            tracing::error!(user_id = %user_id, "Cannot auto-provision user: no email in token. Ensure x-id-token header is present.");
                            return Ok(Status::failed_precondition(
                                "Email required for user provisioning. Ensure x-id-token header is present.",
                            )
                            .into_http());
                        }
                    };

                    let display_name = auth_user.name.clone();

                    let txn = match db.begin().await {
                        Ok(txn) => txn,
                        Err(e) => {
                            tracing::error!(user_id = %user_id, error = %e, "Failed to begin transaction");
                            return Ok(Status::internal(format!("Database error: {e}")).into_http());
                        }
                    };

                    let new_user = users::ActiveModel {
                        id: Set(user_id.clone()),
                        email: Set(email.clone()),
                        display_name: Set(display_name.clone()),
                        avatar_url: Set(None),
                        bio: Set(None),
                        settings: Set(Default::default()),
                        ..Default::default()
                    };

                    let user = match new_user.insert(&txn).await {
                        Ok(u) => u,
                        Err(e) => {
                            tracing::error!(user_id = %user_id, error = %e, "Failed to auto-provision user");
                            return Ok(Status::internal(format!("Failed to create user: {e}")).into_http());
                        }
                    };

                    // Create personal org — follows pattern from services/organizations.rs:72-95
                    let org_name =
                        display_name.unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
                    let org = organizations::ActiveModel {
                        name: Set(org_name),
                        settings: Set(serde_json::json!({})),
                        ..Default::default()
                    };

                    let org = match org.insert(&txn).await {
                        Ok(o) => o,
                        Err(e) => {
                            tracing::error!(user_id = %user_id, error = %e, "Failed to create personal org");
                            return Ok(Status::internal(format!("Failed to create organization: {e}")).into_http());
                        }
                    };

                    let member = organization_members::ActiveModel {
                        organization_id: Set(org.id),
                        user_id: Set(user_id.clone()),
                        role: Set(OrganizationRole::Owner.into()),
                        ..Default::default()
                    };

                    if let Err(e) = member.insert(&txn).await {
                        tracing::error!(user_id = %user_id, error = %e, "Failed to add user as org owner");
                        return Ok(Status::internal(format!("Failed to create organization member: {e}")).into_http());
                    }

                    if let Err(e) = txn.commit().await {
                        tracing::error!(user_id = %user_id, error = %e, "Failed to commit user provisioning transaction");
                        return Ok(Status::internal(format!("Database error: {e}")).into_http());
                    }

                    // Write user cache to Redis for cross-service consumption
                    user_cache
                        .set(
                            &user_id,
                            &CachedUserData {
                                email: user.email.clone(),
                                organizations: vec![CachedOrg {
                                    pid: org.pid.clone(),
                                    tier: PlanTier::Free,
                                    role: OrganizationRole::Owner,
                                    permissions: crate::services::role_defaults::resolve_permissions(
                                        OrganizationRole::Owner,
                                        &Default::default(),
                                    ),
                                }],
                            },
                        )
                        .await;

                    tracing::info!(user_id = %user_id, org_pid = %org.pid, "Auto-provisioned new user with personal org");
                    user
                }
                Err(e) => {
                    tracing::error!(user_id = %user_id, error = %e, "Database error looking up user");
                    return Ok(Status::internal(format!("Database error: {e}")).into_http());
                }
            };

            let context = UserContext {
                id: user.id,
                email: user.email,
            };

            req.extensions_mut().insert(context);

            inner.call(req).await
        })
    }
}
