use super::ActorError;
use crate::{
    AppContext,
    agents::{RuntimeAgent, config::AgentConfig, error::AgentError, runtime::SessionContext},
    api::{
        embedding::{DefaultEmbedding, EmbeddingModel},
        message::{MediaData, MediaSource, Message, MessagePart, MessageRole, MessageStatus},
        store::{EmbeddingStore, MessageStore},
    },
    models::{
        agent_users, agents, conversation_users, conversations, documents, language_models, messages, providers,
        secrets, tools, user_assistants,
    },
    tools::ToolRegistry,
};
use kameo::prelude::*;
use proto_rs::rig::v1;
use rig::tool::ToolDyn;
use sea_orm::{
    ColumnTrait as _, EntityTrait as _, ModelTrait as _, QueryFilter as _, QueryOrder as _, QuerySelect as _,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub(crate) struct LoadedConversationContext {
    pub(crate) conversation: conversations::Model,
    pub(crate) agent: agents::Model,
    pub(crate) model: language_models::Model,
    pub(crate) provider: providers::Model,
    pub(crate) session_ttl: Duration,
}

pub(crate) struct InferenceActor<S, EM = DefaultEmbedding>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    store: Arc<S>,
    ctx: Arc<AppContext<EM>>,
    conversation_id: i64,
    runtime_agent: RuntimeAgent<S, EM>,
    conversation_pid: String,
}

pub(crate) struct LoadedInferenceActor<S, EM = DefaultEmbedding>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    pub(crate) actor: InferenceActor<S, EM>,
}

pub(crate) struct RunInference {
    pub(crate) content: String,
    pub(crate) document_pids: Vec<String>,
    pub(crate) response_tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
    pub(crate) cancel_token: CancellationToken,
}

impl<S, EM> Actor for InferenceActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Args = Self;
    type Error = std::convert::Infallible;

    async fn on_start(state: Self::Args, _actor_ref: ActorRef<Self>) -> Result<Self, Self::Error> {
        Ok(state)
    }
}

impl<S, EM> InferenceActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    pub(crate) async fn load_conversation_context(
        ctx: &Arc<AppContext<EM>>,
        conversation_pid: &str,
        organization_pid: &str,
        user_id: &str,
    ) -> Result<LoadedConversationContext, ActorError> {
        let db = &ctx.db;
        let (conversation, agent, model, provider, assistant) = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(conversation_pid))
            .find_also_related(agents::Entity)
            .find_also(agents::Entity, language_models::Entity)
            .find_also(language_models::Entity, providers::Entity)
            .find_also(agents::Entity, user_assistants::Entity)
            .one(db)
            .await
            .map_err(|e| {
                tracing::error!(conversation_pid, "failed to load conversation");
                ActorError::ConversationLoadFailed(e.to_string())
            })?
            .ok_or_else(|| {
                tracing::warn!(conversation_pid, "conversation not found");
                ActorError::ConversationLoadFailed("Conversation not found".to_string())
            })?;

        let agent: agents::Model = agent.ok_or_else(|| {
            tracing::error!(conversation_pid, "agent not found");
            ActorError::ConversationLoadFailed(format!(
                "agent id {} not found for conversation id {}",
                conversation.agent_id, conversation.id
            ))
        })?;

        let model: language_models::Model = model.ok_or_else(|| {
            tracing::error!(conversation_pid, "model not found");
            ActorError::ConversationLoadFailed(format!(
                "model id {} not found for agent id {}",
                agent.model_id, agent.id,
            ))
        })?;

        let provider: providers::Model = provider.ok_or_else(|| {
            tracing::error!(conversation_pid, "provider not found");
            ActorError::ConversationLoadFailed(format!(
                "provider id {} not found for model id {}",
                model.provider_id, model.id
            ))
        })?;

        if conversation.user_id != user_id {
            // Check if user has shared access (execute permission to send messages)
            let has_share = conversation_users::Entity::has_permission(
                db,
                conversation.id,
                user_id,
                conversation_users::Permissions::EXECUTE,
            )
            .await
            .map_err(|e| {
                tracing::error!(conversation_pid, error = %e, "failed to check conversation share");
                ActorError::ConversationLoadFailed("Failed to check access".to_string())
            })?;

            if !has_share {
                tracing::warn!(
                    conversation_pid,
                    conversation_user_id = conversation.user_id,
                    request_user_id = user_id,
                    "conversation belongs to another user"
                );
                return Err(ActorError::Unauthorized(
                    "conversation belongs to another user".to_string(),
                ));
            }
        }

        let provider_accessible = provider.is_accessible_in_org_context(organization_pid);
        if !provider_accessible {
            tracing::warn!(
                conversation_pid,
                provider_pid = provider.pid,
                provider_org = ?provider.organization_pid,
                provider_created_by = ?provider.created_by,
                request_org = %organization_pid,
                "provider not accessible in this context"
            );

            return Err(ActorError::Unauthorized(
                "provider not accessible in this context".to_string(),
            ));
        }

        // SAFETY: `assistant` comes from the same query via LEFT JOIN on user_assistants.
        // If present and user_id matches, this is the requesting user's assistant agent —
        // it's per-user (cross-org), so skip org-scoping.
        let is_user_assistant = assistant.as_ref().is_some_and(|a| a.user_id == user_id);

        if !agent.is_accessible_in_org(organization_pid) && !is_user_assistant {
            // Check if user has shared execute access to the agent
            let has_agent_share =
                agent_users::Entity::has_permission(db, agent.id, user_id, agent_users::Permissions::EXECUTE)
                    .await
                    .map_err(|e| {
                        tracing::error!(conversation_pid, error = %e, "failed to check agent share");
                        ActorError::ConversationLoadFailed("Failed to check agent access".to_string())
                    })?;

            if !has_agent_share {
                tracing::warn!(
                    conversation_pid,
                    agent_pid = agent.pid,
                    agent_org = ?agent.organization_pid,
                    agent_created_by = agent.created_by,
                    request_org = ?organization_pid,
                    "agent not accessible in this context"
                );

                return Err(ActorError::Unauthorized(
                    "agent not accessible in this context".to_string(),
                ));
            }
        }

        let session_ttl = Duration::from_secs(AgentConfig::from(agent.agent_config()).session_ttl_seconds as u64);

        Ok(LoadedConversationContext {
            conversation,
            agent,
            model,
            provider,
            session_ttl,
        })
    }

    pub(crate) async fn load(
        store: Arc<S>,
        ctx: Arc<AppContext<EM>>,
        conversation_pid: &str,
        organization_pid: String,
        user_id: String,
        tool_registry: ToolRegistry<EM>,
    ) -> Result<LoadedInferenceActor<S, EM>, ActorError> {
        let db = &ctx.db;
        let LoadedConversationContext {
            conversation,
            agent,
            model,
            provider,
            session_ttl: _,
        } = Self::load_conversation_context(&ctx, conversation_pid, &organization_pid, &user_id).await?;

        let api_key_secret: Option<secrets::Model> = provider
            .find_related(secrets::Entity)
            .filter(secrets::Column::SecretType.eq(secrets::SecretKind::ApiKey))
            .one(db)
            .await
            .map_err(|e| {
                tracing::error!(provider_id = provider.id, "failed to query database");
                ActorError::DbError(e)
            })?;

        let agent_id = agent.id;
        let domain_config = AgentConfig::from(agent.agent_config());
        let history_limit = domain_config.history_length as u64;

        let history: Vec<Message> = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(conversation.id))
            .filter(messages::Column::IsDeleted.eq(false))
            .order_by_desc(messages::Column::CreatedAt)
            .limit(history_limit)
            .all(db)
            .await
            .map_err(|e| {
                tracing::error!(conversation_id = conversation.id, "failed to load history");
                ActorError::ConversationLoadFailed(e.to_string())
            })?
            .into_iter()
            .rev()
            .map(Message::from)
            .collect();

        let agent_tools: Vec<tools::Model> = agent
            .find_related(tools::Entity)
            .filter(tools::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(|e| {
                tracing::error!(agent_id = agent.id, "failed to load agent tools");
                ActorError::DbError(e)
            })?;

        tracing::debug!(
            agent_id = agent.id,
            agent_pid = %agent.pid,
            db_tool_count = agent_tools.len(),
            db_tool_names = ?agent_tools.iter().map(|t| &t.name).collect::<Vec<_>>(),
            "loaded tools from database for inference actor"
        );

        let tool_config = AgentConfig::from(agent.agent_config());
        let mut tools_dyn: Vec<Box<dyn ToolDyn>> = Vec::new();
        let mut tool_models_ex: Vec<tools::ModelEx> = Vec::new();

        for tool in agent_tools {
            match tool_registry
                .get_tool(
                    &tool.name,
                    tool.id,
                    user_id.clone(),
                    Some(organization_pid.clone()),
                    Some(conversation.id),
                    &tool_config,
                )
                .await
            {
                Ok(t) => {
                    tracing::debug!(tool_name = %tool.name, "successfully loaded tool");
                    tools_dyn.push(t);
                    tool_models_ex.push(tool.into_ex());
                }
                Err(e) => {
                    tracing::warn!(tool_id = tool.id, tool_name = tool.name, error = %e, "skipping tool");
                }
            }
        }

        let loaded_agent = agents::Entity::assemble_agent(
            agent.into_ex(),
            model.into_ex(),
            provider.into_ex(),
            api_key_secret.map(|s| s.into_ex()),
            tool_models_ex,
        )
        .map_err(|e| match e {
            agents::LoadAgentError::SecretDecryption(crypto) => ActorError::CryptoError(crypto),
            other => {
                tracing::error!(error = %other, "failed to build agent from models");
                ActorError::Agent(AgentError::BuildError)
            }
        })?;

        let runtime_agent = RuntimeAgent::new(
            loaded_agent,
            history,
            tools_dyn,
            ctx.clone(),
            store.clone(),
            SessionContext {
                user_id,
                agent_id,
                conversation_id: conversation.id,
                conversation_pid: conversation.pid.clone(),
            },
        )
        .await
        .map_err(ActorError::Agent)?;

        Ok(LoadedInferenceActor {
            actor: Self {
                store,
                ctx,
                conversation_id: conversation.id,
                runtime_agent,
                conversation_pid: conversation.pid.clone(),
            },
        })
    }
}

impl<S, EM> kameo::message::Message<RunInference> for InferenceActor<S, EM>
where
    S: MessageStore + EmbeddingStore,
    EM: EmbeddingModel,
{
    type Reply = Result<Message, ActorError>;

    async fn handle(&mut self, msg: RunInference, _ctx: &mut Context<Self, Self::Reply>) -> Self::Reply {
        let message_pid = nanoid::nanoid!();
        let doc_models = if !msg.document_pids.is_empty() {
            documents::Entity::find()
                .filter(documents::Column::Pid.is_in(&msg.document_pids))
                .all(&self.ctx.db)
                .await
                .unwrap_or_else(|e| {
                    tracing::warn!(error = %e, "failed to fetch document metadata for message");
                    vec![]
                })
        } else {
            vec![]
        };

        let mut parts = vec![MessagePart::Text {
            id: "part_1".into(),
            content: msg.content,
        }];

        for doc in &doc_models {
            parts.push(MessagePart::Document {
                id: doc.pid.clone(),
                filename: doc.filename.clone(),
                summary: doc.summary.clone(),
                media: Some(MediaData {
                    source: MediaSource::Url(doc.storage_path.clone()),
                    media_type: Some(doc.content_type.clone()),
                }),
            });
        }

        let user_message = Message::new(MessageRole::User, parts)
            .with_pid(message_pid.clone())
            .with_status(MessageStatus::Complete);

        self.store
            .persist(&user_message, self.conversation_id)
            .await
            .map_err(|e| {
                tracing::error!(
                    conversation_id = self.conversation_id,
                    message_pid = message_pid,
                    "failed to persist user message"
                );
                ActorError::Store(e)
            })?;

        let _ = msg
            .response_tx
            .send(Ok(v1::InferResponse {
                event: Some(v1::infer_response::Event::Acknowledged(v1::MessageAcknowledged {
                    conversation_pid: self.conversation_pid.clone(),
                    message_pid: message_pid.clone(),
                })),
            }))
            .await;

        let result = self
            .runtime_agent
            .stream(
                user_message,
                self.conversation_pid.clone(),
                msg.response_tx,
                msg.cancel_token,
            )
            .await;

        let assistant_message = match result {
            Ok(message) => message,
            Err(AgentError::Cancelled(partial)) => partial.with_status(MessageStatus::Cancelled),
            Err(AgentError::IdleTimeout(secs, partial)) => {
                tracing::warn!(
                    conversation_id = self.conversation_id,
                    idle_timeout_seconds = secs,
                    "inference idle timed out"
                );
                partial.with_status(MessageStatus::Cancelled)
            }
            Err(err) => return Err(ActorError::Agent(err)),
        };

        self.store
            .persist(&assistant_message, self.conversation_id)
            .await
            .map_err(|e| {
                tracing::error!(
                    conversation_id = self.conversation_id,
                    message_pid = %assistant_message.pid(),
                    "failed to persist assistant message"
                );
                ActorError::Store(e)
            })?;

        Ok(assistant_message)
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        clients::{QdrantClient, StorageClient},
        test_utils::{MockEmbeddingModel, MockStore, TestContext, init_test_crypto},
        tools::ToolRegistry,
    };
    use serial_test::serial;
    use test_context::test_context;

    const MAX_CHANNEL_MESSAGES: usize = 32;

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

    /// Spawns an InferenceActor via `load()`, sends a RunInference message,
    /// and collects events from the response channel up to (and including) the
    /// first Acknowledged event, then cancels inference so the handler returns
    /// promptly instead of waiting for the full LLM stream timeout.
    async fn run_inference_and_collect_events(
        ctx: &mut TestContext,
        suffix: &str,
    ) -> (Vec<v1::infer_response::Event>, String) {
        let (_, _, _, conversation) = ctx.create_full_setup(suffix).await;
        let store = Arc::new(MockStore);
        let app_ctx = mock_app_ctx(ctx);
        let tool_registry = ToolRegistry::new(app_ctx.clone());

        let loaded = InferenceActor::load(
            store,
            app_ctx,
            &conversation.pid,
            "org_test".into(),
            "user_test".into(),
            tool_registry,
        )
        .await
        .expect("load should succeed with seeded data");

        let actor_ref = InferenceActor::spawn(loaded.actor);

        let (tx, mut rx) = mpsc::channel(MAX_CHANNEL_MESSAGES);
        let cancel_token = CancellationToken::new();
        let cancel_clone = cancel_token.clone();

        // Use `tell` (fire-and-forget) so we don't block waiting for the full
        // stream() call to complete — these tests only need the Acknowledged
        // event which is emitted before inference starts.
        actor_ref
            .tell(RunInference {
                content: "Hello from test".into(),
                document_pids: vec![],
                response_tx: tx,
                cancel_token,
            })
            .send()
            .await
            .expect("should deliver RunInference message");

        let mut events = Vec::new();
        let timeout = tokio::time::Duration::from_secs(10);
        while let Ok(Some(result)) = tokio::time::timeout(timeout, rx.recv()).await {
            if let Ok(response) = result
                && let Some(event) = response.event
            {
                let is_ack = matches!(event, v1::infer_response::Event::Acknowledged(_));
                events.push(event);
                if is_ack {
                    break;
                }
            }
        }

        // Cancel inference so the actor's stream() returns promptly
        cancel_clone.cancel();
        actor_ref.stop_gracefully().await.ok();

        (events, conversation.pid)
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_send_acknowledged_after_persist(ctx: &mut TestContext) {
        init_test_crypto();
        let (events, _) = run_inference_and_collect_events(ctx, "ack-after-persist").await;

        let has_ack = events
            .iter()
            .any(|e| matches!(e, v1::infer_response::Event::Acknowledged(_)));

        assert!(has_ack, "expected MessageAcknowledged event, got: {events:?}");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_send_acknowledged_with_correct_pids(ctx: &mut TestContext) {
        init_test_crypto();
        let (events, conversation_pid) = run_inference_and_collect_events(ctx, "ack-correct-pids").await;

        let ack = events
            .iter()
            .find_map(|e| match e {
                v1::infer_response::Event::Acknowledged(a) => Some(a),
                _ => None,
            })
            .expect("should have acknowledged event");

        assert_eq!(ack.conversation_pid, conversation_pid, "conversation_pid should match");
        assert!(!ack.message_pid.is_empty(), "message_pid should not be empty");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_send_acknowledged_before_inference_starts(ctx: &mut TestContext) {
        init_test_crypto();
        let (events, _) = run_inference_and_collect_events(ctx, "ack-before-inference").await;

        if events.is_empty() {
            panic!("expected at least one event");
        }

        assert!(
            matches!(&events[0], v1::infer_response::Event::Acknowledged(_)),
            "first event should be Acknowledged, got: {:?}",
            &events[0]
        );
    }
}
