use crate::{
    AppContext,
    agents::{
        Agent, BaseAgent,
        components::{AgentComponent, DocumentsComponent, EpisodicMemoryComponent},
        error::AgentError,
    },
    api::{
        embedding::{DefaultEmbedding, EmbeddingModel},
        message::{Message, MessagePart, MessageRole, MessageStatus},
        store::EmbeddingStore,
    },
};
use proto_rs::rig::v1;
use rig::tool::ToolDyn;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub(crate) struct SessionContext {
    pub(crate) user_id: String,
    pub(crate) agent_id: i64,
    pub(crate) conversation_id: i64,
    pub(crate) conversation_pid: String,
}

pub(crate) struct RuntimeAgent<S, EM = DefaultEmbedding>
where
    S: EmbeddingStore,
    EM: EmbeddingModel,
{
    inner: RuntimeAgentInner,
    timeout_seconds: u32,
    idle_timeout_seconds: u32,
    streaming_enabled: bool,
    // Generics kept on the struct for compatibility with the actor chain.
    // They're not used after construction — components erase them.
    _phantom: std::marker::PhantomData<(S, EM)>,
}

enum RuntimeAgentInner {
    Startup(StartupAgent),
    Ephemeral(BaseAgent),
}

impl<S, EM> RuntimeAgent<S, EM>
where
    S: EmbeddingStore,
    EM: EmbeddingModel,
{
    pub(crate) async fn new(
        agent: Agent,
        history: Vec<Message>,
        tools: Vec<Box<dyn ToolDyn>>,
        ctx: Arc<AppContext<EM>>,
        store: Arc<S>,
        session: SessionContext,
    ) -> Result<Self, AgentError> {
        use crate::models::agents::AgentKind;

        let timeout_seconds = agent.config.timeout_seconds;
        let idle_timeout_seconds = agent.config.idle_timeout_seconds;
        let streaming_enabled = agent.config.streaming_enabled;

        let inner = match agent.core.kind {
            AgentKind::Startup => {
                RuntimeAgentInner::Startup(StartupAgent::new(agent, history, tools, ctx, store, session).await?)
            }
            AgentKind::Ephemeral => {
                let components: Vec<Box<dyn AgentComponent>> = vec![Box::new(DocumentsComponent::new(
                    ctx.qdrant.clone(),
                    ctx.embedding.clone(),
                    session.conversation_pid.clone(),
                    agent.config.document_result_count as usize,
                ))];
                let base = BaseAgent::from_loaded(&agent, tools, vec![], components).await?;
                RuntimeAgentInner::Ephemeral(base)
            }
        };

        Ok(Self {
            inner,
            timeout_seconds,
            idle_timeout_seconds,
            streaming_enabled,
            _phantom: std::marker::PhantomData,
        })
    }

    #[expect(dead_code, reason = "Useful inspection surface while the new runtime API settles")]
    pub(crate) fn model_id(&self) -> &str {
        match &self.inner {
            RuntimeAgentInner::Startup(a) => a.model_id(),
            RuntimeAgentInner::Ephemeral(a) => a.model_id(),
        }
    }

    #[expect(dead_code, reason = "Useful inspection surface while the new runtime API settles")]
    pub(crate) fn agent_name(&self) -> Option<&str> {
        match &self.inner {
            RuntimeAgentInner::Startup(a) => a.agent_name(),
            RuntimeAgentInner::Ephemeral(a) => a.agent_name(),
        }
    }

    pub(crate) async fn stream(
        &mut self,
        user_message: Message,
        conversation_pid: String,
        tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
        cancel_token: CancellationToken,
    ) -> Result<Message, AgentError> {
        let wall_duration = Duration::from_secs(self.timeout_seconds as u64);
        let idle_duration = Duration::from_secs(self.idle_timeout_seconds as u64);
        let cancel_clone = cancel_token.clone();
        let tx_clone = tx.clone();
        let timeout_seconds = self.timeout_seconds;

        let result = tokio::time::timeout(wall_duration, async {
            if !self.streaming_enabled {
                return self.complete_and_send(user_message, conversation_pid, tx).await;
            }

            match &mut self.inner {
                RuntimeAgentInner::Startup(a) => {
                    a.stream(user_message, conversation_pid, tx, cancel_token, idle_duration)
                        .await
                }
                RuntimeAgentInner::Ephemeral(a) => {
                    a.infer(user_message, conversation_pid, tx, cancel_token, idle_duration)
                        .await
                }
            }
        })
        .await;

        match result {
            Ok(inner) => inner,
            Err(_elapsed) => {
                cancel_clone.cancel();
                let _ = tx_clone
                    .send(Ok(v1::InferResponse {
                        event: Some(v1::infer_response::Event::Cancelled(v1::InferenceCancelled {
                            reason: format!("Wall-clock timeout after {timeout_seconds}s"),
                        })),
                    }))
                    .await;
                Err(AgentError::Timeout(timeout_seconds))
            }
        }
    }

    async fn complete_and_send(
        &self,
        user_message: Message,
        conversation_pid: String,
        tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
    ) -> Result<Message, AgentError> {
        let prompt = user_message.text_content();

        let response_text = match &self.inner {
            RuntimeAgentInner::Startup(a) => a.complete(&prompt, vec![]).await?,
            RuntimeAgentInner::Ephemeral(a) => a.complete(&prompt, vec![]).await?,
        };

        let message = Message::new(
            MessageRole::Assistant,
            vec![MessagePart::Text {
                id: nanoid::nanoid!(),
                content: response_text,
            }],
        )
        .with_pid(nanoid::nanoid!())
        .with_status(MessageStatus::Complete);

        let proto_msg: v1::Message = message.clone().into();
        let _ = tx
            .send(Ok(v1::InferResponse {
                event: Some(v1::infer_response::Event::Complete(v1::InferenceComplete {
                    conversation_pid,
                    message: Some(proto_msg),
                    metadata: None,
                })),
            }))
            .await;

        Ok(message)
    }

    #[expect(
        dead_code,
        reason = "Non-streaming completion remains a supported runtime entrypoint even if not currently called"
    )]
    pub(crate) async fn complete(&self, prompt: &str, history: Vec<Message>) -> Result<String, AgentError> {
        match &self.inner {
            RuntimeAgentInner::Startup(a) => a.complete(prompt, history).await,
            RuntimeAgentInner::Ephemeral(a) => a.complete(prompt, history).await,
        }
    }
}

/// ReAct pattern — thin wrapper over `BaseAgent`. Rig's `multi_turn` handles
/// the think → tool → observe loop internally. This struct exists so the
/// `RuntimeAgentInner` enum can distinguish pattern types.
struct StartupAgent {
    base: BaseAgent,
}

impl StartupAgent {
    async fn new<S, EM>(
        agent: Agent,
        history: Vec<Message>,
        tools: Vec<Box<dyn ToolDyn>>,
        ctx: Arc<AppContext<EM>>,
        store: Arc<S>,
        session: SessionContext,
    ) -> Result<Self, AgentError>
    where
        S: EmbeddingStore,
        EM: EmbeddingModel,
    {
        let mut components: Vec<Box<dyn AgentComponent>> = Vec::new();

        // Documents component
        components.push(Box::new(DocumentsComponent::new(
            ctx.qdrant.clone(),
            ctx.embedding.clone(),
            session.conversation_pid.clone(),
            agent.config.document_result_count as usize,
        )));

        // Episodic memory component (conditional)
        if agent.config.memory_enabled {
            components.push(Box::new(EpisodicMemoryComponent::new(
                store,
                ctx.embedding.clone(),
                session.user_id.clone(),
                session.agent_id,
                session.conversation_id,
                agent.config.memory_result_count,
                agent.config.memory_similarity_threshold,
            )));
        }

        let base = BaseAgent::from_loaded(&agent, tools, history, components).await?;

        Ok(Self { base })
    }

    fn model_id(&self) -> &str {
        self.base.model_id()
    }

    fn agent_name(&self) -> Option<&str> {
        self.base.agent_name()
    }

    async fn stream(
        &mut self,
        user_message: Message,
        conversation_pid: String,
        tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
        cancel_token: CancellationToken,
        idle_timeout: Duration,
    ) -> Result<Message, AgentError> {
        self.base
            .infer(user_message, conversation_pid, tx, cancel_token, idle_timeout)
            .await
    }

    async fn complete(&self, prompt: &str, history: Vec<Message>) -> Result<String, AgentError> {
        self.base.complete(prompt, history).await
    }
}
