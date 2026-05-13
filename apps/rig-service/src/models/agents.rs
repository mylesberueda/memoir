pub(crate) use super::_entity::agents::*;
use crate::{
    agents::{Agent, AgentCore, AgentIdentity, AgentModel, AgentProvider, config::AgentConfig as LoadedAgentConfig},
    api::tool::Tool,
    models::{agent_users, language_models, providers, secrets, tools},
};
use platform_rs::cache::{ResolvedPermissions, ResourceType};
use proto_rs::rig::v1;
use sea_orm::sea_query::{BinOper, Expr, ExprTrait as _};
use sea_orm::{
    ActiveModelBehavior, ColumnTrait as _, Condition, ConnectionTrait, EntityTrait as _, JoinType, ModelTrait as _,
    QueryFilter as _, QuerySelect as _, RelationTrait as _, Select,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum AgentKind {
    Startup,
    Ephemeral,
}

impl From<AgentKind> for String {
    fn from(kind: AgentKind) -> Self {
        kind.to_string()
    }
}

/// Convert proto `AgentKind` → domain `AgentKind`.
/// `Unspecified` defaults to `Startup` for backward compatibility.
impl From<v1::AgentKind> for AgentKind {
    fn from(kind: v1::AgentKind) -> Self {
        match kind {
            v1::AgentKind::Startup => Self::Startup,
            v1::AgentKind::Ephemeral => Self::Ephemeral,
            v1::AgentKind::Unspecified => Self::Startup,
        }
    }
}

impl Model {
    pub(crate) fn kind(&self) -> AgentKind {
        self.kind.parse().unwrap_or_else(|_| {
            tracing::warn!(agent_id = self.id, kind = %self.kind, "unknown agent kind, defaulting to Startup");
            AgentKind::Startup
        })
    }

    pub(crate) fn agent_config(&self) -> v1::AgentConfig {
        serde_json::from_value(self.config.clone()).unwrap_or_else(|_| {
            tracing::trace!("failed to deserialize config from agent id {}", self.id);
            v1::AgentConfig::default()
        })
    }

    /// Check if agent belongs to the given organization.
    pub(crate) fn is_accessible_in_org(&self, org_pid: &str) -> bool {
        self.organization_pid == org_pid
    }
}

impl ModelEx {
    pub(crate) fn kind(&self) -> AgentKind {
        self.kind.parse().unwrap_or_else(|_| {
            tracing::warn!(agent_id = self.id, kind = %self.kind, "unknown agent kind, defaulting to Startup");
            AgentKind::Startup
        })
    }

    pub(crate) fn agent_config(&self) -> v1::AgentConfig {
        serde_json::from_value(self.config.clone()).unwrap_or_else(|_| {
            tracing::trace!("failed to deserialize config from agent id {}", self.id);
            v1::AgentConfig::default()
        })
    }

    /// Check if agent belongs to the given organization.
    pub(crate) fn is_accessible_in_org(&self, org_pid: &str) -> bool {
        self.organization_pid == org_pid
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LoadAgentError {
    #[error("failed to query agent data: {0}")]
    Query(#[from] sea_orm::DbErr),
    #[error("agent not found")]
    NotFound,
    #[error("agent missing model")]
    MissingModel,
    #[error("agent missing provider")]
    MissingProvider,
    #[error("failed to decrypt provider secret: {0}")]
    SecretDecryption(#[from] common_rs::crypto::CryptoError),
    #[error("invalid or unparseable model capabilities")]
    InvalidCapabilities,
}

impl Entity {
    /// Returns a `Select` with authorization filters applied.
    ///
    /// The query includes a LEFT/INNER JOIN on `agent_users` to check instance shares.
    /// If the user's role grants the required permission on Agents, the query matches
    /// by org membership OR share. Otherwise, only shares are checked.
    ///
    /// The caller chains `.find_also_related(...)` and `.one(db)` as needed.
    pub(crate) fn authorized_query(
        pid: &str,
        user_id: &str,
        org_pid: &str,
        org_perms: &ResolvedPermissions,
        required: agent_users::Permissions,
    ) -> Select<Self> {
        let type_allows = match required {
            p if p == agent_users::Permissions::READ => org_perms.can_read(ResourceType::Agents),
            p if p == agent_users::Permissions::WRITE => org_perms.can_write(ResourceType::Agents),
            p if p == agent_users::Permissions::EXECUTE => org_perms.can_execute(ResourceType::Agents),
            _ => false,
        };

        let mask = required.value();
        let share_filter = Condition::all().add(agent_users::Column::UserId.eq(user_id)).add(
            Expr::col(agent_users::Column::Permissions)
                .binary(BinOper::BitAnd, mask)
                .ne(0),
        );

        let mut query = Self::find()
            .filter(Column::Pid.eq(pid))
            .filter(Column::IsActive.eq(true));

        if type_allows {
            query = query.join(JoinType::LeftJoin, Relation::AgentUsers.def()).filter(
                Condition::any()
                    .add(Column::OrganizationPid.eq(org_pid))
                    .add(share_filter),
            );
        } else {
            query = query
                .join(JoinType::InnerJoin, Relation::AgentUsers.def())
                .filter(share_filter);
        }

        query
    }

    pub(crate) async fn find_agent_by_pid<C>(db: &C, pid: &str) -> Result<Agent, LoadAgentError>
    where
        C: ConnectionTrait,
    {
        let (agent, model, provider) = Self::find()
            .filter(Column::Pid.eq(pid))
            .find_also_related(language_models::Entity)
            .find_also(language_models::Entity, providers::Entity)
            .one(db)
            .await?
            .ok_or(LoadAgentError::NotFound)?;

        let model = model.ok_or(LoadAgentError::MissingModel)?;
        let provider = provider.ok_or(LoadAgentError::MissingProvider)?;

        let secret = provider
            .find_related(secrets::Entity)
            .filter(secrets::Column::SecretType.eq(secrets::SecretKind::ApiKey))
            .one(db)
            .await?
            .map(secrets::Model::into_ex);

        let agent_tools = agent
            .find_related(tools::Entity)
            .filter(tools::Column::IsActive.eq(true))
            .all(db)
            .await?
            .into_iter()
            .map(tools::Model::into_ex)
            .collect();

        Self::assemble_agent(
            agent.into_ex(),
            model.into_ex(),
            provider.into_ex(),
            secret,
            agent_tools,
        )
    }

    pub(crate) fn assemble_agent(
        agent: ModelEx,
        model: language_models::ModelEx,
        provider: providers::ModelEx,
        secret: Option<secrets::ModelEx>,
        tools: Vec<tools::ModelEx>,
    ) -> Result<Agent, LoadAgentError> {
        let agent_kind = agent.kind();
        let agent_config = agent.agent_config();
        let provider_kind = provider.kind();
        let api_key = secret
            .map(|value| value.decrypt().map(|decrypted| decrypted.expose().to_string()))
            .transpose()?;

        let capabilities = model.capabilities_proto().ok_or(LoadAgentError::InvalidCapabilities)?;

        Ok(Agent::new(
            AgentCore {
                name: agent.name,
                kind: agent_kind,
                system_prompt: agent.system_prompt.unwrap_or_default(),
                description: agent.description,
                temperature: agent.temperature,
                is_active: agent.is_active,
            },
            AgentIdentity {
                pid: agent.pid,
                slug: agent.slug,
                organization_pid: agent.organization_pid,
                created_by: agent.created_by,
                created_at: agent.created_at.and_utc(),
                updated_at: agent.updated_at.and_utc(),
            },
            AgentProvider {
                pid: provider.pid,
                name: provider.name,
                kind: provider_kind,
                base_url: provider.base_url,
                api_key,
            },
            AgentModel {
                pid: model.pid,
                model_id: model.model_id,
                capabilities,
            },
            LoadedAgentConfig::from(agent_config),
            tools.into_iter().map(Tool::from).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{providers::ProviderKind, secrets::SecretKind};
    use common_rs::crypto::SecretCrypto as _;
    use sea_orm::prelude::DateTime;
    use std::sync::Once;

    fn make_model(kind: &str) -> Model {
        Model {
            id: 1,
            pid: "agent_test".to_string(),
            organization_pid: "org_test".to_string(),
            created_by: "user_test".to_string(),
            name: "Test Agent".to_string(),
            slug: "test-agent".to_string(),
            kind: kind.to_string(),
            model_id: 1,
            description: None,
            temperature: 0.7,
            system_prompt: None,
            config: serde_json::json!({}),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }

    mod kind {
        use super::*;

        #[test]
        fn should_parse_startup_kind() {
            let model = make_model("startup");

            let result = model.kind();

            assert_eq!(result, AgentKind::Startup);
        }

        #[test]
        fn should_default_to_startup_for_unknown_kind() {
            let model = make_model("unknown-agent-type");

            let result = model.kind();

            assert_eq!(result, AgentKind::Startup);
        }

        #[test]
        fn should_default_to_startup_for_empty_kind() {
            let model = make_model("");

            let result = model.kind();

            assert_eq!(result, AgentKind::Startup);
        }

        #[test]
        fn should_convert_proto_startup_to_domain_startup() {
            assert_eq!(AgentKind::from(v1::AgentKind::Startup), AgentKind::Startup);
        }

        #[test]
        fn should_convert_proto_unspecified_to_domain_startup() {
            assert_eq!(AgentKind::from(v1::AgentKind::Unspecified), AgentKind::Startup);
        }

        #[test]
        fn should_parse_ephemeral_kind() {
            let model = make_model("ephemeral");

            let result = model.kind();

            assert_eq!(result, AgentKind::Ephemeral);
        }

        #[test]
        fn should_convert_proto_ephemeral_to_domain_ephemeral() {
            assert_eq!(AgentKind::from(v1::AgentKind::Ephemeral), AgentKind::Ephemeral);
        }
    }

    fn make_model_ex(kind: &str) -> ModelEx {
        make_model(kind).into_ex()
    }

    fn make_language_model() -> language_models::ModelEx {
        language_models::Model {
            id: 10,
            pid: "model_xyz".to_string(),
            provider_id: 100,
            model_id: "llama3".to_string(),
            name: "Llama 3".to_string(),
            context_window: Some(8192),
            capabilities: serde_json::json!({"thinking": false}),
            metadata: serde_json::json!({}),
            is_active: true,
            deprecation_message: None,
            last_fetched_at: None,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
        .into_ex()
    }

    fn make_provider() -> providers::ModelEx {
        providers::Model {
            id: 100,
            pid: "provider_abc".to_string(),
            organization_pid: None,
            created_by: None,
            name: "Local Ollama".to_string(),
            provider_type: "ollama".to_string(),
            base_url: Some("http://localhost:11434".to_string()),
            is_active: true,
            is_deprecated: false,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
        .into_ex()
    }

    fn make_tool(name: &str) -> tools::ModelEx {
        tools::Model {
            id: 1,
            pid: format!("tool_{name}"),
            name: name.to_string(),
            display_name: format!("Display {name}"),
            description: format!("{name} description"),
            tool_type: "system".to_string(),
            parameters_schema: serde_json::json!({}),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
        .into_ex()
    }

    fn init_test_crypto() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            crate::api::crypto::init(
                common_rs::crypto::LocalCrypto::from_passphrase("test-passphrase", "test-salt").unwrap(),
            );
        });
    }

    mod assemble_loaded {
        use super::*;

        #[test]
        fn should_build_loaded_agent_from_joined_models() {
            let loaded = Entity::assemble_agent(
                make_model_ex("startup"),
                make_language_model(),
                make_provider(),
                None,
                vec![make_tool("web_search")],
            )
            .unwrap();

            assert_eq!(loaded.core.name, "Test Agent");
            assert_eq!(loaded.core.kind, AgentKind::Startup);
            assert_eq!(loaded.identity.pid, "agent_test");
            assert_eq!(loaded.model.model_id, "llama3");
            assert_eq!(loaded.provider.kind, ProviderKind::Ollama);
            assert_eq!(loaded.provider.base_url.as_deref(), Some("http://localhost:11434"));
            assert_eq!(loaded.tools.len(), 1);
            assert_eq!(loaded.config.history_length, 50);
        }

        #[test]
        fn should_return_error_when_model_capabilities_are_invalid() {
            let mut model = make_language_model();
            model.capabilities = serde_json::json!({"thinking": "invalid"});

            let result = Entity::assemble_agent(make_model_ex("startup"), model, make_provider(), None, vec![]);

            assert!(matches!(result, Err(LoadAgentError::InvalidCapabilities)));
        }

        #[test]
        fn should_preserve_optional_description_and_empty_prompt_default() {
            let mut agent = make_model_ex("startup");
            agent.system_prompt = None;
            agent.description = Some("Helpful".to_string());

            let loaded = Entity::assemble_agent(agent, make_language_model(), make_provider(), None, vec![]).unwrap();

            assert_eq!(loaded.core.system_prompt, "");
            assert_eq!(loaded.core.description.as_deref(), Some("Helpful"));
        }

        #[test]
        fn should_decrypt_api_key_when_secret_is_present() {
            init_test_crypto();

            let encrypted = crate::api::crypto::get_crypto().encrypt(b"plaintext-key").unwrap();
            let ciphertext: Vec<u8> = "🔑x0".as_bytes().iter().copied().chain(encrypted).collect();

            let secret = secrets::Model {
                id: 1,
                pid: "secret_123".to_string(),
                secret_type: SecretKind::ApiKey.to_string(),
                encrypted_value: ciphertext,
                created_at: DateTime::default(),
                updated_at: DateTime::default(),
            }
            .into_ex();

            let loaded = Entity::assemble_agent(
                make_model_ex("startup"),
                make_language_model(),
                make_provider(),
                Some(secret),
                vec![],
            )
            .unwrap();

            assert_eq!(loaded.provider.api_key.as_deref(), Some("plaintext-key"));
        }
    }

    mod access_control {
        use super::*;

        #[test]
        fn should_allow_access_for_matching_org() {
            let model = make_model("startup");
            assert!(model.is_accessible_in_org("org_test"));
        }

        #[test]
        fn should_deny_access_for_different_org() {
            let model = make_model("startup");
            assert!(!model.is_accessible_in_org("org_other"));
        }
    }
}
