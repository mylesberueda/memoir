pub(crate) mod registry;

pub(crate) mod assistant;
mod startup;
pub(crate) mod system;

pub(crate) use registry::{ToolRegistry, ToolRegistryError};

use assistant::{CREATE_AGENT_TOOL_NAME, DB_QUERY_TOOL_NAME};
use rig::tool::Tool;
use system::{CurrentTimeTool, DOCUMENT_SEARCH_TOOL_NAME, WebSearchTool};

pub(crate) trait ToToolDisplayName {
    fn to_tool_display_name(&self) -> String;
}

impl ToToolDisplayName for str {
    fn to_tool_display_name(&self) -> String {
        match self {
            CurrentTimeTool::NAME => "Current Time",
            WebSearchTool::NAME => "Web Search",
            DB_QUERY_TOOL_NAME => "Database Query",
            CREATE_AGENT_TOOL_NAME => "Create Agent",
            DOCUMENT_SEARCH_TOOL_NAME => "Document Search",
            tool => tool,
        }
        .to_string()
    }
}

impl ToToolDisplayName for String {
    fn to_tool_display_name(&self) -> String {
        self.as_str().to_tool_display_name()
    }
}
