use proto_rs::chat::v1::{ChatMessage, SendMessageRequest, chat_message::Sender};
use sea_orm::ActiveValue::Set;

use crate::models::MessageKind;

pub(crate) use super::_entity::channel_events::*;

impl ActiveModel {
    pub(crate) fn from_chat_message(channel_id: &str, message: &ChatMessage) -> Self {
        let (event_type, user_id) = match &message.sender {
            Some(Sender::User(user)) => (MessageKind::User, Some(user.user_id.clone())),
            Some(Sender::Agent(agent)) => (MessageKind::Agent, Some(agent.agent_pid.clone())),
            None => (MessageKind::Unknown, None),
        };

        let content = serde_json::json!({
          "content": message.content,
          "parent_pid": message.parent_pid
        });

        let timestamp = chrono::DateTime::parse_from_rfc3339(&message.timestamp)
            .map(|dt| dt.to_utc().into())
            .unwrap_or(chrono::Utc::now().into());

        ActiveModel {
            pid: Set(message.pid.clone()),
            channel_id: Set(channel_id.to_string()),
            message_kind: Set(event_type),
            user_id: Set(user_id),
            sender_name: Set(message.sender_name.clone()),
            content: Set(content),
            timestamp: Set(timestamp),
            is_deleted: Set(message.is_deleted),
            ..Default::default()
        }
    }

    /// Create a [`ChannelEvents::ActiveModel`] from `SendMessageRequest`
    pub(crate) fn from_send_message(user_id: &str, sender_name: &str, message: &SendMessageRequest) -> Self {
        let content = serde_json::json!({
          "content": message.content
        });

        Self {
            pid: Set(nanoid::nanoid!()),
            channel_id: Set(message.channel_pid.to_string()),
            message_kind: Set(MessageKind::User),
            user_id: Set(Some(user_id.into())),
            sender_name: Set(sender_name.into()),
            content: Set(content),
            ..Default::default()
        }
    }
}
