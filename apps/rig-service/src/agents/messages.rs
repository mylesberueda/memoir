use crate::api::message::{Message, MessagePart, MessageRole};
use ::rig::{
    OneOrMany,
    agent::Text,
    message::{AssistantContent, UserContent},
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum MessageConversionError {
    #[error("message has no content parts")]
    EmptyParts,
}

impl TryFrom<Message> for ::rig::message::Message {
    type Error = MessageConversionError;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        match msg.role() {
            MessageRole::User => {
                let contents: Vec<UserContent> = msg
                    .parts
                    .iter()
                    .filter_map(|part| match part {
                        MessagePart::Text { content, .. } => Some(UserContent::Text(Text { text: content.clone() })),
                        MessagePart::ToolResult { result, .. } => {
                            Some(UserContent::ToolResult(::rig::message::ToolResult {
                                id: result.tool_call_id.clone(),
                                call_id: None,
                                content: OneOrMany::one(::rig::message::ToolResultContent::text(&result.result)),
                            }))
                        }
                        MessagePart::Document {
                            id, filename, summary, ..
                        } => {
                            let summary_text = summary.as_deref().unwrap_or("No summary available.");
                            Some(UserContent::Document(::rig::message::Document {
                                data: ::rig::message::DocumentSourceKind::String(format!(
                                    "[Attached: {filename} (pid: {id})]\n{summary_text}",
                                )),
                                media_type: None,
                                additional_params: None,
                            }))
                        }
                        _ => None,
                    })
                    .collect();

                let content = OneOrMany::many(contents).map_err(|_| MessageConversionError::EmptyParts)?;

                Ok(::rig::message::Message::User { content })
            }
            MessageRole::Assistant => {
                let contents: Vec<AssistantContent> = msg
                    .parts
                    .iter()
                    .filter_map(|part| match part {
                        MessagePart::Text { content, .. } => {
                            Some(AssistantContent::Text(Text { text: content.clone() }))
                        }
                        MessagePart::Thinking { content, .. } => {
                            Some(AssistantContent::Reasoning(::rig::message::Reasoning::new(content)))
                        }
                        MessagePart::ToolCall { call, .. } => {
                            Some(AssistantContent::ToolCall(::rig::message::ToolCall::new(
                                call.id.clone(),
                                ::rig::message::ToolFunction::new(call.name.clone(), call.arguments.clone()),
                            )))
                        }
                        _ => None,
                    })
                    .collect();

                let content = OneOrMany::many(contents).map_err(|_| MessageConversionError::EmptyParts)?;

                Ok(::rig::message::Message::Assistant {
                    id: Some(msg.pid),
                    content,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::message::{MessageStatus, ToolCallData, ToolResultData};

    mod to_rig {
        use super::*;

        #[test]
        fn should_convert_user_text_message() {
            let msg = Message::full(
                "msg_1".into(),
                MessageRole::User,
                vec![MessagePart::Text {
                    id: "p1".into(),
                    content: "hello".into(),
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let rig_msg: ::rig::message::Message = msg.try_into().unwrap();

            match rig_msg {
                ::rig::message::Message::User { content } => {
                    assert_eq!(content.len(), 1);
                }
                other => panic!("expected User, got {:?}", other),
            }
        }

        #[test]
        fn should_convert_assistant_with_text_and_thinking() {
            let msg = Message::full(
                "msg_2".into(),
                MessageRole::Assistant,
                vec![
                    MessagePart::Thinking {
                        id: "t1".into(),
                        content: "let me think".into(),
                    },
                    MessagePart::Text {
                        id: "p1".into(),
                        content: "here's the answer".into(),
                    },
                ],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let rig_msg: ::rig::message::Message = msg.try_into().unwrap();

            match rig_msg {
                ::rig::message::Message::Assistant { id, content } => {
                    assert_eq!(id, Some("msg_2".into()));
                    assert_eq!(content.len(), 2);
                }
                other => panic!("expected Assistant, got {:?}", other),
            }
        }

        #[test]
        fn should_convert_assistant_tool_call() {
            let msg = Message::full(
                "msg_3".into(),
                MessageRole::Assistant,
                vec![MessagePart::ToolCall {
                    id: "tc1".into(),
                    call: ToolCallData {
                        id: "call_1".into(),
                        name: "get_weather".into(),
                        arguments: serde_json::json!({"city": "Seattle"}),
                    },
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let rig_msg: ::rig::message::Message = msg.try_into().unwrap();

            match rig_msg {
                ::rig::message::Message::Assistant { content, .. } => {
                    assert_eq!(content.len(), 1);
                }
                other => panic!("expected Assistant, got {:?}", other),
            }
        }

        #[test]
        fn should_convert_user_tool_result() {
            let msg = Message::full(
                "msg_4".into(),
                MessageRole::User,
                vec![MessagePart::ToolResult {
                    id: "tr1".into(),
                    result: ToolResultData {
                        tool_call_id: "call_1".into(),
                        result: "72°F".into(),
                    },
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let rig_msg: ::rig::message::Message = msg.try_into().unwrap();

            match rig_msg {
                ::rig::message::Message::User { content } => {
                    assert_eq!(content.len(), 1);
                }
                other => panic!("expected User, got {:?}", other),
            }
        }

        #[test]
        fn should_convert_user_document_with_summary() {
            let msg = Message::full(
                "msg_5".into(),
                MessageRole::User,
                vec![MessagePart::Document {
                    id: "doc_1".into(),
                    filename: "report.pdf".into(),
                    summary: Some("Q4 financials".into()),
                    media: None,
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let rig_msg: ::rig::message::Message = msg.try_into().unwrap();

            match rig_msg {
                ::rig::message::Message::User { content } => {
                    assert_eq!(content.len(), 1);
                }
                other => panic!("expected User, got {:?}", other),
            }
        }

        #[test]
        fn should_return_an_error_when_message_has_no_rig_supported_parts() {
            let msg = Message::full(
                "msg_6".into(),
                MessageRole::Assistant,
                vec![MessagePart::Metadata {
                    id: "meta_1".into(),
                    model_id: "gpt-4.1".into(),
                    agent_name: "Startup Agent".into(),
                }],
                MessageStatus::Complete,
                chrono::Utc::now(),
            );

            let result = ::rig::message::Message::try_from(msg);

            assert!(matches!(result, Err(MessageConversionError::EmptyParts)));
        }
    }
}
