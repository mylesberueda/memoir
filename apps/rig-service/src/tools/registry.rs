use crate::{
    AppContext,
    agents::config::AgentConfig,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    tools::{
        assistant::{CREATE_AGENT_TOOL_NAME, CreateAgentTool, DB_QUERY_TOOL_NAME, DbQueryTool},
        system::{CurrentTimeTool, DOCUMENT_SEARCH_TOOL_NAME, DocumentSearchTool, WebSearchTool},
    },
};
use rig::tool::{Tool as _, ToolDyn};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct ToolRegistry<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
}

impl<EM: EmbeddingModel> Clone for ToolRegistry<EM> {
    fn clone(&self) -> Self {
        Self { ctx: self.ctx.clone() }
    }
}

impl<EM: EmbeddingModel> ToolRegistry<EM> {
    pub(crate) fn new(ctx: Arc<AppContext<EM>>) -> Self {
        Self { ctx }
    }

    pub(crate) async fn get_tool(
        &self,
        tool_name: &str,
        tool_id: i64,
        user_id: String,
        organization_pid: Option<String>,
        conversation_id: Option<i64>,
        _config: &AgentConfig,
    ) -> Result<Box<dyn ToolDyn>, ToolRegistryError> {
        match tool_name {
            CurrentTimeTool::NAME => Ok(Box::new(CurrentTimeTool::new())),
            WebSearchTool::NAME => Ok(Box::new(WebSearchTool::new(&self.ctx, tool_id).await?)),
            DB_QUERY_TOOL_NAME => Ok(Box::new(DbQueryTool::new(self.ctx.clone(), user_id, organization_pid))),
            CREATE_AGENT_TOOL_NAME => Ok(Box::new(CreateAgentTool::new(
                self.ctx.clone(),
                user_id,
                organization_pid,
            ))),
            DOCUMENT_SEARCH_TOOL_NAME => Ok(Box::new(DocumentSearchTool::new(
                self.ctx.clone(),
                user_id,
                organization_pid,
                conversation_id,
            ))),
            _ => Err(ToolRegistryError::UnknownTool(tool_name.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ToolRegistryError {
    #[error("unknown tool: {0}")]
    UnknownTool(String),
    #[error("db error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("missing secret")]
    MissingSecret,
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::clients::{QdrantClient, StorageClient};
    use crate::test_utils::{TestContext, init_test_crypto};
    use crate::{
        agents::config::AgentConfig,
        api::embedding::{BoxFuture, EmbeddingError, EmbeddingModel},
    };
    use serial_test::serial;
    use test_context::test_context;

    struct MockEmbeddingModel;

    impl EmbeddingModel for MockEmbeddingModel {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.0; 384]) })
        }
        fn dimensions(&self) -> usize {
            384
        }
    }

    fn mock_app_ctx(ctx: &TestContext) -> Arc<AppContext<MockEmbeddingModel>> {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".into());
        let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("test qdrant client");

        let s3_config = aws_sdk_s3::Config::builder()
            .endpoint_url("http://localhost:9000")
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new("test", "test", None, None, "test"))
            .force_path_style(true)
            .behavior_version_latest()
            .build();

        Arc::new(AppContext {
            db: (*ctx.db).clone(),
            redis: ctx.redis.clone(),
            qdrant: QdrantClient::new(qdrant_inner),
            embedding: Arc::new(MockEmbeddingModel),
            storage: StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into()),
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        })
    }

    mod get_tool {
        use super::*;

        fn default_config() -> AgentConfig {
            AgentConfig::builder().build()
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_current_time_tool_by_name(ctx: &mut TestContext) {
            init_test_crypto();
            let registry = ToolRegistry::new(mock_app_ctx(ctx));
            let config = default_config();

            let result = registry
                .get_tool(CurrentTimeTool::NAME, 0, "user_test".into(), None, None, &config)
                .await;

            assert!(result.is_ok(), "should return CurrentTimeTool: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_db_tool_by_name(ctx: &mut TestContext) {
            init_test_crypto();
            let registry = ToolRegistry::new(mock_app_ctx(ctx));
            let config = default_config();

            let result = registry
                .get_tool(DB_QUERY_TOOL_NAME, 0, "user_test".into(), None, None, &config)
                .await;

            assert!(result.is_ok(), "should return DbTool: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_unknown_tool_name(ctx: &mut TestContext) {
            init_test_crypto();
            let registry = ToolRegistry::new(mock_app_ctx(ctx));
            let config = default_config();

            let result = registry
                .get_tool("nonexistent_tool", 0, "user_test".into(), None, None, &config)
                .await;

            match result {
                Err(ToolRegistryError::UnknownTool(name)) => {
                    assert_eq!(name, "nonexistent_tool");
                }
                Err(other) => panic!("expected UnknownTool error, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_web_search_tool_when_tool_and_secret_exist(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = ctx.create_system_tool("web-search", WebSearchTool::NAME).await;
            ctx.create_tool_api_key_secret("web-search", tool.id).await;
            let config = default_config();

            let registry = ToolRegistry::new(mock_app_ctx(ctx));

            let result = registry
                .get_tool(WebSearchTool::NAME, tool.id, "user_test".into(), None, None, &config)
                .await;

            assert!(result.is_ok(), "should return WebSearchTool: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_tool_record_not_found(ctx: &mut TestContext) {
            init_test_crypto();
            let registry = ToolRegistry::new(mock_app_ctx(ctx));
            let config = default_config();

            // Use a tool_id that doesn't exist
            let result = registry
                .get_tool(WebSearchTool::NAME, 999999, "user_test".into(), None, None, &config)
                .await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected
                Err(other) => panic!("expected MissingSecret error, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_no_secret_linked_to_tool(ctx: &mut TestContext) {
            init_test_crypto();
            // Create tool without linking any secret
            let tool = ctx.create_system_tool("no-secret", WebSearchTool::NAME).await;
            let config = default_config();

            let registry = ToolRegistry::new(mock_app_ctx(ctx));

            let result = registry
                .get_tool(WebSearchTool::NAME, tool.id, "user_test".into(), None, None, &config)
                .await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected
                Err(other) => panic!("expected MissingSecret error, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_secret_decryption_fails(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = ctx.create_system_tool("bad-secret", WebSearchTool::NAME).await;
            ctx.create_malformed_tool_secret("bad-secret", tool.id).await;
            let config = default_config();

            let registry = ToolRegistry::new(mock_app_ctx(ctx));

            let result = registry
                .get_tool(WebSearchTool::NAME, tool.id, "user_test".into(), None, None, &config)
                .await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected (decryption failure maps to MissingSecret)
                Err(other) => panic!("expected MissingSecret error, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_create_agent_tool_by_name(ctx: &mut TestContext) {
            init_test_crypto();
            let registry = ToolRegistry::new(mock_app_ctx(ctx));
            let config = default_config();

            let result = registry
                .get_tool(CREATE_AGENT_TOOL_NAME, 0, "user_test".into(), None, None, &config)
                .await;

            assert!(result.is_ok(), "should return CreateAgentTool: {:?}", result.err());
        }
    }
}
