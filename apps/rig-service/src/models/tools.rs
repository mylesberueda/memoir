use proto_rs::rig::v1::Tool;
use sea_orm::ActiveModelBehavior;

pub(crate) use super::_entity::tools::*;

/// Tool types determine which agents can use each tool.
#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum ToolKind {
    /// System tools available to all agents (e.g., current_time, web_search)
    System,
    /// Assistant-only tools for the user's personal assistant (e.g., db_query, create_agent)
    Assistant,
    /// User-defined custom tools
    UserDefined,
}

impl Model {
    pub(crate) fn kind(&self) -> ToolKind {
        self.tool_type.parse().unwrap_or_else(|_| {
            tracing::warn!(tool_id = self.id, tool_type = %self.tool_type, "unknown tool type, defaulting to System");
            ToolKind::System
        })
    }

    pub(crate) fn into_proto(self) -> Tool {
        Tool {
            pid: self.pid,
            name: self.display_name,
            description: self.description,
            tool_type: self.tool_type,
            is_active: self.is_active,
        }
    }
}

impl ModelEx {
    pub(crate) fn into_proto(self) -> Tool {
        Model::from(self).into_proto()
    }
}

impl ActiveModelBehavior for ActiveModel {}
