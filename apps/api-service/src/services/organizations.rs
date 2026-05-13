use crate::{
    AppContext,
    middleware::RequestExt,
    models::{
        OrganizationRole, organization_members, organization_plans, organization_plans::PlanTier, organizations, users,
    },
};
use organization_members::MemberCursor;
use platform_rs::{
    cache::{CachedOrg, CachedUserData, ResolvedPermissions, ResourceType, UserCache},
    ext::RequestAuthExt,
};
use proto_rs::api::v1::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait as _, QueryFilter,
};
use std::sync::Arc;
use strum::IntoEnumIterator as _;
use tonic::{Response, Status, async_trait};
use tracing::instrument;

#[derive(Debug, Clone)]
pub(crate) struct OrganizationService {
    context: Arc<AppContext>,
    user_cache: UserCache,
}

impl OrganizationService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        let user_cache = UserCache::new(context.redis.clone(), crate::REDIS_SERVICE_KEY);
        Self { context, user_cache }
    }

    /// Look up the current tier for an organization from organization_plans.
    /// Returns Free if no plan exists.
    async fn org_tier(&self, org_id: i32) -> PlanTier {
        organization_plans::Entity::find_by_id(org_id)
            .one(self.context.db.as_ref())
            .await
            .ok()
            .flatten()
            .map(|p| organization_plans::parse_tier_or_free(&p.tier))
            .unwrap_or_default()
    }

    /// Add an org entry to a user's cache. Reads existing cache, appends, writes back.
    async fn cache_add_org(&self, user_id: &str, email: &str, org_pid: &str, tier: PlanTier, role: OrganizationRole) {
        let permissions = crate::services::role_defaults::resolve_permissions(role, &Default::default());
        let mut data: CachedUserData = self.user_cache.get(user_id).await.unwrap_or(CachedUserData {
            email: email.to_string(),
            organizations: vec![],
        });
        // Avoid duplicates
        if data.org(org_pid).is_none() {
            data.organizations.push(CachedOrg {
                pid: org_pid.to_string(),
                tier,
                role,
                permissions,
            });
        }
        self.user_cache.set(user_id, &data).await;
    }

    /// Remove an org entry from a user's cache.
    async fn cache_remove_org(&self, user_id: &str, org_pid: &str) {
        if let Some(mut data) = self.user_cache.get(user_id).await {
            data.organizations.retain(|o| o.pid != org_pid);
            self.user_cache.set(user_id, &data).await;
        }
    }

    /// Update the role for an org entry in a user's cache.
    async fn cache_update_role(
        &self,
        user_id: &str,
        org_pid: &str,
        new_role: OrganizationRole,
        overrides_json: &serde_json::Value,
    ) {
        let overrides = serde_json::from_value(overrides_json.clone()).unwrap_or_default();
        let permissions = crate::services::role_defaults::resolve_permissions(new_role, &overrides);

        if let Some(mut data) = self.user_cache.get(user_id).await {
            if let Some(org) = data.organizations.iter_mut().find(|o| o.pid == org_pid) {
                org.role = new_role;
                org.permissions = permissions;
            }
            self.user_cache.set(user_id, &data).await;
        }
    }
}

#[async_trait]
impl organization_service_server::OrganizationService for OrganizationService {
    #[instrument(skip(self, request), fields(user_id))]
    async fn list_organizations(
        &self,
        request: tonic::Request<ListOrganizationsRequest>,
    ) -> std::result::Result<tonic::Response<ListOrganizationsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        tracing::Span::current().record("user_id", &user_id);

        tracing::debug!("fetching user's organizations");
        let memberships = organization_members::Entity::find()
            .filter(organization_members::Column::UserId.eq(&user_id))
            .all(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        let org_ids: Vec<i32> = memberships.iter().map(|m| m.organization_id).collect();

        let orgs = organizations::Entity::find()
            .find_with_related(organization_members::Entity)
            .filter(organizations::Column::Id.is_in(org_ids))
            .all(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;
        tracing::info!("organizations retrieved");

        Ok(Response::new(ListOrganizationsResponse {
            organizations: orgs
                .into_iter()
                .map(|(org, org_members)| org.to_proto(org_members.len() as u64))
                .collect(),
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn create_organization(
        &self,
        request: tonic::Request<CreateOrganizationRequest>,
    ) -> std::result::Result<tonic::Response<CreateOrganizationResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let user_email = request
            .extensions()
            .get::<platform_rs::middleware::auth::User>()
            .and_then(|u| u.email.clone())
            .unwrap_or_default();
        let info = request.get_ref().to_owned();

        tracing::debug!("Creating organization");
        // Convert OrganizationSettings to JsonValue for database storage
        let settings_json = serde_json::to_value(info.settings.unwrap_or_default()).unwrap_or_default();
        let org = organizations::ActiveModel {
            name: Set(info.name),
            slug: Set(info.slug),
            settings: Set(settings_json),
            ..Default::default()
        };

        let org = org
            .insert(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        tracing::debug!("Adding creator as organization owner");
        let member = organization_members::ActiveModel {
            organization_id: Set(org.id),
            user_id: Set(user_id.clone()),
            role: Set(OrganizationRole::Owner.into()),
            ..Default::default()
        };

        member
            .insert(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Update creator's cache with the new org (free tier, Owner role)
        self.cache_add_org(&user_id, &user_email, &org.pid, PlanTier::Free, OrganizationRole::Owner)
            .await;

        tracing::info!("Organization created");

        Ok(Response::new(CreateOrganizationResponse {
            organization: Some(org.to_proto(1)), // When creating an organization, the initial count is 1
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn get_organization(
        &self,
        request: tonic::Request<GetOrganizationRequest>,
    ) -> std::result::Result<tonic::Response<GetOrganizationResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let user_id = &request.user_id()?;
        let org_pid = &request.get_ref().organization_pid;

        if &ctx.organization_pid != org_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        tracing::debug!("Fetching organization");
        let (org, org_member) = organizations::Entity::find()
            .find_also_related(organization_members::Entity)
            .filter(organizations::Column::Pid.eq(org_pid))
            .filter(organization_members::Column::UserId.eq(user_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
            .ok_or(tonic::Status::not_found("Organization not found"))?;
        tracing::info!("Organization retrieved");

        let member_count = organization_members::Entity::find_by_id(org.id)
            .count(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

        let org_member = org_member.ok_or(tonic::Status::permission_denied("User is not in the organization"))?;

        // Resolve permissions from user cache (follows pattern at organization_context.rs:111-116)
        let resolved = request
            .extensions()
            .get::<CachedUserData>()
            .and_then(|data| data.org(org_pid))
            .map(|o| o.permissions.clone())
            .unwrap_or_else(ResolvedPermissions::allow_all);

        let permissions = ResourceType::iter()
            .map(|rt| {
                let p = resolved.get(rt);
                (
                    rt.to_string(),
                    ResourcePermission {
                        read: p.read,
                        write: p.write,
                        execute: p.execute,
                    },
                )
            })
            .collect();

        Ok(Response::new(GetOrganizationResponse {
            organization: Some(org.to_proto(member_count)),
            user_role: org_member.role.to_string(),
            permissions,
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn update_organization(
        &self,
        request: tonic::Request<UpdateOrganizationRequest>,
    ) -> std::result::Result<tonic::Response<UpdateOrganizationResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let req = request.get_ref().to_owned();

        if ctx.organization_pid != req.organization_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        if ctx.user_role != "owner" && ctx.user_role != "admin" {
            return Err(Status::permission_denied(
                "Only owners and admins can update organization",
            ));
        }

        tracing::debug!("Fetching organization");
        let mut org = organizations::Entity::find_by_id(ctx.organization_id)
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?
            .ok_or_else(|| Status::not_found("Organization not found"))?
            .into_active_model();

        if let Some(name) = req.name {
            org.name = Set(name);
        }

        if let Some(slug) = req.slug {
            org.slug = Set(slug);
        }

        if let Some(settings) = req.settings {
            // Convert OrganizationSettings to JsonValue for database storage
            let settings_json = serde_json::to_value(settings).unwrap_or_default();
            org.settings = Set(settings_json);
        }

        tracing::debug!("Updating organization");
        let org = org
            .update(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;
        tracing::info!("Organization updated");

        let member_count = organization_members::Entity::find_by_id(org.id)
            .count(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

        Ok(Response::new(UpdateOrganizationResponse {
            organization: Some(org.to_proto(member_count)),
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn delete_organization(
        &self,
        request: tonic::Request<DeleteOrganizationRequest>,
    ) -> std::result::Result<tonic::Response<DeleteOrganizationResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let org_pid = &request.get_ref().organization_pid;

        if &ctx.organization_pid != org_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        if ctx.user_role != "owner" {
            return Err(Status::permission_denied("Only owners can delete organization"));
        }

        // Find all members before deleting so we can clean their caches
        let members = organization_members::Entity::find()
            .filter(organization_members::Column::OrganizationId.eq(ctx.organization_id))
            .all(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        tracing::debug!("Deleting organization");
        organizations::Entity::delete_by_id(ctx.organization_id)
            .exec(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Remove the deleted org from all members' caches
        for member in &members {
            self.cache_remove_org(&member.user_id, &ctx.organization_pid).await;
        }

        tracing::info!("Organization deleted");

        Ok(Response::new(DeleteOrganizationResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn list_members(
        &self,
        request: tonic::Request<ListMembersRequest>,
    ) -> std::result::Result<tonic::Response<ListMembersResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let org_id = ctx.organization_id;
        let org_pid = ctx.organization_pid.clone();
        let req = request.into_inner();

        if org_pid != req.organization_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        let page_size = if req.page_size == 0 {
            20
        } else {
            (req.page_size as u64).clamp(1, 100)
        };

        // Get total count
        let total = organization_members::Entity::find()
            .filter(organization_members::Column::OrganizationId.eq(org_id))
            .count(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))? as i32;

        // Cursor pagination: sort by (role, created_at, id)
        tracing::debug!("Fetching organization members");
        let mut cursor = organization_members::Entity::find()
            .filter(organization_members::Column::OrganizationId.eq(org_id))
            .cursor_by((
                organization_members::Column::Role,
                organization_members::Column::CreatedAt,
                organization_members::Column::Id,
            ));

        if let Some(ref cursor_str) = req.cursor {
            let c = MemberCursor::try_from(cursor_str.as_str())?;
            cursor.after(c.into_inner());
        }

        let rows = cursor
            .first(page_size)
            .all(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Enrich with user data (separate query to avoid cursor+join issues)
        let user_ids: Vec<String> = rows.iter().map(|m| m.user_id.clone()).collect();
        let user_map: std::collections::HashMap<String, users::Model> = if !user_ids.is_empty() {
            users::Entity::find()
                .filter(users::Column::Id.is_in(&user_ids))
                .all(self.context.db.as_ref())
                .await
                .map_err(|e| Status::internal(format!("Database error: {e}")))?
                .into_iter()
                .map(|u| (u.id.clone(), u))
                .collect()
        } else {
            Default::default()
        };

        // Build cursor for next page (only if this page is full — more results may exist)
        let next_cursor = if rows.len() == page_size as usize {
            rows.last()
                .map(|last| MemberCursor::new((last.role.clone(), last.created_at, last.id)).to_string())
        } else {
            None
        };

        tracing::info!(
            total,
            page_size,
            has_next = next_cursor.is_some(),
            "Organization members retrieved"
        );

        let members = rows
            .into_iter()
            .map(|member| {
                let user = user_map.get(&member.user_id);
                OrganizationMember {
                    pid: member.pid,
                    organization_id: member.organization_id.to_string(),
                    user_id: member.user_id,
                    role: member.role,
                    created_at: member.created_at.and_utc().to_rfc3339(),
                    display_name: user.and_then(|u| u.display_name.clone()),
                    email: user.map(|u| u.email.clone()).unwrap_or_default(),
                }
            })
            .collect();

        Ok(Response::new(ListMembersResponse {
            members,
            next_cursor,
            total,
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_id = %request.get_ref().organization_pid))]
    async fn add_member(
        &self,
        request: tonic::Request<AddMemberRequest>,
    ) -> std::result::Result<tonic::Response<AddMemberResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let req = request.get_ref().to_owned();

        if ctx.organization_pid != req.organization_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        if ctx.user_role != "owner" && ctx.user_role != "admin" {
            return Err(Status::permission_denied("Only owners and admins can add members"));
        }

        // Resolve user_id: prefer email lookup, fall back to direct user_id
        let resolved_user_id = if let Some(email) = &req.email {
            let user = users::Entity::find()
                .filter(users::Column::Email.eq(email.as_str()))
                .one(self.context.db.as_ref())
                .await
                .map_err(|e| Status::internal(format!("Database error: {e}")))?
                .ok_or_else(|| Status::not_found(format!("No user found with email: {email}")))?;
            user.id
        } else if !req.user_id.is_empty() {
            req.user_id
        } else {
            return Err(Status::invalid_argument("Either email or user_id is required"));
        };

        tracing::debug!(resolved_user_id = %resolved_user_id, "Adding member to organization");
        let member = organization_members::ActiveModel {
            organization_id: Set(ctx.organization_id),
            user_id: Set(resolved_user_id),
            role: Set(organization_members::parse_role(req.role)?.to_string()),
            ..Default::default()
        };

        let member = member
            .insert(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Update the added user's cache with the new org membership
        let tier = self.org_tier(ctx.organization_id).await;
        let role = member
            .role
            .parse::<OrganizationRole>()
            .unwrap_or(OrganizationRole::Member);
        self.cache_add_org(&member.user_id, "", &ctx.organization_pid, tier, role)
            .await;

        tracing::info!("Member added to organization");

        Ok(Response::new(AddMemberResponse {
            member: Some(member.into()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn update_member(
        &self,
        request: tonic::Request<UpdateMemberRequest>,
    ) -> std::result::Result<tonic::Response<UpdateMemberResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let req = request.get_ref().to_owned();

        if ctx.organization_pid != req.organization_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        if ctx.user_role != "owner" && ctx.user_role != "admin" {
            return Err(Status::permission_denied("Only owners and admins can update members"));
        }

        tracing::debug!("Fetching member to update");
        let mut member = organization_members::Entity::find()
            .filter(organization_members::Column::OrganizationId.eq(ctx.organization_id))
            .filter(organization_members::Column::UserId.eq(&req.user_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?
            .ok_or_else(|| Status::not_found("Member not found"))?
            .into_active_model();

        if let Some(role) = req.role {
            member.role = Set(organization_members::parse_role(role)?.to_string());
        }

        tracing::debug!("Updating member role");
        let member = member
            .update(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Update the member's cache with the new role
        let new_role = member
            .role
            .parse::<OrganizationRole>()
            .unwrap_or(OrganizationRole::Member);
        self.cache_update_role(&req.user_id, &ctx.organization_pid, new_role, &member.permissions)
            .await;

        tracing::info!("Member role updated");

        Ok(Response::new(UpdateMemberResponse {
            member: Some(member.into()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, organization_pid = %request.get_ref().organization_pid))]
    async fn remove_member(
        &self,
        request: tonic::Request<RemoveMemberRequest>,
    ) -> std::result::Result<tonic::Response<RemoveMemberResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let req = request.get_ref().to_owned();

        if ctx.organization_pid != req.organization_pid {
            return Err(Status::permission_denied(
                "Cannot access organization from current context",
            ));
        }

        if ctx.user_role != "owner" && ctx.user_role != "admin" {
            return Err(Status::permission_denied("Only owners and admins can remove members"));
        }

        tracing::debug!("Fetching member to remove");
        let member = organization_members::Entity::find()
            .filter(organization_members::Column::OrganizationId.eq(ctx.organization_id))
            .filter(organization_members::Column::UserId.eq(&req.user_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?
            .ok_or_else(|| Status::not_found("Member not found"))?;

        tracing::debug!("Removing member from organization");
        let removed_user_id = member.user_id.clone();
        organization_members::Entity::delete_by_id(member.id)
            .exec(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        // Remove the org entry from the removed user's cache
        self.cache_remove_org(&removed_user_id, &ctx.organization_pid).await;

        tracing::info!("Member removed from organization");

        Ok(Response::new(RemoveMemberResponse {}))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use proto_rs::api::v1::organization_service_server::OrganizationService as OrganizationServiceTrait;
    use proto_rs::api::v1::{
        AddMemberRequest, CreateOrganizationRequest, DeleteOrganizationRequest, GetOrganizationRequest,
        ListMembersRequest, ListOrganizationsRequest, RemoveMemberRequest, UpdateMemberRequest,
        UpdateOrganizationRequest,
    };
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_create_organization_with_creator_as_owner(ctx: &mut TestContext) {
        let user = ctx.create_user("org-create").await;
        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.authenticated_request(
            CreateOrganizationRequest {
                name: "Test Company".to_string(),
                slug: format!("test-company-{}", user.id),
                settings: None,
            },
            &user.id,
        );
        let response = service.create_organization(request).await.unwrap().into_inner();

        let org = response.organization.expect("organization should be created");
        assert_eq!(org.name, "Test Company");
        assert_eq!(org.member_count, 1);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_list_all_organizations_user_belongs_to(ctx: &mut TestContext) {
        let user = ctx.create_user("org-list").await;
        let org1 = ctx.create_organization("list-1").await;
        let org2 = ctx.create_organization("list-2").await;
        ctx.create_organization_member(org1.id, &user.id, "owner").await;
        ctx.create_organization_member(org2.id, &user.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.authenticated_request(ListOrganizationsRequest {}, &user.id);
        let response = service.list_organizations(request).await.unwrap().into_inner();

        assert_eq!(response.organizations.len(), 2);
        let org_pids: Vec<String> = response.organizations.iter().map(|o| o.pid.clone()).collect();
        assert!(org_pids.contains(&org1.pid));
        assert!(org_pids.contains(&org2.pid));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_return_empty_list_when_user_has_no_organizations(ctx: &mut TestContext) {
        let user = ctx.create_user("org-list-empty").await;
        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.authenticated_request(ListOrganizationsRequest {}, &user.id);
        let response = service.list_organizations(request).await.unwrap().into_inner();

        assert!(response.organizations.is_empty());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_return_organization_details_for_member(ctx: &mut TestContext) {
        let user = ctx.create_user("org-get").await;
        let org = ctx.create_organization("get-success").await;
        ctx.create_organization_member(org.id, &user.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            GetOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.get_organization(request).await.unwrap().into_inner();

        let returned_org = response.organization.expect("organization should be returned");
        assert_eq!(returned_org.pid, org.pid);
        assert_eq!(returned_org.name, org.name);
        assert_eq!(response.user_role, "owner");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_deny_access_when_requesting_org_outside_current_context(ctx: &mut TestContext) {
        let user = ctx.create_user("org-get-mismatch").await;
        let org1 = ctx.create_organization("get-org1").await;
        let org2 = ctx.create_organization("get-org2").await;
        ctx.create_organization_member(org1.id, &user.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            GetOrganizationRequest {
                organization_pid: org2.pid.clone(),
            },
            &user.id,
            &org1.pid,
            org1.id,
            "owner",
        );
        let error = service.get_organization(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::PermissionDenied);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_owner_to_update_organization(ctx: &mut TestContext) {
        let user = ctx.create_user("org-update").await;
        let org = ctx.create_organization("update-success").await;
        ctx.create_organization_member(org.id, &user.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            UpdateOrganizationRequest {
                organization_pid: org.pid.clone(),
                name: Some("Updated Name".to_string()),
                slug: Some("updated-slug".to_string()),
                settings: None,
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.update_organization(request).await.unwrap().into_inner();

        let updated_org = response.organization.expect("organization should be updated");
        assert_eq!(updated_org.name, "Updated Name");
        assert_eq!(updated_org.slug, "updated-slug");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_deny_member_from_updating_organization(ctx: &mut TestContext) {
        let user = ctx.create_user("org-update-denied").await;
        let org = ctx.create_organization("update-denied").await;
        ctx.create_organization_member(org.id, &user.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            UpdateOrganizationRequest {
                organization_pid: org.pid.clone(),
                name: Some("Updated Name".to_string()),
                slug: None,
                settings: None,
            },
            &user.id,
            &org.pid,
            org.id,
            "member",
        );
        let error = service.update_organization(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::PermissionDenied);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_admin_to_update_organization(ctx: &mut TestContext) {
        let user = ctx.create_user("org-update-admin").await;
        let org = ctx.create_organization("update-admin").await;
        ctx.create_organization_member(org.id, &user.id, "admin").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            UpdateOrganizationRequest {
                organization_pid: org.pid.clone(),
                name: Some("Admin Updated".to_string()),
                slug: None,
                settings: None,
            },
            &user.id,
            &org.pid,
            org.id,
            "admin",
        );
        let response = service.update_organization(request).await.unwrap().into_inner();

        let updated_org = response.organization.expect("organization should be updated");
        assert_eq!(updated_org.name, "Admin Updated");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_owner_to_delete_organization(ctx: &mut TestContext) {
        let user = ctx.create_user("org-delete").await;
        let org = ctx.create_organization("delete-success").await;
        ctx.create_organization_member(org.id, &user.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            DeleteOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.delete_organization(request).await.unwrap();

        let get_request = ctx.organization_request(
            GetOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );
        let error = service.get_organization(get_request).await.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_deny_admin_from_deleting_organization(ctx: &mut TestContext) {
        let user = ctx.create_user("org-delete-denied").await;
        let org = ctx.create_organization("delete-denied").await;
        ctx.create_organization_member(org.id, &user.id, "admin").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            DeleteOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
            &org.pid,
            org.id,
            "admin",
        );
        let error = service.delete_organization(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::PermissionDenied);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_list_all_members_of_organization(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-list-owner").await;
        let member1 = ctx.create_user("member-list-1").await;
        let member2 = ctx.create_user("member-list-2").await;

        let org = ctx.create_organization("list-members").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;
        ctx.create_organization_member(org.id, &member1.id, "admin").await;
        ctx.create_organization_member(org.id, &member2.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 0,
                cursor: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.list_members(request).await.unwrap().into_inner();

        assert_eq!(response.members.len(), 3);
        let user_ids: Vec<String> = response.members.iter().map(|m| m.user_id.clone()).collect();
        assert!(user_ids.contains(&owner.id));
        assert!(user_ids.contains(&member1.id));
        assert!(user_ids.contains(&member2.id));

        // Verify enriched user data is present
        let owner_member = response.members.iter().find(|m| m.user_id == owner.id).unwrap();
        assert_eq!(owner_member.email, owner.email);
        assert_eq!(owner_member.display_name, owner.display_name);

        let m1 = response.members.iter().find(|m| m.user_id == member1.id).unwrap();
        assert_eq!(m1.email, member1.email);
        assert_eq!(m1.display_name, member1.display_name);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_return_none_display_name_when_user_has_no_name(ctx: &mut TestContext) {
        // Create a user without a display name
        let user = ctx.create_user("member-no-name").await;
        let mut active: crate::models::users::ActiveModel = user.clone().into();
        active.display_name = sea_orm::ActiveValue::Set(None);
        let user = active
            .update(ctx.context.db.as_ref())
            .await
            .expect("Failed to clear display_name");

        let org = ctx.create_organization("list-members-no-name").await;
        ctx.create_organization_member(org.id, &user.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());
        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 0,
                cursor: None,
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );

        let response = service.list_members(request).await.unwrap().into_inner();
        assert_eq!(response.members.len(), 1);

        let member = &response.members[0];
        assert_eq!(member.email, user.email);
        assert_eq!(member.display_name, None);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_paginate_members_with_cursor(ctx: &mut TestContext) {
        let owner = ctx.create_user("page-owner").await;
        let m1 = ctx.create_user("page-m1").await;
        let m2 = ctx.create_user("page-m2").await;

        let org = ctx.create_organization("paginate-members").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;
        ctx.create_organization_member(org.id, &m1.id, "member").await;
        ctx.create_organization_member(org.id, &m2.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        // Page 1: fetch 2 of 3
        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 2,
                cursor: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let page1 = service.list_members(request).await.unwrap().into_inner();
        assert_eq!(page1.members.len(), 2);
        assert_eq!(page1.total, 3);
        assert!(
            page1.next_cursor.is_some(),
            "should have next_cursor when more results exist"
        );

        // Page 2: use cursor to fetch remaining
        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 2,
                cursor: page1.next_cursor,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let page2 = service.list_members(request).await.unwrap().into_inner();
        assert_eq!(page2.members.len(), 1, "last page should have remaining records");
        assert_eq!(page2.total, 3);
        assert!(page2.next_cursor.is_none(), "should have no next_cursor on last page");

        // No overlap between pages
        let page1_ids: Vec<&str> = page1.members.iter().map(|m| m.user_id.as_str()).collect();
        let page2_ids: Vec<&str> = page2.members.iter().map(|m| m.user_id.as_str()).collect();
        for id in &page2_ids {
            assert!(!page1_ids.contains(id), "pages should not overlap");
        }
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_return_no_cursor_when_all_results_fit_in_page(ctx: &mut TestContext) {
        let owner = ctx.create_user("no-cursor-owner").await;

        let org = ctx.create_organization("no-cursor-members").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 10,
                cursor: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.list_members(request).await.unwrap().into_inner();
        assert_eq!(response.members.len(), 1);
        assert_eq!(response.total, 1);
        assert!(response.next_cursor.is_none(), "no cursor when all results fit");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_default_page_size_when_zero(ctx: &mut TestContext) {
        let owner = ctx.create_user("default-page-owner").await;

        let org = ctx.create_organization("default-page-members").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        // page_size: 0 should use default (20), not return empty
        let request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 0,
                cursor: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.list_members(request).await.unwrap().into_inner();
        assert_eq!(response.members.len(), 1);
        assert_eq!(response.total, 1);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_prefer_email_over_user_id_in_add_member(ctx: &mut TestContext) {
        let owner = ctx.create_user("email-pref-owner").await;
        let target = ctx.create_user("email-pref-target").await;

        let org = ctx.create_organization("email-pref").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        // Provide both user_id (wrong) and email (correct) — email should win
        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: "wrong-user-id".to_string(),
                role: "member".to_string(),
                email: Some(target.email.clone()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.add_member(request).await.unwrap().into_inner();
        let member = response.member.expect("member should be added");
        assert_eq!(member.user_id, target.id, "should resolve via email, not user_id");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_owner_to_add_member(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-add-owner").await;
        let new_member = ctx.create_user("member-add-new").await;

        let org = ctx.create_organization("add-member").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: new_member.id.clone(),
                role: "member".to_string(),
                email: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.add_member(request).await.unwrap().into_inner();

        let member = response.member.expect("member should be added");
        assert_eq!(member.user_id, new_member.id);
        assert_eq!(member.role, "member");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_deny_member_from_adding_members(ctx: &mut TestContext) {
        let member = ctx.create_user("member-add-denied").await;
        let new_member = ctx.create_user("member-add-denied-new").await;

        let org = ctx.create_organization("add-member-denied").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: new_member.id.clone(),
                role: "member".to_string(),
                email: None,
            },
            &member.id,
            &org.pid,
            org.id,
            "member",
        );
        let error = service.add_member(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::PermissionDenied);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_admin_to_add_member(ctx: &mut TestContext) {
        let admin = ctx.create_user("member-add-admin").await;
        let new_member = ctx.create_user("member-add-admin-new").await;

        let org = ctx.create_organization("add-member-admin").await;
        ctx.create_organization_member(org.id, &admin.id, "admin").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: new_member.id.clone(),
                role: "member".to_string(),
                email: None,
            },
            &admin.id,
            &org.pid,
            org.id,
            "admin",
        );
        let response = service.add_member(request).await.unwrap().into_inner();

        assert!(response.member.is_some());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_add_member_by_email(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-add-email-owner").await;
        let new_member = ctx.create_user("member-add-email-target").await;

        let org = ctx.create_organization("add-member-email").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: String::new(),
                role: "member".to_string(),
                email: Some(new_member.email.clone()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.add_member(request).await.unwrap().into_inner();

        let member = response.member.expect("member should be added via email");
        assert_eq!(member.user_id, new_member.id);
        assert_eq!(member.role, "member");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_reject_add_member_with_unknown_email(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-add-unknown-email").await;

        let org = ctx.create_organization("add-member-unknown-email").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: String::new(),
                role: "member".to_string(),
                email: Some("nonexistent@example.com".to_string()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let error = service.add_member(request).await.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_reject_add_member_with_no_email_or_user_id(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-add-neither").await;

        let org = ctx.create_organization("add-member-neither").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: String::new(),
                role: "member".to_string(),
                email: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let error = service.add_member(request).await.unwrap_err();
        assert_eq!(error.code(), tonic::Code::InvalidArgument);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_owner_to_change_member_role(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-update-owner").await;
        let member = ctx.create_user("member-update-target").await;

        let org = ctx.create_organization("update-member").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            UpdateMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: member.id.clone(),
                role: Some("admin".to_string()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.update_member(request).await.unwrap().into_inner();

        let updated_member = response.member.expect("member should be updated");
        assert_eq!(updated_member.user_id, member.id);
        assert_eq!(updated_member.role, "admin");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_fail_to_update_nonexistent_member(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-update-notfound").await;

        let org = ctx.create_organization("update-member-notfound").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            UpdateMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: "nonexistent-user".to_string(),
                role: Some("admin".to_string()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let error = service.update_member(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_owner_to_remove_member(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-remove-owner").await;
        let member = ctx.create_user("member-remove-target").await;

        let org = ctx.create_organization("remove-member").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            RemoveMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: member.id.clone(),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.remove_member(request).await.unwrap();

        let list_request = ctx.organization_request(
            ListMembersRequest {
                organization_pid: org.pid.clone(),
                page_size: 0,
                cursor: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let list_response = service.list_members(list_request).await.unwrap().into_inner();
        assert_eq!(list_response.members.len(), 1);
        assert_eq!(list_response.members[0].user_id, owner.id);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_fail_to_remove_nonexistent_member(ctx: &mut TestContext) {
        let owner = ctx.create_user("member-remove-notfound").await;

        let org = ctx.create_organization("remove-member-notfound").await;
        ctx.create_organization_member(org.id, &owner.id, "owner").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            RemoveMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: "nonexistent-user".to_string(),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        let error = service.remove_member(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_deny_member_from_removing_members(ctx: &mut TestContext) {
        let member1 = ctx.create_user("member-remove-denied-1").await;
        let member2 = ctx.create_user("member-remove-denied-2").await;

        let org = ctx.create_organization("remove-member-denied").await;
        ctx.create_organization_member(org.id, &member1.id, "member").await;
        ctx.create_organization_member(org.id, &member2.id, "member").await;

        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            RemoveMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: member2.id.clone(),
            },
            &member1.id,
            &org.pid,
            org.id,
            "member",
        );
        let error = service.remove_member(request).await.unwrap_err();

        assert_eq!(error.code(), tonic::Code::PermissionDenied);
    }

    // --- Personal org provisioning behavior tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_provision_user_with_personal_org_as_owner(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("personal-org").await;
        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.authenticated_request(ListOrganizationsRequest {}, &user.id);
        let response = service.list_organizations(request).await.unwrap().into_inner();

        assert_eq!(response.organizations.len(), 1);
        assert_eq!(response.organizations[0].pid, org.pid);
        assert_eq!(response.organizations[0].member_count, 1);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_have_owner_role_in_personal_org(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("personal-role").await;
        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.organization_request(
            GetOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );
        let response = service.get_organization(request).await.unwrap().into_inner();

        assert_eq!(response.user_role, "owner");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_allow_creating_additional_orgs_after_personal_org(ctx: &mut TestContext) {
        let (user, _personal_org) = ctx.create_user_with_personal_org("additional-orgs").await;
        let service = OrganizationService::new(ctx.context.clone());

        let request = ctx.authenticated_request(
            CreateOrganizationRequest {
                name: "Team Org".to_string(),
                slug: format!("team-org-{}", user.id),
                settings: None,
            },
            &user.id,
        );
        let response = service.create_organization(request).await.unwrap().into_inner();
        let team_org = response.organization.expect("team org should be created");

        // Track for cleanup
        let team_org_id: i32 = {
            use sea_orm::ColumnTrait;
            use sea_orm::QueryFilter;
            organizations::Entity::find()
                .filter(organizations::Column::Pid.eq(&team_org.pid))
                .one(ctx.context.db.as_ref())
                .await
                .unwrap()
                .unwrap()
                .id
        };
        ctx.created_organizations.push(team_org_id);

        let request = ctx.authenticated_request(ListOrganizationsRequest {}, &user.id);
        let response = service.list_organizations(request).await.unwrap().into_inner();

        assert_eq!(response.organizations.len(), 2);
    }

    // --- Cache write behavior tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_update_cache_when_creating_organization(ctx: &mut TestContext) {
        let (user, personal_org) = ctx.create_user_with_personal_org("cache-create-org").await;
        let service = OrganizationService::new(ctx.context.clone());
        let cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);

        // Seed cache with the personal org (normally done by provisioning middleware)
        cache
            .set(
                &user.id,
                &CachedUserData {
                    email: user.email.clone(),
                    organizations: vec![CachedOrg::new(
                        personal_org.pid.clone(),
                        PlanTier::Free,
                        OrganizationRole::Owner,
                    )],
                },
            )
            .await;

        let request = ctx.authenticated_request(
            CreateOrganizationRequest {
                name: "Cache Test Org".to_string(),
                slug: format!("cache-test-org-{}", user.id),
                settings: None,
            },
            &user.id,
        );
        let response = service.create_organization(request).await.unwrap().into_inner();
        let new_org = response.organization.expect("org should be created");

        // Track for cleanup
        let new_org_id: i32 = {
            organizations::Entity::find()
                .filter(organizations::Column::Pid.eq(&new_org.pid))
                .one(ctx.context.db.as_ref())
                .await
                .unwrap()
                .unwrap()
                .id
        };
        ctx.created_organizations.push(new_org_id);

        // Verify cache has both orgs
        let cached = cache.get(&user.id).await.expect("Cache should have user data");
        assert_eq!(cached.organizations.len(), 2, "Cache should have personal + new org");
        assert!(
            cached.org(&personal_org.pid).is_some(),
            "Personal org should be in cache"
        );
        assert!(cached.org(&new_org.pid).is_some(), "New org should be in cache");

        let new_org_cache = cached.org(&new_org.pid).unwrap();
        assert_eq!(new_org_cache.tier, PlanTier::Free);
        assert_eq!(new_org_cache.role, OrganizationRole::Owner);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_update_cache_when_adding_member(ctx: &mut TestContext) {
        let (owner, org) = ctx.create_user_with_personal_org("cache-add-owner").await;
        let new_member = ctx.create_user("cache-add-member").await;
        let service = OrganizationService::new(ctx.context.clone());
        let cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);

        // Seed the new member's cache (simulating they were provisioned)
        cache
            .set(
                &new_member.id,
                &CachedUserData {
                    email: new_member.email.clone(),
                    organizations: vec![],
                },
            )
            .await;

        let request = ctx.organization_request(
            AddMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: new_member.id.clone(),
                role: OrganizationRole::Member.to_string(),
                email: None,
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.add_member(request).await.unwrap();

        // Verify the new member's cache has the org
        let cached = cache.get(&new_member.id).await.expect("Cache should have member data");
        assert!(cached.org(&org.pid).is_some(), "Org should be in member's cache");
        assert_eq!(cached.org(&org.pid).unwrap().role, OrganizationRole::Member);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_update_cache_when_changing_member_role(ctx: &mut TestContext) {
        let (owner, org) = ctx.create_user_with_personal_org("cache-role-owner").await;
        let member = ctx.create_user("cache-role-member").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;
        let service = OrganizationService::new(ctx.context.clone());
        let cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);

        // Seed the member's cache with current role
        cache
            .set(
                &member.id,
                &CachedUserData {
                    email: member.email.clone(),
                    organizations: vec![CachedOrg::new(
                        org.pid.clone(),
                        PlanTier::Free,
                        OrganizationRole::Member,
                    )],
                },
            )
            .await;

        let request = ctx.organization_request(
            UpdateMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: member.id.clone(),
                role: Some(OrganizationRole::Admin.to_string()),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.update_member(request).await.unwrap();

        // Verify cache role was updated
        let cached = cache.get(&member.id).await.expect("Cache should have member data");
        assert_eq!(cached.org(&org.pid).unwrap().role, OrganizationRole::Admin);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_update_cache_when_removing_member(ctx: &mut TestContext) {
        let (owner, org) = ctx.create_user_with_personal_org("cache-remove-owner").await;
        let member = ctx.create_user("cache-remove-member").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;
        let service = OrganizationService::new(ctx.context.clone());
        let cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);

        // Seed the member's cache with the org
        cache
            .set(
                &member.id,
                &CachedUserData {
                    email: member.email.clone(),
                    organizations: vec![CachedOrg::new(
                        org.pid.clone(),
                        PlanTier::Free,
                        OrganizationRole::Member,
                    )],
                },
            )
            .await;

        let request = ctx.organization_request(
            RemoveMemberRequest {
                organization_pid: org.pid.clone(),
                user_id: member.id.clone(),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.remove_member(request).await.unwrap();

        // Verify cache no longer has the org
        let cached = cache.get(&member.id).await.expect("Cache should have member data");
        assert!(
            cached.org(&org.pid).is_none(),
            "Org should be removed from member's cache"
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_clean_all_members_caches_when_deleting_organization(ctx: &mut TestContext) {
        let (owner, org) = ctx.create_user_with_personal_org("cache-delete-org-owner").await;
        let member = ctx.create_user("cache-delete-org-member").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;
        let service = OrganizationService::new(ctx.context.clone());
        let cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);

        // Seed both users' caches with the org
        cache
            .set(
                &owner.id,
                &CachedUserData {
                    email: owner.email.clone(),
                    organizations: vec![CachedOrg::new(org.pid.clone(), PlanTier::Free, OrganizationRole::Owner)],
                },
            )
            .await;
        cache
            .set(
                &member.id,
                &CachedUserData {
                    email: member.email.clone(),
                    organizations: vec![CachedOrg::new(
                        org.pid.clone(),
                        PlanTier::Free,
                        OrganizationRole::Member,
                    )],
                },
            )
            .await;

        // Delete the org
        let request = ctx.organization_request(
            DeleteOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &owner.id,
            &org.pid,
            org.id,
            "owner",
        );
        service.delete_organization(request).await.unwrap();

        // Verify both users' caches no longer have the org
        let owner_cached = cache.get(&owner.id).await.expect("Owner cache should exist");
        assert!(
            owner_cached.org(&org.pid).is_none(),
            "Deleted org should be removed from owner's cache"
        );

        let member_cached = cache.get(&member.id).await.expect("Member cache should exist");
        assert!(
            member_cached.org(&org.pid).is_none(),
            "Deleted org should be removed from member's cache"
        );
    }

    /// Verifies that handlers not requiring org context succeed even when
    /// OrganizationContext is missing from the request (simulates the middleware
    /// passthrough when a user sends a stale x-organization-id cookie for an
    /// org they're not a member of).
    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_list_organizations_without_org_context(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("no-org-ctx").await;
        let service = OrganizationService::new(ctx.context.clone());

        // Use authenticated_request (no OrganizationContext in extensions)
        // This simulates the middleware passthrough when org_context skips validation
        let request = ctx.authenticated_request(ListOrganizationsRequest {}, &user.id);
        let response = service.list_organizations(request).await.unwrap().into_inner();

        assert_eq!(response.organizations.len(), 1);
        assert_eq!(response.organizations[0].pid, org.pid);
    }

    /// Verifies that handlers requiring org context return a clear error when
    /// OrganizationContext is missing (not a hard 403 from middleware, but a
    /// handler-level failed_precondition).
    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(orgs)]
    async fn should_fail_get_organization_without_org_context(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("no-org-ctx-get").await;
        let service = OrganizationService::new(ctx.context.clone());

        // Request with auth but no OrganizationContext — simulates stale cookie passthrough
        let request = ctx.authenticated_request(
            GetOrganizationRequest {
                organization_pid: org.pid.clone(),
            },
            &user.id,
        );
        let err = service.get_organization(request).await.unwrap_err();

        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert!(err.message().contains("Organization required"));
    }
}
