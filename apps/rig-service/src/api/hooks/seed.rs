//! System provider and tool seeding lifecycle hook.

use super::{Hooks, HooksError};
use crate::{
    AppContext,
    api::embedding::EmbeddingModel,
    clients,
    models::{language_models, providers, secrets, tools},
    tools::{
        assistant::{CREATE_AGENT_TOOL_NAME, DB_QUERY_TOOL_NAME},
        system::{CurrentTimeTool, DOCUMENT_SEARCH_TOOL_NAME, WebSearchTool},
    },
};
use rig::tool::Tool as _;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _, TryIntoModel,
};
use std::sync::Arc;
use strum::IntoEnumIterator as _;

struct ProviderEnvironmentConfig<'a> {
    name: &'a str,
    base_url: Option<&'a str>,
    api_key: Option<&'a str>,
    #[allow(dead_code)]
    test_model: Option<&'a str>,
}

const SYSTEM_OLLAMA: ProviderEnvironmentConfig = ProviderEnvironmentConfig {
    name: "Ollama (System)",
    base_url: Some("OLLAMA_BASE_URL"),
    api_key: None,
    test_model: Some("OLLAMA_TEST_MODEL"),
};

const SYSTEM_OPENAI: ProviderEnvironmentConfig = ProviderEnvironmentConfig {
    name: "OpenAI (System)",
    base_url: Some("OPENAI_BASE_URL"),
    api_key: Some("OPENAI_API_KEY"),
    test_model: None,
};

const SYSTEM_GEMINI: ProviderEnvironmentConfig = ProviderEnvironmentConfig {
    name: "Gemini (System)",
    base_url: None, // Uses default: https://generativelanguage.googleapis.com
    api_key: Some("GEMINI_API_KEY"),
    test_model: None,
};

struct ToolConfig {
    /// The internal tool name (e.g., "current_time")
    name: &'static str,
    /// Human-readable display name (e.g., "Current Time")
    display_name: &'static str,
    /// Description shown to LLM for tool selection
    description: &'static str,
    /// Tool kind determining which agents can use this tool
    kind: tools::ToolKind,
}

const SEED_TOOLS: &[ToolConfig] = &[
    // System tools - available to all agents
    ToolConfig {
        name: CurrentTimeTool::NAME,
        display_name: "Current Time",
        description: "Get the current date and time in ISO8601 format.",
        kind: tools::ToolKind::System,
    },
    ToolConfig {
        name: WebSearchTool::NAME,
        display_name: "Web Search",
        description: "Search the web for information using Firecrawl.",
        kind: tools::ToolKind::System,
    },
    ToolConfig {
        name: DOCUMENT_SEARCH_TOOL_NAME,
        display_name: "Document Search",
        description: "Search through documents uploaded to a conversation.",
        kind: tools::ToolKind::System,
    },
    // Assistant tools - only for user's personal assistant
    ToolConfig {
        name: DB_QUERY_TOOL_NAME,
        display_name: "Database Query",
        description: "Query conversations, messages, agents, models, and tools.",
        kind: tools::ToolKind::Assistant,
    },
    ToolConfig {
        name: CREATE_AGENT_TOOL_NAME,
        display_name: "Create Agent",
        description: "Create a new AI agent with custom configuration.",
        kind: tools::ToolKind::Assistant,
    },
];

pub(crate) struct Seed<EM>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
}

impl<EM> Seed<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>) -> Self {
        Self { ctx }
    }

    pub(crate) async fn init(ctx: Arc<AppContext<EM>>) -> Result<(), HooksError> {
        Self::new(ctx).on_startup().await
    }

    async fn seed_provider(&self, kind: providers::ProviderKind) -> Result<(), HooksError> {
        let config = match kind {
            providers::ProviderKind::Ollama => SYSTEM_OLLAMA,
            providers::ProviderKind::Openai => SYSTEM_OPENAI,
            providers::ProviderKind::Gemini => SYSTEM_GEMINI,
        };

        let base_url = match config.base_url {
            Some(var) => match std::env::var(var) {
                Ok(url) => Some(url),
                Err(_) => {
                    tracing::info!("{} missing in environment, skipping {}", var, config.name);
                    return Ok(());
                }
            },
            None => None,
        };

        let api_key = match config.api_key {
            Some(var) => match std::env::var(var) {
                Ok(api_key) => Some(api_key),
                Err(_) => {
                    tracing::info!("{} missing in environment, skipping {}", var, config.name);
                    return Ok(());
                }
            },
            None => None,
        };

        let existing = providers::Entity::load()
            .filter(providers::Column::Name.eq(config.name))
            .filter(providers::Column::CreatedBy.is_null())
            .filter(providers::Column::OrganizationPid.is_null())
            .one(&self.ctx.db)
            .await?;

        let provider = match existing {
            Some(p) => {
                tracing::debug!(provider_id = p.id, "{} provider already exists", config.name);

                let provider = if p.base_url.as_deref() != base_url.as_deref() {
                    let mut active = p.clone().into_active_model();
                    active.base_url = Set(base_url.clone());
                    active.update(&self.ctx.db).await?
                } else {
                    p.clone()
                };

                provider.into_active_model()
            }
            None => {
                tracing::info!("Creating {} provider", config.name);

                let provider = providers::ActiveModel::builder()
                    .set_name(config.name)
                    .set_provider_type(kind.to_string())
                    .set_base_url(base_url)
                    .set_is_active(true);

                let provider = provider.save(&self.ctx.db).await?;
                tracing::info!(provider_id = provider.id.as_ref(), "Created {} provider", config.name);

                provider
            }
        };

        if let Some(ref api_key) = api_key {
            let query = secrets::Entity::find()
                .inner_join(providers::Entity)
                .filter(providers::Column::Id.eq(*provider.id.as_ref()))
                .filter(secrets::Column::SecretType.eq(secrets::SecretKind::ApiKey.to_string()));

            let secret = query.one(&self.ctx.db).await.map_err(HooksError::Database)?;

            match secret {
                Some(m) => {
                    tracing::debug!(provider_id = provider.id.as_ref(), "updating secret");
                    m.into_ex()
                        .into_active_model()
                        .set_encrypted_value(api_key.as_bytes())
                        .save(&self.ctx.db)
                        .await
                        .map_err(HooksError::Database)?;
                }
                None => {
                    provider
                        .clone()
                        .add_secret(
                            secrets::ActiveModel::builder()
                                .set_encrypted_value(api_key.as_bytes())
                                .set_secret_type(secrets::SecretKind::ApiKey.to_string()),
                        )
                        .save(&self.ctx.db)
                        .await
                        .map_err(HooksError::Database)?;
                }
            }
        }

        self.sync_models(
            &provider.try_into_model().map_err(HooksError::Database)?,
            api_key.as_deref(),
        )
        .await?;

        Ok(())
    }

    async fn sync_models(&self, provider: &providers::ModelEx, api_key: Option<&str>) -> Result<(), HooksError> {
        let fetched = clients::fetch_models(
            provider
                .provider_type
                .parse::<providers::ProviderKind>()
                .map_err(|_| HooksError::Config("Invalid provider kind.".into()))?,
            provider.base_url.as_deref(),
            api_key,
        )
        .await?;

        if fetched.is_empty() {
            tracing::debug!(
                provider_id = provider.id,
                provider_name = provider.name,
                "No models found"
            );
            return Ok(());
        }

        tracing::debug!(
            provider_id = provider.id,
            provider_name = provider.name,
            model_count = fetched.len(),
            "Fetched models"
        );

        let existing: Vec<language_models::Model> = language_models::Entity::find()
            .filter(language_models::Column::ProviderId.eq(provider.id))
            .all(&self.ctx.db)
            .await?;

        let existing_model_ids: std::collections::HashSet<&str> =
            existing.iter().map(|m| m.model_id.as_str()).collect();

        let models = fetched
            .into_iter()
            .filter(|m| !existing_model_ids.contains(m.model_id.as_str()))
            .map(|m| {
                language_models::ActiveModel::builder()
                    .set_provider_id(provider.id)
                    .set_model_id(m.model_id)
                    .set_name(m.name)
                    .set_context_window(m.context_window)
                    .set_capabilities(serde_json::to_value(m.capabilities).unwrap_or_else(|_| {
                        tracing::warn!("Failed to deserialize model capabilities. Setting defaults.");
                        serde_json::Value::default()
                    }))
                    .set_metadata(serde_json::to_value(&m.metadata).unwrap_or_else(|_| {
                        tracing::warn!("Failed to deserialize model metadata. Setting defaults.");
                        serde_json::Value::default()
                    }))
                    .set_is_active(true)
                    .set_last_fetched_at(Some(chrono::Utc::now().naive_utc()))
            })
            .collect::<Vec<language_models::ActiveModelEx>>();

        if !models.is_empty() {
            language_models::Entity::insert_many(models).exec(&self.ctx.db).await?;

            tracing::info!(
                provider_id = provider.id,
                provider_name = provider.name,
                "Synced new models"
            );
        }

        Ok(())
    }

    async fn seed_tool(&self, config: &ToolConfig) -> Result<(), HooksError> {
        let tool_type = config.kind.to_string();

        let existing = tools::Entity::find()
            .filter(tools::Column::Name.eq(config.name))
            .one(&self.ctx.db)
            .await?;

        match existing {
            Some(tool) => {
                tracing::trace!(tool_id = tool.id, tool_name = config.name, "Tool already exists");

                // Update fields if they've changed
                let needs_update = tool.display_name != config.display_name
                    || tool.description != config.description
                    || tool.tool_type != tool_type;

                if needs_update {
                    let mut active = tool.into_active_model();
                    active.display_name = Set(config.display_name.to_string());
                    active.description = Set(config.description.to_string());
                    active.tool_type = Set(tool_type);
                    active.update(&self.ctx.db).await?;
                    tracing::info!(tool_name = config.name, "Updated tool");
                }
            }
            None => {
                tracing::info!(tool_name = config.name, tool_type = %tool_type, "Creating tool");

                let tool = tools::ActiveModel {
                    name: Set(config.name.to_string()),
                    display_name: Set(config.display_name.to_string()),
                    description: Set(config.description.to_string()),
                    tool_type: Set(tool_type),
                    parameters_schema: Set(serde_json::json!({})),
                    is_active: Set(true),
                    ..Default::default()
                };

                let result = tool.insert(&self.ctx.db).await?;
                tracing::info!(tool_id = result.id, tool_name = config.name, "Created tool");
            }
        }

        Ok(())
    }
}

impl<EM> Hooks for Seed<EM>
where
    EM: EmbeddingModel,
{
    async fn on_startup(&self) -> Result<(), HooksError> {
        // Seed system providers
        for provider in providers::ProviderKind::iter() {
            self.seed_provider(provider).await?
        }

        // Seed tools
        for tool_config in SEED_TOOLS {
            self.seed_tool(tool_config).await?
        }

        Ok(())
    }
}
