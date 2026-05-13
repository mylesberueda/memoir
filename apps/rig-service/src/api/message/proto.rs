use super::*;
use proto_rs::rig::v1;

impl From<v1::Message> for Message {
    fn from(proto: v1::Message) -> Self {
        let role = match proto.role.as_str() {
            "assistant" => MessageRole::Assistant,
            _ => MessageRole::User,
        };

        let status = v1::MessageStatus::try_from(proto.status).unwrap_or(v1::MessageStatus::Complete);
        let status = match status {
            v1::MessageStatus::Complete => MessageStatus::Complete,
            v1::MessageStatus::Cancelled => MessageStatus::Cancelled,
            v1::MessageStatus::Error | v1::MessageStatus::Unspecified => MessageStatus::Error,
        };

        let created_at = chrono::DateTime::parse_from_rfc3339(&proto.created_at)
            .map(|dt| dt.to_utc())
            .unwrap_or_else(|_| chrono::Utc::now());
        let parts = proto.parts.into_iter().filter_map(proto_part_to_message_part).collect();

        Self::full(proto.pid, role, parts, status, created_at)
    }
}

impl From<Message> for v1::Message {
    fn from(msg: Message) -> Self {
        let status: v1::MessageStatus = match msg.status() {
            MessageStatus::Complete => v1::MessageStatus::Complete,
            MessageStatus::Cancelled => v1::MessageStatus::Cancelled,
            MessageStatus::Error => v1::MessageStatus::Error,
        };

        let created_at = msg.created_at().to_rfc3339();
        let pid = msg.pid;
        let role = msg.role.to_string();
        let parts = msg.parts.into_iter().map(message_part_to_proto).collect();

        v1::Message {
            pid,
            role,
            status: status.into(),
            parts,
            created_at,
        }
    }
}

pub(crate) fn proto_part_to_message_part(part: v1::MessagePart) -> Option<MessagePart> {
    let kind = v1::MessagePartKind::try_from(part.kind).ok()?;

    match kind {
        v1::MessagePartKind::Text => Some(MessagePart::Text {
            id: part.id,
            content: part.content?,
        }),
        v1::MessagePartKind::Thinking => Some(MessagePart::Thinking {
            id: part.id,
            content: part.content?,
        }),
        v1::MessagePartKind::ToolCall => {
            let tc = part.tool_call?;
            let arguments = tc
                .arguments
                .as_ref()
                .map(|s| serde_json::to_value(s).unwrap_or_default())
                .unwrap_or_default();
            Some(MessagePart::ToolCall {
                id: part.id,
                call: ToolCallData {
                    id: tc.id,
                    name: tc.name,
                    arguments,
                },
            })
        }
        v1::MessagePartKind::ToolResult => {
            let tr = part.tool_result?;
            Some(MessagePart::ToolResult {
                id: part.id,
                result: ToolResultData {
                    tool_call_id: tr.tool_call_id,
                    result: tr.result,
                },
            })
        }
        v1::MessagePartKind::Document => Some(MessagePart::Document {
            id: part.id,
            filename: part.content.unwrap_or_default(),
            summary: part.summary,
            media: part.media.map(proto_media_to_media_data),
        }),
        v1::MessagePartKind::Metadata => {
            let content = part.content?;
            let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
            Some(MessagePart::Metadata {
                id: part.id,
                model_id: parsed
                    .get("model_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                agent_name: parsed
                    .get("agent_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
            })
        }
        _ => None,
    }
}

pub(crate) fn message_part_to_proto(part: MessagePart) -> v1::MessagePart {
    match part {
        MessagePart::Text { id, content } => v1::MessagePart {
            id,
            kind: v1::MessagePartKind::Text.into(),
            status: v1::MessagePartStatus::Complete.into(),
            content: Some(content),
            tool_call: None,
            tool_result: None,
            media: None,
            summary: None,
        },
        MessagePart::Thinking { id, content } => v1::MessagePart {
            id,
            kind: v1::MessagePartKind::Thinking.into(),
            status: v1::MessagePartStatus::Complete.into(),
            content: Some(content),
            tool_call: None,
            tool_result: None,
            media: None,
            summary: None,
        },
        MessagePart::ToolCall { id, call } => {
            let args_struct: pbjson_types::Struct = serde_json::from_value(call.arguments).unwrap_or_default();
            v1::MessagePart {
                id,
                kind: v1::MessagePartKind::ToolCall.into(),
                status: v1::MessagePartStatus::Complete.into(),
                content: None,
                tool_call: Some(v1::ToolCall {
                    id: call.id,
                    name: call.name,
                    arguments: Some(args_struct),
                    status: v1::ToolExecutionStatus::Completed.into(),
                }),
                tool_result: None,
                media: None,
                summary: None,
            }
        }
        MessagePart::ToolResult { id, result } => v1::MessagePart {
            id,
            kind: v1::MessagePartKind::ToolResult.into(),
            status: v1::MessagePartStatus::Complete.into(),
            content: None,
            tool_call: None,
            tool_result: Some(v1::ToolResult {
                tool_call_id: result.tool_call_id,
                result: result.result,
                status: v1::ToolExecutionStatus::Completed.into(),
            }),
            media: None,
            summary: None,
        },
        MessagePart::Document {
            id,
            filename,
            summary,
            media,
        } => v1::MessagePart {
            id,
            kind: v1::MessagePartKind::Document.into(),
            status: v1::MessagePartStatus::Complete.into(),
            content: Some(filename),
            tool_call: None,
            tool_result: None,
            media: media.map(media_data_to_proto),
            summary,
        },
        MessagePart::Metadata {
            id,
            model_id,
            agent_name,
        } => v1::MessagePart {
            id,
            kind: v1::MessagePartKind::Metadata.into(),
            status: v1::MessagePartStatus::Complete.into(),
            content: Some(serde_json::json!({"model_id": model_id, "agent_name": agent_name}).to_string()),
            tool_call: None,
            tool_result: None,
            media: None,
            summary: None,
        },
    }
}

fn proto_media_to_media_data(media: v1::MediaContent) -> MediaData {
    let source = match media.source {
        Some(v1::media_content::Source::Url(url)) => MediaSource::Url(url),
        Some(v1::media_content::Source::Data(bytes)) => MediaSource::Data(bytes.to_vec()),
        Some(v1::media_content::Source::Base64(b64)) => MediaSource::Base64(b64),
        None => MediaSource::Url(String::new()),
    };
    MediaData {
        source,
        media_type: media.media_type,
    }
}

fn media_data_to_proto(data: MediaData) -> v1::MediaContent {
    let source = match data.source {
        MediaSource::Url(url) => Some(v1::media_content::Source::Url(url)),
        MediaSource::Data(bytes) => Some(v1::media_content::Source::Data(bytes.into())),
        MediaSource::Base64(b64) => Some(v1::media_content::Source::Base64(b64)),
    };
    v1::MediaContent {
        source,
        media_type: data.media_type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_proto_text(pid: &str, role: &str, content: &str) -> v1::Message {
        v1::Message {
            pid: pid.into(),
            role: role.into(),
            status: v1::MessageStatus::Complete.into(),
            parts: vec![v1::MessagePart {
                id: "part_1".into(),
                kind: v1::MessagePartKind::Text.into(),
                status: v1::MessagePartStatus::Complete.into(),
                content: Some(content.into()),
                tool_call: None,
                tool_result: None,
                media: None,
                summary: None,
            }],
            created_at: "2024-01-01T00:00:00+00:00".into(), // 2024-01-01 00:00:00 UTC
        }
    }

    mod from_proto {
        use super::*;

        #[test]
        fn should_convert_user_text_message() {
            let proto = make_proto_text("msg_1", "user", "hello");

            let msg: Message = proto.into();

            assert_eq!(msg.pid(), "msg_1");
            assert_eq!(msg.role(), MessageRole::User);
            assert_eq!(msg.status(), MessageStatus::Complete);
            assert_eq!(msg.text_content(), "hello");
            assert_eq!(msg.created_at().to_rfc3339(), "2024-01-01T00:00:00+00:00");
        }

        #[test]
        fn should_convert_assistant_text_message() {
            let proto = make_proto_text("msg_2", "assistant", "hi there");

            let msg: Message = proto.into();

            assert_eq!(msg.role(), MessageRole::Assistant);
            assert_eq!(msg.text_content(), "hi there");
        }

        #[test]
        fn should_convert_cancelled_status() {
            let mut proto = make_proto_text("msg_3", "assistant", "partial");
            proto.status = v1::MessageStatus::Cancelled.into();

            let msg: Message = proto.into();

            assert_eq!(msg.status(), MessageStatus::Cancelled);
        }

        #[test]
        fn should_convert_tool_call_part() {
            let proto = v1::Message {
                pid: "msg_tc".into(),
                role: "assistant".into(),
                status: v1::MessageStatus::Complete.into(),
                parts: vec![v1::MessagePart {
                    id: "tc_1".into(),
                    kind: v1::MessagePartKind::ToolCall.into(),
                    status: v1::MessagePartStatus::Complete.into(),
                    content: None,
                    tool_call: Some(v1::ToolCall {
                        id: "call_1".into(),
                        name: "get_weather".into(),
                        arguments: Some(serde_json::from_value(serde_json::json!({"city": "Seattle"})).unwrap()),
                        status: v1::ToolExecutionStatus::Completed.into(),
                    }),
                    tool_result: None,
                    media: None,
                    summary: None,
                }],
                created_at: "2024-01-01T00:00:00+00:00".into(),
            };

            let msg: Message = proto.into();

            match &msg.parts()[0] {
                MessagePart::ToolCall { id, call } => {
                    assert_eq!(id, "tc_1");
                    assert_eq!(call.id, "call_1");
                    assert_eq!(call.name, "get_weather");
                    assert_eq!(call.arguments, serde_json::json!({"city": "Seattle"}));
                }
                other => panic!("expected ToolCall, got {:?}", other),
            }
        }

        #[test]
        fn should_convert_document_part_with_media() {
            let proto = v1::Message {
                pid: "msg_doc".into(),
                role: "user".into(),
                status: v1::MessageStatus::Complete.into(),
                parts: vec![v1::MessagePart {
                    id: "doc_1".into(),
                    kind: v1::MessagePartKind::Document.into(),
                    status: v1::MessagePartStatus::Complete.into(),
                    content: Some("report.pdf".into()),
                    tool_call: None,
                    tool_result: None,
                    media: Some(v1::MediaContent {
                        media_type: Some("application/pdf".into()),
                        source: Some(v1::media_content::Source::Url("s3://bucket/report.pdf".into())),
                    }),
                    summary: Some("Q4 financial report".into()),
                }],
                created_at: "2024-01-01T00:00:00+00:00".into(),
            };

            let msg: Message = proto.into();

            match &msg.parts()[0] {
                MessagePart::Document {
                    filename,
                    summary,
                    media,
                    ..
                } => {
                    assert_eq!(filename, "report.pdf");
                    assert_eq!(summary.as_deref(), Some("Q4 financial report"));
                    let m = media.as_ref().unwrap();
                    assert_eq!(m.media_type.as_deref(), Some("application/pdf"));
                    assert!(matches!(&m.source, MediaSource::Url(u) if u == "s3://bucket/report.pdf"));
                }
                other => panic!("expected Document, got {:?}", other),
            }
        }

        #[test]
        fn should_skip_unrecognized_part_kinds() {
            let proto = v1::Message {
                pid: "msg_skip".into(),
                role: "user".into(),
                status: v1::MessageStatus::Complete.into(),
                parts: vec![
                    v1::MessagePart {
                        id: "p1".into(),
                        kind: v1::MessagePartKind::Text.into(),
                        status: v1::MessagePartStatus::Complete.into(),
                        content: Some("hello".into()),
                        tool_call: None,
                        tool_result: None,
                        media: None,
                        summary: None,
                    },
                    v1::MessagePart {
                        id: "p2".into(),
                        kind: v1::MessagePartKind::Image.into(),
                        status: v1::MessagePartStatus::Complete.into(),
                        content: None,
                        tool_call: None,
                        tool_result: None,
                        media: None,
                        summary: None,
                    },
                ],
                created_at: "2024-01-01T00:00:00+00:00".into(),
            };

            let msg: Message = proto.into();

            assert_eq!(msg.parts().len(), 1);
        }
    }

    mod to_proto {
        use super::*;

        #[test]
        fn should_roundtrip_text_message() {
            let original = make_proto_text("msg_rt", "user", "roundtrip");

            let msg: Message = original.into();
            let back: v1::Message = msg.into();

            assert_eq!(back.pid, "msg_rt");
            assert_eq!(back.role, "user");
            assert_eq!(back.status, i32::from(v1::MessageStatus::Complete));
            assert_eq!(back.parts.len(), 1);
            assert_eq!(back.parts[0].content, Some("roundtrip".into()));
            assert_eq!(back.created_at, "2024-01-01T00:00:00+00:00");
        }

        #[test]
        fn should_convert_tool_result_to_proto() {
            let msg = Message::full(
                "msg_tr".into(),
                MessageRole::User,
                vec![MessagePart::ToolResult {
                    id: "tr_1".into(),
                    result: ToolResultData {
                        tool_call_id: "call_1".into(),
                        result: "72°F".into(),
                    },
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let proto: v1::Message = msg.into();

            assert_eq!(proto.parts.len(), 1);
            let tr = proto.parts[0].tool_result.as_ref().unwrap();
            assert_eq!(tr.tool_call_id, "call_1");
            assert_eq!(tr.result, "72°F");
        }
    }
}
