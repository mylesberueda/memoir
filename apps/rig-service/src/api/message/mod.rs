pub(crate) mod proto;

/// Message author role.
#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, strum::Display, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum MessageRole {
    User,
    Assistant,
}

/// Completion status of a message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, strum::Display, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum MessageStatus {
    Complete,
    Cancelled,
    Error,
}

/// A tool call captured as part of an assistant message.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ToolCallData {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) arguments: serde_json::Value,
}

/// The result returned by a tool execution.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ToolResultData {
    pub(crate) tool_call_id: String,
    pub(crate) result: String,
}

/// Media source — mirrors proto `media_content::Source` but owned.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MediaSource {
    Url(String),
    Data(Vec<u8>),
    Base64(String),
}

/// Media attached to a document/image/audio/video part.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MediaData {
    pub(crate) source: MediaSource,
    pub(crate) media_type: Option<String>,
}

/// A single part of a message — typed enum instead of the proto bag-of-optionals.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MessagePart {
    Text {
        id: String,
        content: String,
    },
    Thinking {
        id: String,
        content: String,
    },
    ToolCall {
        id: String,
        call: ToolCallData,
    },
    ToolResult {
        id: String,
        result: ToolResultData,
    },
    Document {
        id: String,
        filename: String,
        summary: Option<String>,
        media: Option<MediaData>,
    },
    Metadata {
        id: String,
        model_id: String,
        agent_name: String,
    },
}

impl MessagePart {
    #[allow(dead_code, reason = "Useful method for later. Also seems to be used in tests.")]
    /// The part's id regardless of variant.
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Text { id, .. }
            | Self::Thinking { id, .. }
            | Self::ToolCall { id, .. }
            | Self::ToolResult { id, .. }
            | Self::Document { id, .. }
            | Self::Metadata { id, .. } => id,
        }
    }
}

/// High-fidelity message type used throughout the app-facing message API.
///
/// Boundary-specific conversions live alongside their boundaries; the core
/// message model is now a plain type rather than a typestate container.
pub(crate) struct Message {
    pub(crate) pid: String,
    pub(crate) role: MessageRole,
    pub(crate) parts: Vec<MessagePart>,
    pub(crate) status: MessageStatus,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self {
            pid: self.pid.clone(),
            role: self.role,
            parts: self.parts.clone(),
            status: self.status,
            created_at: self.created_at,
        }
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("pid", &self.pid)
            .field("role", &self.role)
            .field("parts", &self.parts)
            .field("status", &self.status)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl Message {
    /// The message's role.
    pub(crate) fn role(&self) -> MessageRole {
        self.role
    }

    /// The message's parts.
    pub(crate) fn parts(&self) -> &[MessagePart] {
        &self.parts
    }

    /// Concatenated text content from all Text parts.
    pub(crate) fn text_content(&self) -> String {
        self.parts
            .iter()
            .filter_map(|p| match p {
                MessagePart::Text { content, .. } => Some(content.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Create a new message with generated identity and a complete status.
    pub(crate) fn new(role: MessageRole, parts: Vec<MessagePart>) -> Self {
        Self {
            pid: nanoid::nanoid!(),
            role,
            parts,
            status: MessageStatus::Complete,
            created_at: chrono::Utc::now(),
        }
    }

    /// Construct a fully-formed message directly (all fields known at once).
    pub(crate) fn full(
        pid: String,
        role: MessageRole,
        parts: Vec<MessagePart>,
        status: MessageStatus,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            pid,
            role,
            parts,
            status,
            created_at,
        }
    }

    pub(crate) fn with_pid(mut self, pid: String) -> Self {
        self.pid = pid;
        self
    }

    /// Assign status with `created_at` defaulting to now.
    pub(crate) fn with_status(self, status: MessageStatus) -> Self {
        self.with_status_at(status, chrono::Utc::now())
    }

    /// Assign status with an explicit timestamp.
    pub(crate) fn with_status_at(mut self, status: MessageStatus, created_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.status = status;
        self.created_at = created_at;
        self
    }

    pub(crate) fn pid(&self) -> &str {
        &self.pid
    }

    pub(crate) fn status(&self) -> MessageStatus {
        self.status
    }

    pub(crate) fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_part(id: &str, content: &str) -> MessagePart {
        MessagePart::Text {
            id: id.into(),
            content: content.into(),
        }
    }

    mod construction {
        use super::*;

        #[test]
        fn should_create_content_only_message() {
            let msg = Message::new(MessageRole::User, vec![text_part("p1", "hello")]);

            assert_eq!(msg.role(), MessageRole::User);
            assert_eq!(msg.parts().len(), 1);
            assert!(!msg.pid.is_empty());
            assert_eq!(msg.status, MessageStatus::Complete);
        }

        #[test]
        fn should_create_fully_formed_message() {
            let now = chrono::Utc::now();
            let msg = Message::full(
                "msg_123".into(),
                MessageRole::Assistant,
                vec![text_part("p1", "hi")],
                MessageStatus::Complete,
                now,
            );

            assert_eq!(msg.pid(), "msg_123");
            assert_eq!(msg.role(), MessageRole::Assistant);
            assert_eq!(msg.status(), MessageStatus::Complete);
            assert_eq!(msg.created_at(), now);
        }
    }

    mod mutations {
        use super::*;

        #[test]
        fn should_override_pid() {
            let msg = Message::new(MessageRole::User, vec![text_part("p1", "hello")]);

            let msg = msg.with_pid("msg_abc".into());

            assert_eq!(msg.pid(), "msg_abc");
            assert_eq!(msg.role(), MessageRole::User);
        }

        #[test]
        fn should_override_status() {
            let msg = Message::new(MessageRole::User, vec![text_part("p1", "hello")]);

            let msg = msg.with_status(MessageStatus::Complete);

            assert_eq!(msg.status(), MessageStatus::Complete);
            assert!(msg.created_at().timestamp_millis() > 0);
        }

        #[test]
        fn should_allow_pid_and_status_updates_in_either_order() {
            let parts = vec![text_part("p1", "hello")];

            let msg_a = Message::new(MessageRole::User, parts.clone())
                .with_pid("msg_1".into())
                .with_status(MessageStatus::Complete);

            let msg_b = Message::new(MessageRole::User, parts)
                .with_status(MessageStatus::Complete)
                .with_pid("msg_1".into());

            assert_eq!(msg_a.pid(), msg_b.pid());
            assert_eq!(msg_a.role(), msg_b.role());
            assert_eq!(msg_a.status(), msg_b.status());
        }

        #[test]
        fn should_use_explicit_timestamp_with_status_at() {
            let ts = chrono::DateTime::from_timestamp_millis(1704067200000).unwrap();
            let msg = Message::new(MessageRole::User, vec![]).with_status_at(MessageStatus::Complete, ts);

            assert_eq!(msg.created_at(), ts);
        }
    }

    mod text_content {
        use super::*;

        #[test]
        fn should_concatenate_text_parts() {
            let msg = Message::new(
                MessageRole::Assistant,
                vec![
                    text_part("p1", "Hello "),
                    MessagePart::Thinking {
                        id: "t1".into(),
                        content: "hmm".into(),
                    },
                    text_part("p2", "world!"),
                ],
            );

            assert_eq!(msg.text_content(), "Hello world!");
        }

        #[test]
        fn should_return_empty_string_when_no_text_parts() {
            let msg = Message::new(
                MessageRole::Assistant,
                vec![MessagePart::Thinking {
                    id: "t1".into(),
                    content: "hmm".into(),
                }],
            );

            assert_eq!(msg.text_content(), "");
        }
    }

    mod message_part {
        use super::*;

        #[test]
        fn should_return_id_for_all_variants() {
            let parts = [
                MessagePart::Text {
                    id: "t1".into(),
                    content: "x".into(),
                },
                MessagePart::Thinking {
                    id: "t2".into(),
                    content: "x".into(),
                },
                MessagePart::ToolCall {
                    id: "t3".into(),
                    call: ToolCallData {
                        id: "c1".into(),
                        name: "f".into(),
                        arguments: serde_json::json!({}),
                    },
                },
                MessagePart::ToolResult {
                    id: "t4".into(),
                    result: ToolResultData {
                        tool_call_id: "c1".into(),
                        result: "ok".into(),
                    },
                },
                MessagePart::Document {
                    id: "t5".into(),
                    filename: "f.pdf".into(),
                    summary: None,
                    media: None,
                },
            ];

            let ids: Vec<&str> = parts.iter().map(|p| p.id()).collect();
            assert_eq!(ids, vec!["t1", "t2", "t3", "t4", "t5"]);
        }
    }
}
