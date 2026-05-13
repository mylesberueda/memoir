//! Ringbuffer-based conversation history with compaction and snapshot building.
//!
//! History is infrastructure, not a component — every agent pattern uses it.
//! Compaction (LLM-driven summarization of old messages) is an internal concern
//! managed transparently via `push_and_snapshot`.

use crate::{
    agents::error::AgentError,
    api::{
        Ringbuffer,
        message::{Message, MessagePart, MessageRole, MessageStatus},
    },
};
use std::{future::Future, pin::Pin};

/// Minimum number of messages before compaction can trigger, regardless of
/// the threshold ratio. Prevents compaction on tiny histories.
const MIN_COMPACTION_MESSAGES: usize = 5;

pub(crate) struct HistoryComponent {
    history: Ringbuffer<Message>,
    compaction_threshold: f32,
    compaction_keep_ratio: f32,
}

impl HistoryComponent {
    pub(crate) fn new(
        capacity: usize,
        initial_history: Vec<Message>,
        compaction_threshold: f32,
        compaction_keep_ratio: f32,
    ) -> Self {
        let mut ring = Ringbuffer::new(capacity);
        ring.extend(initial_history);
        Self {
            history: ring,
            compaction_threshold,
            compaction_keep_ratio,
        }
    }

    pub(crate) fn push(&mut self, message: Message) {
        self.history.push(message);
    }

    #[allow(dead_code, reason = "Used in tests; available for future agent patterns")]
    pub(crate) fn len(&self) -> usize {
        self.history.len()
    }

    #[expect(
        dead_code,
        reason = "Available for future agent patterns that need direct history access"
    )]
    pub(crate) fn capacity(&self) -> usize {
        self.history.capacity()
    }

    #[expect(
        dead_code,
        reason = "Available for future agent patterns that need direct history access"
    )]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Message> {
        self.history.iter()
    }

    #[expect(
        dead_code,
        reason = "Available for future agent patterns that need direct history access"
    )]
    pub(crate) fn drain(&mut self) -> Vec<Message> {
        self.history.drain()
    }

    #[expect(
        dead_code,
        reason = "Available for future agent patterns that need direct history access"
    )]
    pub(crate) fn extend(&mut self, messages: impl IntoIterator<Item = Message>) {
        self.history.extend(messages);
    }

    /// Push a user message, compact if needed, and return a snapshot ready for
    /// streaming. The `complete_fn` callback is called only if compaction
    /// triggers — it should produce an LLM summary of the provided transcript.
    pub(crate) async fn push_and_snapshot<'f>(
        &mut self,
        message: Message,
        complete_fn: impl FnOnce(String) -> Pin<Box<dyn Future<Output = Result<String, AgentError>> + Send + 'f>>,
    ) -> Vec<Message> {
        let compaction_summary = self.maybe_compact(complete_fn).await;
        self.history.push(message);
        self.build_snapshot(compaction_summary.as_deref())
    }

    /// Build a snapshot of the history for passing to the Rig stream call.
    pub(crate) fn build_snapshot(&self, compaction_summary: Option<&str>) -> Vec<Message> {
        let mut snapshot = Vec::new();

        if let Some(summary) = compaction_summary {
            let summary_text = format!(
                "<compaction_summary role=\"reference_only\">\n\
                 Summary of earlier conversation:\n{summary}\n\
                 </compaction_summary>"
            );
            snapshot.push(
                Message::new(
                    MessageRole::User,
                    vec![MessagePart::Text {
                        id: "compaction_summary".into(),
                        content: summary_text,
                    }],
                )
                .with_pid(nanoid::nanoid!())
                .with_status(MessageStatus::Complete),
            );
        }

        snapshot.extend(self.history.iter().cloned());
        snapshot
    }

    fn needs_compaction(&self) -> bool {
        let threshold_count = (self.history.capacity() as f32 * self.compaction_threshold) as usize;
        self.history.len() >= threshold_count && self.history.len() >= MIN_COMPACTION_MESSAGES
    }

    fn split<'a>(&self, messages: &'a [Message]) -> (&'a [Message], &'a [Message]) {
        let keep_count = (messages.len() as f32 * self.compaction_keep_ratio).max(1.0) as usize;
        let split_at = messages.len().saturating_sub(keep_count);
        messages.split_at(split_at)
    }

    async fn maybe_compact<'f>(
        &mut self,
        complete_fn: impl FnOnce(String) -> Pin<Box<dyn Future<Output = Result<String, AgentError>> + Send + 'f>>,
    ) -> Option<String> {
        if !self.needs_compaction() {
            return None;
        }

        let all_messages = self.history.drain();
        let (to_summarize, to_keep) = self.split(&all_messages);
        let transcript = format_transcript(to_summarize);
        let summarize_prompt = format!(
            "Summarize the following conversation history concisely, preserving key facts, \
             decisions, and context. Focus on information you would need to continue the \
             conversation coherently.\n\n{transcript}"
        );

        let summary_result = complete_fn(summarize_prompt).await;
        self.history.extend(to_keep.iter().cloned());
        summary_result.ok()
    }
}

fn format_message_for_transcript(msg: &Message) -> String {
    let text = msg
        .parts()
        .iter()
        .filter_map(|part| match part {
            MessagePart::Text { content, .. } => Some(content.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    match msg.role() {
        MessageRole::User => format!("User: {text}"),
        MessageRole::Assistant => format!("Assistant: {text}"),
    }
}

fn format_transcript(messages: &[Message]) -> String {
    messages
        .iter()
        .map(format_message_for_transcript)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(role: MessageRole, text: &str) -> Message {
        Message::full(
            nanoid::nanoid!(),
            role,
            vec![MessagePart::Text {
                id: "p1".into(),
                content: text.into(),
            }],
            MessageStatus::Complete,
            chrono::Utc::now(),
        )
    }

    fn user_msg(text: &str) -> Message {
        make_msg(MessageRole::User, text)
    }

    fn assistant_msg(text: &str) -> Message {
        make_msg(MessageRole::Assistant, text)
    }

    /// Default compaction config: threshold=1.0, keep_ratio=0.2
    fn history_with_defaults(capacity: usize, msgs: Vec<Message>) -> HistoryComponent {
        HistoryComponent::new(capacity, msgs, 1.0, 0.2)
    }

    /// Stub complete_fn that returns a fixed summary.
    fn stub_complete(_prompt: String) -> Pin<Box<dyn Future<Output = Result<String, AgentError>> + Send>> {
        Box::pin(async { Ok("Summary of earlier conversation.".to_string()) })
    }

    /// Stub complete_fn that fails.
    fn failing_complete(_prompt: String) -> Pin<Box<dyn Future<Output = Result<String, AgentError>> + Send>> {
        Box::pin(async { Err(AgentError::CompletionError("unavailable".into())) })
    }

    mod construction {
        use super::*;

        #[test]
        fn should_create_with_initial_messages() {
            let msgs = vec![user_msg("hello"), assistant_msg("hi")];
            let comp = history_with_defaults(10, msgs);
            assert_eq!(comp.len(), 2);
        }
    }

    mod build_snapshot {
        use super::*;

        #[test]
        fn should_build_snapshot_without_compaction() {
            let msgs = vec![user_msg("hello"), assistant_msg("hi")];
            let comp = history_with_defaults(10, msgs);

            let snapshot = comp.build_snapshot(None);
            assert_eq!(snapshot.len(), 2);
            assert_eq!(snapshot[0].text_content(), "hello");
            assert_eq!(snapshot[1].text_content(), "hi");
        }

        #[test]
        fn should_build_snapshot_with_compaction_summary() {
            let msgs = vec![user_msg("latest message")];
            let comp = history_with_defaults(10, msgs);

            let snapshot = comp.build_snapshot(Some("Earlier context summary"));
            assert_eq!(snapshot.len(), 2, "should prepend compaction summary");
            assert!(
                snapshot[0].text_content().contains("Earlier context summary"),
                "first message should contain summary"
            );
            assert!(
                snapshot[0].text_content().contains("<compaction_summary"),
                "summary should be wrapped in compaction_summary tags"
            );
            assert_eq!(snapshot[1].text_content(), "latest message");
        }
    }

    mod push_and_snapshot {
        use super::*;

        #[tokio::test]
        async fn should_include_pushed_message_in_snapshot() {
            let mut comp = history_with_defaults(10, vec![]);
            let msg = user_msg("hello");

            let snapshot = comp.push_and_snapshot(msg, stub_complete).await;

            assert_eq!(snapshot.len(), 1);
            assert_eq!(snapshot[0].text_content(), "hello");
        }

        #[tokio::test]
        async fn should_return_snapshot_without_compaction_when_below_threshold() {
            let msgs = vec![user_msg("one"), assistant_msg("two")];
            let mut comp = history_with_defaults(10, msgs);
            let msg = user_msg("three");

            let snapshot = comp.push_and_snapshot(msg, stub_complete).await;

            assert_eq!(snapshot.len(), 3);
            assert_eq!(snapshot[0].text_content(), "one");
            assert_eq!(snapshot[2].text_content(), "three");
        }

        #[tokio::test]
        async fn should_compact_and_include_summary_when_at_threshold() {
            // Capacity 5, fill with 5 messages (hits threshold=1.0 and >= MIN_COMPACTION_MESSAGES)
            let msgs: Vec<_> = (0..5).map(|i| user_msg(&format!("msg {i}"))).collect();
            let mut comp = history_with_defaults(5, msgs);

            let snapshot = comp.push_and_snapshot(user_msg("new message"), stub_complete).await;

            // Should have: compaction_summary + kept messages (20% of 5 = 1) + new message
            assert!(
                snapshot[0].text_content().contains("Summary of earlier conversation"),
                "first message should be compaction summary, got: {}",
                snapshot[0].text_content()
            );
            let last = snapshot.last().unwrap();
            assert_eq!(last.text_content(), "new message");
        }

        #[tokio::test]
        async fn should_skip_summary_when_completion_fails() {
            let msgs: Vec<_> = (0..5).map(|i| user_msg(&format!("msg {i}"))).collect();
            let mut comp = history_with_defaults(5, msgs);

            let snapshot = comp.push_and_snapshot(user_msg("new"), failing_complete).await;

            // No compaction summary — just kept messages + new message
            assert!(
                !snapshot[0].text_content().contains("<compaction_summary"),
                "should not have compaction summary when completion fails"
            );
        }
    }

    mod compaction_logic {
        use super::*;

        #[test]
        fn should_not_trigger_when_buffer_below_threshold() {
            let comp = history_with_defaults(10, vec![user_msg("a"), user_msg("b"), user_msg("c"), user_msg("d")]);
            assert!(!comp.needs_compaction());
        }

        #[test]
        fn should_not_trigger_when_buffer_has_fewer_than_5_messages() {
            let comp = history_with_defaults(4, vec![user_msg("a"), user_msg("b"), user_msg("c"), user_msg("d")]);
            assert!(!comp.needs_compaction());
        }

        #[test]
        fn should_trigger_when_buffer_at_capacity_with_5_or_more() {
            let msgs: Vec<_> = (0..10).map(|i| user_msg(&format!("msg {i}"))).collect();
            let comp = history_with_defaults(10, msgs);
            assert!(comp.needs_compaction());
        }

        #[test]
        fn should_trigger_when_buffer_exceeds_fractional_threshold() {
            let msgs: Vec<_> = (0..9).map(|i| user_msg(&format!("msg {i}"))).collect();
            let comp = HistoryComponent::new(10, msgs, 0.9, 0.2);
            assert!(comp.needs_compaction());
        }

        #[test]
        fn should_not_trigger_just_below_fractional_threshold() {
            let msgs: Vec<_> = (0..8).map(|i| user_msg(&format!("msg {i}"))).collect();
            let comp = HistoryComponent::new(10, msgs, 0.9, 0.2);
            assert!(!comp.needs_compaction());
        }

        #[test]
        fn should_split_80_20_by_default() {
            let comp = history_with_defaults(10, vec![]);
            let msgs: Vec<_> = (0..10).map(|i| user_msg(&format!("msg {i}"))).collect();
            let (to_summarize, to_keep) = comp.split(&msgs);
            assert_eq!(to_summarize.len(), 8);
            assert_eq!(to_keep.len(), 2);
        }

        #[test]
        fn should_keep_at_least_one_message() {
            let comp = HistoryComponent::new(10, vec![], 1.0, 0.0);
            let msgs: Vec<_> = (0..5).map(|i| user_msg(&format!("msg {i}"))).collect();
            let (to_summarize, to_keep) = comp.split(&msgs);
            assert_eq!(to_keep.len(), 1);
            assert_eq!(to_summarize.len(), 4);
        }
    }

    mod format_transcript_tests {
        use super::*;

        #[test]
        fn should_format_user_message_with_user_prefix() {
            let msg = user_msg("What is Rust?");
            let result = format_message_for_transcript(&msg);
            assert_eq!(result, "User: What is Rust?");
        }

        #[test]
        fn should_format_assistant_message_with_assistant_prefix() {
            let msg = assistant_msg("Rust is a systems language.");
            let result = format_message_for_transcript(&msg);
            assert_eq!(result, "Assistant: Rust is a systems language.");
        }

        #[test]
        fn should_skip_non_text_parts() {
            let msg = Message::full(
                nanoid::nanoid!(),
                MessageRole::Assistant,
                vec![MessagePart::Thinking {
                    id: "t1".into(),
                    content: "let me think".into(),
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );
            let result = format_message_for_transcript(&msg);
            assert_eq!(result, "Assistant: ");
        }

        #[test]
        fn should_format_conversation_as_transcript() {
            let msgs = vec![
                user_msg("What is Rust?"),
                assistant_msg("Rust is a systems language."),
                user_msg("Tell me more."),
            ];
            let result = format_transcript(&msgs);
            assert_eq!(
                result,
                "User: What is Rust?\nAssistant: Rust is a systems language.\nUser: Tell me more."
            );
        }
    }
}
