use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::tools,
};
use platform_rs::ext::RequestAuthExt;
use proto_rs::rig::v1;
use sea_orm::{ColumnTrait as _, EntityLoaderTrait, QueryFilter, QueryOrder};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub(crate) struct ToolService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    context: Arc<AppContext<EM>>,
}

impl<EM> ToolService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(context: Arc<AppContext<EM>>) -> Self {
        Self { context }
    }
}

#[tonic::async_trait]
impl<EM> v1::tool_service_server::ToolService for ToolService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn list_tools(
        &self,
        request: tonic::Request<v1::ListToolsRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListToolsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();

        tracing::Span::current()
            .record("user_id", &user_id)
            .record("organization_pid", &organization_pid);

        let req = request.into_inner();
        let is_active = req.is_active.unwrap_or(true);

        let mut query = tools::Entity::load().filter(tools::Column::IsActive.eq(is_active));

        // Filter by tool_type if specified
        if let Some(ref tool_type) = req.tool_type {
            query = query.filter(tools::Column::ToolType.eq(tool_type));
        }

        let query = query
            .order_by_desc(tools::Column::CreatedAt)
            .paginate(&self.context.db, req.page_size);

        tracing::debug!("fetching tools");
        let tools = query
            .fetch_page(req.page - 1)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch all tools");
                tonic::Status::internal("Failed to fetch tools")
            })?
            .into_iter()
            .map(tools::ModelEx::into_proto)
            .collect();

        let total = query.num_items().await.map_err(|e| {
            tracing::error!(error = %e, "failed to get tool count");
            tonic::Status::internal("Failed to fetch tool count")
        })?;
        tracing::info!("tools retrieved");

        Ok(tonic::Response::new(v1::ListToolsResponse {
            tools,
            total,
            page: req.page,
            page_size: req.page_size,
        }))
    }

    #[instrument(skip(self, request), fields(user_id, tool_pid))]
    async fn get_tool(
        &self,
        request: tonic::Request<v1::GetToolRequest>,
    ) -> std::result::Result<tonic::Response<v1::GetToolResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.into_inner();

        tracing::Span::current()
            .record("user_id", &user_id)
            .record("tool_pid", &req.pid);

        tracing::debug!("fetching tool");
        let tool = tools::Entity::load()
            .filter_by_pid(&req.pid)
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch tool");
                tonic::Status::internal("Failed to fetch tool")
            })?
            .ok_or_else(|| tonic::Status::not_found("Tool not found"))?;

        tracing::info!("tool retrieved");

        Ok(tonic::Response::new(v1::GetToolResponse {
            tool: Some(tool.into_proto()),
        }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::tool_service_server::ToolService as _;
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

    fn unauthenticated_request<T>(inner: T) -> tonic::Request<T> {
        tonic::Request::new(inner)
    }

    mod list_tools {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_return_tools_matching_active_filter(ctx: &mut TestContext) {
            // Given: active and inactive tools exist
            let active_tool = ctx.create_tool("active", "test_tool_active", "system").await;
            let inactive_tool = ctx.create_inactive_tool("inactive", "test_tool_inactive").await;

            let service = ToolService::new(ctx.app_ctx());

            // When: requesting active tools
            let request = authenticated_request(
                v1::ListToolsRequest {
                    is_active: Some(true),
                    tool_type: None,
                    page: 1,
                    page_size: 100,
                },
                "user_test",
                None,
            );
            let response = service.list_tools(request).await.expect("should succeed");
            let response = response.into_inner();

            // Then: only active tools are returned
            assert!(
                response.tools.iter().any(|t| t.pid == active_tool.pid),
                "active tool should be in response"
            );
            assert!(
                !response.tools.iter().any(|t| t.pid == inactive_tool.pid),
                "inactive tool should not be in response"
            );

            // When: requesting inactive tools
            let request = authenticated_request(
                v1::ListToolsRequest {
                    is_active: Some(false),
                    tool_type: None,
                    page: 1,
                    page_size: 100,
                },
                "user_test",
                None,
            );
            let response = service.list_tools(request).await.expect("should succeed");
            let response = response.into_inner();

            // Then: only inactive tools are returned
            assert!(
                response.tools.iter().any(|t| t.pid == inactive_tool.pid),
                "inactive tool should be in response"
            );
            assert!(
                !response.tools.iter().any(|t| t.pid == active_tool.pid),
                "active tool should not be in response"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_default_to_active_tools(ctx: &mut TestContext) {
            // Given: active and inactive tools exist
            let active_tool = ctx.create_tool("default-active", "default_tool_active", "system").await;
            let inactive_tool = ctx
                .create_inactive_tool("default-inactive", "default_tool_inactive")
                .await;

            let service = ToolService::new(ctx.app_ctx());

            // When: requesting without specifying is_active
            let request = authenticated_request(
                v1::ListToolsRequest {
                    is_active: None,
                    tool_type: None,
                    page: 1,
                    page_size: 100,
                },
                "user_test",
                None,
            );
            let response = service.list_tools(request).await.expect("should succeed");
            let response = response.into_inner();

            // Then: only active tools are returned
            assert!(
                response.tools.iter().any(|t| t.pid == active_tool.pid),
                "active tool should be in response"
            );
            assert!(
                !response.tools.iter().any(|t| t.pid == inactive_tool.pid),
                "inactive tool should not be in response when is_active defaults to true"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_include_pagination_metadata(ctx: &mut TestContext) {
            // Given: multiple tools exist
            ctx.create_tool("page-1", "page_tool_1", "system").await;
            ctx.create_tool("page-2", "page_tool_2", "system").await;
            ctx.create_tool("page-3", "page_tool_3", "system").await;

            let service = ToolService::new(ctx.app_ctx());

            // When: requesting with specific pagination
            let request = authenticated_request(
                v1::ListToolsRequest {
                    is_active: Some(true),
                    tool_type: None,
                    page: 1,
                    page_size: 2,
                },
                "user_test",
                None,
            );
            let response = service.list_tools(request).await.expect("should succeed");
            let response = response.into_inner();

            // Then: pagination metadata reflects the request and data
            assert_eq!(response.page, 1, "page should match request");
            assert_eq!(response.page_size, 2, "page_size should match request");
            assert!(response.total >= 3, "total should reflect all matching tools");
            assert!(response.tools.len() <= 2, "tools returned should respect page_size");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_reject_unauthenticated_requests(ctx: &mut TestContext) {
            let service = ToolService::new(ctx.app_ctx());

            // When: requesting without authentication
            let request = unauthenticated_request(v1::ListToolsRequest {
                is_active: None,
                tool_type: None,
                page: 1,
                page_size: 10,
            });
            let response = service.list_tools(request).await;

            // Then: request is rejected
            assert!(response.is_err(), "unauthenticated request should fail");
            let status = response.unwrap_err();
            assert_eq!(status.code(), tonic::Code::Unauthenticated);
        }
    }

    mod get_tool {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_return_tool_by_pid(ctx: &mut TestContext) {
            // Given: a tool exists
            let tool = ctx.create_tool("get-test", "get_test_tool", "system").await;
            let service = ToolService::new(ctx.app_ctx());

            // When: requesting by pid
            let request = authenticated_request(v1::GetToolRequest { pid: tool.pid.clone() }, "user_test", None);
            let response = service.get_tool(request).await;

            // Then: the tool is returned with correct data
            assert!(response.is_ok(), "should succeed: {:?}", response.err());
            let returned_tool = response.unwrap().into_inner().tool.expect("tool should be present");
            assert_eq!(returned_tool.pid, tool.pid);
            assert_eq!(returned_tool.name, tool.display_name);
            assert_eq!(returned_tool.description, tool.description);
            assert_eq!(returned_tool.tool_type, tool.tool_type);
            assert_eq!(returned_tool.is_active, tool.is_active);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_return_not_found_for_nonexistent_tool(ctx: &mut TestContext) {
            // Given: no tool with this pid exists
            let service = ToolService::new(ctx.app_ctx());

            // When: requesting a nonexistent pid
            let request = authenticated_request(
                v1::GetToolRequest {
                    pid: "nonexistent-pid".to_string(),
                },
                "user_test",
                None,
            );
            let response = service.get_tool(request).await;

            // Then: not found error is returned
            assert!(response.is_err(), "should fail for nonexistent tool");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        async fn should_reject_unauthenticated_requests(ctx: &mut TestContext) {
            // Given: a tool exists
            let tool = ctx.create_tool("get-unauth", "get_unauth_tool", "system").await;
            let service = ToolService::new(ctx.app_ctx());

            // When: requesting without authentication
            let request = unauthenticated_request(v1::GetToolRequest { pid: tool.pid });
            let response = service.get_tool(request).await;

            // Then: request is rejected
            assert!(response.is_err(), "unauthenticated request should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::Unauthenticated);
        }
    }
}
