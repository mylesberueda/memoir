use crate::{
    AppContext,
    agents::{
        components::{AgentComponent, DocumentsComponent, EpisodicMemoryComponent},
        config::AgentConfig,
        runtime::SessionContext,
    },
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    api::store::EmbeddingStore,
};
use std::sync::Arc;

#[allow(dead_code, reason = "Used once wired into InferenceActor::load")]
pub(crate) struct ComponentRegistry<S, EM = DefaultEmbedding>
where
    S: EmbeddingStore,
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
    store: Arc<S>,
}

impl<S: EmbeddingStore, EM: EmbeddingModel> Clone for ComponentRegistry<S, EM> {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            store: self.store.clone(),
        }
    }
}

#[allow(dead_code, reason = "Used once wired into InferenceActor::load")]
impl<S, EM> ComponentRegistry<S, EM>
where
    S: EmbeddingStore,
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>, store: Arc<S>) -> Self {
        Self { ctx, store }
    }

    /// Build all components for a given agent config and session.
    /// Returns a vec of boxed components ready to pass to `BaseAgent::from_loaded`.
    pub(crate) fn build_for_agent(
        &self,
        config: &AgentConfig,
        session: &SessionContext,
    ) -> Vec<Box<dyn AgentComponent>> {
        let mut components: Vec<Box<dyn AgentComponent>> = Vec::new();

        components.push(Box::new(DocumentsComponent::new(
            self.ctx.qdrant.clone(),
            self.ctx.embedding.clone(),
            session.conversation_pid.clone(),
            config.document_result_count as usize,
        )));

        if config.memory_enabled {
            components.push(Box::new(EpisodicMemoryComponent::new(
                self.store.clone(),
                self.ctx.embedding.clone(),
                session.user_id.clone(),
                session.agent_id,
                session.conversation_id,
                config.memory_result_count,
                config.memory_similarity_threshold,
            )));
        }

        components
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        clients::{QdrantClient, StorageClient},
        test_utils::{MockEmbeddingModel, MockStore, TestContext},
    };
    use serial_test::serial;
    use test_context::test_context;

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

    fn default_session() -> SessionContext {
        SessionContext {
            user_id: "user_123".into(),
            agent_id: 1,
            conversation_id: 100,
            conversation_pid: "conv_abc".into(),
        }
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_documents_component_always(ctx: &mut TestContext) {
        let app_ctx = mock_app_ctx(ctx);
        let registry = ComponentRegistry::new(app_ctx, Arc::new(MockStore));
        let config = AgentConfig::builder().memory_enabled(false).build();
        let session = default_session();

        let components = registry.build_for_agent(&config, &session);

        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name(), "documents");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_episodic_memory_when_memory_enabled(ctx: &mut TestContext) {
        let app_ctx = mock_app_ctx(ctx);
        let registry = ComponentRegistry::new(app_ctx, Arc::new(MockStore));
        let config = AgentConfig::builder().memory_enabled(true).build();
        let session = default_session();

        let components = registry.build_for_agent(&config, &session);

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].name(), "documents");
        assert_eq!(components[1].name(), "episodic_memory");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_return_episodic_memory_when_memory_disabled(ctx: &mut TestContext) {
        let app_ctx = mock_app_ctx(ctx);
        let registry = ComponentRegistry::new(app_ctx, Arc::new(MockStore));
        let config = AgentConfig::builder().memory_enabled(false).build();
        let session = default_session();

        let components = registry.build_for_agent(&config, &session);

        let has_memory = components.iter().any(|c| c.name() == "episodic_memory");
        assert!(!has_memory);
    }
}
