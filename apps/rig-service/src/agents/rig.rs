use super::Agent;
use crate::agents::{
    MessageConversionError,
    components::{AgentComponent, HistoryComponent},
    error::AgentError,
    streaming::MessageBuilder,
    system_prompt::SystemPromptBuilder,
};
use crate::api::message::Message;
use proto_rs::rig::v1;
use rig::{
    agent::MultiTurnStreamItem, client::CompletionClient, providers, streaming::StreamingChat as _, tool::ToolDyn,
    vector_store::VectorStoreIndexDyn,
};
use std::{sync::Arc, time::Duration};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::{Stream, StreamExt as _};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub(crate) struct StreamArgs {
    pub(crate) user_message: Message,
    pub(crate) history: Vec<Message>,
    pub(crate) conversation_pid: String,
    pub(crate) model_id: String,
    pub(crate) agent_name: Option<String>,
    pub(crate) tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
    pub(crate) cancel_token: CancellationToken,
    pub(crate) idle_timeout: Duration,
}

const MAX_TOOLS_PER_REQUEST: usize = 10;

pub(crate) type DynamicContextStore = Arc<RwLock<Vec<(usize, Box<dyn VectorStoreIndexDyn + Send + Sync>)>>>;

enum InnerAgent {
    Gemini(rig::agent::Agent<providers::gemini::completion::CompletionModel>),
    Ollama(rig::agent::Agent<providers::ollama::CompletionModel>),
    Openai(rig::agent::Agent<providers::openai::responses_api::ResponsesCompletionModel>),
}

pub(crate) struct BaseAgent {
    inner: InnerAgent,
    canary_token: String,
    model_id: String,
    agent_name: Option<String>,
    history: HistoryComponent,
    components: Vec<Box<dyn AgentComponent>>,
}

impl BaseAgent {
    pub(crate) async fn from_loaded(
        agent: &Agent,
        tools: Vec<Box<dyn ToolDyn>>,
        history: Vec<Message>,
        components: Vec<Box<dyn AgentComponent>>,
    ) -> Result<Self, AgentError> {
        let canary_token = format!("<!-- CANARY:{} -->", nanoid::nanoid!(12));
        let model_id = agent.model.model_id.clone();
        let agent_name = (!agent.core.name.is_empty()).then(|| agent.core.name.clone());

        // Collect system prompt sections from components
        let prompt_sections: Vec<&str> = components.iter().filter_map(|c| c.system_prompt_section()).collect();

        let mut builder = SystemPromptBuilder::new(&agent.core.system_prompt, &canary_token);
        for section in &prompt_sections {
            builder = builder.section(section);
        }
        let spec = RigAgentSpec::from_loaded(agent, builder.build());

        let inner = match spec.provider_kind {
            crate::models::providers::ProviderKind::Gemini => InnerAgent::Gemini(build_gemini(&spec, tools)?),
            crate::models::providers::ProviderKind::Ollama => InnerAgent::Ollama(build_ollama(&spec, tools)?),
            crate::models::providers::ProviderKind::Openai => InnerAgent::Openai(build_openai(&spec, tools)?),
        };

        let history = HistoryComponent::new(
            agent.config.history_length as usize,
            history,
            agent.config.compaction_threshold,
            agent.config.compaction_keep_ratio,
        );

        let mut base = Self {
            inner,
            canary_token,
            model_id,
            agent_name,
            history,
            components,
        };

        // Initialize components (register vector indices, etc.)
        let dynamic_context = base.dynamic_context().clone();
        for component in &mut base.components {
            component.init(&dynamic_context).await;
        }

        Ok(base)
    }

    pub(crate) fn model_id(&self) -> &str {
        &self.model_id
    }

    pub(crate) fn agent_name(&self) -> Option<&str> {
        self.agent_name.as_deref()
    }

    /// Full single-inference cycle with component hooks.
    /// Pattern structs call this to run one inference with all component behavior.
    pub(crate) async fn infer(
        &mut self,
        user_message: Message,
        conversation_pid: String,
        tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
        cancel_token: CancellationToken,
        idle_timeout: Duration,
    ) -> Result<Message, AgentError> {
        let mut message = user_message;

        // Pre-stream hooks
        for c in &mut self.components {
            c.on_pre_stream(&mut message).await;
        }

        // History: push, compact if needed, build snapshot.
        // Borrow `inner` separately to avoid capturing all of `self` in the closure.
        let inner = &self.inner;
        let snapshot = self
            .history
            .push_and_snapshot(message.clone(), move |prompt| {
                Box::pin(async move {
                    let rig_history = vec![];
                    use rig::completion::Chat as _;
                    match inner {
                        InnerAgent::Gemini(agent) => agent
                            .chat(&prompt, rig_history)
                            .await
                            .map_err(|e| AgentError::CompletionError(e.to_string())),
                        InnerAgent::Ollama(agent) => agent
                            .chat(&prompt, rig_history)
                            .await
                            .map_err(|e| AgentError::CompletionError(e.to_string())),
                        InnerAgent::Openai(agent) => agent
                            .chat(&prompt, rig_history)
                            .await
                            .map_err(|e| AgentError::CompletionError(e.to_string())),
                    }
                })
            })
            .await;

        // Stream
        let assistant_message = self
            .stream_inner(StreamArgs {
                user_message: message.clone(),
                history: snapshot,
                conversation_pid,
                model_id: self.model_id.clone(),
                agent_name: self.agent_name.clone(),
                tx,
                cancel_token,
                idle_timeout,
            })
            .await?;

        // Push assistant message to history
        self.history.push(assistant_message.clone());

        // Post-stream hooks
        for c in &self.components {
            c.on_post_stream(&message, &assistant_message).await;
        }

        Ok(assistant_message)
    }

    /// Low-level stream dispatch to the Rig provider. Called by `infer()` and
    /// directly by pattern structs that manage their own lifecycle (during migration).
    pub(crate) async fn stream_inner(&self, args: StreamArgs) -> Result<Message, AgentError> {
        let rig_user_msg: rig::message::Message = args
            .user_message
            .try_into()
            .map_err(|e: MessageConversionError| AgentError::StreamError(format!("user message conversion: {e}")))?;

        let rig_history: Vec<rig::message::Message> = args
            .history
            .into_iter()
            .filter_map(|m| rig::message::Message::try_from(m).ok())
            .collect();

        let builder = MessageBuilder::new(args.conversation_pid, args.model_id, args.agent_name);

        let message: Message = match &self.inner {
            InnerAgent::Gemini(agent) => {
                let stream = agent
                    .stream_chat(rig_user_msg, rig_history)
                    .multi_turn(MAX_TOOLS_PER_REQUEST)
                    .await;
                Self::process_stream(stream, builder, args.tx, args.cancel_token, args.idle_timeout).await?
            }
            InnerAgent::Ollama(agent) => {
                let stream = agent
                    .stream_chat(rig_user_msg, rig_history)
                    .multi_turn(MAX_TOOLS_PER_REQUEST)
                    .await;
                Self::process_stream(stream, builder, args.tx, args.cancel_token, args.idle_timeout).await?
            }
            InnerAgent::Openai(agent) => {
                let stream = agent
                    .stream_chat(rig_user_msg, rig_history)
                    .multi_turn(MAX_TOOLS_PER_REQUEST)
                    .await;
                Self::process_stream(stream, builder, args.tx, args.cancel_token, args.idle_timeout).await?
            }
        };

        if message.text_content().contains(&self.canary_token) {
            tracing::warn!(message_pid = %message.pid(), "SECURITY: canary token detected in output");
        }

        Ok(message)
    }

    pub(crate) async fn complete(&self, prompt: &str, history: Vec<Message>) -> Result<String, AgentError> {
        use rig::completion::Chat as _;

        let rig_history: Vec<rig::message::Message> = history
            .into_iter()
            .filter_map(|m| rig::message::Message::try_from(m).ok())
            .collect();

        match &self.inner {
            InnerAgent::Gemini(agent) => agent
                .chat(prompt, rig_history)
                .await
                .map_err(|e| AgentError::CompletionError(e.to_string())),
            InnerAgent::Ollama(agent) => agent
                .chat(prompt, rig_history)
                .await
                .map_err(|e| AgentError::CompletionError(e.to_string())),
            InnerAgent::Openai(agent) => agent
                .chat(prompt, rig_history)
                .await
                .map_err(|e| AgentError::CompletionError(e.to_string())),
        }
    }

    pub(crate) fn dynamic_context(&self) -> &DynamicContextStore {
        match &self.inner {
            InnerAgent::Gemini(a) => &a.dynamic_context,
            InnerAgent::Ollama(a) => &a.dynamic_context,
            InnerAgent::Openai(a) => &a.dynamic_context,
        }
    }

    async fn process_stream<S, R, E>(
        mut stream: S,
        mut builder: MessageBuilder,
        tx: mpsc::Sender<Result<v1::InferResponse, tonic::Status>>,
        cancel_token: CancellationToken,
        idle_timeout: Duration,
    ) -> Result<Message, AgentError>
    where
        S: Stream<Item = Result<MultiTurnStreamItem<R>, E>> + Unpin,
        R: Send,
        E: std::fmt::Display,
    {
        let idle_seconds = idle_timeout.as_secs() as u32;
        let idle_sleep = tokio::time::sleep(idle_timeout);
        tokio::pin!(idle_sleep);

        loop {
            tokio::select! {
                biased;
                _ = cancel_token.cancelled() => {
                    let partial_proto = builder.build_message_with_status(v1::MessageStatus::Cancelled);
                    let _ = tx.send(Ok(v1::InferResponse {
                        event: Some(v1::infer_response::Event::Cancelled(v1::InferenceCancelled {
                            reason: "User requested cancellation".into(),
                        })),
                    })).await;
                    return Err(AgentError::Cancelled(partial_proto.into()));
                }
                () = &mut idle_sleep => {
                    cancel_token.cancel();
                    let partial_proto = builder.build_message_with_status(v1::MessageStatus::Cancelled);
                    let _ = tx.send(Ok(v1::InferResponse {
                        event: Some(v1::infer_response::Event::Cancelled(v1::InferenceCancelled {
                            reason: format!("No activity for {idle_seconds}s"),
                        })),
                    })).await;
                    return Err(AgentError::IdleTimeout(idle_seconds, partial_proto.into()));
                }
                item = stream.next() => {
                    idle_sleep.as_mut().reset(tokio::time::Instant::now() + idle_timeout);

                    match item {
                        Some(Ok(item)) => {
                            for event in builder.process_item(item) {
                                let message = if let v1::infer_response::Event::Complete(ref complete) = event {
                                    complete.message.clone().map(Message::from)
                                } else {
                                    None
                                };
                                let _ = tx.send(Ok(v1::InferResponse { event: Some(event) })).await;
                                if let Some(message) = message {
                                    return Ok(message);
                                }
                            }
                        }
                        Some(Err(e)) => return Err(AgentError::StreamError(e.to_string())),
                        None => return Err(AgentError::StreamEndedWithoutFinalResponse),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RigAgentSpec {
    #[expect(
        dead_code,
        reason = "Kept on the rig spec for provider-level tracing and future request correlation"
    )]
    pub(crate) agent_pid: String,
    pub(crate) provider_kind: crate::models::providers::ProviderKind,
    pub(crate) model_id: String,
    pub(crate) temperature: f64,
    pub(crate) built_system_prompt: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) base_url: Option<String>,
    pub(crate) api_key: Option<String>,
    pub(crate) capabilities: v1::ModelCapabilities,
    pub(crate) thinking_enabled: bool,
    pub(crate) max_tokens: Option<u32>,
}

impl RigAgentSpec {
    pub(crate) fn from_loaded(agent: &Agent, built_system_prompt: String) -> Self {
        Self {
            agent_pid: agent.identity.pid.clone(),
            provider_kind: agent.provider.kind,
            model_id: agent.model.model_id.clone(),
            temperature: agent.core.temperature,
            built_system_prompt,
            name: (!agent.core.name.is_empty()).then(|| agent.core.name.clone()),
            description: agent.core.description.clone(),
            base_url: agent.provider.base_url.clone(),
            api_key: agent.provider.api_key.clone(),
            capabilities: agent.model.capabilities,
            thinking_enabled: agent.config.thinking_enabled,
            max_tokens: agent.config.max_tokens,
        }
    }
}

fn build_openai(
    spec: &RigAgentSpec,
    tools: Vec<Box<dyn ToolDyn>>,
) -> Result<rig::agent::Agent<providers::openai::responses_api::ResponsesCompletionModel>, AgentError> {
    let base_url = spec
        .base_url
        .as_deref()
        .ok_or_else(|| AgentError::MissingRequired("base_url".into()))?;
    let api_key = spec
        .api_key
        .as_deref()
        .ok_or_else(|| AgentError::MissingRequired("api_key".into()))?;

    let client = providers::openai::Client::builder()
        .base_url(base_url)
        .api_key(api_key)
        .build()
        .map_err(|_| AgentError::BuildError)?;

    let mut builder = client
        .agent(&spec.model_id)
        .preamble(&spec.built_system_prompt)
        .temperature(spec.temperature)
        .tools(tools);

    if spec.thinking_enabled && spec.capabilities.thinking {
        builder = builder.additional_params(serde_json::json!({ "think": true }));
    }
    if let Some(name) = &spec.name {
        builder = builder.name(name);
    }
    if let Some(desc) = &spec.description {
        builder = builder.description(desc);
    }
    if let Some(max_tokens) = spec.max_tokens.map(|t| t as u64) {
        builder = builder.max_tokens(max_tokens);
    }

    Ok(builder.build())
}

fn build_ollama(
    spec: &RigAgentSpec,
    tools: Vec<Box<dyn ToolDyn>>,
) -> Result<rig::agent::Agent<providers::ollama::CompletionModel>, AgentError> {
    let mut client_builder = providers::ollama::Client::builder().api_key(rig::client::Nothing);
    if let Some(base_url) = spec.base_url.as_deref() {
        client_builder = client_builder.base_url(base_url);
    }
    let client = client_builder.build().map_err(|_| AgentError::BuildError)?;

    let mut builder = client
        .agent(&spec.model_id)
        .preamble(&spec.built_system_prompt)
        .temperature(spec.temperature)
        .tools(tools);

    if spec.thinking_enabled && spec.capabilities.thinking {
        builder = builder.additional_params(serde_json::json!({ "think": true }));
    }
    if let Some(name) = &spec.name {
        builder = builder.name(name);
    }
    if let Some(desc) = &spec.description {
        builder = builder.description(desc);
    }
    if let Some(max_tokens) = spec.max_tokens.map(|t| t as u64) {
        builder = builder.max_tokens(max_tokens);
    }

    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::error::AgentError;
    use crate::agents::streaming::MessageBuilder;
    use rig::agent::{FinalResponse, MultiTurnStreamItem};
    use rig::message::Text;
    use rig::streaming::StreamedAssistantContent;

    /// Creates a text stream item (same helper as in streaming.rs tests).
    fn text_item(text: &str) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(Text { text: text.to_string() }))
    }

    /// Creates a FinalResponse stream item that signals successful completion.
    fn final_item() -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::FinalResponse(FinalResponse::empty())
    }

    /// Shared test harness: creates the channel, builder, and cancel token needed
    /// by `process_stream`, then runs `process_stream` and returns the result
    /// along with collected events from the channel.
    async fn run_process_stream(
        items: Vec<Result<MultiTurnStreamItem<()>, String>>,
        idle_timeout: Duration,
        cancel_token: CancellationToken,
    ) -> (Result<Message, AgentError>, Vec<v1::infer_response::Event>) {
        let (tx, mut rx) = mpsc::channel(64);
        let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
        let stream = tokio_stream::iter(items);

        let result = BaseAgent::process_stream(stream, builder, tx, cancel_token, idle_timeout).await;

        // Drain the channel to collect all sent events.
        let mut events = vec![];
        rx.close();
        while let Some(Ok(resp)) = rx.recv().await {
            if let Some(event) = resp.event {
                events.push(event);
            }
        }
        (result, events)
    }

    mod idle_timeout {
        use super::*;

        #[tokio::test(start_paused = true)]
        async fn should_return_idle_timeout_when_stream_produces_no_items() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(5);

            // A stream that yields Pending forever (never produces items).
            let (tx_ch, mut rx_ch) = mpsc::channel::<Result<v1::InferResponse, tonic::Status>>(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            let stream = tokio_stream::pending::<Result<MultiTurnStreamItem<()>, String>>();

            let result = BaseAgent::process_stream(stream, builder, tx_ch, cancel_token.clone(), idle).await;

            // Drain events.
            rx_ch.close();
            let mut events = vec![];
            while let Some(Ok(resp)) = rx_ch.recv().await {
                if let Some(event) = resp.event {
                    events.push(event);
                }
            }

            assert!(
                matches!(&result, Err(AgentError::IdleTimeout(5, _))),
                "expected IdleTimeout, got: {result:?}"
            );
            assert!(
                cancel_token.is_cancelled(),
                "cancel token should be cancelled on idle timeout"
            );

            // Should have sent a Cancelled event with the idle reason.
            assert!(events.iter().any(|e| matches!(
                e,
                v1::infer_response::Event::Cancelled(c) if c.reason.contains("No activity")
            )));
        }

        #[tokio::test(start_paused = true)]
        async fn should_complete_successfully_when_items_arrive_before_idle_deadline() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(10);

            // Items arrive immediately (no delay), well within idle timeout.
            let items = vec![Ok(text_item("Hello")), Ok(final_item())];
            let (result, events) = run_process_stream(items, idle, cancel_token).await;

            assert!(result.is_ok(), "expected Ok, got: {result:?}");
            assert!(
                events
                    .iter()
                    .any(|e| matches!(e, v1::infer_response::Event::Complete(_)))
            );
        }

        #[tokio::test(start_paused = true)]
        async fn should_reset_idle_timer_on_each_stream_item() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(10);

            // Create a stream where items arrive at 7s intervals (within the 10s idle
            // timeout each time), but total wall time (21s) exceeds the idle timeout.
            let (item_tx, item_rx) = tokio::sync::mpsc::channel::<Result<MultiTurnStreamItem<()>, String>>(8);
            let stream = tokio_stream::wrappers::ReceiverStream::new(item_rx);

            let (event_tx, mut event_rx) = mpsc::channel(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let handle =
                tokio::spawn(
                    async move { BaseAgent::process_stream(stream, builder, event_tx, cancel_token, idle).await },
                );

            // Send items at 7-second intervals (within idle budget each time).
            for _ in 0..3 {
                tokio::time::advance(Duration::from_secs(7)).await;
                item_tx.send(Ok(text_item("chunk"))).await.unwrap();
            }
            // Send final response.
            tokio::time::advance(Duration::from_secs(1)).await;
            item_tx.send(Ok(final_item())).await.unwrap();
            drop(item_tx);

            let result = handle.await.unwrap();
            assert!(result.is_ok(), "expected Ok despite 22s total, got: {result:?}");

            event_rx.close();
        }

        #[tokio::test(start_paused = true)]
        async fn should_cancel_token_on_idle_timeout() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(5);

            let (tx_ch, mut rx_ch) = mpsc::channel(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            let stream = tokio_stream::pending::<Result<MultiTurnStreamItem<()>, String>>();

            let _ = BaseAgent::process_stream(stream, builder, tx_ch, cancel_token.clone(), idle).await;

            rx_ch.close();
            while rx_ch.recv().await.is_some() {}

            assert!(cancel_token.is_cancelled());
        }

        #[tokio::test(start_paused = true)]
        async fn should_include_partial_message_in_idle_timeout_error() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(5);

            // Stream that yields one item then blocks forever (sender kept alive).
            let (item_tx, item_rx) = tokio::sync::mpsc::channel::<Result<MultiTurnStreamItem<()>, String>>(8);
            let stream = tokio_stream::wrappers::ReceiverStream::new(item_rx);

            let (event_tx, mut event_rx) = mpsc::channel(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let handle =
                tokio::spawn(
                    async move { BaseAgent::process_stream(stream, builder, event_tx, cancel_token, idle).await },
                );

            // Send a text item, then hold the sender open (stream blocks on next()).
            item_tx.send(Ok(text_item("partial content"))).await.unwrap();
            tokio::task::yield_now().await;

            // Advance past idle timeout — stream is blocked, no more items.
            tokio::time::advance(Duration::from_secs(6)).await;

            let result = handle.await.unwrap();
            drop(item_tx);

            event_rx.close();
            while event_rx.recv().await.is_some() {}

            match result {
                Err(AgentError::IdleTimeout(5, partial)) => {
                    let text = partial.text_content();
                    assert_eq!(
                        text, "partial content",
                        "partial message should contain accumulated text"
                    );
                }
                other => panic!("expected IdleTimeout with partial message, got: {other:?}"),
            }
        }

        #[tokio::test(start_paused = true)]
        async fn should_return_cancelled_not_idle_timeout_when_both_fire() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(5);

            let (event_tx, mut event_rx) = mpsc::channel(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            // Stream that blocks forever.
            let stream = tokio_stream::pending::<Result<MultiTurnStreamItem<()>, String>>();

            // Cancel the token before process_stream even starts — both cancellation
            // and idle sleep (after time advance) will be ready. Biased select should
            // pick cancellation.
            cancel_token.cancel();

            let result = BaseAgent::process_stream(stream, builder, event_tx, cancel_token, idle).await;

            event_rx.close();
            while event_rx.recv().await.is_some() {}

            assert!(
                matches!(&result, Err(AgentError::Cancelled(_))),
                "cancellation should take priority over idle timeout, got: {result:?}"
            );
        }
    }

    mod stream_errors {
        use super::*;

        #[tokio::test(start_paused = true)]
        async fn should_return_stream_error_on_provider_error() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(60);

            let items: Vec<Result<MultiTurnStreamItem<()>, String>> =
                vec![Err("provider connection reset".to_string())];
            let (result, _events) = run_process_stream(items, idle, cancel_token).await;

            match result {
                Err(AgentError::StreamError(msg)) => {
                    assert!(msg.contains("provider connection reset"));
                }
                other => panic!("expected StreamError, got: {other:?}"),
            }
        }

        #[tokio::test(start_paused = true)]
        async fn should_return_stream_ended_without_final_when_stream_closes_early() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(60);

            // Text items but no FinalResponse — stream just ends.
            let items = vec![Ok(text_item("Hello"))];
            let (result, _events) = run_process_stream(items, idle, cancel_token).await;

            assert!(
                matches!(&result, Err(AgentError::StreamEndedWithoutFinalResponse)),
                "expected StreamEndedWithoutFinalResponse, got: {result:?}"
            );
        }
    }

    mod cancellation {
        use super::*;

        #[tokio::test(start_paused = true)]
        async fn should_return_cancelled_with_partial_message_on_cancel() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(60);

            // Stream that yields one item then blocks forever (sender kept alive).
            let (item_tx, item_rx) = tokio::sync::mpsc::channel::<Result<MultiTurnStreamItem<()>, String>>(8);
            let stream = tokio_stream::wrappers::ReceiverStream::new(item_rx);

            let (event_tx, mut event_rx) = mpsc::channel(64);
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let cancel_clone = cancel_token.clone();
            let handle =
                tokio::spawn(
                    async move { BaseAgent::process_stream(stream, builder, event_tx, cancel_token, idle).await },
                );

            // Send one item, let it be processed, then cancel.
            item_tx.send(Ok(text_item("partial"))).await.unwrap();
            tokio::task::yield_now().await;
            cancel_clone.cancel();

            let result = handle.await.unwrap();
            drop(item_tx);

            event_rx.close();
            let mut events = vec![];
            while let Some(Ok(resp)) = event_rx.recv().await {
                if let Some(event) = resp.event {
                    events.push(event);
                }
            }

            match result {
                Err(AgentError::Cancelled(partial)) => {
                    let text = partial.text_content();
                    assert_eq!(text, "partial", "cancelled message should contain accumulated text");
                }
                other => panic!("expected Cancelled with partial message, got: {other:?}"),
            }

            assert!(events.iter().any(|e| matches!(
                e,
                v1::infer_response::Event::Cancelled(c) if c.reason.contains("User requested")
            )));
        }
    }

    mod successful_stream {
        use super::*;

        #[tokio::test(start_paused = true)]
        async fn should_send_complete_event_and_return_message_on_successful_stream() {
            let cancel_token = CancellationToken::new();
            let idle = Duration::from_secs(60);

            let items = vec![Ok(text_item("Hello ")), Ok(text_item("world!")), Ok(final_item())];
            let (result, events) = run_process_stream(items, idle, cancel_token).await;

            assert!(result.is_ok(), "expected successful completion, got: {result:?}");

            // Should contain a Complete event.
            assert!(
                events
                    .iter()
                    .any(|e| matches!(e, v1::infer_response::Event::Complete(_)))
            );
        }
    }
}

fn build_gemini(
    spec: &RigAgentSpec,
    tools: Vec<Box<dyn ToolDyn>>,
) -> Result<rig::agent::Agent<providers::gemini::completion::CompletionModel>, AgentError> {
    let api_key = spec
        .api_key
        .as_deref()
        .ok_or_else(|| AgentError::MissingRequired("api_key".into()))?;

    let client = providers::gemini::Client::new(api_key).map_err(|_| AgentError::BuildError)?;
    let mut builder = client
        .agent(&spec.model_id)
        .preamble(&spec.built_system_prompt)
        .temperature(spec.temperature)
        .tools(tools);

    if spec.thinking_enabled && spec.capabilities.thinking {
        builder = builder.additional_params(serde_json::json!({
            "generationConfig": {
                "thinkingConfig": {
                    "thinkingBudget": 8192,
                    "includeThoughts": true
                }
            }
        }));
    }
    if let Some(name) = &spec.name {
        builder = builder.name(name);
    }
    if let Some(desc) = &spec.description {
        builder = builder.description(desc);
    }
    if let Some(max_tokens) = spec.max_tokens.map(|t| t as u64) {
        builder = builder.max_tokens(max_tokens);
    }

    Ok(builder.build())
}
