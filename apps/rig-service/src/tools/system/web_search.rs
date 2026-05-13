use super::*;
use crate::{
    AppContext,
    api::embedding::EmbeddingModel,
    models::{secrets, tools},
};
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};

#[derive(Debug)]
pub(crate) struct WebSearchTool {
    #[allow(dead_code)]
    api_key: String,
}

impl WebSearchTool {
    pub(crate) async fn new<EM>(ctx: &AppContext<EM>, tool_id: i64) -> Result<Self, ToolRegistryError>
    where
        EM: EmbeddingModel,
    {
        let (_tool, secret) = tools::Entity::find()
            .filter(tools::Column::Id.eq(tool_id))
            .find_also_related(secrets::Entity)
            .one(&ctx.db)
            .await
            .map_err(|e| {
                tracing::error!("failed to fetch secret");
                ToolRegistryError::DbErr(e)
            })?
            .ok_or(ToolRegistryError::MissingSecret)?;

        let secret = secret
            .ok_or_else(|| {
                tracing::error!("missing secret");
                ToolRegistryError::MissingSecret
            })?
            .decrypt()
            .map_err(|e| {
                tracing::error!(error = %e, "failed to decrypt secret");
                ToolRegistryError::MissingSecret
            })?;

        Ok(Self {
            api_key: secret.expose().to_string(),
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub(crate) struct WebSearchToolArgs {}

impl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";
    type Error = WebSearchToolError;
    type Args = WebSearchToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Use Firecrawl to perform a web search".to_string(),
            parameters: serde_json::to_value(schema_for!(WebSearchToolArgs))
                .expect("schema serialization should not fail"),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum WebSearchToolError {
    #[allow(dead_code)]
    #[error("invalid timezone: {0}")]
    InvalidTimezone(String),
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        api::embedding::{BoxFuture, EmbeddingError, EmbeddingModel},
        clients::{QdrantClient, StorageClient},
        test_utils::{TestContext, init_test_crypto},
    };
    use serial_test::serial;
    use std::sync::Arc;
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

    mod new {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_tool_when_tool_and_secret_exist(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = ctx.create_system_tool("ws-create", WebSearchTool::NAME).await;
            ctx.create_tool_api_key_secret("ws-create", tool.id).await;

            let app_ctx = mock_app_ctx(ctx);
            let result = WebSearchTool::new(&app_ctx, tool.id).await;

            assert!(result.is_ok(), "should create WebSearchTool: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_tool_not_found(ctx: &mut TestContext) {
            init_test_crypto();

            let app_ctx = mock_app_ctx(ctx);
            let result = WebSearchTool::new(&app_ctx, 999999).await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected
                Err(other) => panic!("expected MissingSecret, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_no_secret_linked(ctx: &mut TestContext) {
            init_test_crypto();
            // Create tool but don't link a secret
            let tool = ctx.create_system_tool("ws-no-secret", WebSearchTool::NAME).await;

            let app_ctx = mock_app_ctx(ctx);
            let result = WebSearchTool::new(&app_ctx, tool.id).await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected
                Err(other) => panic!("expected MissingSecret, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_missing_secret_error_when_decryption_fails(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = ctx.create_system_tool("ws-bad-decrypt", WebSearchTool::NAME).await;
            ctx.create_malformed_tool_secret("ws-bad-decrypt", tool.id).await;

            let app_ctx = mock_app_ctx(ctx);
            let result = WebSearchTool::new(&app_ctx, tool.id).await;

            match result {
                Err(ToolRegistryError::MissingSecret) => {} // expected (decryption failure maps to MissingSecret)
                Err(other) => panic!("expected MissingSecret, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }
    }

    mod definition {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_have_name_web_search(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = ctx.create_system_tool("ws-name", WebSearchTool::NAME).await;
            ctx.create_tool_api_key_secret("ws-name", tool.id).await;

            assert_eq!(WebSearchTool::NAME, "web_search");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_mention_firecrawl_in_description(ctx: &mut TestContext) {
            init_test_crypto();
            let tool_record = ctx.create_system_tool("ws-desc", WebSearchTool::NAME).await;
            ctx.create_tool_api_key_secret("ws-desc", tool_record.id).await;

            let app_ctx = mock_app_ctx(ctx);
            let tool = WebSearchTool::new(&app_ctx, tool_record.id)
                .await
                .expect("should create tool");
            let definition = tool.definition("".to_string()).await;

            assert!(
                definition.description.to_lowercase().contains("firecrawl"),
                "description should mention Firecrawl: {}",
                definition.description
            );
        }
    }
}
