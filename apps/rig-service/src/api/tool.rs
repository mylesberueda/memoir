use crate::models::tools;

/// Domain tool type — the canonical representation within the service.
///
/// Constructed from the DB model via [`From<tools::Model>`]. Converted to
/// proto at the gRPC boundary via [`Tool::into_proto`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tool {
    pid: String,
    name: String,
    description: String,
    kind: tools::ToolKind,
    is_active: bool,
}

#[expect(unused, reason = "Loads of useful methods.")]
impl Tool {
    pub(crate) fn pid(&self) -> &str {
        &self.pid
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn kind(&self) -> tools::ToolKind {
        self.kind
    }

    pub(crate) fn is_active(&self) -> bool {
        self.is_active
    }
}

impl From<tools::Model> for Tool {
    fn from(model: tools::Model) -> Self {
        Self {
            pid: model.pid.clone(),
            name: model.display_name.clone(),
            description: model.description.clone(),
            kind: model.kind(),
            is_active: model.is_active,
        }
    }
}

impl From<tools::ModelEx> for Tool {
    fn from(model: tools::ModelEx) -> Self {
        Self::from(tools::Model::from(model))
    }
}

impl From<Tool> for proto_rs::rig::v1::Tool {
    fn from(tool: Tool) -> Self {
        Self {
            pid: tool.pid,
            name: tool.name,
            description: tool.description,
            tool_type: tool.kind.to_string(),
            is_active: tool.is_active,
        }
    }
}

impl From<proto_rs::rig::v1::Tool> for Tool {
    fn from(proto: proto_rs::rig::v1::Tool) -> Self {
        Self {
            pid: proto.pid,
            name: proto.name,
            description: proto.description,
            kind: proto.tool_type.parse().unwrap_or(tools::ToolKind::System),
            is_active: proto.is_active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::DateTime;

    fn make_tool_model(name: &str, tool_type: &str) -> tools::Model {
        tools::Model {
            id: 1,
            pid: format!("tool_{name}"),
            name: name.to_string(),
            display_name: format!("Display {name}"),
            description: format!("{name} description"),
            tool_type: tool_type.to_string(),
            parameters_schema: serde_json::json!({}),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }

    mod from_model {
        use super::*;

        #[test]
        fn should_convert_system_tool() {
            let tool: Tool = make_tool_model("web_search", "system").into();

            assert_eq!(tool.pid(), "tool_web_search");
            assert_eq!(tool.name(), "Display web_search");
            assert_eq!(tool.kind(), tools::ToolKind::System);
            assert!(tool.is_active());
        }

        #[test]
        fn should_convert_assistant_tool() {
            let tool: Tool = make_tool_model("db_query", "assistant").into();

            assert_eq!(tool.kind(), tools::ToolKind::Assistant);
        }

        #[test]
        fn should_convert_to_proto() {
            let tool: Tool = make_tool_model("web_search", "system").into();
            let proto: proto_rs::rig::v1::Tool = tool.into();

            assert_eq!(proto.pid, "tool_web_search");
            assert_eq!(proto.name, "Display web_search");
            assert_eq!(proto.tool_type, "system");
            assert!(proto.is_active);
        }
    }
}
