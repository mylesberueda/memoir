use crate::{
    AppContext,
    actors::{ActorError, ChatSessionActor},
    api::{
        embedding::{DefaultEmbedding, EmbeddingModel},
        store::{EmbeddingStore, MessageStore},
    },
    models::{agents, conversations},
    tools::ToolRegistry,
};
use kameo::prelude::*;
use proto_rs::rig::v1;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, EntityTrait as _, QueryFilter as _, TryIntoModel,
};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};

type ConversationPid = String;
type RequestId = String;

pub(crate) struct SessionRegistryActor<S, EM = DefaultEmbedding>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    store: Arc<S>,
    ctx: Arc<AppContext<EM>>,
    sessions: HashMap<ConversationPid, ActorRef<ChatSessionActor<S, EM>>>,
    active_requests: HashMap<RequestId, ConversationPid>,
    tool_registry: ToolRegistry<EM>,
}

impl<S, EM> Actor for SessionRegistryActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Args = (Arc<S>, Arc<AppContext<EM>>, ToolRegistry<EM>);
    type Error = std::convert::Infallible;

    async fn on_start(args: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        Ok(Self {
            store: args.0,
            ctx: args.1,
            sessions: HashMap::new(),
            active_requests: HashMap::new(),
            tool_registry: args.2,
        })
    }

    /// Called when a linked actor (session) dies for any reason
    async fn on_link_died(
        &mut self,
        _actor_ref: WeakActorRef<Self>,
        id: ActorId,
        reason: ActorStopReason,
    ) -> Result<ControlFlow<ActorStopReason>, Self::Error> {
        // Find and remove the conversation_pid for this actor
        let mut dead_conversation_pid = None;
        self.sessions.retain(|conversation_pid, actor_ref| {
            if actor_ref.id() == id {
                tracing::debug!(
                    conversation_pid = %conversation_pid,
                    actor_id = %id,
                    reason = ?reason,
                    "session removed from registry (link died)"
                );
                dead_conversation_pid = Some(conversation_pid.clone());
                false
            } else {
                true
            }
        });

        // Also clean up any active requests for this conversation
        if let Some(ref dead_pid) = dead_conversation_pid {
            self.active_requests.retain(|_, conv_pid| conv_pid != dead_pid);
        }

        Ok(ControlFlow::Continue(()))
    }
}

#[derive(Debug)]
pub(crate) struct GetOrCreateSession {
    pub(crate) request: v1::InferRequest,
    pub(crate) organization_pid: String,
    pub(crate) user_id: String,
}

impl<S, EM> Message<GetOrCreateSession> for SessionRegistryActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = Result<(ActorRef<ChatSessionActor<S, EM>>, String), ActorError>;

    async fn handle(&mut self, msg: GetOrCreateSession, ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let conversation_pid = match msg.request.conversation_pid {
            Some(pid) => {
                if let Some(actor_ref) = self.sessions.get(&pid) {
                    if !msg.request.request_id.is_empty() {
                        tracing::debug!(
                            request_id = %msg.request.request_id,
                            conversation_pid = %pid,
                            "GetOrCreateSession: registering active request (existing session)"
                        );
                        self.active_requests.insert(msg.request.request_id.clone(), pid.clone());
                    }
                    return Ok((actor_ref.clone(), pid));
                }
                pid
            }
            None => {
                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(&msg.request.agent_pid))
                    .one(&self.ctx.db)
                    .await
                    .map_err(ActorError::DbError)?
                    .ok_or_else(|| {
                        tracing::error!(agent_pid = &msg.request.agent_pid, "missing agent");
                        ActorError::ConversationLoadFailed("Failed to get agent".into())
                    })?;

                let model = conversations::ActiveModel {
                    user_id: Set(msg.user_id.clone()),
                    organization_pid: Set(msg.organization_pid.clone()),
                    agent_id: Set(agent.id),
                    title: Set(Some(format!("Chat with {}", agent.name))),
                    ..Default::default()
                };

                let model = model
                    .save(&self.ctx.db)
                    .await
                    .map_err(ActorError::DbError)?
                    .try_into_model()
                    .map_err(ActorError::DbError)?;

                model.pid
            }
        };

        let actor = ChatSessionActor::new(
            self.store.clone(),
            self.ctx.clone(),
            &conversation_pid,
            msg.organization_pid,
            msg.user_id,
            self.tool_registry.clone(),
        )
        .await?;
        let actor_ref = ChatSessionActor::spawn_link(ctx.actor_ref(), actor).await;

        self.sessions.insert(conversation_pid.clone(), actor_ref.clone());

        // Register active request for cancellation lookup by request_id
        if !msg.request.request_id.is_empty() {
            tracing::debug!(
                request_id = %msg.request.request_id,
                conversation_pid = %conversation_pid,
                "GetOrCreateSession: registering active request"
            );
            self.active_requests
                .insert(msg.request.request_id.clone(), conversation_pid.clone());
        }

        Ok((actor_ref, conversation_pid))
    }
}

/// Get an existing session by conversation_pid or request_id (for cancellation)
#[derive(Debug)]
pub(crate) struct GetSession {
    pub(crate) conversation_pid: String,
    pub(crate) request_id: Option<String>,
}

impl<S, EM> Message<GetSession> for SessionRegistryActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = Option<ActorRef<ChatSessionActor<S, EM>>>;

    async fn handle(&mut self, msg: GetSession, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        if !msg.conversation_pid.is_empty()
            && let Some(session) = self.sessions.get(&msg.conversation_pid)
        {
            return Some(session.clone());
        }

        if let Some(ref request_id) = msg.request_id
            && let Some(conversation_pid) = self.active_requests.get(request_id)
        {
            tracing::debug!(
                request_id = %request_id,
                conversation_pid = %conversation_pid,
                "GetSession: found session via request_id lookup"
            );
            return self.sessions.get(conversation_pid).cloned();
        }

        None
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        AppContext,
        api::{embedding::EmbeddingError, memory::EpisodicMemory, message::Message, store::StoreError},
        clients::{QdrantClient, StorageClient},
        test_utils::{TestContext, init_test_crypto},
    };
    use kameo::error::SendError;
    use serial_test::serial;
    use test_context::test_context;

    /// Mock store for testing registry without real persistence
    #[derive(Debug, Clone)]
    struct MockStore;

    impl MessageStore for MockStore {
        async fn persist(&self, _message: &Message, _conversation_id: i64) -> Result<(), StoreError> {
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

    struct MockEmbeddingModel;

    impl EmbeddingModel for MockEmbeddingModel {
        fn embed(&self, _text: &str) -> crate::api::embedding::BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.0; 384]) })
        }

        fn dimensions(&self) -> usize {
            384
        }
    }

    /// Build a mock AppContext for tests — uses dummy qdrant/storage clients that won't be called.
    fn mock_app_ctx(ctx: &TestContext) -> Arc<AppContext<MockEmbeddingModel>> {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".into());
        let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("test qdrant client");
        let qdrant = QdrantClient::new(qdrant_inner);

        // Storage won't be used in session registry tests, but AppContext requires it.
        let s3_config = aws_sdk_s3::Config::builder()
            .endpoint_url("http://localhost:9000")
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new("test", "test", None, None, "test"))
            .force_path_style(true)
            .behavior_version_latest()
            .build();

        let storage = StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into());

        Arc::new(AppContext {
            db: (*ctx.db).clone(),
            redis: ctx.redis.clone(),
            qdrant,
            embedding: Arc::new(MockEmbeddingModel),
            storage,
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        })
    }

    mod get_or_create_session {
        use super::*;

        fn make_request(agent_pid: &str, conversation_pid: Option<String>) -> v1::InferRequest {
            v1::InferRequest {
                agent_pid: agent_pid.to_string(),
                conversation_pid,
                message: "test message".to_string(),
                request_id: format!("req-{}", nanoid::nanoid!()),
                document_pids: vec![],
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_new_session_for_existing_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("new-session").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref = SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            let result = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some(conversation.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await;

            assert!(result.is_ok(), "should create session: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_conversation_when_pid_not_provided(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("auto-create-conv").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx);
            let registry_ref = SessionRegistryActor::spawn((store, mock_app_ctx(ctx), tool_registry));

            // Request without conversation_pid - should create one
            let result = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, None),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await;

            assert!(
                result.is_ok(),
                "should create session with new conversation: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_cached_session_for_known_conversation(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("cached-session").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref = SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            // First request - creates session
            let (first_ref, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some(conversation.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create session");

            // Second request - should return cached session
            let (second_ref, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some(conversation.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should return cached session");

            // Both refs should point to the same actor (same ID)
            assert_eq!(first_ref.id(), second_ref.id(), "should return same actor ref");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_conversation_not_found(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, _) = ctx.create_full_setup("conv-not-found").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref = SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            let result = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some("nonexistent-conversation".to_string())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await;

            // Handler returns Err -> kameo wraps it in SendError::HandlerError
            match result {
                Err(SendError::HandlerError(ActorError::ConversationLoadFailed(_))) => {} // expected
                Err(SendError::HandlerError(other)) => {
                    panic!("expected ConversationLoadFailed, got {:?}", other)
                }
                Err(other) => panic!("expected HandlerError, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_agent_not_found(ctx: &mut TestContext) {
            init_test_crypto();

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref = SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            // Request with nonexistent agent and no conversation
            let result = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request("nonexistent-agent", None),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await;

            match result {
                Err(SendError::HandlerError(ActorError::ConversationLoadFailed(_))) => {} // expected
                Err(SendError::HandlerError(other)) => {
                    panic!("expected ConversationLoadFailed for missing agent, got {:?}", other)
                }
                Err(other) => panic!("expected HandlerError, got {:?}", other),
                Ok(_) => panic!("expected error, got Ok"),
            }
        }
    }

    mod on_link_died {
        use super::*;

        fn make_request(agent_pid: &str, conversation_pid: Option<String>) -> v1::InferRequest {
            v1::InferRequest {
                agent_pid: agent_pid.to_string(),
                conversation_pid,
                message: "test message".to_string(),
                request_id: format!("req-{}", nanoid::nanoid!()),
                document_pids: vec![],
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_remove_session_when_it_stops(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("link-died").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref: ActorRef<SessionRegistryActor<MockStore, MockEmbeddingModel>> =
                SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            // First request - creates session
            let (first_ref, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some(conversation.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create session");

            // Stop the session
            first_ref.stop_gracefully().await.expect("should stop");
            first_ref.wait_for_shutdown().await;

            // Wait for link death notification
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Request again - should create NEW session (different ID)
            let (second_ref, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent.pid, Some(conversation.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create new session after link died");

            assert_ne!(
                first_ref.id(),
                second_ref.id(),
                "should create new actor after previous session stopped"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_keep_other_sessions_when_one_dies(ctx: &mut TestContext) {
            init_test_crypto();

            // Create two separate conversations
            let (_, _, agent_a, conversation_a) = ctx.create_full_setup("multi-session-a").await;
            let (_, _, agent_b, conversation_b) = ctx.create_full_setup("multi-session-b").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref = SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            // Create both sessions
            let (session_a, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent_a.pid, Some(conversation_a.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create session A");

            let (session_b, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent_b.pid, Some(conversation_b.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create session B");

            let session_b_id = session_b.id();

            // Stop session A
            session_a.stop_gracefully().await.expect("should stop A");
            session_a.wait_for_shutdown().await;

            // Wait for link death notification
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Session B should still be cached (same ID returned)
            let (session_b_again, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent_b.pid, Some(conversation_b.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should return cached session B");

            assert_eq!(
                session_b_id,
                session_b_again.id(),
                "session B should still be cached after session A died"
            );

            // Session A should create a new actor
            let (session_a_new, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: make_request(&agent_a.pid, Some(conversation_a.pid.clone())),
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create new session A");

            assert_ne!(session_a.id(), session_a_new.id(), "session A should be recreated");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_resolve_a_session_by_request_id_after_that_session_stops(ctx: &mut TestContext) {
            init_test_crypto();
            let (_, _, agent, conversation) = ctx.create_full_setup("stale-request-lookup").await;

            let store = Arc::new(MockStore);
            let mock_ctx = mock_app_ctx(ctx);
            let tool_registry = ToolRegistry::new(mock_ctx.clone());
            let registry_ref: ActorRef<SessionRegistryActor<MockStore, MockEmbeddingModel>> =
                SessionRegistryActor::spawn((store, mock_ctx, tool_registry));

            let request_id = "req-stale-session".to_string();
            let (session_ref, _) = registry_ref
                .ask(GetOrCreateSession {
                    request: v1::InferRequest {
                        agent_pid: agent.pid.clone(),
                        conversation_pid: Some(conversation.pid.clone()),
                        message: "test message".to_string(),
                        request_id: request_id.clone(),
                        document_pids: vec![],
                    },
                    organization_pid: "org_test".into(),
                    user_id: "user_test".into(),
                })
                .await
                .expect("should create session");

            session_ref.stop_gracefully().await.expect("should stop session");
            session_ref.wait_for_shutdown().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            let resolved = registry_ref
                .ask(GetSession {
                    conversation_pid: String::new(),
                    request_id: Some(request_id),
                })
                .await
                .expect("registry lookup should succeed");

            assert!(
                resolved.is_none(),
                "stale request id should not resolve a stopped session"
            );
        }
    }
}
