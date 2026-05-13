use crate::tools::ToToolDisplayName;
use proto_rs::rig::v1;
use rig::{
    agent::MultiTurnStreamItem,
    streaming::{StreamedAssistantContent, StreamedUserContent, ToolCallDeltaContent},
};
use std::collections::HashMap;

/// Tracks the kind of the currently active content part for detecting switches.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveContentKind {
    Text,
    Thinking,
}

/// Represents a part in the unified arrival-order timeline.
/// Content (text/thinking) is stored inline; tool calls and results
/// reference data in their respective HashMaps.
enum OrderedPart {
    Text { id: String, content: String },
    Thinking { id: String, content: String },
    ToolCallWithResult { internal_call_id: String },
}

/// Builds a complete Message by accumulating parts during streaming.
///
/// Processes rig stream items, updates internal state, and produces proto
/// events for streaming to clients. Preserves the temporal order of ALL parts
/// (text, thinking, tool calls, tool results) so the final message reflects
/// the LLM's chain of thought.
pub(crate) struct MessageBuilder {
    /// Conversation PID for the v1::InferenceComplete event
    conversation_pid: String,
    /// Model identifier (e.g. "qwen3:14b") for metadata part
    model_id: String,
    /// Agent name for metadata part
    agent_name: Option<String>,
    /// All parts in arrival order — the single source of truth for ordering.
    ordered_parts: Vec<OrderedPart>,
    /// The kind and index (into ordered_parts) of the currently active content
    /// part. When the LLM switches from text→thinking or vice versa, we close
    /// the current part and start a new one.
    active_content: Option<(ActiveContentKind, usize)>,
    /// Tool calls by internal_call_id (may receive deltas before complete)
    tool_calls: HashMap<String, ToolCallBuilder>,
    /// Tool results by internal_call_id
    tool_results: HashMap<String, String>,
    /// Part ID counter for generating IDs
    part_counter: u32,
}

#[derive(Default)]
struct ToolCallBuilder {
    name: String,
    arguments_json: String,
}

impl MessageBuilder {
    pub(crate) fn new(conversation_pid: String, model_id: String, agent_name: Option<String>) -> Self {
        Self {
            conversation_pid,
            model_id,
            agent_name,
            ordered_parts: Vec::new(),
            active_content: None,
            tool_calls: HashMap::new(),
            tool_results: HashMap::new(),
            part_counter: 0,
        }
    }

    fn next_part_id(&mut self) -> String {
        self.part_counter += 1;
        format!("part_{}", self.part_counter)
    }

    /// Processes a stream item, updates internal state, and returns events to send.
    /// Returns empty vec for unrecognized items or provider-specific finals that shouldn't be sent.
    /// May return multiple events (e.g., PartStart followed by PartDelta for new parts).
    pub(crate) fn process_item<R>(&mut self, item: MultiTurnStreamItem<R>) -> Vec<v1::infer_response::Event> {
        match item {
            MultiTurnStreamItem::StreamAssistantItem(content) => self.process_assistant(content),
            MultiTurnStreamItem::StreamUserItem(content) => self.process_user(content),
            MultiTurnStreamItem::FinalResponse(_) => vec![self.finalize()],
            // MultiTurnStreamItem is #[non_exhaustive]
            _ => vec![],
        }
    }

    /// Finalizes the message and returns the v1::InferenceComplete event.
    fn finalize(&mut self) -> v1::infer_response::Event {
        let message = self.build_message();
        v1::infer_response::Event::Complete(v1::InferenceComplete {
            conversation_pid: std::mem::take(&mut self.conversation_pid),
            message: Some(message),
            metadata: None,
        })
    }

    /// Builds the final message from accumulated state with the given status.
    pub(crate) fn build_message_with_status(
        &mut self,
        status: proto_rs::rig::v1::MessageStatus,
    ) -> proto_rs::rig::v1::Message {
        let parts = self.build_parts();
        proto_rs::rig::v1::Message {
            pid: nanoid::nanoid!(),
            role: "assistant".into(),
            status: status.into(),
            parts,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Builds the final message from accumulated state (status: Complete).
    fn build_message(&mut self) -> proto_rs::rig::v1::Message {
        self.build_message_with_status(proto_rs::rig::v1::MessageStatus::Complete)
    }

    /// Consumes the builder and returns the final message. Useful for testing.
    #[cfg(test)]
    pub(crate) fn into_message(mut self) -> proto_rs::rig::v1::Message {
        self.build_message()
    }

    /// Consumes the builder and returns a cancelled message. Useful for testing.
    #[cfg(test)]
    pub(crate) fn into_cancelled_message(mut self) -> proto_rs::rig::v1::Message {
        self.build_message_with_status(proto_rs::rig::v1::MessageStatus::Cancelled)
    }

    /// Ensures there is an active content part of the given kind. If the current
    /// active part is a different kind, ends the current part and starts a new one.
    /// Returns (part_id, is_new, end_events) where end_events contains a PartEnd
    /// for the previous part if we switched kinds.
    fn ensure_content_part(
        &mut self,
        kind: ActiveContentKind,
        provided_id: Option<String>,
    ) -> (String, bool, Vec<v1::infer_response::Event>) {
        let mut end_events = Vec::new();

        if let Some((active_kind, idx)) = self.active_content {
            if active_kind == kind {
                // Same kind — continue accumulating into the current part
                let part_id = match &self.ordered_parts[idx] {
                    OrderedPart::Text { id, .. } | OrderedPart::Thinking { id, .. } => id.clone(),
                    OrderedPart::ToolCallWithResult { .. } => {
                        unreachable!("active_content should never point to a tool call")
                    }
                };
                return (part_id, false, end_events);
            }
            // Different kind — close the previous content part
            let prev_id = match &self.ordered_parts[idx] {
                OrderedPart::Text { id, .. } | OrderedPart::Thinking { id, .. } => id.clone(),
                OrderedPart::ToolCallWithResult { .. } => {
                    unreachable!("active_content should never point to a tool call")
                }
            };
            end_events.push(v1::infer_response::Event::PartEnd(v1::PartEnd {
                part_id: prev_id,
                status: v1::MessagePartStatus::Complete.into(),
            }));
        }

        // Start a new content part
        let id = provided_id.unwrap_or_else(|| self.next_part_id());
        let ordered_part = match kind {
            ActiveContentKind::Text => OrderedPart::Text {
                id: id.clone(),
                content: String::new(),
            },
            ActiveContentKind::Thinking => OrderedPart::Thinking {
                id: id.clone(),
                content: String::new(),
            },
        };
        let idx = self.ordered_parts.len();
        self.ordered_parts.push(ordered_part);
        self.active_content = Some((kind, idx));

        (id, true, end_events)
    }

    /// Appends text to the currently active content part.
    fn append_to_active_content(&mut self, text: &str) {
        if let Some((_, idx)) = self.active_content {
            match &mut self.ordered_parts[idx] {
                OrderedPart::Text { content, .. } | OrderedPart::Thinking { content, .. } => {
                    content.push_str(text);
                }
                OrderedPart::ToolCallWithResult { .. } => {}
            }
        }
    }

    fn process_assistant<R>(&mut self, content: StreamedAssistantContent<R>) -> Vec<v1::infer_response::Event> {
        match content {
            StreamedAssistantContent::Text(text) => {
                let (part_id, is_new, mut events) = self.ensure_content_part(ActiveContentKind::Text, None);
                self.append_to_active_content(&text.text);
                if is_new {
                    events.push(v1::infer_response::Event::PartStart(v1::PartStart {
                        part_id: part_id.clone(),
                        kind: v1::MessagePartKind::Text.into(),
                        tool_name: None,
                        tool_call_id: None,
                    }));
                }
                events.push(v1::infer_response::Event::PartDelta(v1::PartDelta {
                    part_id,
                    kind: v1::MessagePartKind::Text.into(),
                    delta: Some(v1::part_delta::Delta::Content(text.text)),
                }));
                events
            }
            StreamedAssistantContent::ToolCall {
                tool_call,
                internal_call_id,
            } => {
                let args_str = serde_json::to_string(&tool_call.function.arguments).unwrap_or_else(|_| {
                    tracing::warn!(tool_call_id = tool_call.id, "failed to serialize tool call args");
                    "{}".into()
                });
                // Use internal_call_id for reliable correlation (provider id may not be unique, e.g. Gemini)
                self.tool_calls.insert(
                    internal_call_id.clone(),
                    ToolCallBuilder {
                        name: tool_call.function.name.clone(),
                        arguments_json: args_str,
                    },
                );
                // Record in arrival order (only if first time seeing this tool call)
                if !self.ordered_parts.iter().any(|p| matches!(p, OrderedPart::ToolCallWithResult { internal_call_id: id } if id == &internal_call_id)) {
                    self.ordered_parts.push(OrderedPart::ToolCallWithResult {
                        internal_call_id: internal_call_id.clone(),
                    });
                }
                let args_struct: pbjson_types::Struct = serde_json::from_value(tool_call.function.arguments.clone())
                    .unwrap_or_else(|_| {
                        tracing::warn!(
                            tool_call_id = tool_call.id,
                            "failed to deserialize convert tool call args"
                        );
                        pbjson_types::Struct::default()
                    });

                // When a tool call arrives, end any active content part
                let mut events = Vec::new();
                if let Some((_, idx)) = self.active_content.take() {
                    let prev_id = match &self.ordered_parts[idx] {
                        OrderedPart::Text { id, .. } | OrderedPart::Thinking { id, .. } => id.clone(),
                        OrderedPart::ToolCallWithResult { .. } => {
                            unreachable!("active_content should never point to a tool call")
                        }
                    };
                    events.push(v1::infer_response::Event::PartEnd(v1::PartEnd {
                        part_id: prev_id,
                        status: v1::MessagePartStatus::Complete.into(),
                    }));
                }

                // Emit PartStart + PartDelta for the tool call. Don't send PartEnd yet -
                // the tool is about to execute. PartEnd will be sent when the tool result arrives.
                events.push(v1::infer_response::Event::PartStart(v1::PartStart {
                    part_id: internal_call_id.clone(),
                    kind: v1::MessagePartKind::ToolCall.into(),
                    tool_name: Some(tool_call.function.name.to_tool_display_name()),
                    tool_call_id: Some(tool_call.id.clone()),
                }));
                events.push(v1::infer_response::Event::PartDelta(v1::PartDelta {
                    part_id: internal_call_id,
                    kind: v1::MessagePartKind::ToolCall.into(),
                    delta: Some(v1::part_delta::Delta::Arguments(args_struct)),
                }));
                events
            }
            StreamedAssistantContent::ToolCallDelta {
                id,
                internal_call_id,
                content,
            } => match content {
                ToolCallDeltaContent::Name(name) => {
                    // Store the name for the final message, keyed by internal_call_id for reliable correlation
                    self.tool_calls.entry(internal_call_id.clone()).or_default().name = name.clone();
                    if !self.ordered_parts.iter().any(|p| matches!(p, OrderedPart::ToolCallWithResult { internal_call_id: id } if id == &internal_call_id)) {
                        self.ordered_parts.push(OrderedPart::ToolCallWithResult {
                            internal_call_id: internal_call_id.clone(),
                        });
                    }
                    vec![v1::infer_response::Event::PartStart(v1::PartStart {
                        part_id: internal_call_id.clone(),
                        kind: v1::MessagePartKind::ToolCall.into(),
                        tool_name: Some(name.to_tool_display_name()),
                        tool_call_id: Some(id),
                    })]
                }
                ToolCallDeltaContent::Delta(delta) => {
                    self.tool_calls
                        .entry(internal_call_id.clone())
                        .or_default()
                        .arguments_json
                        .push_str(&delta);
                    vec![v1::infer_response::Event::PartDelta(v1::PartDelta {
                        part_id: internal_call_id,
                        kind: v1::MessagePartKind::ToolCall.into(),
                        delta: Some(v1::part_delta::Delta::Content(delta)),
                    })]
                }
            },
            StreamedAssistantContent::Reasoning(reasoning) => {
                let text = reasoning.display_text();
                let (part_id, is_new, mut events) = self.ensure_content_part(ActiveContentKind::Thinking, reasoning.id);
                self.append_to_active_content(&text);
                if is_new {
                    events.push(v1::infer_response::Event::PartStart(v1::PartStart {
                        part_id: part_id.clone(),
                        kind: v1::MessagePartKind::Thinking.into(),
                        tool_name: None,
                        tool_call_id: None,
                    }));
                }
                events.push(v1::infer_response::Event::PartDelta(v1::PartDelta {
                    part_id,
                    kind: v1::MessagePartKind::Thinking.into(),
                    delta: Some(v1::part_delta::Delta::Content(text)),
                }));
                events
            }
            StreamedAssistantContent::ReasoningDelta { id, reasoning } => {
                let (part_id, is_new, mut events) = self.ensure_content_part(ActiveContentKind::Thinking, id);
                self.append_to_active_content(&reasoning);
                if is_new {
                    events.push(v1::infer_response::Event::PartStart(v1::PartStart {
                        part_id: part_id.clone(),
                        kind: v1::MessagePartKind::Thinking.into(),
                        tool_name: None,
                        tool_call_id: None,
                    }));
                }
                events.push(v1::infer_response::Event::PartDelta(v1::PartDelta {
                    part_id,
                    kind: v1::MessagePartKind::Thinking.into(),
                    delta: Some(v1::part_delta::Delta::Content(reasoning)),
                }));
                events
            }
            // Provider-specific final, ignore (FinalResponse handles actual completion)
            StreamedAssistantContent::Final(_) => vec![],
        }
    }

    fn process_user(&mut self, content: StreamedUserContent) -> Vec<v1::infer_response::Event> {
        match content {
            StreamedUserContent::ToolResult {
                tool_result,
                internal_call_id,
            } => {
                let content_str = tool_result
                    .content
                    .iter()
                    .filter_map(|c| match c {
                        rig::message::ToolResultContent::Text(t) => Some(t.text.clone()),
                        rig::message::ToolResultContent::Image(_) => {
                            tracing::warn!(tool_result_id = %tool_result.id, "image content in tool result not yet supported, dropping");
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");
                self.tool_results.insert(internal_call_id.clone(), content_str.clone());
                // Ensure this tool call has an entry in ordered_parts (may not exist if
                // only a result arrived without a preceding ToolCall event)
                if !self.ordered_parts.iter().any(|p| matches!(p, OrderedPart::ToolCallWithResult { internal_call_id: id } if id == &internal_call_id)) {
                    self.ordered_parts.push(OrderedPart::ToolCallWithResult {
                        internal_call_id: internal_call_id.clone(),
                    });
                }
                // When tool result arrives:
                // 1. Send PartEnd for the TOOL_CALL (now it's actually complete)
                // 2. Send PartStart + PartDelta + PartEnd for the TOOL_RESULT
                vec![
                    // Complete the tool call (use internal_call_id to match the PartStart)
                    v1::infer_response::Event::PartEnd(v1::PartEnd {
                        part_id: internal_call_id.clone(),
                        status: v1::MessagePartStatus::Complete.into(),
                    }),
                    // Start the tool result
                    v1::infer_response::Event::PartStart(v1::PartStart {
                        part_id: format!("{}_result", internal_call_id),
                        kind: v1::MessagePartKind::ToolResult.into(),
                        tool_name: None,
                        tool_call_id: Some(internal_call_id.clone()),
                    }),
                    v1::infer_response::Event::PartDelta(v1::PartDelta {
                        part_id: format!("{}_result", internal_call_id),
                        kind: v1::MessagePartKind::ToolResult.into(),
                        delta: Some(v1::part_delta::Delta::Content(content_str)),
                    }),
                    v1::infer_response::Event::PartEnd(v1::PartEnd {
                        part_id: format!("{}_result", internal_call_id),
                        status: v1::MessagePartStatus::Complete.into(),
                    }),
                ]
            }
        }
    }

    /// Builds v1::MessageParts in arrival order — text, thinking, tool calls, and
    /// tool results are all interleaved according to when they were received.
    fn build_parts(&mut self) -> Vec<v1::MessagePart> {
        let mut parts = Vec::new();

        // Prepend metadata part with agent identity info when available
        if self.agent_name.is_some() {
            let metadata_content = serde_json::json!({
                "model_id": self.model_id,
                "agent_name": self.agent_name.as_deref().unwrap_or_default(),
            });
            parts.push(v1::MessagePart {
                id: self.next_part_id(),
                kind: v1::MessagePartKind::Metadata.into(),
                status: v1::MessagePartStatus::Complete.into(),
                content: Some(metadata_content.to_string()),
                tool_call: None,
                tool_result: None,
                media: None,
                summary: None,
            });
        }

        let mut tool_calls = std::mem::take(&mut self.tool_calls);
        let mut tool_results = std::mem::take(&mut self.tool_results);

        for ordered_part in std::mem::take(&mut self.ordered_parts) {
            match ordered_part {
                OrderedPart::Text { id, content } => {
                    if !content.is_empty() {
                        parts.push(v1::MessagePart {
                            id,
                            kind: v1::MessagePartKind::Text.into(),
                            status: v1::MessagePartStatus::Complete.into(),
                            content: Some(content),
                            tool_call: None,
                            tool_result: None,
                            media: None,
                            summary: None,
                        });
                    }
                }
                OrderedPart::Thinking { id, content } => {
                    if !content.is_empty() {
                        parts.push(v1::MessagePart {
                            id,
                            kind: v1::MessagePartKind::Thinking.into(),
                            status: v1::MessagePartStatus::Complete.into(),
                            content: Some(content),
                            tool_call: None,
                            tool_result: None,
                            media: None,
                            summary: None,
                        });
                    }
                }
                OrderedPart::ToolCallWithResult { internal_call_id } => {
                    // Emit the tool call
                    if let Some(tc) = tool_calls.remove(&internal_call_id) {
                        let args_struct: pbjson_types::Struct = serde_json::from_str(&tc.arguments_json)
                            .unwrap_or_else(|e| {
                                tracing::error!(error = e.to_string(), "Failed to deserialize tool call args");
                                pbjson_types::Struct::default()
                            });
                        parts.push(v1::MessagePart {
                            id: internal_call_id.clone(),
                            kind: v1::MessagePartKind::ToolCall.into(),
                            status: v1::MessagePartStatus::Complete.into(),
                            content: None,
                            tool_call: Some(v1::ToolCall {
                                id: internal_call_id.clone(),
                                name: tc.name.to_tool_display_name(),
                                arguments: Some(args_struct),
                                status: v1::ToolExecutionStatus::Completed.into(),
                            }),
                            tool_result: None,
                            media: None,
                            summary: None,
                        });
                    }
                    // Emit the tool result immediately after its call
                    if let Some(result) = tool_results.remove(&internal_call_id) {
                        parts.push(v1::MessagePart {
                            id: format!("{}_result", internal_call_id),
                            kind: v1::MessagePartKind::ToolResult.into(),
                            status: v1::MessagePartStatus::Complete.into(),
                            content: None,
                            tool_call: None,
                            tool_result: Some(v1::ToolResult {
                                tool_call_id: internal_call_id,
                                result,
                                status: v1::ToolExecutionStatus::Completed.into(),
                            }),
                            media: None,
                            summary: None,
                        });
                    }
                }
            }
        }

        parts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::{
        OneOrMany,
        message::{Reasoning, Text, ToolCall, ToolFunction, ToolResult, ToolResultContent},
    };

    // Helper to create text stream item
    fn text_item(text: &str) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(Text { text: text.to_string() }))
    }

    // Helper to create reasoning stream item
    fn reasoning_item(text: &str, id: Option<&str>) -> MultiTurnStreamItem<()> {
        let reasoning = Reasoning::new(text).optional_id(id.map(|s| s.to_string()));
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Reasoning(reasoning))
    }

    // Helper to create reasoning delta stream item
    fn reasoning_delta_item(text: &str, id: Option<&str>) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ReasoningDelta {
            id: id.map(|s| s.to_string()),
            reasoning: text.to_string(),
        })
    }

    // Helper to create tool call stream item
    fn tool_call_item(
        id: &str,
        internal_call_id: &str,
        name: &str,
        args: serde_json::Value,
    ) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ToolCall {
            tool_call: ToolCall::new(
                id.to_string(),
                ToolFunction {
                    name: name.to_string(),
                    arguments: args,
                },
            ),
            internal_call_id: internal_call_id.to_string(),
        })
    }

    // Helper to create tool call delta (name) stream item
    fn tool_call_name_item(id: &str, internal_call_id: &str, name: &str) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ToolCallDelta {
            id: id.to_string(),
            internal_call_id: internal_call_id.to_string(),
            content: ToolCallDeltaContent::Name(name.to_string()),
        })
    }

    // Helper to create tool call delta (args) stream item
    fn tool_call_delta_item(id: &str, internal_call_id: &str, delta: &str) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::ToolCallDelta {
            id: id.to_string(),
            internal_call_id: internal_call_id.to_string(),
            content: ToolCallDeltaContent::Delta(delta.to_string()),
        })
    }

    // Helper to create tool result stream item
    fn tool_result_item(internal_call_id: &str, result: &str) -> MultiTurnStreamItem<()> {
        MultiTurnStreamItem::StreamUserItem(StreamedUserContent::ToolResult {
            tool_result: ToolResult {
                id: "provider_id".to_string(), // Provider ID (not used for correlation)
                call_id: None,
                content: OneOrMany::one(ToolResultContent::Text(Text {
                    text: result.to_string(),
                })),
            },
            internal_call_id: internal_call_id.to_string(),
        })
    }

    mod process_item {
        use super::*;

        // Note: FinalResponse test removed because rig's FinalResponse is crate-private.
        // The behavior is tested implicitly through the stream loop in ollama/openai.

        #[test]
        fn should_return_part_start_then_delta_for_first_text() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events = builder.process_item(text_item("Hello"));

            assert_eq!(events.len(), 2);
            assert!(
                matches!(&events[0], v1::infer_response::Event::PartStart(s) if s.kind == i32::from(v1::MessagePartKind::Text))
            );
            assert!(
                matches!(&events[1], v1::infer_response::Event::PartDelta(d) if d.kind == i32::from(v1::MessagePartKind::Text))
            );
            if let v1::infer_response::Event::PartDelta(delta) = &events[1] {
                assert!(matches!(&delta.delta, Some(v1::part_delta::Delta::Content(c)) if c == "Hello"));
            }
        }

        #[test]
        fn should_return_same_part_id_for_consecutive_text() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events1 = builder.process_item(text_item("Hello "));
            let events2 = builder.process_item(text_item("world"));

            // First call: PartStart + PartDelta, second call: just PartDelta
            assert_eq!(events1.len(), 2);
            assert_eq!(events2.len(), 1);

            let id1 = match &events1[1] {
                v1::infer_response::Event::PartDelta(d) => d.part_id.clone(),
                _ => panic!("Expected PartDelta"),
            };
            let id2 = match &events2[0] {
                v1::infer_response::Event::PartDelta(d) => d.part_id.clone(),
                _ => panic!("Expected PartDelta"),
            };

            assert_eq!(id1, id2);
        }

        #[test]
        fn should_return_part_start_then_delta_for_first_thinking() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events = builder.process_item(reasoning_item("thinking...", None));

            assert_eq!(events.len(), 2);
            assert!(
                matches!(&events[0], v1::infer_response::Event::PartStart(s) if s.kind == i32::from(v1::MessagePartKind::Thinking))
            );
            assert!(
                matches!(&events[1], v1::infer_response::Event::PartDelta(d) if d.kind == i32::from(v1::MessagePartKind::Thinking))
            );
        }

        #[test]
        fn should_return_part_start_and_delta_for_tool_call() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events = builder.process_item(tool_call_item(
                "call_1",
                "internal_1",
                "get_weather",
                serde_json::json!({}),
            ));

            // Tool call should NOT send PartEnd - that happens when the tool result arrives
            assert_eq!(events.len(), 2);
            // First event should be PartStart - part_id uses internal_call_id for reliable correlation
            if let v1::infer_response::Event::PartStart(start) = &events[0] {
                assert_eq!(start.kind, i32::from(v1::MessagePartKind::ToolCall));
                assert_eq!(start.part_id, "internal_1");
                assert_eq!(start.tool_name, Some("get_weather".to_string()));
            } else {
                panic!("Expected PartStart as first event");
            }
            // Second event should be PartDelta with arguments
            assert!(
                matches!(&events[1], v1::infer_response::Event::PartDelta(d) if d.kind == i32::from(v1::MessagePartKind::ToolCall) && d.part_id == "internal_1")
            );
        }

        #[test]
        fn should_return_part_start_for_tool_call_name() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events = builder.process_item(tool_call_name_item("call_1", "internal_1", "search"));

            assert_eq!(events.len(), 1);
            if let v1::infer_response::Event::PartStart(start) = &events[0] {
                assert_eq!(start.kind, i32::from(v1::MessagePartKind::ToolCall));
                assert_eq!(start.tool_name, Some("search".to_string()));
            } else {
                panic!("Expected PartStart");
            }
        }

        #[test]
        fn should_complete_tool_call_and_emit_tool_result_events() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            let events = builder.process_item(tool_result_item("call_1", "72°F"));

            // Tool result should:
            // 1. Complete the tool call (v1::PartEnd for call_1)
            // 2. Start, delta, and end the tool result (call_1_result)
            assert_eq!(events.len(), 4);

            // First event: PartEnd for the tool call
            if let v1::infer_response::Event::PartEnd(end) = &events[0] {
                assert_eq!(end.part_id, "call_1");
                assert_eq!(end.status, i32::from(v1::MessagePartStatus::Complete));
            } else {
                panic!("Expected PartEnd for tool call as first event");
            }

            // Second event: PartStart for tool result
            if let v1::infer_response::Event::PartStart(start) = &events[1] {
                assert_eq!(start.kind, i32::from(v1::MessagePartKind::ToolResult));
                assert_eq!(start.part_id, "call_1_result");
                assert_eq!(start.tool_call_id, Some("call_1".to_string()));
            } else {
                panic!("Expected PartStart for tool result as second event");
            }

            // Third event: PartDelta with result content
            assert!(
                matches!(&events[2], v1::infer_response::Event::PartDelta(d) if d.kind == i32::from(v1::MessagePartKind::ToolResult) && d.part_id == "call_1_result")
            );

            // Fourth event: PartEnd for tool result
            if let v1::infer_response::Event::PartEnd(end) = &events[3] {
                assert_eq!(end.part_id, "call_1_result");
                assert_eq!(end.status, i32::from(v1::MessagePartStatus::Complete));
            } else {
                panic!("Expected PartEnd for tool result as fourth event");
            }
        }

        #[test]
        fn should_emit_part_end_and_part_start_when_switching_kinds() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            // Start with thinking
            let events1 = builder.process_item(reasoning_item("thinking...", None));
            assert_eq!(events1.len(), 2); // PartStart + PartDelta

            let thinking_part_id = match &events1[0] {
                v1::infer_response::Event::PartStart(s) => s.part_id.clone(),
                _ => panic!("Expected PartStart"),
            };

            // Switch to text — should emit PartEnd for thinking, then PartStart + PartDelta for text
            let events2 = builder.process_item(text_item("Hello"));
            assert_eq!(events2.len(), 3); // PartEnd(thinking) + PartStart(text) + PartDelta(text)

            // First event: PartEnd for thinking
            if let v1::infer_response::Event::PartEnd(end) = &events2[0] {
                assert_eq!(end.part_id, thinking_part_id);
                assert_eq!(end.status, i32::from(v1::MessagePartStatus::Complete));
            } else {
                panic!("Expected PartEnd for thinking as first event");
            }

            // Second event: PartStart for text
            assert!(
                matches!(&events2[1], v1::infer_response::Event::PartStart(s) if s.kind == i32::from(v1::MessagePartKind::Text))
            );

            // Third event: PartDelta for text
            assert!(
                matches!(&events2[2], v1::infer_response::Event::PartDelta(d) if d.kind == i32::from(v1::MessagePartKind::Text))
            );
        }

        #[test]
        fn should_end_active_content_part_when_tool_call_arrives() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);

            // Start with text
            let events1 = builder.process_item(text_item("Hello"));
            assert_eq!(events1.len(), 2);

            let text_part_id = match &events1[0] {
                v1::infer_response::Event::PartStart(s) => s.part_id.clone(),
                _ => panic!("Expected PartStart"),
            };

            // Tool call arrives — should emit PartEnd for text, then tool call events
            let events2 = builder.process_item(tool_call_item("call_1", "internal_1", "search", serde_json::json!({})));
            assert_eq!(events2.len(), 3); // PartEnd(text) + PartStart(tool) + PartDelta(tool)

            if let v1::infer_response::Event::PartEnd(end) = &events2[0] {
                assert_eq!(end.part_id, text_part_id);
            } else {
                panic!("Expected PartEnd for text");
            }

            assert!(
                matches!(&events2[1], v1::infer_response::Event::PartStart(s) if s.kind == i32::from(v1::MessagePartKind::ToolCall))
            );
        }
    }

    mod into_message {
        use super::*;

        #[test]
        fn should_accumulate_text_content() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello "));
            builder.process_item(text_item("world!"));

            let message = builder.into_message();
            let text_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::Text));

            assert!(text_part.is_some());
            assert_eq!(text_part.unwrap().content, Some("Hello world!".to_string()));
            assert_eq!(message.role, "assistant");
        }

        #[test]
        fn should_produce_text_part() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello"));

            let message = builder.into_message();

            assert_eq!(message.parts.len(), 1);
            assert_eq!(message.parts[0].kind, i32::from(v1::MessagePartKind::Text));
            assert_eq!(message.parts[0].status, i32::from(v1::MessagePartStatus::Complete));
            assert_eq!(message.parts[0].content, Some("Hello".to_string()));
        }

        #[test]
        fn should_produce_thinking_part() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(reasoning_item("Let me ", Some("think_1")));
            builder.process_item(reasoning_delta_item("think...", Some("think_1")));

            let message = builder.into_message();

            let thinking = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::Thinking));
            assert!(thinking.is_some());
            assert_eq!(thinking.unwrap().content, Some("Let me think...".to_string()));
        }

        #[test]
        fn should_produce_tool_call_part() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(tool_call_item(
                "call_1",
                "internal_1",
                "get_weather",
                serde_json::json!({"city": "Seattle"}),
            ));

            let message = builder.into_message();

            let tool_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::ToolCall));
            assert!(tool_part.is_some());
            let tc = tool_part.unwrap().tool_call.as_ref().unwrap();
            // Tool call ID in message uses internal_call_id for reliable correlation
            assert_eq!(tc.id, "internal_1");
            assert_eq!(tc.name, "get_weather");
        }

        #[test]
        fn should_produce_tool_result_part() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(tool_result_item("call_1", "72°F and sunny"));

            let message = builder.into_message();

            let result_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::ToolResult));
            assert!(result_part.is_some());
            // Tool result part ID should be {tool_call_id}_result
            assert_eq!(result_part.unwrap().id, "call_1_result");
            let tr = result_part.unwrap().tool_result.as_ref().unwrap();
            assert_eq!(tr.tool_call_id, "call_1");
            assert_eq!(tr.result, "72°F and sunny");
        }

        #[test]
        fn should_produce_empty_message_when_no_items() {
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            let message = builder.into_message();

            assert!(message.parts.is_empty());
        }

        #[test]
        fn should_include_all_part_types_in_arrival_order() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello"));
            builder.process_item(reasoning_item("thinking", None));
            builder.process_item(tool_call_item("call_1", "internal_1", "tool", serde_json::json!({})));
            builder.process_item(tool_result_item("internal_1", "result")); // Use internal_call_id to correlate

            let message = builder.into_message();

            assert_eq!(message.parts.len(), 4);
            // Parts should be in arrival order: Text, Thinking, ToolCall, ToolResult
            assert_eq!(message.parts[0].kind, i32::from(v1::MessagePartKind::Text));
            assert_eq!(message.parts[1].kind, i32::from(v1::MessagePartKind::Thinking));
            assert_eq!(message.parts[2].kind, i32::from(v1::MessagePartKind::ToolCall));
            assert_eq!(message.parts[3].kind, i32::from(v1::MessagePartKind::ToolResult));
        }

        #[test]
        fn should_set_complete_status_on_all_parts() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello"));
            builder.process_item(reasoning_item("thinking", None));

            let message = builder.into_message();

            for part in &message.parts {
                assert_eq!(part.status, i32::from(v1::MessagePartStatus::Complete));
            }
        }

        #[test]
        fn should_store_tool_name_from_delta_name_event() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            // Simulate streaming: first name, then argument deltas
            builder.process_item(tool_call_name_item("call_1", "internal_1", "search_web"));
            builder.process_item(tool_call_delta_item("call_1", "internal_1", r#"{"query":"#));
            builder.process_item(tool_call_delta_item("call_1", "internal_1", r#""rust"}"#));

            let message = builder.into_message();

            let tool_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::ToolCall));
            assert!(tool_part.is_some());
            let tc = tool_part.unwrap().tool_call.as_ref().unwrap();
            // Tool call ID in message uses internal_call_id for reliable correlation
            assert_eq!(tc.id, "internal_1");
            assert_eq!(tc.name, "search_web", "tool name should be stored from Name delta");
        }

        #[test]
        fn should_preserve_arrival_order_with_tools_interleaved() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            // Simulate: thinking → tool_call → tool_result → text
            builder.process_item(reasoning_item("let me search", None));
            builder.process_item(tool_call_item(
                "call_1",
                "internal_1",
                "search",
                serde_json::json!({"q": "rust"}),
            ));
            builder.process_item(tool_result_item("internal_1", "found docs"));
            builder.process_item(text_item("Here's what I found"));

            let message = builder.into_message();

            assert_eq!(message.parts.len(), 4);
            assert_eq!(message.parts[0].kind, i32::from(v1::MessagePartKind::Thinking));
            assert_eq!(message.parts[0].content, Some("let me search".to_string()));
            assert_eq!(message.parts[1].kind, i32::from(v1::MessagePartKind::ToolCall));
            assert_eq!(message.parts[1].tool_call.as_ref().unwrap().name, "search");
            assert_eq!(message.parts[2].kind, i32::from(v1::MessagePartKind::ToolResult));
            assert_eq!(message.parts[2].tool_result.as_ref().unwrap().result, "found docs");
            assert_eq!(message.parts[3].kind, i32::from(v1::MessagePartKind::Text));
            assert_eq!(message.parts[3].content, Some("Here's what I found".to_string()));
        }

        #[test]
        fn should_produce_separate_parts_when_interleaved() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            // Simulate: reasoning → text → reasoning → text
            builder.process_item(reasoning_item("thinking step 1", None));
            builder.process_item(text_item("response 1"));
            builder.process_item(reasoning_item("thinking step 2", None));
            builder.process_item(text_item("response 2"));

            let message = builder.into_message();

            // Should produce 4 separate content parts in arrival order
            let content_parts: Vec<_> = message
                .parts
                .iter()
                .filter(|p| {
                    p.kind == i32::from(v1::MessagePartKind::Text) || p.kind == i32::from(v1::MessagePartKind::Thinking)
                })
                .collect();
            assert_eq!(content_parts.len(), 4, "should have 4 interleaved content parts");

            // Verify order: Thinking, Text, Thinking, Text
            assert_eq!(content_parts[0].kind, i32::from(v1::MessagePartKind::Thinking));
            assert_eq!(content_parts[0].content, Some("thinking step 1".to_string()));
            assert_eq!(content_parts[1].kind, i32::from(v1::MessagePartKind::Text));
            assert_eq!(content_parts[1].content, Some("response 1".to_string()));
            assert_eq!(content_parts[2].kind, i32::from(v1::MessagePartKind::Thinking));
            assert_eq!(content_parts[2].content, Some("thinking step 2".to_string()));
            assert_eq!(content_parts[3].kind, i32::from(v1::MessagePartKind::Text));
            assert_eq!(content_parts[3].content, Some("response 2".to_string()));

            // All parts should have unique IDs
            let ids: Vec<_> = content_parts.iter().map(|p| &p.id).collect();
            for (i, id) in ids.iter().enumerate() {
                for (j, other_id) in ids.iter().enumerate() {
                    if i != j {
                        assert_ne!(id, other_id, "part IDs must be unique");
                    }
                }
            }
        }
    }

    mod into_cancelled_message {
        use super::*;
        use proto_rs::rig::v1::MessageStatus;

        #[test]
        fn should_set_cancelled_status_on_message() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello"));

            let message = builder.into_cancelled_message();

            assert_eq!(
                message.status,
                i32::from(MessageStatus::Cancelled),
                "message status should be Cancelled"
            );
        }

        #[test]
        fn should_preserve_accumulated_text_content() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(text_item("Hello "));
            builder.process_item(text_item("world"));

            let message = builder.into_cancelled_message();

            let text_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::Text));
            assert!(text_part.is_some(), "should have text part");
            assert_eq!(
                text_part.unwrap().content,
                Some("Hello world".to_string()),
                "should preserve accumulated text"
            );
        }

        #[test]
        fn should_preserve_accumulated_thinking_content() {
            let mut builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            builder.process_item(reasoning_item("thinking ", None));
            builder.process_item(reasoning_delta_item("more", None));

            let message = builder.into_cancelled_message();

            let thinking_part = message
                .parts
                .iter()
                .find(|p| p.kind == i32::from(v1::MessagePartKind::Thinking));
            assert!(thinking_part.is_some(), "should have thinking part");
            assert_eq!(
                thinking_part.unwrap().content,
                Some("thinking more".to_string()),
                "should preserve accumulated thinking"
            );
        }

        #[test]
        fn should_produce_empty_message_when_cancelled_immediately() {
            let builder = MessageBuilder::new("conv_test".into(), "test-model".into(), None);
            let message = builder.into_cancelled_message();

            assert!(
                message.parts.is_empty(),
                "should have no parts when cancelled immediately"
            );
            assert_eq!(message.status, i32::from(MessageStatus::Cancelled));
        }
    }
}
