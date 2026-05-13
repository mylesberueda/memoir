use super::{
    ActorError,
    inference::{InferenceActor, RunInference},
};
use crate::{
    AppContext,
    api::{
        embedding::{DefaultEmbedding, EmbeddingModel},
        store::{EmbeddingStore, MessageStore},
    },
    tools::ToolRegistry,
};
use kameo::prelude::*;
use proto_rs::rig::v1;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// How often to check if the session has expired
const TTL_CHECK_INTERVAL: Duration = Duration::from_secs(60);
type AgentPid = String;

pub(crate) struct ChatSessionActor<S, EM = DefaultEmbedding>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    store: Arc<S>,
    ctx: Arc<AppContext<EM>>,
    conversation_id: i64,
    conversation_pid: String,
    conversation_agent_pid: AgentPid,
    organization_pid: String,
    user_id: String,
    tool_registry: ToolRegistry<EM>,
    inference_actors: HashMap<AgentPid, ActorRef<InferenceActor<S, EM>>>,
    last_activity: chrono::DateTime<chrono::Utc>,
    session_ttl: Duration,
    active_request: Option<ActiveRequest>,
}

struct ActiveRequest {
    request_id: String,
    cancel_token: CancellationToken,
}

impl<S, EM> Actor for ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Args = Self;
    type Error = std::convert::Infallible;

    async fn on_start(state: Self::Args, actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        let weak_ref = actor_ref.downgrade();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(TTL_CHECK_INTERVAL);
            loop {
                interval.tick().await;
                match weak_ref.upgrade() {
                    Some(actor) => {
                        if actor.tell(CheckTTL).send().await.is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        });

        Ok(state)
    }
}

impl<S, EM> ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    pub(crate) async fn new(
        store: Arc<S>,
        ctx: Arc<AppContext<EM>>,
        conversation_pid: &str,
        organization_pid: String,
        user_id: String,
        tool_registry: ToolRegistry<EM>,
    ) -> Result<Self, ActorError> {
        let loaded =
            InferenceActor::<S, EM>::load_conversation_context(&ctx, conversation_pid, &organization_pid, &user_id)
                .await?;

        let conversation_id = loaded.conversation.id;
        let conversation_pid = loaded.conversation.pid;
        let conversation_agent_pid = loaded.agent.pid;
        let session_ttl = loaded.session_ttl;

        tracing::debug!(
            conversation_id,
            session_ttl_secs = session_ttl.as_secs(),
            "chat session created"
        );

        Ok(Self {
            store,
            ctx,
            conversation_id,
            conversation_pid,
            conversation_agent_pid,
            organization_pid,
            user_id,
            tool_registry,
            inference_actors: HashMap::new(),
            last_activity: chrono::Utc::now(),
            session_ttl,
            active_request: None,
        })
    }

    fn touch(&mut self) {
        self.last_activity = chrono::Utc::now();
    }

    async fn get_or_spawn_inference_actor(
        &mut self,
        ctx: &Context<Self, Result<(), ActorError>>,
        agent_pid: &str,
    ) -> Result<ActorRef<InferenceActor<S, EM>>, ActorError> {
        if let Some(actor_ref) = self.inference_actors.get(agent_pid) {
            return Ok(actor_ref.clone());
        }

        let loaded = InferenceActor::load(
            self.store.clone(),
            self.ctx.clone(),
            &self.conversation_pid,
            self.organization_pid.clone(),
            self.user_id.clone(),
            self.tool_registry.clone(),
        )
        .await?;

        let actor_ref = InferenceActor::spawn_link(ctx.actor_ref(), loaded.actor).await;
        self.inference_actors.insert(agent_pid.to_string(), actor_ref.clone());
        Ok(actor_ref)
    }
}

struct ClearActiveRequest {
    request_id: String,
}

impl<S, EM> kameo::message::Message<ClearActiveRequest> for ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = ();

    async fn handle(&mut self, msg: ClearActiveRequest, _ctx: &mut Context<Self, Self::Reply>) {
        if self
            .active_request
            .as_ref()
            .is_some_and(|active| active.request_id == msg.request_id)
        {
            self.active_request = None;
        }
    }
}

/// Internal message to check TTL expiration
struct CheckTTL;

impl<S, EM> kameo::message::Message<CheckTTL> for ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = ();

    async fn handle(&mut self, _msg: CheckTTL, ctx: &mut Context<Self, Self::Reply>) {
        let elapsed = chrono::Utc::now().signed_duration_since(self.last_activity);
        let ttl_chrono = chrono::Duration::from_std(self.session_ttl).unwrap_or(chrono::Duration::seconds(1800));

        if elapsed > ttl_chrono {
            tracing::info!(
                conversation_id = self.conversation_id,
                elapsed_secs = elapsed.num_seconds(),
                ttl_secs = ttl_chrono.num_seconds(),
                "session TTL expired, stopping"
            );
            ctx.stop();
        }
    }
}

pub(crate) struct StreamMessage {
    pub(crate) request_id: String,
    pub(crate) content: String,
    pub(crate) response_tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
    pub(crate) cancel_token: CancellationToken,
    pub(crate) document_pids: Vec<String>,
}

impl<S, EM> kameo::message::Message<StreamMessage> for ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = Result<(), ActorError>;

    async fn handle(&mut self, msg: StreamMessage, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        self.touch();

        tracing::debug!(
            request_id = %msg.request_id,
            conversation_id = self.conversation_id,
            conversation_pid = %self.conversation_pid,
            content_len = msg.content.len(),
            "ChatSessionActor: handling StreamMessage"
        );

        tracing::debug!(
            request_id = %msg.request_id,
            "ChatSessionActor: delegating inference to child actor"
        );

        let agent_pid = self.conversation_agent_pid.clone();
        let inference_actor = self.get_or_spawn_inference_actor(_ctx, &agent_pid).await?;
        self.active_request = Some(ActiveRequest {
            request_id: msg.request_id.clone(),
            cancel_token: msg.cancel_token.clone(),
        });
        let session_ref = _ctx.actor_ref().clone();
        tokio::spawn(async move {
            let request_id = msg.request_id;
            if let Err(error) = inference_actor
                .ask(RunInference {
                    content: msg.content,
                    document_pids: msg.document_pids,
                    response_tx: msg.response_tx,
                    cancel_token: msg.cancel_token,
                })
                .await
            {
                tracing::error!(
                    request_id = %request_id,
                    error = %error,
                    "ChatSessionActor: child inference failed"
                );
            }
            let _ = session_ref
                .tell(ClearActiveRequest {
                    request_id: request_id.clone(),
                })
                .send()
                .await;
        });

        Ok(())
    }
}

/// Message to cancel an active inference request
#[derive(Debug)]
pub(crate) struct CancelInference {
    pub(crate) request_id: String,
}

impl<S, EM> kameo::message::Message<CancelInference> for ChatSessionActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = bool;

    async fn handle(&mut self, msg: CancelInference, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if let Some(active) = &self.active_request
            && active.request_id == msg.request_id
        {
            active.cancel_token.cancel();
            return true;
        }

        false
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        api::{embedding::EmbeddingError, memory::EpisodicMemory, message::Message, store::StoreError},
        test_utils::{TestContext, init_test_crypto},
    };
    use kameo::error::SendError;
    use serial_test::serial;
    use std::sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    };
    use test_context::test_context;

    struct MockEmbeddingModel;

    impl EmbeddingModel for MockEmbeddingModel {
        fn embed(&self, _text: &str) -> crate::api::embedding::BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.0; 384]) })
        }
        fn dimensions(&self) -> usize {
            384
        }
    }

    fn mock_app_ctx(ctx: &TestContext) -> Arc<AppContext<MockEmbeddingModel>> {
        mock_app_ctx_impl(ctx, Arc::new(MockEmbeddingModel))
    }

    fn mock_app_ctx_impl(ctx: &TestContext, embedding: Arc<MockEmbeddingModel>) -> Arc<AppContext<MockEmbeddingModel>> {
        use crate::clients::{QdrantClient, StorageClient};

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
            embedding,
            storage: StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into()),
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        })
    }

    #[derive(Debug)]
    struct MockStore;

    impl MessageStore for MockStore {
        async fn persist(&self, _message: &Message, _conversation_id: i64) -> Result<(), crate::api::StoreError> {
            Ok(())
        }
    }

    impl EmbeddingStore for MockStore {
        async fn update_embedding(&self, _pid: &str, _embedding: Vec<f32>) -> Result<(), StoreError> {
            Ok(())
        }

        async fn retrieve_similar(
            &self,
            _query_embedding: Vec<f32>,
            _user_id: &str,
            _agent_id: i64,
            _exclude_conversation_id: i64,
            _limit: u32,
            _min_similarity: f32,
        ) -> Result<Vec<EpisodicMemory>, StoreError> {
            Ok(vec![])
        }
    }

    #[derive(Debug)]
    struct TrackingStore {
        persisted: Mutex<Vec<Message>>,
        fail_on_call: Option<usize>,
        call_count: AtomicUsize,
    }

    impl TrackingStore {
        fn new() -> Self {
            Self {
                persisted: Mutex::new(Vec::new()),
                fail_on_call: None,
                call_count: AtomicUsize::new(0),
            }
        }

        fn fail_on_call(mut self, n: usize) -> Self {
            self.fail_on_call = Some(n);
            self
        }

        fn get_persisted(&self) -> Vec<Message> {
            self.persisted.lock().unwrap().clone()
        }
    }

    impl MessageStore for TrackingStore {
        async fn persist(&self, message: &Message, _conversation_id: i64) -> Result<(), crate::api::StoreError> {
            let count = self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.fail_on_call == Some(count) {
                return Err(crate::api::StoreError::Internal("configured failure".into()));
            }
            self.persisted.lock().unwrap().push(message.clone());
            Ok(())
        }
    }

    impl EmbeddingStore for TrackingStore {
        async fn update_embedding(&self, _pid: &str, _embedding: Vec<f32>) -> Result<(), StoreError> {
            Ok(())
        }

        async fn retrieve_similar(
            &self,
            _query_embedding: Vec<f32>,
            _user_id: &str,
            _agent_id: i64,
            _exclude_conversation_id: i64,
            _limit: u32,
            _min_similarity: f32,
        ) -> Result<Vec<EpisodicMemory>, StoreError> {
            Ok(vec![])
        }
    }

    mod new {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_load_conversation_by_pid(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("load-conv").await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(app_ctx.clone());
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                tool_registry,
            )
            .await;

            assert!(result.is_ok(), "should successfully create actor: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_conversation_not_found(ctx: &mut TestContext) {
            init_test_crypto();
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                "nonexistent-conversation-pid",
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::ConversationLoadFailed(_)) => {}
                Err(other) => panic!("expected ConversationLoadFailed, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_api_secret_not_found_for_openai(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_provider("no-secret-openai", "openai").await;
            let model = ctx.create_openai_model("no-secret-openai", provider.id).await;
            let agent = ctx.create_agent("no-secret-openai", model.id).await;
            let conversation = ctx.create_conversation("no-secret-openai", agent.id).await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create chat session actor before worker load");
            let actor_ref = ChatSessionActor::spawn(actor);
            let (tx, _rx) = mpsc::channel(32);

            let result = actor_ref
                .ask(StreamMessage {
                    request_id: "req-missing-api-key".to_string(),
                    content: "Hello".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await;

            match result {
                Err(SendError::HandlerError(ActorError::Agent(_))) => {}
                Err(SendError::HandlerError(other)) => {
                    panic!("expected Agent error for missing API key, got {:?}", other)
                }
                Err(other) => panic!("expected HandlerError, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_succeed_when_ollama_has_no_api_secret(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_provider("no-secret-ollama", "ollama").await;
            let model = ctx.create_model("no-secret-ollama", provider.id).await;
            let agent = ctx.create_agent("no-secret-ollama", model.id).await;
            let conversation = ctx.create_conversation("no-secret-ollama", agent.id).await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "Ollama should succeed without API key: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_fail_when_secret_decryption_fails(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_provider("bad-decrypt", "ollama").await;
            ctx.create_malformed_secret("bad-decrypt", provider.id).await;
            let model = ctx.create_model("bad-decrypt", provider.id).await;
            let agent = ctx.create_agent("bad-decrypt", model.id).await;
            let conversation = ctx.create_conversation("bad-decrypt", agent.id).await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create chat session actor before worker load");
            let actor_ref = ChatSessionActor::spawn(actor);
            let (tx, _rx) = mpsc::channel(32);

            let result = actor_ref
                .ask(StreamMessage {
                    request_id: "req-bad-secret".to_string(),
                    content: "Hello".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await;

            match result {
                Err(SendError::HandlerError(ActorError::CryptoError(_))) => {}
                Err(SendError::HandlerError(other)) => {
                    panic!("expected CryptoError for malformed secret, got {:?}", other)
                }
                Err(other) => panic!("expected HandlerError, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_load_history_when_messages_exist(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("with-history").await;

            ctx.create_message("hist-1", conversation.id, "user", "Hello").await;
            ctx.create_message("hist-2", conversation.id, "assistant", "Hi there!")
                .await;
            ctx.create_message("hist-3", conversation.id, "user", "How are you?")
                .await;

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should successfully create actor with history: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_succeed_when_history_exceeds_configured_limit(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("large-history").await;

            for i in 0..15 {
                let role = if i % 2 == 0 { "user" } else { "assistant" };
                ctx.create_message(&format!("msg-{i}"), conversation.id, role, &format!("Message {i}"))
                    .await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should successfully create actor despite large history: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_user_with_system_provider_in_personal_org(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("auth-personal", "ollama").await;
            ctx.create_api_key_secret("auth-personal", provider.id).await;
            let model = ctx.create_model("auth-personal", provider.id).await;
            let agent = ctx.create_personal_agent("auth-personal", model.id, "user_test").await;
            let conversation = ctx
                .create_conversation_for_user("auth-personal", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "personal_org_user_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should allow personal user with personal provider: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_personal_user_with_other_users_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("auth-other-user", "ollama", "other_user")
                .await;
            ctx.create_api_key_secret("auth-other-user", provider.id).await;
            let model = ctx.create_model("auth-other-user", provider.id).await;
            let agent = ctx
                .create_personal_agent("auth-other-user", model.id, "user_test")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-other-user", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "personal_org_user_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::Unauthorized(msg)) => {
                    assert!(msg.contains("provider"), "error should mention provider: {msg}");
                }
                Err(other) => panic!("expected Unauthorized for provider, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_personal_user_with_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("auth-system", "ollama").await;
            ctx.create_api_key_secret("auth-system", provider.id).await;
            let model = ctx.create_model("auth-system", provider.id).await;
            let agent = ctx.create_personal_agent("auth-system", model.id, "user_test").await;
            let conversation = ctx
                .create_conversation_for_user("auth-system", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "personal_org_user_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should allow personal user with system provider: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_when_provider_in_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            // Provider in org_abc, but agent + request in org_test
            let provider = ctx
                .create_org_provider("auth-org-deny", "ollama", "org_abc", "user_test")
                .await;
            ctx.create_api_key_secret("auth-org-deny", provider.id).await;
            let model = ctx.create_model("auth-org-deny", provider.id).await;
            let agent = ctx.create_agent("auth-org-deny", model.id).await;
            let conversation = ctx
                .create_conversation_for_user("auth-org-deny", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::Unauthorized(msg)) => {
                    assert!(msg.contains("provider"), "error should mention provider: {msg}");
                }
                Err(other) => panic!("expected Unauthorized for provider, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_org_user_with_org_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_org_provider("auth-org-allow", "ollama", "org_abc", "user_test")
                .await;
            ctx.create_api_key_secret("auth-org-allow", provider.id).await;
            let model = ctx.create_model("auth-org-allow", provider.id).await;
            let agent = ctx
                .create_org_agent("auth-org-allow", model.id, "org_abc", "user_test")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-org-allow", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_abc".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should allow org user with org provider: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_org_user_with_different_org_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_org_provider("auth-diff-org", "ollama", "org_xyz", "user_test")
                .await;
            ctx.create_api_key_secret("auth-diff-org", provider.id).await;
            let model = ctx.create_model("auth-diff-org", provider.id).await;
            let agent = ctx
                .create_org_agent("auth-diff-org", model.id, "org_abc", "user_test")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-diff-org", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_abc".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::Unauthorized(msg)) => {
                    assert!(msg.contains("provider"), "error should mention provider: {msg}");
                }
                Err(other) => panic!("expected Unauthorized for provider, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_org_user_with_system_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("auth-org-sys", "ollama").await;
            ctx.create_api_key_secret("auth-org-sys", provider.id).await;
            let model = ctx.create_model("auth-org-sys", provider.id).await;
            let agent = ctx
                .create_org_agent("auth-org-sys", model.id, "org_abc", "user_test")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-org-sys", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_abc".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should allow org user with system provider: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_org_user_with_personal_provider(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx
                .create_personal_provider("auth-org-personal", "ollama", "user_test")
                .await;
            ctx.create_api_key_secret("auth-org-personal", provider.id).await;
            let model = ctx.create_model("auth-org-personal", provider.id).await;
            let agent = ctx
                .create_org_agent("auth-org-personal", model.id, "org_abc", "user_test")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-org-personal", agent.id, "user_test")
                .await;
            let store = Arc::new(MockStore);

            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_abc".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::Unauthorized(msg)) => {
                    assert!(msg.contains("provider"), "error should mention provider: {msg}");
                }
                Err(other) => panic!("expected Unauthorized for provider, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_other_users_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("auth-other-conv", "ollama").await;
            ctx.create_api_key_secret("auth-other-conv", provider.id).await;
            let model = ctx.create_model("auth-other-conv", provider.id).await;
            let agent = ctx
                .create_personal_agent("auth-other-conv", model.id, "other_user")
                .await;
            let conversation = ctx
                .create_conversation_for_user("auth-other-conv", agent.id, "other_user")
                .await;
            let store = Arc::new(MockStore);

            // Use the agent's org so the agent access check passes,
            // but user_test != other_user so the conversation ownership check fails
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "personal_org_other_user".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            match result {
                Err(ActorError::Unauthorized(msg)) => {
                    assert!(msg.contains("conversation"), "error should mention conversation: {msg}");
                }
                Err(other) => panic!("expected Unauthorized for conversation, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }
    }

    mod tools {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_load_active_tools_for_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("tools-active").await;

            let tool = ctx.create_system_tool("tools-active", "current_time").await;
            ctx.link_tool_to_agent(agent.id, tool.id).await;

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should create actor with active tool: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_inactive_tools_when_loading_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("tools-inactive").await;

            let inactive_tool = ctx.create_inactive_tool("tools-inactive", "some_tool").await;
            ctx.link_tool_to_agent(agent.id, inactive_tool.id).await;

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should create actor, skipping inactive tools: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_session_with_empty_toolset_when_agent_has_no_tools(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("tools-none").await;

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(result.is_ok(), "should create actor with no tools: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_warn_and_continue_when_single_tool_fails_to_load(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("tools-fail").await;

            let tool_without_secret = ctx.create_system_tool("tools-fail", "web_search").await;
            ctx.link_tool_to_agent(agent.id, tool_without_secret.id).await;

            let working_tool = ctx.create_system_tool("tools-working", "current_time").await;
            ctx.link_tool_to_agent(agent.id, working_tool.id).await;

            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);
            let result = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "should create actor, skipping failed tool: {:?}",
                result.err()
            );
        }
    }

    mod stream_message {
        use crate::api::message::{MessageRole, MessageStatus};

        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_persist_user_message_before_streaming(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("persist-user").await;
            let store = Arc::new(TrackingStore::new());

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store.clone(),
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create actor");
            let actor_ref = ChatSessionActor::spawn(actor);

            let (tx, mut rx) = mpsc::channel(32);
            actor_ref
                .ask(StreamMessage {
                    request_id: "req-test-1".to_string(),
                    content: "Hello, assistant!".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await
                .expect("should send message");

            while rx.recv().await.is_some() {}

            let persisted = store.get_persisted();
            assert!(!persisted.is_empty(), "should persist at least user message");
            assert_eq!(
                persisted[0].role(),
                MessageRole::User,
                "first persisted message should be user"
            );
            assert_eq!(
                persisted[0].text_content(),
                "Hello, assistant!",
                "user message content should match"
            );
            assert!(!persisted[0].pid().is_empty(), "user message pid should be set");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_persist_assistant_message_after_streaming(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("persist-assistant").await;
            let store = Arc::new(TrackingStore::new());

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store.clone(),
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create actor");
            let actor_ref = ChatSessionActor::spawn(actor);

            let (tx, mut rx) = mpsc::channel(32);
            actor_ref
                .ask(StreamMessage {
                    request_id: "req-test-2".to_string(),
                    content: "Say hello".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await
                .expect("should send message");

            while rx.recv().await.is_some() {}

            let persisted = store.get_persisted();
            assert_eq!(persisted.len(), 2, "should persist both user and assistant messages");
            assert_eq!(persisted[0].role(), MessageRole::User, "first message should be user");
            assert_eq!(
                persisted[1].role(),
                MessageRole::Assistant,
                "second message should be assistant"
            );
            assert!(
                !persisted[1].text_content().is_empty() || !persisted[1].parts().is_empty(),
                "assistant message should have content or parts"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_persist_messages_when_user_persistence_fails(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("fail-user-persist").await;
            let store = Arc::new(TrackingStore::new().fail_on_call(0));

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store.clone(),
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create actor");
            let actor_ref = ChatSessionActor::spawn(actor);

            let (tx, mut rx) = mpsc::channel(32);
            actor_ref
                .ask(StreamMessage {
                    request_id: "req-test-3".to_string(),
                    content: "Hello".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await
                .expect("should accept request");

            while rx.recv().await.is_some() {}

            let persisted = store.get_persisted();
            assert!(persisted.is_empty(), "no messages should be persisted on failure");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_only_persist_user_message_when_assistant_persistence_fails(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("fail-assistant-persist").await;
            let store = Arc::new(TrackingStore::new().fail_on_call(1));

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store.clone(),
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create actor");
            let actor_ref = ChatSessionActor::spawn(actor);

            let (tx, mut rx) = mpsc::channel(32);
            actor_ref
                .ask(StreamMessage {
                    request_id: "req-test-4".to_string(),
                    content: "Hello".to_string(),
                    response_tx: tx,
                    cancel_token: CancellationToken::new(),
                    document_pids: vec![],
                })
                .await
                .expect("should accept request");

            while rx.recv().await.is_some() {}

            let persisted = store.get_persisted();
            assert_eq!(persisted.len(), 1, "only user message should be persisted");
            assert_eq!(
                persisted[0].role(),
                MessageRole::User,
                "persisted message should be user"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_persist_a_cancelled_partial_assistant_message_when_inference_is_cancelled(
            ctx: &mut TestContext,
        ) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("cancelled-partial-persist").await;
            let store = Arc::new(TrackingStore::new());

            let app_ctx = mock_app_ctx(ctx);
            let actor = ChatSessionActor::new(
                store.clone(),
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx.clone()),
            )
            .await
            .expect("should create actor");
            let actor_ref = ChatSessionActor::spawn(actor);

            let cancel_token = CancellationToken::new();
            cancel_token.cancel();

            let (tx, mut rx) = mpsc::channel(32);
            actor_ref
                .ask(StreamMessage {
                    request_id: "req-test-cancelled".to_string(),
                    content: "Write a long response about distributed systems and cancellation handling.".to_string(),
                    response_tx: tx,
                    cancel_token,
                    document_pids: vec![],
                })
                .await
                .expect("should send message");

            while rx.recv().await.is_some() {}

            let persisted = store.get_persisted();
            assert_eq!(
                persisted.len(),
                2,
                "should persist user and cancelled assistant messages"
            );
            assert_eq!(persisted[0].role(), MessageRole::User, "first message should be user");
            assert_eq!(
                persisted[1].role(),
                MessageRole::Assistant,
                "second message should be assistant"
            );
            assert_eq!(
                persisted[1].status(),
                MessageStatus::Cancelled,
                "assistant message should be persisted with cancelled status"
            );
        }
    }

    mod ttl {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_stop_when_ttl_has_expired(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("session-ttl-expired").await;
            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);

            let mut actor = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx),
            )
            .await
            .expect("should create actor");

            actor.session_ttl = std::time::Duration::from_secs(1);
            actor.last_activity = chrono::Utc::now() - chrono::Duration::seconds(5);

            let actor_ref = ChatSessionActor::spawn(actor);
            actor_ref.tell(CheckTTL).send().await.expect("should deliver ttl check");

            tokio::time::timeout(std::time::Duration::from_secs(1), actor_ref.wait_for_shutdown())
                .await
                .expect("actor should stop after ttl expiration");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_remain_alive_when_ttl_has_not_expired(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, _, conversation) = ctx.create_full_setup("session-ttl-active").await;
            let store = Arc::new(MockStore);
            let app_ctx = mock_app_ctx(ctx);

            let mut actor = ChatSessionActor::new(
                store,
                app_ctx.clone(),
                &conversation.pid,
                "org_test".into(),
                "user_test".into(),
                ToolRegistry::new(app_ctx),
            )
            .await
            .expect("should create actor");

            actor.session_ttl = std::time::Duration::from_secs(30);
            actor.last_activity = chrono::Utc::now();

            let actor_ref = ChatSessionActor::spawn(actor);
            actor_ref.tell(CheckTTL).send().await.expect("should deliver ttl check");

            let cancelled = actor_ref
                .ask(CancelInference {
                    request_id: "unknown-request".into(),
                })
                .await
                .expect("actor should still be alive");

            assert!(!cancelled, "should report no active request to cancel");

            actor_ref.stop_gracefully().await.expect("should stop actor cleanly");
            actor_ref.wait_for_shutdown().await;
        }
    }
}
