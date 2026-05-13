use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::{document_group_memberships, document_groups, documents},
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1::*;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, EntityTrait as _, PaginatorTrait as _, QueryFilter as _,
    QueryOrder as _, QuerySelect as _,
};
use std::sync::Arc;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;

#[derive(Debug, Clone)]
pub(crate) struct DocumentGroupService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    context: Arc<AppContext<EM>>,
}

impl<EM> DocumentGroupService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(context: Arc<AppContext<EM>>) -> Self {
        Self { context }
    }

    async fn membership_count(&self, group_id: i64) -> Result<i32, tonic::Status> {
        let count = document_group_memberships::Entity::find()
            .filter(document_group_memberships::Column::GroupId.eq(group_id))
            .count(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to count group memberships");
                tonic::Status::internal("Failed to count documents in group")
            })?;
        Ok(count as i32)
    }
}

#[tonic::async_trait]
impl<EM> document_group_service_server::DocumentGroupService for DocumentGroupService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn create_group(
        &self,
        request: tonic::Request<CreateGroupRequest>,
    ) -> Result<tonic::Response<CreateGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();

        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);

        if req.name.is_empty() {
            return Err(tonic::Status::invalid_argument("name is required"));
        }

        let group = document_groups::ActiveModelEx::new()
            .set_user_id(&user_id)
            .set_organization_pid(organization_pid)
            .set_name(&req.name)
            .set_description(req.description)
            .set_is_org_shared(req.is_org_shared);

        let group = group.insert(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to create group");
            tonic::Status::internal("Failed to create group")
        })?;

        Ok(tonic::Response::new(CreateGroupResponse {
            group: Some(group.into_proto(0)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn get_group(
        &self,
        request: tonic::Request<GetGroupRequest>,
    ) -> Result<tonic::Response<GetGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_id = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let group = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.eq(&req.pid))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch group");
                tonic::Status::internal("Failed to fetch group")
            })?
            .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

        if !group.is_accessible(&user_id, &org_id) {
            return Err(tonic::Status::not_found("Group not found"));
        }

        let doc_count = self.membership_count(group.id).await?;

        Ok(tonic::Response::new(GetGroupResponse {
            group: Some(group.into_proto(doc_count)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), page, page_size)
    )]
    async fn list_groups(
        &self,
        request: tonic::Request<ListGroupsRequest>,
    ) -> Result<tonic::Response<ListGroupsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_id = request.organization_pid()?;
        let req = request.into_inner();

        let page = req.page.max(1) as u64;
        let page_size = if req.page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            (req.page_size as u64).clamp(1, MAX_PAGE_SIZE)
        };

        let span = tracing::Span::current();
        span.record("page", page);
        span.record("page_size", page_size);

        let mut query = document_groups::Entity::find();

        // Access scoping: own groups + org-shared groups in the same org
        query = query.filter(
            sea_orm::Condition::any()
                .add(document_groups::Column::UserId.eq(&user_id))
                .add(
                    sea_orm::Condition::all()
                        .add(document_groups::Column::IsOrgShared.eq(true))
                        .add(document_groups::Column::OrganizationPid.eq(&org_id)),
                ),
        );

        let total = query.clone().count(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count groups");
            tonic::Status::internal("Failed to count groups")
        })? as i32;

        let groups = query
            .order_by_desc(document_groups::Column::CreatedAt)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch groups");
                tonic::Status::internal("Failed to fetch groups")
            })?;

        // Batch-fetch membership counts
        let group_ids: Vec<i64> = groups.iter().map(|g| g.id).collect();
        let counts: std::collections::HashMap<i64, i32> = if group_ids.is_empty() {
            std::collections::HashMap::new()
        } else {
            document_group_memberships::Entity::find()
                .filter(document_group_memberships::Column::GroupId.is_in(group_ids))
                .select_only()
                .column(document_group_memberships::Column::GroupId)
                .column_as(document_group_memberships::Column::Id.count(), "count")
                .group_by(document_group_memberships::Column::GroupId)
                .into_tuple::<(i64, i64)>()
                .all(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to count group memberships");
                    tonic::Status::internal("Failed to count group memberships")
                })?
                .into_iter()
                .map(|(gid, cnt)| (gid, cnt as i32))
                .collect()
        };

        let groups = groups
            .into_iter()
            .map(|g| {
                let count = counts.get(&g.id).copied().unwrap_or(0);
                g.into_proto(count)
            })
            .collect();

        Ok(tonic::Response::new(ListGroupsResponse {
            groups,
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn delete_group(
        &self,
        request: tonic::Request<DeleteGroupRequest>,
    ) -> Result<tonic::Response<DeleteGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_id = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let group = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.eq(&req.pid))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch group");
                tonic::Status::internal("Failed to fetch group")
            })?
            .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

        if !group.is_accessible(&user_id, &org_id) {
            return Err(tonic::Status::not_found("Group not found"));
        }

        if !group.is_owner(&user_id) {
            return Err(tonic::Status::permission_denied(
                "Only the group owner can delete this group",
            ));
        }

        document_groups::Entity::delete_by_id(group.id)
            .exec(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to delete group");
                tonic::Status::internal("Failed to delete group")
            })?;

        Ok(tonic::Response::new(DeleteGroupResponse {}))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn update_group(
        &self,
        request: tonic::Request<UpdateGroupRequest>,
    ) -> Result<tonic::Response<UpdateGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_id = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let group = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.eq(&req.pid))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch group");
                tonic::Status::internal("Failed to fetch group")
            })?
            .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

        if !group.is_accessible(&user_id, &org_id) {
            return Err(tonic::Status::not_found("Group not found"));
        }

        if !group.is_owner(&user_id) {
            return Err(tonic::Status::permission_denied(
                "Only the group owner can update this group",
            ));
        }

        let group_id = group.id;
        let mut active: document_groups::ActiveModel = group.into();

        if let Some(name) = &req.name {
            if name.is_empty() {
                return Err(tonic::Status::invalid_argument("name cannot be empty"));
            }
            active.name = Set(name.clone());
        }

        if let Some(description) = req.description {
            active.description = Set(Some(description));
        }

        if let Some(is_org_shared) = req.is_org_shared {
            active.is_org_shared = Set(is_org_shared);
        }

        let group = active.update(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to update group");
            tonic::Status::internal("Failed to update group")
        })?;

        let doc_count = self.membership_count(group_id).await?;

        Ok(tonic::Response::new(UpdateGroupResponse {
            group: Some(group.into_proto(doc_count)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), group_pid)
    )]
    async fn add_documents_to_group(
        &self,
        request: tonic::Request<AddDocumentsToGroupRequest>,
    ) -> Result<tonic::Response<AddDocumentsToGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("group_pid", &req.group_pid);

        let group = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.eq(&req.group_pid))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch group");
                tonic::Status::internal("Failed to fetch group")
            })?
            .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

        if !group.is_accessible(&user_id, &organization_pid) {
            return Err(tonic::Status::not_found("Group not found"));
        }

        if !req.document_pids.is_empty() {
            // Fetch documents that the user can access
            let docs = documents::Entity::find()
                .filter(documents::Column::Pid.is_in(&req.document_pids))
                .all(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to fetch documents");
                    tonic::Status::internal("Failed to fetch documents")
                })?;

            let accessible_doc_ids: Vec<i64> = docs
                .into_iter()
                .filter(|d| d.is_accessible(&user_id, &organization_pid))
                // Document and group must be in the same org
                .filter(|d| d.organization_pid == group.organization_pid)
                .map(|d| d.id)
                .collect();

            if !accessible_doc_ids.is_empty() {
                let memberships: Vec<document_group_memberships::ActiveModel> = accessible_doc_ids
                    .iter()
                    .map(|doc_id| document_group_memberships::ActiveModel {
                        id: sea_orm::ActiveValue::NotSet,
                        document_id: Set(*doc_id),
                        group_id: Set(group.id),
                        created_at: sea_orm::ActiveValue::NotSet,
                    })
                    .collect();

                document_group_memberships::Entity::insert_many(memberships)
                    .on_conflict(
                        sea_orm::sea_query::OnConflict::columns([
                            document_group_memberships::Column::DocumentId,
                            document_group_memberships::Column::GroupId,
                        ])
                        .do_nothing()
                        .to_owned(),
                    )
                    .exec_without_returning(&self.context.db)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = %e, "failed to add documents to group");
                        tonic::Status::internal("Failed to add documents to group")
                    })?;
            }
        }

        let doc_count = self.membership_count(group.id).await?;

        Ok(tonic::Response::new(AddDocumentsToGroupResponse {
            group: Some(group.into_proto(doc_count)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), group_pid)
    )]
    async fn remove_documents_from_group(
        &self,
        request: tonic::Request<RemoveDocumentsFromGroupRequest>,
    ) -> Result<tonic::Response<RemoveDocumentsFromGroupResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let org_id = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("group_pid", &req.group_pid);

        let group = document_groups::Entity::find()
            .filter(document_groups::Column::Pid.eq(&req.group_pid))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch group");
                tonic::Status::internal("Failed to fetch group")
            })?
            .ok_or_else(|| tonic::Status::not_found("Group not found"))?;

        if !group.is_accessible(&user_id, &org_id) {
            return Err(tonic::Status::not_found("Group not found"));
        }

        if !req.document_pids.is_empty() {
            let doc_ids: Vec<i64> = documents::Entity::find()
                .filter(documents::Column::Pid.is_in(&req.document_pids))
                .all(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to fetch documents");
                    tonic::Status::internal("Failed to fetch documents")
                })?
                .into_iter()
                .filter(|d| d.is_accessible(&user_id, &org_id))
                .map(|d| d.id)
                .collect();

            if !doc_ids.is_empty() {
                document_group_memberships::Entity::delete_many()
                    .filter(document_group_memberships::Column::GroupId.eq(group.id))
                    .filter(document_group_memberships::Column::DocumentId.is_in(doc_ids))
                    .exec(&self.context.db)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = %e, "failed to remove documents from group");
                        tonic::Status::internal("Failed to remove documents from group")
                    })?;
            }
        }

        let doc_count = self.membership_count(group.id).await?;

        Ok(tonic::Response::new(RemoveDocumentsFromGroupResponse {
            group: Some(group.into_proto(doc_count)),
        }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::document_group_service_server::DocumentGroupService as _;
    use serial_test::serial;
    use test_context::test_context;

    fn authenticated_request<T>(inner: T, user_id: &str, org_pid: Option<&str>) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        request.extensions_mut().insert(User {
            id: user_id.to_string(),
            email: Some(format!("{user_id}@test.com")),
            name: Some(format!("Test User {user_id}")),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        });
        if let Some(org) = org_pid {
            request.extensions_mut().insert(OrganizationPid(org.to_string()));
        }
        request
    }

    fn create_service(ctx: &TestContext) -> DocumentGroupService<crate::test_utils::MockEmbeddingModel> {
        DocumentGroupService::new(ctx.app_ctx())
    }

    mod create_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_group_with_valid_name(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateGroupRequest {
                    name: "My Research".to_string(),
                    description: Some("Papers and notes".to_string()),
                    is_org_shared: false,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_group(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let group = response.unwrap().into_inner().group.expect("should have group");
            assert_eq!(group.name, "My Research");
            assert_eq!(group.description, Some("Papers and notes".to_string()));
            assert!(!group.is_org_shared);
            assert_eq!(group.document_count, 0);

            // Track for cleanup
            let db_group = document_groups::Entity::find()
                .filter(document_groups::Column::Pid.eq(&group.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_document_groups.push(db_group.id);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_name(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateGroupRequest {
                    name: "".to_string(),
                    description: None,
                    is_org_shared: false,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_create_group_without_org_context(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateGroupRequest {
                    name: "Personal Notes".to_string(),
                    description: Some("My private docs".to_string()),
                    is_org_shared: false,
                },
                "user_no_org",
                None, // no org context
            );

            let result = service.create_group(request).await;
            assert!(result.is_err(), "should reject without org context");
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_default_is_org_shared_to_false(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                CreateGroupRequest {
                    name: "Private by Default".to_string(),
                    description: None,
                    is_org_shared: false,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.create_group(request).await.unwrap().into_inner();
            let group = response.group.expect("should have group");
            assert!(!group.is_org_shared, "new group should default to private");

            // Track for cleanup
            let db_group = document_groups::Entity::find()
                .filter(document_groups::Column::Pid.eq(&group.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            ctx.created_document_groups.push(db_group.id);
        }
    }

    mod get_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_group_for_owner(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("get-owner", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                GetGroupRequest { pid: group.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_group(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let returned = response.unwrap().into_inner().group.expect("should have group");
            assert_eq!(returned.pid, group.pid);
            assert_eq!(returned.name, group.name);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_shared_group_for_org_member(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("get-shared", "user_owner", "org_test", true)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                GetGroupRequest { pid: group.pid.clone() },
                "user_other",
                Some("org_test"),
            );

            let response = service.get_group(request).await;
            assert!(response.is_ok(), "org member should access shared group");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_private_group_to_org_member(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("get-private", "user_owner", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                GetGroupRequest { pid: group.pid.clone() },
                "user_other",
                Some("org_test"),
            );

            let response = service.get_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_group_to_different_org(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("get-difforg", "user_owner", "org_a", true)
                .await;
            let service = create_service(ctx);

            let request =
                authenticated_request(GetGroupRequest { pid: group.pid.clone() }, "user_other", Some("org_b"));

            let response = service.get_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_pid(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                GetGroupRequest {
                    pid: "grp_nonexistent".to_string(),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_document_count(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("get-count", "user_test", "org_test", false)
                .await;
            let doc1 = ctx.create_ready_document("get-count-1", "user_test", "org_test").await;
            let doc2 = ctx.create_ready_document("get-count-2", "user_test", "org_test").await;
            let doc3 = ctx.create_ready_document("get-count-3", "user_test", "org_test").await;
            ctx.add_document_to_group(doc1.id, group.id).await;
            ctx.add_document_to_group(doc2.id, group.id).await;
            ctx.add_document_to_group(doc3.id, group.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                GetGroupRequest { pid: group.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.get_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 3);
        }
    }

    mod list_groups {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_owned_groups(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("list-owned", "user_list", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListGroupsRequest { page: 1, page_size: 20 },
                "user_list",
                Some("org_test"),
            );

            let response = service.list_groups(request).await.unwrap().into_inner();
            assert!(
                response.groups.iter().any(|g| g.pid == group.pid),
                "should include owned group"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_shared_org_groups(ctx: &mut TestContext) {
            let shared = ctx
                .create_document_group("list-shared", "user_owner", "org_list", true)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListGroupsRequest { page: 1, page_size: 20 },
                "user_member",
                Some("org_list"),
            );

            let response = service.list_groups(request).await.unwrap().into_inner();
            assert!(
                response.groups.iter().any(|g| g.pid == shared.pid),
                "org member should see shared groups"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_exclude_private_groups_from_other_users(ctx: &mut TestContext) {
            let private = ctx
                .create_document_group("list-priv", "user_owner", "org_excl", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                ListGroupsRequest { page: 1, page_size: 20 },
                "user_member",
                Some("org_excl"),
            );

            let response = service.list_groups(request).await.unwrap().into_inner();
            assert!(
                !response.groups.iter().any(|g| g.pid == private.pid),
                "should not see other user's private group"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_paginate_results(ctx: &mut TestContext) {
            // Create 3 groups for pagination testing
            ctx.create_document_group("list-pg-1", "user_pg", "org_pg", false).await;
            ctx.create_document_group("list-pg-2", "user_pg", "org_pg", false).await;
            ctx.create_document_group("list-pg-3", "user_pg", "org_pg", false).await;

            let service = create_service(ctx);

            let request = authenticated_request(ListGroupsRequest { page: 1, page_size: 2 }, "user_pg", Some("org_pg"));

            let response = service.list_groups(request).await.unwrap().into_inner();
            assert_eq!(response.groups.len(), 2, "page 1 should have 2 items");
            assert!(response.total >= 3, "total should be at least 3");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_document_counts(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("list-counts", "user_cnt", "org_cnt", false)
                .await;
            let doc1 = ctx.create_ready_document("list-cnt-1", "user_cnt", "org_cnt").await;
            let doc2 = ctx.create_ready_document("list-cnt-2", "user_cnt", "org_cnt").await;
            ctx.add_document_to_group(doc1.id, group.id).await;
            ctx.add_document_to_group(doc2.id, group.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                ListGroupsRequest { page: 1, page_size: 20 },
                "user_cnt",
                Some("org_cnt"),
            );

            let response = service.list_groups(request).await.unwrap().into_inner();
            let found = response.groups.iter().find(|g| g.pid == group.pid);
            assert!(found.is_some(), "should include the group");
            assert_eq!(found.unwrap().document_count, 2, "should have correct document count");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_for_no_groups(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                ListGroupsRequest { page: 1, page_size: 20 },
                "user_with_no_groups",
                Some("org_empty"),
            );

            let response = service.list_groups(request).await.unwrap().into_inner();
            assert!(response.groups.is_empty(), "should have no groups");
            assert_eq!(response.total, 0);
        }
    }

    mod update_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_name(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-name", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: Some("Renamed Group".to_string()),
                    description: None,
                    is_org_shared: None,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_group(request).await.unwrap().into_inner();
            let updated = response.group.expect("should have group");
            assert_eq!(updated.name, "Renamed Group");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_toggle_is_org_shared(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-share", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: None,
                    description: None,
                    is_org_shared: Some(true),
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_group(request).await.unwrap().into_inner();
            let updated = response.group.expect("should have group");
            assert!(updated.is_org_shared, "should now be shared");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_name(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-empty", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: Some("".to_string()),
                    description: None,
                    is_org_shared: None,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_description(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-desc", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: None,
                    description: Some("Updated description".to_string()),
                    is_org_shared: None,
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.update_group(request).await.unwrap().into_inner();
            let updated = response.group.expect("should have group");
            assert_eq!(updated.description, Some("Updated description".to_string()));
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_non_owner_update(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-deny", "user_owner", "org_test", true)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: Some("Hijacked".to_string()),
                    description: None,
                    is_org_shared: None,
                },
                "user_other",
                Some("org_test"),
            );

            let response = service.update_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_update_for_wrong_user(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("upd-wrong", "user_owner", "org_a", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                UpdateGroupRequest {
                    pid: group.pid.clone(),
                    name: Some("Hijacked".to_string()),
                    description: None,
                    is_org_shared: None,
                },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.update_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod delete_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_delete_group_for_owner(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("del-owner", "user_test", "org_test", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteGroupRequest { pid: group.pid.clone() },
                "user_test",
                Some("org_test"),
            );

            let response = service.delete_group(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            // Verify it's gone
            let get_request = authenticated_request(
                GetGroupRequest { pid: group.pid.clone() },
                "user_test",
                Some("org_test"),
            );
            let get_response = service.get_group(get_request).await;
            assert!(get_response.is_err());
            assert_eq!(get_response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_non_owner_delete(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("del-deny", "user_owner", "org_test", true)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteGroupRequest { pid: group.pid.clone() },
                "user_other",
                Some("org_test"),
            );

            let response = service.delete_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_delete_for_wrong_user(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("del-wrong", "user_owner", "org_a", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteGroupRequest { pid: group.pid.clone() },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.delete_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_delete_documents_in_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("del-docs", "user_test", "org_test", false)
                .await;
            let doc = ctx.create_ready_document("del-docs", "user_test", "org_test").await;
            ctx.add_document_to_group(doc.id, group.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                DeleteGroupRequest { pid: group.pid.clone() },
                "user_test",
                Some("org_test"),
            );
            service.delete_group(request).await.unwrap();

            // Document should still exist
            let still_exists = documents::Entity::find()
                .filter(documents::Column::Id.eq(doc.id))
                .one(ctx.db.as_ref())
                .await
                .unwrap();
            assert!(still_exists.is_some(), "document should survive group deletion");
        }
    }

    mod add_documents_to_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_add_accessible_documents(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("add-docs", "user_test", "org_test", false)
                .await;
            let doc = ctx.create_ready_document("add-docs", "user_test", "org_test").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.add_documents_to_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 1);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_be_idempotent(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("add-idem", "user_test", "org_test", false)
                .await;
            let doc = ctx.create_ready_document("add-idem", "user_test", "org_test").await;
            let service = create_service(ctx);

            // Add once
            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.add_documents_to_group(request).await.unwrap();

            // Add again — should not error or double-count
            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            let response = service.add_documents_to_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 1, "should not double-count");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_inaccessible_documents(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("add-inac", "user_test", "org_test", false)
                .await;
            let other_doc = ctx.create_ready_document("add-inac", "user_other", "org_other").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![other_doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.add_documents_to_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 0, "inaccessible doc should be skipped");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_cross_org_documents(ctx: &mut TestContext) {
            // User owns a doc in org_a but the group is in org_b
            let group = ctx
                .create_document_group("add-xorg", "user_multi", "org_b", false)
                .await;
            let doc = ctx.create_ready_document("add-xorg", "user_multi", "org_a").await;
            let service = create_service(ctx);

            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_multi",
                Some("org_b"),
            );

            let response = service.add_documents_to_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 0, "cross-org doc should be skipped");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_add_documents_to_personal_group(ctx: &mut TestContext) {
            // Personal group (no org) with a personal document
            let group = ctx
                .create_document_group_with_org(
                    "add-personal",
                    "user_personal_add",
                    "personal_org_user_personal_add",
                    false,
                )
                .await;
            let doc = ctx
                .create_ready_document_with_org(
                    "add-personal-doc",
                    "user_personal_add",
                    "personal_org_user_personal_add",
                )
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_personal_add",
                Some("personal_org_user_personal_add"),
            );

            let response = service.add_documents_to_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(
                returned.document_count, 1,
                "personal doc should be added to personal group"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_inaccessible_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("add-noacc", "user_owner", "org_a", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                AddDocumentsToGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![],
                },
                "user_stranger",
                Some("org_b"), // different org than group's org_a
            );

            let response = service.add_documents_to_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod remove_documents_from_group {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_remove_documents_from_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("rm-docs", "user_test", "org_test", false)
                .await;
            let doc = ctx.create_ready_document("rm-docs", "user_test", "org_test").await;
            ctx.add_document_to_group(doc.id, group.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                RemoveDocumentsFromGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.remove_documents_from_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 0, "document should be removed");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_inaccessible_documents(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("rm-inac", "user_test", "org_test", true)
                .await;
            let other_doc = ctx.create_ready_document("rm-inac", "user_other", "org_other").await;
            // Force-add via fixture (bypassing access check)
            ctx.add_document_to_group(other_doc.id, group.id).await;

            let service = create_service(ctx);

            // user_test tries to remove other_doc — should be silently skipped
            let request = authenticated_request(
                RemoveDocumentsFromGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![other_doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );

            let response = service.remove_documents_from_group(request).await.unwrap().into_inner();
            let returned = response.group.expect("should have group");
            assert_eq!(returned.document_count, 1, "inaccessible doc should remain");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_delete_documents(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("rm-nodelete", "user_test", "org_test", false)
                .await;
            let doc = ctx.create_ready_document("rm-nodelete", "user_test", "org_test").await;
            ctx.add_document_to_group(doc.id, group.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                RemoveDocumentsFromGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![doc.pid.clone()],
                },
                "user_test",
                Some("org_test"),
            );
            service.remove_documents_from_group(request).await.unwrap();

            // Document should still exist in DB
            let still_exists = documents::Entity::find()
                .filter(documents::Column::Id.eq(doc.id))
                .one(ctx.db.as_ref())
                .await
                .unwrap();
            assert!(still_exists.is_some(), "document should survive removal from group");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_inaccessible_group(ctx: &mut TestContext) {
            let group = ctx
                .create_document_group("rm-noacc", "user_owner", "org_a", false)
                .await;
            let service = create_service(ctx);

            let request = authenticated_request(
                RemoveDocumentsFromGroupRequest {
                    group_pid: group.pid.clone(),
                    document_pids: vec![],
                },
                "user_stranger",
                Some("org_stranger"),
            );

            let response = service.remove_documents_from_group(request).await;
            assert!(response.is_err());
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }
}
