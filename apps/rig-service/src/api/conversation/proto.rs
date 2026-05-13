use super::*;
use proto_rs::rig::v1;

impl From<Conversation> for v1::Conversation {
    fn from(conversation: Conversation) -> Self {
        Self {
            pid: conversation.model.pid,
            user_id: Some(conversation.model.user_id),
            agent_pid: conversation.agent_pid,
            title: conversation.model.title,
            messages: conversation.messages,
            message_count: conversation.model.message_count,
            last_message_at: conversation.model.last_message_at.map(|t| t.and_utc().to_rfc3339()),
            created_at: conversation.model.created_at.and_utc().to_rfc3339(),
            updated_at: conversation.model.updated_at.and_utc().to_rfc3339(),
        }
    }
}
