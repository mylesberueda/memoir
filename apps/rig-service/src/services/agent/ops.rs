use super::AgentServiceError;
use crate::models::{agent_tools, agents, language_models, providers, tools};
use common_rs::ext::SlugifyExt as _;
use proto_rs::rig::v1;
use sea_orm::{ActiveValue::Set, ColumnTrait as _, DatabaseConnection, EntityTrait, QueryFilter as _};
use std::sync::Arc;
use tracing::instrument;

pub(crate) struct AgentOps;

pub(crate) struct CreateResult {
    pub(crate) agent: agents::ModelEx,
    pub(crate) model: language_models::ModelEx,
    pub(crate) provider: providers::ModelEx,
    pub(crate) tools: Vec<tools::ModelEx>,
}

impl AgentOps {
    #[instrument(skip(db), fields(user_id, organization_pid, model_pid, provider_pid))]
    pub(crate) async fn create(
        db: Arc<DatabaseConnection>,
        user_id: &str,
        organization_pid: &str,
        req: v1::CreateAgentRequest,
    ) -> Result<CreateResult, AgentServiceError> {
        let slug = req.name.slugify();
        let config = req
            .config
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .unwrap_or(serde_json::json!({}));

        let (model, provider) = language_models::Entity::find_by_pid(req.model_pid)
            .find_also_related(providers::Entity)
            .one(db.as_ref())
            .await?
            .ok_or(AgentServiceError::ModelNotFound)?;

        tracing::Span::current().record("model_pid", &model.pid);

        let provider = provider.ok_or_else(|| {
            tracing::error!("provider not found");
            AgentServiceError::ProviderNotFound
        })?;

        tracing::Span::current().record("provider_pid", &provider.pid);

        let agent = agents::ActiveModelEx::new()
            .set_organization_pid(organization_pid.to_owned())
            .set_created_by(user_id)
            .set_name(req.name)
            .set_slug(slug)
            .set_kind(agents::AgentKind::from(
                v1::AgentKind::try_from(req.kind).unwrap_or(v1::AgentKind::Startup),
            ))
            .set_model_id(model.id)
            .set_temperature(req.temperature as f64 / 100.0)
            .set_system_prompt(Some(req.system_prompt))
            .set_config(config)
            .set_is_active(true);

        let agent = agent.insert(db.as_ref()).await.map_err(AgentServiceError::DbError)?;

        let tools = tools::Entity::load()
            .filter(tools::Column::Pid.is_in(&req.tool_pids))
            .filter(tools::Column::IsActive.eq(true))
            .all(db.as_ref())
            .await?;

        tracing::debug!(
            agent_id = agent.id,
            tool_count = tools.len(),
            tool_names = ?tools.iter().map(|t| &t.name).collect::<Vec<_>>(),
            "attaching tools to agent"
        );

        if !tools.is_empty() {
            let junctions: Vec<agent_tools::ActiveModel> = tools
                .iter()
                .map(|t| agent_tools::ActiveModel {
                    agent_id: Set(agent.id),
                    tool_id: Set(t.id),
                    config: Set(serde_json::json!({})),
                })
                .collect();

            tracing::debug!(
                agent_id = agent.id,
                junction_count = junctions.len(),
                "inserting agent_tools junctions"
            );

            agent_tools::Entity::insert_many(junctions)
                .on_conflict(
                    sea_orm::sea_query::OnConflict::columns([
                        agent_tools::Column::AgentId,
                        agent_tools::Column::ToolId,
                    ])
                    .do_nothing()
                    .to_owned(),
                )
                .exec(db.as_ref())
                .await?;

            tracing::debug!(agent_id = agent.id, "agent_tools insert completed");
        } else {
            tracing::warn!(agent_id = agent.id, "no tools to attach - agent_tools is empty");
        }

        Ok(CreateResult {
            agent,
            model: model.into_ex(),
            provider: provider.into_ex(),
            tools,
        })
    }
}
