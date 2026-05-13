use crate::{
    AppContext,
    middleware::RequestExt,
    models::{organization_members, users},
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::api::v1::{
    DeleteAccountRequest, DeleteAccountResponse, GetUsersRequest, GetUsersResponse, MeRequest, MeResponse,
    UpdateProfileRequest, UpdateProfileResponse, UserSummary, user_service_server,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::sync::Arc;
use tonic::{Response, Status};
use tracing::instrument;

#[derive(Debug, Clone)]
pub(crate) struct UserService {
    context: Arc<AppContext>,
}

impl UserService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }
}

#[tonic::async_trait]
impl user_service_server::UserService for UserService {
    #[instrument(skip(self, request), fields(user_id))]
    async fn me(
        &self,
        request: tonic::Request<MeRequest>,
    ) -> std::result::Result<tonic::Response<MeResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        tracing::Span::current().record("user_id", &user_id);

        tracing::debug!("Fetching user profile");
        let user = users::Entity::find_by_id(user_id)
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?
            .ok_or(Status::not_found("User not found"))?;
        tracing::info!("User profile retrieved");

        Ok(Response::new(MeResponse {
            user: Some(user.into()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id))]
    async fn update_profile(
        &self,
        request: tonic::Request<UpdateProfileRequest>,
    ) -> std::result::Result<tonic::Response<UpdateProfileResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        tracing::Span::current().record("user_id", &user_id);

        tracing::debug!("Fetching user profile");
        let mut user: users::ActiveModel = users::Entity::find_by_id(user_id)
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| Status::not_found(format!("Database error: {e}")))?
            .ok_or(Status::not_found("User not found"))?
            .into();

        let info = request.get_ref().to_owned();

        if let Some(display_name) = info.display_name {
            user.display_name = Set(Some(display_name));
        }

        if let Some(avatar_url) = info.avatar_url {
            user.avatar_url = Set(Some(avatar_url));
        }

        if let Some(bio) = info.bio {
            user.bio = Set(Some(bio));
        }

        if let Some(settings) = info.settings {
            // Convert UserSettings to JsonValue for database storage
            let settings_json = serde_json::to_value(settings).unwrap_or_default();
            user.settings = Set(settings_json);
        }

        tracing::debug!("Updating user profile");
        let user = user
            .update(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;
        tracing::info!("User profile updated");

        Ok(Response::new(UpdateProfileResponse {
            user: Some(user.into()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id))]
    async fn delete_account(
        &self,
        request: tonic::Request<DeleteAccountRequest>,
    ) -> std::result::Result<tonic::Response<DeleteAccountResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        tracing::Span::current().record("user_id", &user_id);

        tracing::debug!("Deleting user account");
        users::Entity::delete_by_id(user_id)
            .exec(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;
        tracing::info!("User account deleted");

        Ok(Response::new(DeleteAccountResponse {}))
    }

    #[instrument(skip(self, request), fields(count, organization_id))]
    async fn get_users(
        &self,
        request: tonic::Request<GetUsersRequest>,
    ) -> std::result::Result<tonic::Response<GetUsersResponse>, tonic::Status> {
        let ctx = request.organization_context()?;
        let org_id = ctx.organization_id;
        tracing::Span::current().record("organization_id", org_id);

        let user_ids = request.into_inner().user_ids;
        tracing::Span::current().record("count", user_ids.len());

        if user_ids.is_empty() {
            return Ok(Response::new(GetUsersResponse { users: vec![] }));
        }

        if user_ids.len() > 100 {
            return Err(Status::invalid_argument("Cannot fetch more than 100 users at once"));
        }

        // Only return users who are members of the requesting org
        tracing::debug!("Batch fetching users scoped to org");
        let found = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::OrganizationMembers.def())
            .filter(users::Column::Id.is_in(&user_ids))
            .filter(organization_members::Column::OrganizationId.eq(org_id))
            .all(self.context.db.as_ref())
            .await
            .map_err(|e| Status::internal(format!("Database error: {e}")))?;

        let users = found
            .into_iter()
            .map(|u| UserSummary {
                id: u.id,
                display_name: u.display_name,
                email: u.email,
            })
            .collect();

        tracing::info!("Batch user lookup complete");
        Ok(Response::new(GetUsersResponse { users }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use proto_rs::api::v1::user_service_server::UserService as UserServiceTrait;
    use proto_rs::api::v1::{DeleteAccountRequest, MeRequest, UpdateProfileRequest};
    use serial_test::serial;
    use test_context::test_context;

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_me_success(ctx: &mut TestContext) {
        let user = ctx.create_user("me-success").await;
        let service = UserService::new(ctx.context.clone());

        let request = ctx.authenticated_request(MeRequest {}, &user.id);
        let result = service.me(request).await;

        assert!(result.is_ok(), "me() should succeed");
        let response = result.unwrap().into_inner();
        assert!(response.user.is_some(), "user should be present in response");

        let returned_user = response.user.unwrap();
        assert_eq!(returned_user.id, user.id);
        assert_eq!(returned_user.email, user.email);
        assert_eq!(returned_user.display_name, user.display_name);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_me_user_not_found(ctx: &mut TestContext) {
        let service = UserService::new(ctx.context.clone());

        // Use a non-existent user ID
        let request = ctx.authenticated_request(MeRequest {}, "nonexistent-user-id");
        let result = service.me(request).await;

        assert!(result.is_err(), "me() should fail for non-existent user");
        let error = result.unwrap_err();
        assert_eq!(error.code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_update_profile_success(ctx: &mut TestContext) {
        let user = ctx.create_user("update-success").await;
        let service = UserService::new(ctx.context.clone());

        let update_req = UpdateProfileRequest {
            display_name: Some("Updated Name".to_string()),
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            bio: Some("Updated bio".to_string()),
            settings: None,
        };

        let request = ctx.authenticated_request(update_req, &user.id);
        let result = service.update_profile(request).await;

        assert!(result.is_ok(), "update_profile() should succeed");
        let response = result.unwrap().into_inner();
        assert!(response.user.is_some(), "user should be present in response");

        let updated_user = response.user.unwrap();
        assert_eq!(updated_user.display_name, Some("Updated Name".to_string()));
        assert_eq!(
            updated_user.avatar_url,
            Some("https://example.com/avatar.png".to_string())
        );
        assert_eq!(updated_user.bio, Some("Updated bio".to_string()));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_update_profile_partial_update(ctx: &mut TestContext) {
        let user = ctx.create_user("update-partial").await;

        // Convert to ActiveModel and update initial values
        let active_user = users::ActiveModel {
            id: Set(user.id.clone()),
            display_name: Set(Some("Original Name".to_string())),
            bio: Set(Some("Original bio".to_string())),
            ..Default::default()
        };

        let user = active_user
            .update(ctx.context.db.as_ref())
            .await
            .expect("Failed to update user");

        let service = UserService::new(ctx.context.clone());

        // Only update display_name, leave bio unchanged
        let update_req = UpdateProfileRequest {
            display_name: Some("New Name".to_string()),
            avatar_url: None,
            bio: None,
            settings: None,
        };

        let request = ctx.authenticated_request(update_req, &user.id);
        let result = service.update_profile(request).await;

        assert!(result.is_ok(), "update_profile() should succeed");
        let response = result.unwrap().into_inner();
        let updated_user = response.user.unwrap();

        assert_eq!(updated_user.display_name, Some("New Name".to_string()));
        assert_eq!(updated_user.bio, Some("Original bio".to_string())); // Should be unchanged
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_delete_account_success(ctx: &mut TestContext) {
        let user = ctx.create_user("delete-success").await;
        let service = UserService::new(ctx.context.clone());

        let request = ctx.authenticated_request(DeleteAccountRequest {}, &user.id);
        let result = service.delete_account(request).await;

        assert!(result.is_ok(), "delete_account() should succeed");

        // Verify deletion by trying to fetch the user - should return NotFound
        let me_request = ctx.authenticated_request(MeRequest {}, &user.id);
        let me_result = service.me(me_request).await;

        assert!(me_result.is_err(), "me() should fail after deletion");
        let error = me_result.unwrap_err();
        assert_eq!(
            error.code(),
            tonic::Code::NotFound,
            "Should return NotFound after deletion"
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn test_delete_account_nonexistent_user(ctx: &mut TestContext) {
        let service = UserService::new(ctx.context.clone());

        let request = ctx.authenticated_request(DeleteAccountRequest {}, "nonexistent-user-id");
        let result = service.delete_account(request).await;

        // SeaORM delete operations don't fail if the record doesn't exist
        // They just return 0 rows affected, so this should still succeed
        assert!(
            result.is_ok(),
            "delete_account() should succeed even for non-existent user"
        );
    }

    // --- GetUsers tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn should_return_users_in_same_org(ctx: &mut TestContext) {
        let (user_a, org) = ctx.create_user_with_personal_org("get-users-a").await;
        let user_b = ctx.create_user("get-users-b").await;
        ctx.create_organization_member(org.id, &user_b.id, "member").await;

        let service = UserService::new(ctx.context.clone());
        let request = ctx.organization_request(
            GetUsersRequest {
                user_ids: vec![user_a.id.clone(), user_b.id.clone()],
            },
            &user_a.id,
            &org.pid,
            org.id,
            "owner",
        );

        let response = service.get_users(request).await.unwrap().into_inner();
        assert_eq!(response.users.len(), 2);

        let ids: Vec<&str> = response.users.iter().map(|u| u.id.as_str()).collect();
        assert!(ids.contains(&user_a.id.as_str()));
        assert!(ids.contains(&user_b.id.as_str()));

        // Verify display data is returned
        let found_a = response.users.iter().find(|u| u.id == user_a.id).unwrap();
        assert_eq!(found_a.email, user_a.email);
        assert_eq!(found_a.display_name, user_a.display_name);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn should_exclude_users_not_in_requesting_org(ctx: &mut TestContext) {
        let (user_a, org_a) = ctx.create_user_with_personal_org("get-users-org-a").await;
        let (user_b, _org_b) = ctx.create_user_with_personal_org("get-users-org-b").await;

        let service = UserService::new(ctx.context.clone());

        // Request from org_a — user_b is NOT a member of org_a
        let request = ctx.organization_request(
            GetUsersRequest {
                user_ids: vec![user_a.id.clone(), user_b.id.clone()],
            },
            &user_a.id,
            &org_a.pid,
            org_a.id,
            "owner",
        );

        let response = service.get_users(request).await.unwrap().into_inner();
        assert_eq!(response.users.len(), 1, "should only return user_a who is in org_a");
        assert_eq!(response.users[0].id, user_a.id);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn should_return_empty_for_empty_input(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("get-users-empty").await;

        let service = UserService::new(ctx.context.clone());
        let request = ctx.organization_request(
            GetUsersRequest { user_ids: vec![] },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );

        let response = service.get_users(request).await.unwrap().into_inner();
        assert!(response.users.is_empty());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn should_return_partial_results_for_mixed_ids(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("get-users-partial").await;

        let service = UserService::new(ctx.context.clone());
        let request = ctx.organization_request(
            GetUsersRequest {
                user_ids: vec![user.id.clone(), "nonexistent-user-id".to_string()],
            },
            &user.id,
            &org.pid,
            org.id,
            "owner",
        );

        let response = service.get_users(request).await.unwrap().into_inner();
        assert_eq!(response.users.len(), 1, "should only return the existing user");
        assert_eq!(response.users[0].id, user.id);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(users)]
    async fn should_reject_more_than_100_ids(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("get-users-limit").await;

        let service = UserService::new(ctx.context.clone());
        let ids: Vec<String> = (0..101).map(|i| format!("user-{i}")).collect();
        let request = ctx.organization_request(GetUsersRequest { user_ids: ids }, &user.id, &org.pid, org.id, "owner");

        let result = service.get_users(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }
}
