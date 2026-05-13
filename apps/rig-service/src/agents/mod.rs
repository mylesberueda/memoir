pub(crate) mod components;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod messages;
pub(crate) mod rig;
pub(crate) mod runtime;
pub(crate) mod streaming;
pub(crate) mod system_prompt;

pub(crate) use config::AgentConfig;
pub(crate) use messages::MessageConversionError;
pub(crate) use rig::BaseAgent;
pub(crate) use runtime::RuntimeAgent;

use crate::{
    api::tool::Tool,
    models::{agents::AgentKind, providers::ProviderKind},
};
use chrono::{DateTime, Utc};
use proto_rs::rig::v1;

#[derive(Debug, Clone)]
pub(crate) struct Agent {
    pub(crate) core: AgentCore,
    pub(crate) identity: AgentIdentity,
    pub(crate) provider: AgentProvider,
    pub(crate) model: AgentModel,
    pub(crate) config: AgentConfig,
    pub(crate) tools: Vec<Tool>,
}

impl Agent {
    pub(crate) fn new(
        core: AgentCore,
        identity: AgentIdentity,
        provider: AgentProvider,
        model: AgentModel,
        config: AgentConfig,
        tools: Vec<Tool>,
    ) -> Self {
        Self {
            core,
            identity,
            provider,
            model,
            config,
            tools,
        }
    }
}

impl From<&Agent> for v1::Agent {
    fn from(agent: &Agent) -> Self {
        let kind = match agent.core.kind {
            AgentKind::Startup => v1::AgentKind::Startup as i32,
            AgentKind::Ephemeral => v1::AgentKind::Ephemeral as i32,
        };

        Self {
            identifier: Some(v1::agent::Identifier::Pid(agent.identity.pid.clone())),
            name: agent.core.name.clone(),
            slug: agent.identity.slug.clone(),
            kind,
            model: Some(v1::AgentModel {
                pid: agent.model.pid.clone(),
                model_id: agent.model.model_id.clone(),
                provider: Some(v1::AgentProvider {
                    pid: agent.provider.pid.clone(),
                    name: agent.provider.name.clone(),
                }),
            }),
            temperature: (agent.core.temperature * 100.0).round() as i32,
            system_prompt: agent.core.system_prompt.clone(),
            tools: agent.tools.iter().cloned().map(Into::into).collect(),
            config: Some(agent.config.clone().into()),
            is_active: agent.core.is_active,
            created_by_user_id: agent.identity.created_by.clone(),
            created_at: agent.identity.created_at.to_rfc3339(),
            updated_at: agent.identity.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AgentCore {
    pub(crate) name: String,
    pub(crate) kind: AgentKind,
    pub(crate) system_prompt: String,
    pub(crate) description: Option<String>,
    pub(crate) temperature: f64,
    pub(crate) is_active: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct AgentIdentity {
    pub(crate) pid: String,
    pub(crate) slug: String,
    #[expect(
        dead_code,
        reason = "Retained for follow-on access-control and message-pattern decisions"
    )]
    pub(crate) organization_pid: String,
    pub(crate) created_by: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub(crate) struct AgentProvider {
    pub(crate) pid: String,
    pub(crate) name: String,
    pub(crate) kind: ProviderKind,
    pub(crate) base_url: Option<String>,
    pub(crate) api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct AgentModel {
    pub(crate) pid: String,
    pub(crate) model_id: String,
    pub(crate) capabilities: v1::ModelCapabilities,
}
