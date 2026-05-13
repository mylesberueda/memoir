use crate::{
    AppContext,
    models::organization_plans::{self, PlanTier},
    models::organizations,
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::api::v1::{
    GetOrganizationPlanRequest, GetOrganizationPlanResponse, UpdateOrganizationPlanRequest,
    UpdateOrganizationPlanResponse, admin_service_server,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait as _, EntityTrait, IntoActiveModel, QueryFilter as _};
use std::sync::Arc;
use tonic::{Response, Status, async_trait};
use tracing::instrument;

#[derive(Debug, Clone)]
pub(crate) struct AdminService {
    context: Arc<AppContext>,
}

impl AdminService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }

    /// Resolves an organization PID (public string ID) to the internal integer ID.
    async fn resolve_org_id(&self, org_pid: &str) -> Result<i32, Status> {
        let org = organizations::Entity::find()
            .filter(organizations::Column::Pid.eq(org_pid))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?
            .ok_or_else(|| Status::not_found("Organization not found"))?;
        Ok(org.id)
    }
}

#[async_trait]
impl admin_service_server::AdminService for AdminService {
    #[instrument(skip(self), fields(organization_pid))]
    async fn get_organization_plan(
        &self,
        request: tonic::Request<GetOrganizationPlanRequest>,
    ) -> std::result::Result<tonic::Response<GetOrganizationPlanResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("organization_pid", &req.organization_pid);

        tracing::debug!(user_id = %user_id, "Fetching organization plan");
        let org_id = self.resolve_org_id(&req.organization_pid).await?;

        let org_plan = organization_plans::Entity::find_by_id(org_id)
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;
        tracing::info!("Organization plan retrieved");

        Ok(Response::new(GetOrganizationPlanResponse {
            plan: org_plan.map(|p| p.into()),
        }))
    }

    #[instrument(skip(self), fields(organization_pid))]
    async fn update_organization_plan(
        &self,
        request: tonic::Request<UpdateOrganizationPlanRequest>,
    ) -> std::result::Result<tonic::Response<UpdateOrganizationPlanResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();
        tracing::Span::current().record("organization_pid", &req.organization_pid);

        tracing::debug!(user_id = %user_id, "Updating organization plan");
        let org_id = self.resolve_org_id(&req.organization_pid).await?;

        organization_plans::Entity::load();

        let existing_plan = organization_plans::Entity::find_by_id(org_id)
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        let org_plan = match existing_plan {
            Some(p) => {
                // Update existing plan
                let mut active_model = p.into_active_model();
                if let Some(tier) = req.tier {
                    let validated_tier = tier.parse::<PlanTier>().unwrap_or_else(|_| {
                        tracing::error!(tier = tier, "invalid tier, defaulting to free");
                        PlanTier::Free
                    });

                    active_model.tier = Set(validated_tier.into());
                }
                tracing::debug!("Updating organization plan");
                active_model
                    .update(self.context.db.as_ref())
                    .await
                    .map_err(|e| Status::internal(format!("Database error: {e}")))?
            }
            None => {
                // Create new plan
                tracing::info!("Organization plan not found, creating new plan");
                let tier = req.tier.unwrap_or("free".to_string());
                let validated_tier = tier.parse::<PlanTier>().unwrap_or_else(|_| {
                    tracing::error!(tier = tier, "invalid tier, defaulting to free");
                    PlanTier::Free
                });
                let new_plan = organization_plans::ActiveModel {
                    organization_id: Set(org_id),
                    tier: Set(validated_tier.into()),
                    expires_at: Set(None),
                    ..Default::default()
                };
                tracing::debug!("Inserting new organization plan");
                new_plan
                    .insert(self.context.db.as_ref())
                    .await
                    .map_err(|e| Status::internal(format!("Database error: {e}")))?
            }
        };
        tracing::info!("Organization plan saved");

        Ok(Response::new(UpdateOrganizationPlanResponse {
            plan: Some(org_plan.into()),
        }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use proto_rs::api::v1::admin_service_server::AdminService as AdminServiceTrait;
    use proto_rs::api::v1::{GetOrganizationPlanRequest, UpdateOrganizationPlanRequest};
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_return_plan_when_org_has_plan(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("get-plan").await;
        ctx.create_organization_plan(org.id, "pro").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            GetOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
        );
        let response = service.get_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be present");
        assert_eq!(plan.organization_pid, org.id.to_string());
        assert_eq!(plan.tier, "pro");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_return_none_when_org_has_no_plan(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("no-plan").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            GetOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
        );
        let response = service.get_organization_plan(request).await.unwrap().into_inner();

        // Assert
        assert!(response.plan.is_none());
    }

    // ==================== update_organization_plan ====================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_update_tier_when_plan_exists(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("update-tier").await;
        ctx.create_organization_plan(org.id, "free").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            UpdateOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
                tier: Some("pro".to_string()),
                expires_at: None,
            },
            &user.id,
        );
        let response = service.update_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be present");
        assert_eq!(plan.tier, "pro");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_create_plan_with_specified_tier_when_no_plan_exists(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("create-with-tier").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            UpdateOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
                tier: Some("enterprise".to_string()),
                expires_at: None,
            },
            &user.id,
        );
        let response = service.update_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be created");
        assert_eq!(plan.organization_pid, org.id.to_string());
        assert_eq!(plan.tier, "enterprise");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_create_plan_with_free_tier_when_no_plan_exists_and_no_tier_specified(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("create-default").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            UpdateOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
                tier: None,
                expires_at: None,
            },
            &user.id,
        );
        let response = service.update_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be created");
        assert_eq!(plan.tier, "free");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_preserve_tier_when_updating_without_tier_specified(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("preserve-tier").await;
        ctx.create_organization_plan(org.id, "pro").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            UpdateOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
                tier: None,
                expires_at: None,
            },
            &user.id,
        );
        let response = service.update_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be present");
        assert_eq!(plan.tier, "pro");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_default_to_free_tier_when_invalid_tier_provided(ctx: &mut TestContext) {
        // Arrange
        let (user, org) = ctx.create_user_with_personal_org("invalid-tier").await;
        ctx.create_organization_plan(org.id, "pro").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            UpdateOrganizationPlanRequest {
                organization_pid: org.pid.clone(),
                tier: Some("invalid-tier-name".to_string()),
                expires_at: None,
            },
            &user.id,
        );
        let response = service.update_organization_plan(request).await.unwrap().into_inner();

        // Assert
        let plan = response.plan.expect("plan should be present");
        assert_eq!(plan.tier, "free");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_return_not_found_when_org_pid_does_not_exist(ctx: &mut TestContext) {
        // Arrange
        let user = ctx.create_user("bogus-org").await;
        let service = AdminService::new(ctx.context.clone());

        // Act
        let request = ctx.authenticated_request(
            GetOrganizationPlanRequest {
                organization_pid: "nonexistent_org_pid_12345".to_string(),
            },
            &user.id,
        );
        let result = service.get_organization_plan(request).await;

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
    }
}
