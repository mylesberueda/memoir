mod error;
pub(crate) mod ops;

use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    consts::REDIS_USER_CACHE_KEY,
    models::{agent_tools, agent_users, agents, language_models, providers, tools, user_assistants},
    tools::{
        assistant::{CreateAgentTool, DbQueryTool},
        system::{CurrentTimeTool, DOCUMENT_SEARCH_TOOL_NAME},
    },
};
use error::AgentServiceError;
pub(crate) use ops::AgentOps;
use platform_rs::{
    cache::{ResourceType, UserCache},
    ext::RequestAuthExt,
};
use proto_rs::rig::v1;
use rig::tool::Tool as _;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, Condition, ConnectionTrait as _, EntityTrait as _,
    IntoActiveModel as _, JoinType, ModelTrait, PaginatorTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _, RelationTrait as _,
};
use std::sync::Arc;
use tracing::instrument;

const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;

#[derive(Debug, Clone)]
pub(crate) struct AgentService<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    context: Arc<AppContext<EM>>,
}

impl<EM> AgentService<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(context: Arc<AppContext<EM>>) -> Self {
        Self { context }
    }
}

#[tonic::async_trait]
impl<EM> v1::agent_service_server::AgentService for AgentService<EM>
where
    EM: EmbeddingModel,
{
    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok())
    )]
    async fn list_agents(
        &self,
        request: tonic::Request<v1::ListAgentsRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListAgentsResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid().ok();
        let req = request.into_inner();

        let page = req.page.max(1) as u64;
        let page_size = if req.page_size == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            (req.page_size as u64).clamp(1, MAX_PAGE_SIZE)
        };

        let mut query = agents::Entity::find();

        // Access control: filter by org or personal context, including shared agents
        match &organization_pid {
            Some(org_pid) => {
                query = query
                    .join(JoinType::LeftJoin, agents::Relation::AgentUsers.def())
                    .filter(
                        Condition::any()
                            .add(agents::Column::OrganizationPid.eq(org_pid))
                            .add(agent_users::Column::UserId.eq(&user_id)),
                    );
            }
            None => {
                query = query
                    .filter(agents::Column::OrganizationPid.is_null())
                    .filter(agents::Column::CreatedBy.eq(&user_id));
            }
        }

        if let Some(filter_user_id) = &req.user_id {
            query = query.filter(agents::Column::CreatedBy.eq(filter_user_id));
        }

        if let Some(is_active) = req.is_active {
            query = query.filter(agents::Column::IsActive.eq(is_active));
        }

        if req.with_assistants != Some(true) {
            query = query
                .join(JoinType::LeftJoin, agents::Relation::UserAssistants.def())
                .filter(user_assistants::Column::AgentId.is_null());
        }

        if let Some(provider_pid) = &req.provider_pid {
            let models = language_models::Entity::find()
                .find_also_related(crate::models::providers::Entity)
                .all(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to query language models");
                    tonic::Status::internal("Failed to filter by provider")
                })?;

            let model_ids: Vec<i64> = models
                .into_iter()
                .filter_map(|(model, provider)| {
                    provider.and_then(|p| if p.pid == *provider_pid { Some(model.id) } else { None })
                })
                .collect();

            query = query.filter(agents::Column::ModelId.is_in(model_ids));
        }

        let total = query.clone().count(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to count agents");
            tonic::Status::internal("Failed to count agents")
        })? as i32;

        let results = query
            .find_also_related(language_models::Entity)
            .find_also(language_models::Entity, providers::Entity)
            .order_by_desc(agents::Column::CreatedAt)
            .offset((page - 1) * page_size)
            .limit(page_size)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch agents");
                tonic::Status::internal("Failed to fetch agents")
            })?;

        let agent_ids: Vec<i64> = results.iter().map(|(agent, _, _)| agent.id).collect();

        let all_agent_tools: Vec<(agent_tools::Model, tools::Model)> = agent_tools::Entity::find()
            .filter(agent_tools::Column::AgentId.is_in(agent_ids.clone()))
            .find_also_related(tools::Entity)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch tools for agents");
                tonic::Status::internal("Failed to fetch tools")
            })?
            .into_iter()
            .filter_map(|(at, t)| t.map(|tool| (at, tool)))
            .collect();

        let tools_by_agent: std::collections::HashMap<i64, Vec<tools::Model>> =
            all_agent_tools
                .into_iter()
                .fold(std::collections::HashMap::new(), |mut acc, (at, tool)| {
                    acc.entry(at.agent_id).or_default().push(tool);
                    acc
                });

        let agents = results
            .into_iter()
            .filter_map(|(agent, model, provider)| {
                let agent_tools = tools_by_agent
                    .get(&agent.id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|t| t.into_ex())
                    .collect();

                let provider = provider.or_else(|| {
                    tracing::error!(agent_id = agent.id, "agent missing provider");
                    None
                })?;

                let model = model.or_else(|| {
                    tracing::error!(agent_id = agent.id, "agent missing model");
                    None
                })?;

                agents::Entity::assemble_agent(agent.into_ex(), model.into_ex(), provider.into_ex(), None, agent_tools)
                    .map(|a| v1::Agent::from(&a))
                    .map_err(|e| tracing::error!("failed to build domain agent: {e}"))
                    .ok()
            })
            .collect();

        Ok(tonic::Response::new(v1::ListAgentsResponse {
            agents,
            total,
            page: page as i32,
            page_size: page_size as i32,
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn get_agent(
        &self,
        request: tonic::Request<v1::GetAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::GetAgentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let perms = request
            .org_permissions()
            .unwrap_or_else(|_| platform_rs::cache::ResolvedPermissions::allow_all());
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let (agent, model, provider) = agents::Entity::authorized_query(
            &req.pid,
            &user_id,
            &organization_pid,
            &perms,
            agent_users::Permissions::READ,
        )
        .find_also_related(language_models::Entity)
        .find_also(language_models::Entity, providers::Entity)
        .one(&self.context.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to fetch agent");
            tonic::Status::internal("Failed to fetch agent")
        })?
        .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

        let model = model.ok_or_else(|| {
            tracing::error!(agent_id = agent.id, "missing model on agent");
            tonic::Status::internal("Missing model on agent")
        })?;

        let provider = provider.ok_or_else(|| {
            tracing::error!(agent_id = agent.id, "missing provider on agent");
            tonic::Status::internal("Missing provider on agent")
        })?;

        // Fetch tools for this agent via junction table
        let fetched_tools: Vec<tools::ModelEx> = agent_tools::Entity::find()
            .filter(agent_tools::Column::AgentId.eq(agent.id))
            .find_also_related(tools::Entity)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, agent_id = agent.id, "failed to fetch agent tools");
                tonic::Status::internal("Failed to fetch agent tools")
            })?
            .into_iter()
            .filter_map(|(_, tool)| tool.map(tools::Model::into_ex))
            .collect();

        let agent = agents::Entity::assemble_agent(
            agent.into_ex(),
            model.into_ex(),
            provider.into_ex(),
            None,
            fetched_tools,
        )
        .map_err(|e| {
            tracing::error!(error = %e, "failed to build domain agent");
            tonic::Status::internal("Failed to build agent")
        })?;

        Ok(tonic::Response::new(v1::GetAgentResponse {
            agent: Some(v1::Agent::from(&agent)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok())
    )]
    async fn create_agent(
        &self,
        request: tonic::Request<v1::CreateAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::CreateAgentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        if !request
            .org_permissions()
            .map(|p| p.can_write(ResourceType::Agents))
            .unwrap_or(true)
        {
            return Err(tonic::Status::permission_denied(
                "Insufficient permissions to create agents",
            ));
        }
        let req = request.into_inner();

        // Validate required fields
        if req.name.is_empty() {
            return Err(tonic::Status::invalid_argument("name is required"));
        }
        if req.model_pid.is_empty() {
            return Err(tonic::Status::invalid_argument("model is required"));
        }
        if req.system_prompt.is_empty() {
            return Err(tonic::Status::invalid_argument("system_prompt is required"));
        }

        let res = AgentOps::create(Arc::new(self.context.db.clone()), &user_id, &organization_pid, req).await?;

        let agent =
            agents::Entity::assemble_agent(res.agent, res.model, res.provider, None, res.tools).map_err(|e| {
                tracing::error!(error = %e, "failed to build domain agent");
                tonic::Status::internal("Failed to build agent")
            })?;

        Ok(tonic::Response::new(v1::CreateAgentResponse {
            agent: Some(v1::Agent::from(&agent)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn update_agent(
        &self,
        request: tonic::Request<v1::UpdateAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::UpdateAgentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let perms = request
            .org_permissions()
            .unwrap_or_else(|_| platform_rs::cache::ResolvedPermissions::allow_all());
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let (existing, model, provider) = agents::Entity::authorized_query(
            &req.pid,
            &user_id,
            &organization_pid,
            &perms,
            agent_users::Permissions::WRITE,
        )
        .find_also_related(language_models::Entity)
        .find_also(language_models::Entity, providers::Entity)
        .one(&self.context.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "failed to fetch agent");
            tonic::Status::internal("Failed to fetch agent")
        })?
        .ok_or_else(|| {
            tracing::error!("failed to find existing agent");
            tonic::Status::not_found("Failed to fetch agent")
        })?;

        let mut model = model.ok_or_else(|| {
            tracing::error!("failed to find model");
            tonic::Status::not_found("Failed to find model.")
        })?;

        let mut provider = provider.ok_or_else(|| {
            tracing::error!("failed to find provider");
            tonic::Status::not_found("Failed to find provider.")
        })?;

        let mut active = existing.clone().into_ex().into_active_model();

        if let Some(name) = &req.name {
            active = active.set_name(name);
        }

        if let Some(slug) = &req.slug {
            active = active.set_slug(slug);
        }

        if let Some(model_pid) = &req.model_pid
            && let Some((m, p)) = language_models::Entity::find_by_pid(model_pid)
                .find_also_related(providers::Entity)
                .one(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, model_pid = &model_pid, "failed to find model");
                    tonic::Status::not_found("New model not found.")
                })?
            && let Some(p) = p
        {
            active = active.set_model_id(m.id);

            model = m;
            provider = p;
        }

        if let Some(temperature) = req.temperature {
            active = active.set_temperature(temperature as f64 / 100.0);
        }

        if let Some(prompt) = req.system_prompt {
            active = active.set_system_prompt(Some(prompt));
        }

        if let Some(config) = &req.config {
            active = active.set_config(serde_json::to_value(config).unwrap_or_default());
        }

        if let Some(is_active) = req.is_active {
            active = active.set_is_active(is_active);
        }

        if let Some(kind) = req.kind {
            active = active.set_kind(agents::AgentKind::from(
                v1::AgentKind::try_from(kind).unwrap_or(v1::AgentKind::Startup),
            ));
        }

        let updated = active.update(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to update agent");
            tonic::Status::internal("Failed to update agent")
        })?;

        // SAFETY(_): We don't need the values of the tools after mutation, and
        // this way saves 3 - 5 roundtrips.
        if !req.tools.is_empty() {
            let (to_add, to_remove): (Vec<_>, Vec<_>) = req.tools.into_iter().partition(|t| t.is_active);

            let to_add: Vec<String> = to_add.into_iter().map(|t| t.pid).collect();
            let to_remove: Vec<String> = to_remove.into_iter().map(|t| t.pid).collect();

            self.context
                .db
                .execute_raw(sea_orm::Statement::from_sql_and_values(
                    sea_orm::DbBackend::Postgres,
                    r#"
                    WITH
                    deleted AS (
                        DELETE FROM agent_tools
                        WHERE agent_id = $1
                        AND tool_id IN (SELECT id FROM tools WHERE pid = ANY($2))
                    ),
                    inserted AS (
                        INSERT INTO agent_tools (agent_id, tool_id, config)
                        SELECT $1, id, '{}'::jsonb
                        FROM tools
                        WHERE pid = ANY($3) AND is_active = true
                        ON CONFLICT (agent_id, tool_id) DO NOTHING
                    )
                    SELECT 1
                    "#,
                    [updated.id.into(), to_remove.into(), to_add.into()],
                ))
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to update agent tools");
                    tonic::Status::internal("Failed to update agent tools.")
                })?;
        }

        let final_tools = updated
            .find_related(tools::Entity)
            .all(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to query tools");
                tonic::Status::not_found("Failed to find new tools.")
            })?
            .into_iter()
            .map(|m| m.into_ex())
            .collect();

        let agent = agents::Entity::assemble_agent(updated, model.into_ex(), provider.into_ex(), None, final_tools)
            .map_err(|e| {
                tracing::error!(error = %e, "failed to build domain agent");
                tonic::Status::internal("Failed to build agent")
            })?;

        Ok(tonic::Response::new(v1::UpdateAgentResponse {
            agent: Some(v1::Agent::from(&agent)),
        }))
    }

    #[instrument(
        skip(self, request),
        fields(user_id = request.user_id()?, organization_pid = ?request.organization_pid().ok(), pid)
    )]
    async fn delete_agent(
        &self,
        request: tonic::Request<v1::DeleteAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::DeleteAgentResponse>, tonic::Status> {
        let _user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        if !request
            .org_permissions()
            .map(|p| p.can_write(ResourceType::Agents))
            .unwrap_or(true)
        {
            return Err(tonic::Status::permission_denied(
                "Insufficient permissions to delete agents",
            ));
        }
        let req = request.into_inner();
        tracing::Span::current().record("pid", &req.pid);

        let existing = agents::Entity::find()
            .filter(agents::Column::Pid.eq(&req.pid))
            .filter(agents::Column::IsActive.eq(true))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch agent");
                tonic::Status::internal("Failed to fetch agent")
            })?
            .ok_or(tonic::Status::not_found("Agent not found"))?;

        if !existing.is_accessible_in_org(&organization_pid) {
            return Err(tonic::Status::not_found("Agent not found"));
        }

        let mut active: agents::ActiveModel = existing.into();
        active.is_active = Set(false); // Soft-delete

        active.update(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to delete agent");
            tonic::Status::internal("Failed to delete agent")
        })?;

        Ok(tonic::Response::new(v1::DeleteAgentResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, organization_pid))]
    async fn get_user_assistant(
        &self,
        request: tonic::Request<v1::GetUserAssistantRequest>,
    ) -> std::result::Result<tonic::Response<v1::GetUserAssistantResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        tracing::Span::current().record("user_id", &user_id);
        tracing::Span::current().record("organization_pid", &organization_pid);

        let (agent, model, provider, assistant_tools) = match agents::Entity::find()
            .join(JoinType::InnerJoin, agents::Relation::UserAssistants.def())
            .filter(user_assistants::Column::UserId.eq(&user_id))
            .filter(agents::Column::IsActive.eq(true))
            .find_also_related(language_models::Entity)
            .find_also(language_models::Entity, providers::Entity)
            .filter(
                Condition::all()
                    .add(providers::Column::OrganizationPid.is_null())
                    .add(providers::Column::CreatedBy.is_null()),
            )
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to query assistant");
                tonic::Status::internal("Failed to query assistant.")
            })? {
            Some((agent, model, provider)) => {
                let model = model.ok_or_else(|| {
                    tracing::error!(agent_id = agent.id, "missing model on agent");
                    tonic::Status::internal("Missing model on agent")
                })?;

                let provider = provider.ok_or_else(|| {
                    tracing::error!(agent_id = agent.id, "missing provider on agent");
                    tonic::Status::internal("Missing provider on agent")
                })?;

                let assistant_tools: Vec<tools::ModelEx> = agent_tools::Entity::find()
                    .find_also_related(tools::Entity)
                    .filter(agent_tools::Column::AgentId.eq(agent.id))
                    .all(&self.context.db)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = %e, agent_id = agent.id, "failed to fetch assistant tools");
                        tonic::Status::internal("Failed to fetch assistant tools")
                    })?
                    .into_iter()
                    .filter_map(|(_, tool)| tool.map(|t| t.into_ex()))
                    .collect();

                Ok::<(agents::ModelEx, language_models::ModelEx, providers::ModelEx, _), tonic::Status>((
                    agent.into_ex(),
                    model.into_ex(),
                    provider.into_ex(),
                    assistant_tools,
                ))
            }
            None => {
                let assistant_provider = std::env::var("ASSISTANT_PROVIDER").map_err(|e| {
                    tracing::error!(error = %e, "ASSISTANT_PROVIDER env var must be set.");
                    tonic::Status::internal("No default provider configured.")
                })?;

                let assistant_model = std::env::var("ASSISTANT_MODEL").map_err(|e| {
                    tracing::error!(error = %e, "ASSISTANT_MODEL env var must be set.");
                    tonic::Status::internal("No default model configured.")
                })?;

                let assistant_tool_names = [
                    DbQueryTool::<DefaultEmbedding>::NAME,
                    CreateAgentTool::<DefaultEmbedding>::NAME,
                    CurrentTimeTool::NAME,
                    DOCUMENT_SEARCH_TOOL_NAME,
                ];

                let model = language_models::Entity::load()
                    .filter(language_models::Column::ModelId.eq(assistant_model))
                    .with(providers::Entity)
                    .filter(providers::Column::ProviderType.eq(assistant_provider))
                    .one(&self.context.db)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = %e, "failed to fetch model");
                        tonic::Status::internal("Failed to fetch model.")
                    })?
                    .ok_or_else(|| {
                        tracing::error!("failed to fetch model");
                        tonic::Status::internal("Failed to fetch model.")
                    })?;

                let provider = model.providers.into_option().ok_or_else(|| {
                    tracing::error!("failed to fetch provider");
                    tonic::Status::internal("Failed to fetch provider.")
                })?;

                let tool_pids = tools::Entity::load()
                    .filter(tools::Column::Name.is_in(assistant_tool_names))
                    .all(&self.context.db)
                    .await
                    .map_err(|e| {
                        tracing::error!(error = %e, "failed to fetch tools");
                        tonic::Status::internal("Failed to fetch assistant tools.")
                    })?
                    .into_iter()
                    .map(|t| t.pid)
                    .collect();

                let res = AgentOps::create(
                    Arc::new(self.context.db.clone()),
                    &user_id,
                    &organization_pid,
                    v1::CreateAgentRequest {
                        name: "Assistant".to_string(),
                        model_pid: model.pid,
                        temperature: 40,
                        system_prompt: "You are a helpful assistant agent for the Startup platform".to_string(),
                        config: None,
                        provider_pid: Some(provider.pid),
                        tool_pids,
                        kind: v1::AgentKind::Startup.into(),
                    },
                )
                .await?;

                user_assistants::Entity::insert(
                    user_assistants::ActiveModelEx::new()
                        .set_user_id(&user_id)
                        .set_agent_id(res.agent.id),
                )
                .on_conflict(
                    sea_orm::sea_query::OnConflict::column(user_assistants::Column::UserId)
                        .update_column(user_assistants::Column::AgentId)
                        .to_owned(),
                )
                .exec(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(agent_id = &res.agent.id, error = %e, "failed to assign agent to assistant");
                    tonic::Status::internal("Failed to save your Assistant agent.")
                })?;

                Ok((res.agent, res.model, res.provider, res.tools))
            }
        }?;

        let agent = agents::Entity::assemble_agent(agent, model, provider, None, assistant_tools).map_err(|e| {
            tracing::error!(error = %e, "failed to build domain agent");
            tonic::Status::internal("Failed to build agent")
        })?;

        Ok(tonic::Response::new(v1::GetUserAssistantResponse {
            agent: Some(v1::Agent::from(&agent)),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, agent_pid))]
    async fn share_agent(
        &self,
        request: tonic::Request<v1::ShareAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::ShareAgentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("agent_pid", &req.agent_pid);

        // Find agent and verify ownership
        let agent = agents::Entity::find()
            .filter(agents::Column::Pid.eq(&req.agent_pid))
            .filter(agents::Column::IsActive.eq(true))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch agent");
                tonic::Status::internal("Failed to fetch agent")
            })?
            .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

        // Only the creator can share
        if agent.created_by != user_id {
            return Err(tonic::Status::permission_denied("Only the agent owner can share"));
        }

        // Agent must be in the request's org
        if !agent.is_accessible_in_org(&organization_pid) {
            return Err(tonic::Status::not_found("Agent not found"));
        }

        // Verify recipient is a member of the agent's org
        let cache = UserCache::new(self.context.redis.clone(), REDIS_USER_CACHE_KEY);
        let recipient_data = cache
            .get(&req.user_id)
            .await
            .ok_or_else(|| tonic::Status::failed_precondition("Recipient user not found"))?;
        if recipient_data.org(&agent.organization_pid).is_none() {
            return Err(tonic::Status::failed_precondition(
                "Recipient is not a member of the agent's organization",
            ));
        }

        let permissions = agent_users::Permissions::from(req.permissions as i16);

        // Upsert share record
        let existing = agent_users::Entity::find()
            .filter(agent_users::Column::AgentId.eq(agent.id))
            .filter(agent_users::Column::UserId.eq(&req.user_id))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to check existing share");
                tonic::Status::internal("Failed to share agent")
            })?;

        if let Some(existing) = existing {
            existing
                .into_active_model()
                .into_ex()
                .set_permissions(permissions.value())
                .update(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to update share");
                    tonic::Status::internal("Failed to update share")
                })?;
        } else {
            agent_users::ActiveModelEx::new()
                .set_agent_id(agent.id)
                .set_user_id(&req.user_id)
                .set_permissions(permissions.value())
                .set_shared_by(&user_id)
                .insert(&self.context.db)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to create share");
                    tonic::Status::internal("Failed to share agent")
                })?;
        }

        tracing::info!(shared_with = %req.user_id, permissions = %permissions, "Agent shared");
        Ok(tonic::Response::new(v1::ShareAgentResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, agent_pid))]
    async fn unshare_agent(
        &self,
        request: tonic::Request<v1::UnshareAgentRequest>,
    ) -> std::result::Result<tonic::Response<v1::UnshareAgentResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let req = request.into_inner();
        tracing::Span::current().record("agent_pid", &req.agent_pid);

        let agent = agents::Entity::find()
            .filter(agents::Column::Pid.eq(&req.agent_pid))
            .filter(agents::Column::IsActive.eq(true))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch agent");
                tonic::Status::internal("Failed to fetch agent")
            })?
            .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

        if agent.created_by != user_id {
            return Err(tonic::Status::permission_denied("Only the agent owner can unshare"));
        }

        if !agent.is_accessible_in_org(&organization_pid) {
            return Err(tonic::Status::not_found("Agent not found"));
        }

        agent_users::Entity::delete_many()
            .filter(agent_users::Column::AgentId.eq(agent.id))
            .filter(agent_users::Column::UserId.eq(&req.user_id))
            .exec(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to unshare agent");
                tonic::Status::internal("Failed to unshare agent")
            })?;

        tracing::info!(unshared_from = %req.user_id, "Agent unshared");
        Ok(tonic::Response::new(v1::UnshareAgentResponse {}))
    }

    #[instrument(skip(self, request), fields(user_id, agent_pid))]
    async fn list_agent_shares(
        &self,
        request: tonic::Request<v1::ListAgentSharesRequest>,
    ) -> std::result::Result<tonic::Response<v1::ListAgentSharesResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let organization_pid = request.organization_pid()?;
        let auth_token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .unwrap_or_default()
            .to_string();
        let req = request.into_inner();
        tracing::Span::current().record("agent_pid", &req.agent_pid);

        let agent = agents::Entity::find()
            .filter(agents::Column::Pid.eq(&req.agent_pid))
            .filter(agents::Column::IsActive.eq(true))
            .one(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to fetch agent");
                tonic::Status::internal("Failed to fetch agent")
            })?
            .ok_or_else(|| tonic::Status::not_found("Agent not found"))?;

        // Must be owner or have access to list shares
        if agent.created_by != user_id && !agent.is_accessible_in_org(&organization_pid) {
            return Err(tonic::Status::not_found("Agent not found"));
        }

        let page_size = if req.page_size == 0 {
            20
        } else {
            (req.page_size as u64).clamp(1, 100)
        };

        // Total count
        let total = agent_users::Entity::find()
            .filter(agent_users::Column::AgentId.eq(agent.id))
            .count(&self.context.db)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to count shares");
                tonic::Status::internal("Failed to count shares")
            })? as i32;

        // Cursor pagination: sort by (created_at, id)
        let mut cursor = agent_users::Entity::find()
            .filter(agent_users::Column::AgentId.eq(agent.id))
            .cursor_by((agent_users::Column::CreatedAt, agent_users::Column::Id));

        if let Some(ref cursor_str) = req.cursor {
            let c = agent_users::ShareCursor::try_from(cursor_str.as_str())?;
            cursor.after(c.into_inner());
        }

        let share_records = cursor.first(page_size).all(&self.context.db).await.map_err(|e| {
            tracing::error!(error = %e, "failed to list shares");
            tonic::Status::internal("Failed to list shares")
        })?;

        let next_cursor = if share_records.len() == page_size as usize {
            share_records
                .last()
                .map(|last| agent_users::ShareCursor::new((last.created_at, last.id)).to_string())
        } else {
            None
        };

        // Enrich with user display data from api-service
        let share_user_ids: Vec<String> = share_records.iter().map(|s| s.user_id.clone()).collect();
        let user_map = if !auth_token.is_empty() && !share_records.is_empty() {
            self.context
                .api_service
                .get_users(&auth_token, &organization_pid, share_user_ids)
                .await
                .unwrap_or_default()
        } else {
            Default::default()
        };

        let shares = share_records
            .into_iter()
            .map(|s| {
                let user_info = user_map.get(&s.user_id);
                v1::AgentShare {
                    display_name: user_info.and_then(|u| u.display_name.clone()),
                    email: user_info.map(|u| u.email.clone()).unwrap_or_default(),
                    user_id: s.user_id,
                    permissions: s.permissions as i32,
                    shared_by: s.shared_by,
                    created_at: s.created_at.and_utc().to_rfc3339(),
                }
            })
            .collect();

        Ok(tonic::Response::new(v1::ListAgentSharesResponse {
            shares,
            next_cursor,
            total,
        }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::{TestContext, init_test_crypto};
    use platform_rs::middleware::{auth::User, organization::OrganizationPid};
    use proto_rs::rig::v1::agent_service_server::AgentService as _;
    use serial_test::serial;
    use test_context::test_context;

    /// Creates an authenticated tonic::Request with user_id in User
    fn authenticated_request<T>(inner: T, user_id: &str, org_pid: Option<&str>) -> tonic::Request<T> {
        authenticated_request_with_token(inner, user_id, org_pid, None)
    }

    /// Creates an authenticated tonic::Request with real tokens for service-to-service calls.
    fn authenticated_request_with_token<T>(
        inner: T,
        user_id: &str,
        org_pid: Option<&str>,
        tokens: Option<&platform_rs::test_utils::TokenPair>,
    ) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        request.extensions_mut().insert(User {
            id: user_id.to_string(),
            email: Some(format!("{user_id}@test.com")),
            name: Some(format!("Test User {user_id}")),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        });
        if let Some(org) = org_pid {
            request.extensions_mut().insert(OrganizationPid(org.to_string()));
        }
        if let Some(tp) = tokens {
            request
                .metadata_mut()
                .insert("authorization", format!("Bearer {}", tp.access_token).parse().unwrap());
            if let Some(id_token) = &tp.id_token {
                request.metadata_mut().insert("x-id-token", id_token.parse().unwrap());
            }
        }
        request
    }

    /// Creates AgentService for testing
    fn create_service(ctx: &TestContext) -> AgentService<crate::test_utils::MockEmbeddingModel> {
        AgentService::new(ctx.app_ctx())
    }

    mod list_agents {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_empty_list_when_no_agents(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListAgentsRequest {
                    user_id: None,
                    is_active: None,
                    provider_pid: None,
                    page: 1,
                    page_size: 20,
                    with_assistants: None,
                },
                "user_with_no_agents",
                Some("org_empty"),
            );

            let response = service.list_agents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(response.agents.is_empty(), "should have no agents");
            assert_eq!(response.total, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_agents_for_user(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("list-agents", "ollama").await;
            ctx.create_api_key_secret("list-agents", provider.id).await;
            let model = ctx.create_model("list-agents", provider.id).await;
            let agent = ctx.create_personal_agent("list-agents", model.id, "user_test").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::ListAgentsRequest {
                    user_id: None,
                    is_active: None,
                    provider_pid: None,
                    page: 1,
                    page_size: 20,
                    with_assistants: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.list_agents(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response = response.unwrap().into_inner();
            assert!(!response.agents.is_empty(), "should have agents");

            let returned_agent = response
                .agents
                .iter()
                .find(|a| {
                    a.identifier
                        .as_ref()
                        .map(|id| matches!(id, proto_rs::rig::v1::agent::Identifier::Pid(p) if p == &agent.pid))
                        .unwrap_or(false)
                })
                .expect("should include created agent");

            // Verify join chain: agent → model → provider
            let agent_model = returned_agent.model.as_ref().expect("should have model");
            assert_eq!(
                agent_model.pid, model.pid,
                "should populate model pid from language_models join"
            );
            assert_eq!(
                agent_model.model_id, model.model_id,
                "should populate model_id from language_models join"
            );
            let agent_provider = agent_model.provider.as_ref().expect("should have provider");
            assert_eq!(
                agent_provider.pid, provider.pid,
                "should populate provider pid from providers join"
            );
            assert_eq!(
                agent_provider.name, provider.name,
                "should populate provider name from providers join"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_is_active(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("list-active", "ollama").await;
            ctx.create_api_key_secret("list-active", provider.id).await;
            let model = ctx.create_model("list-active", provider.id).await;
            let agent = ctx.create_personal_agent("list-active", model.id, "user_test").await;

            let service = create_service(ctx);

            // Delete the agent (soft delete)
            let delete_request = authenticated_request(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );
            service.delete_agent(delete_request).await.unwrap();

            // List only active agents - should not include deleted agent
            let request = authenticated_request(
                v1::ListAgentsRequest {
                    user_id: None,
                    is_active: Some(true),
                    provider_pid: None,
                    page: 1,
                    page_size: 20,
                    with_assistants: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.list_agents(request).await.unwrap().into_inner();
            assert!(
                !response.agents.iter().any(|a| {
                    a.identifier
                        .as_ref()
                        .map(|id| matches!(id, proto_rs::rig::v1::agent::Identifier::Pid(p) if p == &agent.pid))
                        .unwrap_or(false)
                }),
                "should not include deleted agent when filtering for active"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_respect_pagination(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("list-page", "ollama").await;
            ctx.create_api_key_secret("list-page", provider.id).await;
            let model = ctx.create_model("list-page", provider.id).await;

            // Create 5 agents
            for i in 0..5 {
                ctx.create_personal_agent(&format!("list-page-{i}"), model.id, "user_test")
                    .await;
            }

            let service = create_service(ctx);

            // Request page 1 with size 2
            let request = authenticated_request(
                v1::ListAgentsRequest {
                    user_id: None,
                    is_active: None,
                    provider_pid: None,
                    page: 1,
                    page_size: 2,
                    with_assistants: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.list_agents(request).await.unwrap().into_inner();
            assert_eq!(response.agents.len(), 2, "should return 2 agents");
            assert!(response.total >= 5, "total should be at least 5");
            assert_eq!(response.page, 1);
            assert_eq!(response.page_size, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_other_users_agents(ctx: &mut TestContext) {
            init_test_crypto();
            // Create agent for other_user
            let provider = ctx.create_system_provider("list-iso", "ollama").await;
            ctx.create_api_key_secret("list-iso", provider.id).await;
            let model = ctx.create_model("list-iso", provider.id).await;
            let other_agent = ctx.create_personal_agent("list-iso", model.id, "other_user").await;

            let service = create_service(ctx);

            // Request as user_test
            let request = authenticated_request(
                v1::ListAgentsRequest {
                    user_id: None,
                    is_active: None,
                    provider_pid: None,
                    page: 1,
                    page_size: 20,
                    with_assistants: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.list_agents(request).await.unwrap().into_inner();
            assert!(
                !response.agents.iter().any(|a| {
                    a.identifier
                        .as_ref()
                        .map(|id| matches!(id, proto_rs::rig::v1::agent::Identifier::Pid(p) if p == &other_agent.pid))
                        .unwrap_or(false)
                }),
                "should not include other user's agent"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_when_unauthenticated(ctx: &mut TestContext) {
            let service = create_service(ctx);

            // Request without auth extensions
            let request = tonic::Request::new(v1::ListAgentsRequest {
                user_id: None,
                is_active: None,
                provider_pid: None,
                page: 1,
                page_size: 20,
                with_assistants: None,
            });

            let response = service.list_agents(request).await;
            assert!(response.is_err(), "should fail without authentication");
            assert_eq!(
                response.unwrap_err().code(),
                tonic::Code::Unauthenticated,
                "should return UNAUTHENTICATED"
            );
        }
    }

    mod get_agent {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_agent_by_pid(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("get-agent", "ollama").await;
            ctx.create_api_key_secret("get-agent", provider.id).await;
            let model = ctx.create_model("get-agent", provider.id).await;
            let agent = ctx.create_personal_agent("get-agent", model.id, "user_test").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.get_agent(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let response_agent = response.unwrap().into_inner().agent.unwrap();
            assert!(matches!(
                response_agent.identifier,
                Some(proto_rs::rig::v1::agent::Identifier::Pid(ref p)) if p == &agent.pid
            ));
            assert_eq!(response_agent.name, agent.name);

            // Verify join chain: agent → model → provider
            let agent_model = response_agent.model.as_ref().expect("should have model");
            assert_eq!(
                agent_model.pid, model.pid,
                "should populate model pid from language_models join"
            );
            assert_eq!(
                agent_model.model_id, model.model_id,
                "should populate model_id from language_models join"
            );
            let agent_provider = agent_model.provider.as_ref().expect("should have provider");
            assert_eq!(
                agent_provider.pid, provider.pid,
                "should populate provider pid from providers join"
            );
            assert_eq!(
                agent_provider.name, provider.name,
                "should populate provider name from providers join"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_agent_kind_config_and_tools_through_loaded_boundary(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("get-agent-rich", "ollama").await;
            ctx.create_api_key_secret("get-agent-rich", provider.id).await;
            let model = ctx.create_model("get-agent-rich", provider.id).await;
            let tool = ctx.create_tool("get-agent-rich-tool", "search_docs", "system").await;

            let service = create_service(ctx);

            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let create_request = authenticated_request(
                v1::CreateAgentRequest {
                    name: format!("Loaded Boundary Agent {ts}"),
                    model_pid: model.pid.clone(),
                    temperature: 65,
                    system_prompt: "Loaded agent prompt".to_string(),
                    config: Some(v1::AgentConfig {
                        base: Some(v1::BaseAgentConfig {
                            history_length: Some(7),
                            session_ttl_seconds: Some(900),
                            streaming_enabled: Some(true),
                            thinking_enabled: Some(false),
                            timeout_seconds: Some(45),
                            idle_timeout_seconds: Some(120),
                            memory_enabled: Some(false),
                            memory_result_count: Some(3),
                            memory_similarity_threshold: Some(0.42),
                            document_result_count: Some(9),
                            compaction_threshold: Some(0.85),
                            compaction_keep_ratio: Some(0.15),
                            max_tokens: Some(321),
                        }),
                        kind_config: Some(v1::agent_config::KindConfig::Startup(v1::StartupAgentConfig {
                            use_system_providers_on_creation: Some(false),
                        })),
                    }),
                    provider_pid: None,
                    tool_pids: vec![tool.pid.clone()],
                    kind: v1::AgentKind::Ephemeral.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let created = service
                .create_agent(create_request)
                .await
                .unwrap()
                .into_inner()
                .agent
                .unwrap();
            let created_pid = match created.identifier.as_ref() {
                Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) => pid.clone(),
                other => panic!("expected pid identifier, got {other:?}"),
            };

            let get_request = authenticated_request(
                v1::GetAgentRequest { pid: created_pid },
                "user_test",
                Some("personal_org_user_test"),
            );
            let fetched = service
                .get_agent(get_request)
                .await
                .unwrap()
                .into_inner()
                .agent
                .unwrap();

            assert_eq!(fetched.kind, v1::AgentKind::Ephemeral as i32);
            assert_eq!(fetched.tools.len(), 1, "should return attached tools");
            assert_eq!(fetched.tools[0].pid, tool.pid);
            assert_eq!(fetched.tools[0].name, tool.display_name);

            let config = fetched.config.expect("should return config");
            let base = config.base.expect("should return base config");
            assert_eq!(base.history_length, Some(7));
            assert_eq!(base.thinking_enabled, Some(false));
            assert_eq!(base.memory_enabled, Some(false));
            assert_eq!(base.document_result_count, Some(9));
            assert_eq!(base.max_tokens, Some(321));

            match config.kind_config {
                Some(v1::agent_config::KindConfig::Startup(startup)) => {
                    assert_eq!(startup.use_system_providers_on_creation, Some(false));
                }
                other => panic!("expected startup kind config, got {other:?}"),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_agent(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetAgentRequest {
                    pid: "nonexistent-agent".to_string(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.get_agent(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_agent(ctx: &mut TestContext) {
            init_test_crypto();
            // Create agent for other_user
            let provider = ctx.create_system_provider("get-iso", "ollama").await;
            ctx.create_api_key_secret("get-iso", provider.id).await;
            let model = ctx.create_model("get-iso", provider.id).await;
            let other_agent = ctx.create_personal_agent("get-iso", model.id, "other_user").await;

            let service = create_service(ctx);

            // Try to get as user_test
            let request = authenticated_request(
                v1::GetAgentRequest {
                    pid: other_agent.pid.clone(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.get_agent(request).await;
            assert!(response.is_err(), "should fail for other user's agent");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod create_agent {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_agent_with_valid_data(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("create-agent", "ollama").await;
            ctx.create_api_key_secret("create-agent", provider.id).await;
            let model = ctx.create_model("create-agent", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "My Test Agent".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let agent = response.unwrap().into_inner().agent.unwrap();
            assert!(agent.identifier.is_some(), "should have identifier");
            assert_eq!(agent.name, "My Test Agent");
            let agent_model = agent.model.as_ref().expect("should have model");
            assert_eq!(agent_model.pid, model.pid);
            assert_eq!(agent_model.model_id, model.model_id);
            assert_eq!(agent.temperature, 70);
            assert_eq!(agent.system_prompt, "You are helpful.");
            assert!(agent.is_active);

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &agent.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_missing_name(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("create-noname", "ollama").await;
            ctx.create_api_key_secret("create-noname", provider.id).await;
            let model = ctx.create_model("create-noname", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: String::new(), // Empty name
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_err(), "should fail for empty name");
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_invalid_model(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "Test Agent".to_string(),
                    model_pid: "nonexistent-model".to_string(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_err(), "should fail for invalid model");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_error_for_missing_system_prompt(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("create-noprompt", "ollama").await;
            ctx.create_api_key_secret("create-noprompt", provider.id).await;
            let model = ctx.create_model("create-noprompt", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "Test Agent".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: String::new(), // Empty prompt
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_err(), "should fail for empty system_prompt");
            assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);
        }
    }

    mod update_agent {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_agent_name(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("update-name", "ollama").await;
            ctx.create_api_key_secret("update-name", provider.id).await;
            let model = ctx.create_model("update-name", provider.id).await;
            let agent = ctx.create_personal_agent("update-name", model.id, "user_test").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Updated Name".to_string()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("You are a helpful assistant.".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.update_agent(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let updated = response.unwrap().into_inner().agent.unwrap();
            assert_eq!(updated.name, "Updated Name");
            // Slug should remain unchanged when not explicitly provided
            assert_eq!(
                updated.slug, agent.slug,
                "slug should not auto-change when name changes"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_all_fields(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("update-all", "ollama").await;
            ctx.create_api_key_secret("update-all", provider.id).await;
            let model = ctx.create_model("update-all", provider.id).await;
            let agent = ctx.create_personal_agent("update-all", model.id, "user_test").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("New Name".to_string()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(50),
                    system_prompt: Some("New system prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.update_agent(request).await.unwrap().into_inner();
            let updated = response.agent.unwrap();

            assert_eq!(updated.name, "New Name");
            assert_eq!(updated.temperature, 50);
            assert_eq!(updated.system_prompt, "New system prompt");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("update-notfound", "ollama").await;
            ctx.create_api_key_secret("update-notfound", provider.id).await;
            let model = ctx.create_model("update-notfound", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: "nonexistent-agent".to_string(),
                    name: Some("New Name".to_string()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Test prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.update_agent(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_agent(ctx: &mut TestContext) {
            init_test_crypto();
            // Create agent for other_user
            let provider = ctx.create_system_provider("update-iso", "ollama").await;
            ctx.create_api_key_secret("update-iso", provider.id).await;
            let model = ctx.create_model("update-iso", provider.id).await;
            let other_agent = ctx.create_personal_agent("update-iso", model.id, "other_user").await;

            let service = create_service(ctx);

            // Try to update as user_test
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: other_agent.pid.clone(),
                    name: Some("Hacked Name".to_string()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Hacked prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.update_agent(request).await;
            assert!(response.is_err(), "should fail for other user's agent");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod delete_agent {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_soft_delete_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("delete-agent", "ollama").await;
            ctx.create_api_key_secret("delete-agent", provider.id).await;
            let model = ctx.create_model("delete-agent", provider.id).await;
            let agent = ctx.create_personal_agent("delete-agent", model.id, "user_test").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.delete_agent(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            // Verify it's soft-deleted
            let get_request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );
            let _get_response = service.get_agent(get_request).await;
            // Agent still exists but is_active=false, so it should still be found
            // but let's check it was marked inactive in the database
            let deleted = crate::models::agents::Entity::find()
                .filter(crate::models::agents::Column::Pid.eq(&agent.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .unwrap();
            assert!(!deleted.is_active, "agent should be marked inactive");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_nonexistent_agent(ctx: &mut TestContext) {
            let service = create_service(ctx);

            let request = authenticated_request(
                v1::DeleteAgentRequest {
                    pid: "nonexistent-agent".to_string(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.delete_agent(request).await;
            assert!(response.is_err(), "should fail");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_already_deleted_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("delete-twice", "ollama").await;
            ctx.create_api_key_secret("delete-twice", provider.id).await;
            let model = ctx.create_model("delete-twice", provider.id).await;
            let agent = ctx.create_personal_agent("delete-twice", model.id, "user_test").await;

            let service = create_service(ctx);

            // Delete once
            let request = authenticated_request(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );
            service.delete_agent(request).await.unwrap();

            // Try to delete again
            let request = authenticated_request(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("personal_org_user_test"),
            );
            let response = service.delete_agent(request).await;
            assert!(response.is_err(), "should fail for already deleted");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_other_users_agent(ctx: &mut TestContext) {
            init_test_crypto();
            // Create agent for other_user
            let provider = ctx.create_system_provider("delete-iso", "ollama").await;
            ctx.create_api_key_secret("delete-iso", provider.id).await;
            let model = ctx.create_model("delete-iso", provider.id).await;
            let other_agent = ctx.create_personal_agent("delete-iso", model.id, "other_user").await;

            let service = create_service(ctx);

            // Try to delete as user_test
            let request = authenticated_request(
                v1::DeleteAgentRequest {
                    pid: other_agent.pid.clone(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.delete_agent(request).await;
            assert!(response.is_err(), "should fail for other user's agent");
            assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);
        }
    }

    mod get_user_assistant {
        use super::*;
        use crate::models::user_assistants;

        // =========================================================================
        // Happy Path Tests
        // =========================================================================

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_existing_assistant_via_junction_table(ctx: &mut TestContext) {
            init_test_crypto();

            // Create assistant backed by system provider (required for get_user_assistant)
            let agent = ctx
                .create_system_user_assistant("get-assist-junc", "user_assist_junc")
                .await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_assist_junc".to_string(),
                },
                "user_assist_junc",
                Some("personal_org_user_assist_junc"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let assistant = response.unwrap().into_inner().agent.unwrap();
            assert!(matches!(
                assistant.identifier,
                Some(proto_rs::rig::v1::agent::Identifier::Pid(ref p)) if p == &agent.pid
            ));
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_assistant_when_none_exists(ctx: &mut TestContext) {
            init_test_crypto();
            // Set up the provider/model that ASSISTANT_PROVIDER/ASSISTANT_MODEL env vars point to
            let provider = ctx.create_system_provider("create-assist", "ollama").await;
            ctx.create_api_key_secret("create-assist", provider.id).await;
            ctx.create_model("create-assist", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "new_user_create".to_string(),
                },
                "new_user_create",
                Some("personal_org_new_user_create"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let assistant = response.unwrap().into_inner().agent.unwrap();
            assert_eq!(assistant.name, "Assistant");
            assert!(assistant.is_active);
            assert_eq!(assistant.created_by_user_id, "new_user_create");

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_same_assistant_on_subsequent_calls(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("idempotent", "ollama").await;
            ctx.create_api_key_secret("idempotent", provider.id).await;
            ctx.create_model("idempotent", provider.id).await;

            let service = create_service(ctx);

            // First call - creates assistant
            let request1 = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_idempotent".to_string(),
                },
                "user_idempotent",
                Some("personal_org_user_idempotent"),
            );
            let response1 = service.get_user_assistant(request1).await.unwrap().into_inner();
            let assistant1 = response1.agent.unwrap();

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant1.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_agents.push(created.id);
            }

            // Second call - should return same assistant
            let request2 = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_idempotent".to_string(),
                },
                "user_idempotent",
                Some("personal_org_user_idempotent"),
            );
            let response2 = service.get_user_assistant(request2).await.unwrap().into_inner();
            let assistant2 = response2.agent.unwrap();

            assert_eq!(
                assistant1.identifier, assistant2.identifier,
                "should return same assistant"
            );
        }

        // =========================================================================
        // Junction Table Behavior Tests
        // =========================================================================

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_lookup_assistant_by_junction_not_kind(ctx: &mut TestContext) {
            init_test_crypto();
            let (_provider, model) = ctx.get_system_provider_and_model().await;

            // Create a regular agent with kind="startup" but NOT linked via junction
            let _regular_agent = ctx
                .create_personal_agent("junc-not-kind-regular", model.id, "user_junc_lookup")
                .await;

            // Create the actual assistant linked via junction (backed by system provider)
            let assistant_agent = ctx
                .create_system_user_assistant("junc-not-kind-assist", "user_junc_lookup")
                .await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_junc_lookup".to_string(),
                },
                "user_junc_lookup",
                Some("personal_org_user_junc_lookup"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let returned = response.unwrap().into_inner().agent.unwrap();

            // Should return the junction-linked assistant, not the regular agent
            assert!(
                matches!(
                    returned.identifier,
                    Some(proto_rs::rig::v1::agent::Identifier::Pid(ref p)) if p == &assistant_agent.pid
                ),
                "should return junction-linked assistant, not regular agent"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_return_inactive_assistant(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("inactive-assist", "ollama").await;
            ctx.create_api_key_secret("inactive-assist", provider.id).await;
            let model = ctx.create_model("inactive-assist", provider.id).await;

            // Create assistant and link it
            let agent = ctx
                .create_user_assistant("inactive-assist", model.id, "user_inactive")
                .await;

            // Deactivate the assistant
            let mut active: agents::ActiveModel = agent.into();
            active.is_active = Set(false);
            active.update(ctx.db.as_ref()).await.unwrap();

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_inactive".to_string(),
                },
                "user_inactive",
                Some("personal_org_user_inactive"),
            );

            // Should create a new assistant since the existing one is inactive
            let response = service.get_user_assistant(request).await;
            assert!(
                response.is_ok(),
                "should succeed by creating new assistant: {:?}",
                response.err()
            );

            let assistant = response.unwrap().into_inner().agent.unwrap();
            assert!(assistant.is_active, "should return an active assistant");
            assert_eq!(assistant.name, "Assistant", "should be newly created assistant");

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_agents.push(created.id);
            }
        }

        // =========================================================================
        // Assistant Creation Behavior Tests
        // =========================================================================

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_assistant_in_personal_context(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("personal-ctx", "ollama").await;
            ctx.create_api_key_secret("personal-ctx", provider.id).await;
            ctx.create_model("personal-ctx", provider.id).await;

            let service = create_service(ctx);

            // Request with org context - assistant should still be personal
            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_personal_ctx".to_string(),
                },
                "user_personal_ctx",
                Some("org_test"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let assistant = response.unwrap().into_inner().agent.unwrap();

            // Track for cleanup and verify personal context
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();

                assert!(
                    !created.organization_pid.is_empty(),
                    "assistant should have an organization_pid"
                );

                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_use_system_provider_model(ctx: &mut TestContext) {
            init_test_crypto();
            // The seed lifecycle creates a system provider with ASSISTANT_MODEL
            // Verify assistant uses the system provider's model, not a personal one
            let assistant_model_id = std::env::var("ASSISTANT_MODEL").expect("ASSISTANT_MODEL should be set");

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_system_model".to_string(),
                },
                "user_system_model",
                Some("personal_org_user_system_model"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let assistant = response.unwrap().into_inner().agent.unwrap();
            let assistant_model = assistant.model.as_ref().expect("should have model");

            // Verify the assistant's model is from the system provider
            // by checking the model_id matches the env var
            let model = language_models::Entity::find()
                .filter(language_models::Column::Pid.eq(&assistant_model.pid))
                .one(ctx.db.as_ref())
                .await
                .unwrap()
                .expect("model should exist");

            assert_eq!(
                model.model_id, assistant_model_id,
                "should use system provider's model configured via ASSISTANT_MODEL env var"
            );
            assert_eq!(
                assistant_model.model_id, assistant_model_id,
                "should populate model_id on agent's nested model"
            );

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();
                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_insert_junction_record_on_creation(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("junc-insert", "ollama").await;
            ctx.create_api_key_secret("junc-insert", provider.id).await;
            ctx.create_model("junc-insert", provider.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_junc_insert".to_string(),
                },
                "user_junc_insert",
                Some("personal_org_user_junc_insert"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let assistant = response.unwrap().into_inner().agent.unwrap();

            // Verify junction record exists
            let junction = user_assistants::Entity::find_by_id("user_junc_insert".to_string())
                .one(ctx.db.as_ref())
                .await
                .unwrap();

            assert!(junction.is_some(), "junction record should exist");

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &assistant.identifier {
                let created = crate::models::agents::Entity::find()
                    .filter(crate::models::agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();

                assert_eq!(
                    junction.unwrap().agent_id,
                    created.id,
                    "junction should link to created agent"
                );

                ctx.created_agents.push(created.id);
            }
        }
    }

    mod tool_attachment {
        use super::*;

        /// Helper to get tool IDs attached to an agent
        async fn get_attached_tool_ids(ctx: &TestContext, agent_id: i64) -> Vec<i64> {
            agent_tools::Entity::find()
                .filter(agent_tools::Column::AgentId.eq(agent_id))
                .all(ctx.db.as_ref())
                .await
                .unwrap()
                .iter()
                .map(|at| at.tool_id)
                .collect()
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_attach_tools_on_create(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("create-tools", "ollama").await;
            ctx.create_api_key_secret("create-tools", provider.id).await;
            let model = ctx.create_model("create-tools", provider.id).await;

            let tool1 = ctx.create_tool("create-tools-1", "test_tool_1", "system").await;
            let tool2 = ctx.create_tool("create-tools-2", "test_tool_2", "system").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "Agent With Tools".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![tool1.pid.clone(), tool2.pid.clone()],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_ok(), "should succeed: {:?}", response.err());

            let agent = response.unwrap().into_inner().agent.unwrap();

            // Track for cleanup
            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &agent.identifier {
                let created = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();

                // Verify tools are attached
                let attached = get_attached_tool_ids(ctx, created.id).await;
                assert_eq!(attached.len(), 2, "should have 2 tools attached");
                assert!(attached.contains(&tool1.id), "should contain tool1");
                assert!(attached.contains(&tool2.id), "should contain tool2");

                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_replace_tools_on_update(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("update-tools", "ollama").await;
            ctx.create_api_key_secret("update-tools", provider.id).await;
            let model = ctx.create_model("update-tools", provider.id).await;
            let agent = ctx.create_personal_agent("update-tools", model.id, "user_test").await;

            let tool1 = ctx.create_tool("update-tools-1", "tool_one", "system").await;
            let tool2 = ctx.create_tool("update-tools-2", "tool_two", "system").await;
            let tool3 = ctx.create_tool("update-tools-3", "tool_three", "system").await;

            let service = create_service(ctx);

            // First update: attach tool1 and tool2
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some(agent.name.clone()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Test prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![
                        v1::UpdateAgentToolRequest {
                            pid: tool1.pid.clone(),
                            is_active: true,
                        },
                        v1::UpdateAgentToolRequest {
                            pid: tool2.pid.clone(),
                            is_active: true,
                        },
                    ],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            service.update_agent(request).await.unwrap();

            let attached = get_attached_tool_ids(ctx, agent.id).await;
            assert_eq!(attached.len(), 2, "should have 2 tools after first update");
            assert!(attached.contains(&tool1.id));
            assert!(attached.contains(&tool2.id));

            // Second update: add tool3, remove tool1 and tool2 (incremental behavior)
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some(agent.name.clone()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Test prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![
                        v1::UpdateAgentToolRequest {
                            pid: tool1.pid.clone(),
                            is_active: false, // Remove tool1
                        },
                        v1::UpdateAgentToolRequest {
                            pid: tool2.pid.clone(),
                            is_active: false, // Remove tool2
                        },
                        v1::UpdateAgentToolRequest {
                            pid: tool3.pid.clone(),
                            is_active: true, // Add tool3
                        },
                    ],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            service.update_agent(request).await.unwrap();

            let attached = get_attached_tool_ids(ctx, agent.id).await;
            assert_eq!(attached.len(), 1, "should have 1 tool after second update");
            assert!(attached.contains(&tool3.id), "should only contain tool3");
            assert!(!attached.contains(&tool1.id), "tool1 should be detached");
            assert!(!attached.contains(&tool2.id), "tool2 should be detached");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_ignore_empty_tools_list(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("ignore-empty", "ollama").await;
            ctx.create_api_key_secret("ignore-empty", provider.id).await;
            let model = ctx.create_model("ignore-empty", provider.id).await;
            let agent = ctx.create_personal_agent("ignore-empty", model.id, "user_test").await;

            let tool = ctx.create_tool("ignore-empty-1", "tool_to_keep", "system").await;

            let service = create_service(ctx);

            // Attach a tool
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some(agent.name.clone()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Test prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![v1::UpdateAgentToolRequest {
                        pid: tool.pid.clone(),
                        is_active: true,
                    }],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            service.update_agent(request).await.unwrap();
            assert_eq!(get_attached_tool_ids(ctx, agent.id).await.len(), 1);

            // Empty tools list should be a no-op
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some(agent.name.clone()),
                    slug: None,
                    model_pid: Some(model.pid.clone()),
                    temperature: Some(70),
                    system_prompt: Some("Test prompt".to_string()),
                    config: None,
                    provider_pid: None,
                    tools: vec![],
                    is_active: Some(true),
                    kind: None,
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            service.update_agent(request).await.unwrap();

            let attached = get_attached_tool_ids(ctx, agent.id).await;
            assert_eq!(attached.len(), 1, "tools should remain unchanged with empty list");
            assert!(attached.contains(&tool.id), "original tool should still be attached");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_nonexistent_tool_pids(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("skip-nonexist", "ollama").await;
            ctx.create_api_key_secret("skip-nonexist", provider.id).await;
            let model = ctx.create_model("skip-nonexist", provider.id).await;

            let tool = ctx.create_tool("skip-nonexist-1", "real_tool", "system").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "Agent With Mixed Tools".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![tool.pid.clone(), "nonexistent-tool-pid".to_string()],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_ok(), "should succeed despite nonexistent tool");

            let agent = response.unwrap().into_inner().agent.unwrap();

            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &agent.identifier {
                let created = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();

                let attached = get_attached_tool_ids(ctx, created.id).await;
                assert_eq!(attached.len(), 1, "should only attach the existing tool");
                assert!(attached.contains(&tool.id));

                ctx.created_agents.push(created.id);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_skip_inactive_tools(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("skip-inactive", "ollama").await;
            ctx.create_api_key_secret("skip-inactive", provider.id).await;
            let model = ctx.create_model("skip-inactive", provider.id).await;

            let active_tool = ctx.create_tool("skip-inactive-1", "active_tool", "system").await;
            let inactive_tool = ctx.create_inactive_tool("skip-inactive-2", "inactive_tool").await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::CreateAgentRequest {
                    name: "Agent Skipping Inactive".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: 70,
                    system_prompt: "You are helpful.".to_string(),
                    config: None,
                    provider_pid: None,
                    tool_pids: vec![active_tool.pid.clone(), inactive_tool.pid.clone()],
                    kind: v1::AgentKind::Startup.into(),
                },
                "user_test",
                Some("personal_org_user_test"),
            );

            let response = service.create_agent(request).await;
            assert!(response.is_ok(), "should succeed despite inactive tool");

            let agent = response.unwrap().into_inner().agent.unwrap();

            if let Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) = &agent.identifier {
                let created = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .unwrap();

                let attached = get_attached_tool_ids(ctx, created.id).await;
                assert_eq!(attached.len(), 1, "should only attach active tool");
                assert!(attached.contains(&active_tool.id));
                assert!(!attached.contains(&inactive_tool.id), "should not attach inactive tool");

                ctx.created_agents.push(created.id);
            }
        }
    }

    mod org_access_control {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_access_to_agent_from_different_org(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("org-deny", "ollama").await;
            ctx.create_api_key_secret("org-deny", provider.id).await;
            let model = ctx.create_model("org-deny", provider.id).await;

            // Create agent in org_1
            let agent = ctx.create_org_agent("org-deny", model.id, "org_1", "user_test").await;

            let service = create_service(ctx);

            // Try to access from org_2
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("org_2"),
            );

            let result = service.get_agent(request).await;
            assert!(result.is_err(), "should deny access from different org");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_access_to_agent_from_same_org(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("org-allow", "ollama").await;
            ctx.create_api_key_secret("org-allow", provider.id).await;
            let model = ctx.create_model("org-allow", provider.id).await;

            // Create agent in org_1
            let agent = ctx.create_org_agent("org-allow", model.id, "org_1", "user_test").await;

            let service = create_service(ctx);

            // Access from org_1
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "user_test",
                Some("org_1"),
            );

            let result = service.get_agent(request).await;
            assert!(result.is_ok(), "should allow access from same org: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_assistant_access_from_any_org(ctx: &mut TestContext) {
            init_test_crypto();

            // Create provider and model needed for assistant auto-creation
            let provider = ctx.create_system_provider("assist-cross-org", "ollama").await;
            ctx.create_api_key_secret("assist-cross-org", provider.id).await;
            ctx.create_model("assist-cross-org", provider.id).await;

            let service = create_service(ctx);

            // Get assistant from org_1 — triggers auto-creation in org_1
            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_cross_org".to_string(),
                },
                "user_cross_org",
                Some("org_1"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(
                response.is_ok(),
                "should create assistant in org_1: {:?}",
                response.err()
            );

            let assistant_pid = match &response.unwrap().into_inner().agent.unwrap().identifier {
                Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) => pid.clone(),
                _ => panic!("expected pid identifier"),
            };

            // Now access the same assistant from org_2 — should succeed
            let request = authenticated_request(
                v1::GetUserAssistantRequest {
                    user_id: "user_cross_org".to_string(),
                },
                "user_cross_org",
                Some("org_2"),
            );

            let response = service.get_user_assistant(request).await;
            assert!(
                response.is_ok(),
                "should access assistant from org_2: {:?}",
                response.err()
            );

            let same_pid = match &response.unwrap().into_inner().agent.unwrap().identifier {
                Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) => pid.clone(),
                _ => panic!("expected pid identifier"),
            };

            // Same assistant regardless of org context
            assert_eq!(assistant_pid, same_pid, "should be the same assistant from both orgs");

            // But the same agent accessed via get_agent (not assistant path) from org_2 should fail
            let request = authenticated_request(
                v1::GetAgentRequest {
                    pid: assistant_pid.clone(),
                },
                "user_cross_org",
                Some("org_2"),
            );

            let result = service.get_agent(request).await;
            assert!(
                result.is_err(),
                "get_agent should deny cross-org access even for assistant"
            );
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_get_agent_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("get-no-org", "ollama").await;
            ctx.create_api_key_secret("get-no-org", provider.id).await;
            let model = ctx.create_model("get-no-org", provider.id).await;
            let agent = ctx.create_agent("get-no-org", model.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "user_test",
                None, // no org context
            );

            let result = service.get_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_update_agent_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("update-no-org", "ollama").await;
            ctx.create_api_key_secret("update-no-org", provider.id).await;
            let model = ctx.create_model("update-no-org", provider.id).await;
            let agent = ctx.create_agent("update-no-org", model.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Updated".to_string()),
                    ..Default::default()
                },
                "user_test",
                None,
            );

            let result = service.update_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_delete_agent_without_org_context(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("delete-no-org", "ollama").await;
            ctx.create_api_key_secret("delete-no-org", provider.id).await;
            let model = ctx.create_model("delete-no-org", provider.id).await;
            let agent = ctx.create_agent("delete-no-org", model.id).await;

            let service = create_service(ctx);

            let request = authenticated_request(v1::DeleteAgentRequest { pid: agent.pid.clone() }, "user_test", None);

            let result = service.delete_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }
    }

    mod agent_sharing {
        use super::*;
        use crate::consts::REDIS_USER_CACHE_KEY;
        use platform_rs::cache::{CachedOrg, CachedUserData, OrgRole, PlanTier, UserCache};

        /// Seed the user cache so the org membership check passes at share time.
        async fn seed_recipient_cache(ctx: &TestContext, user_id: &str, org_pid: &str) {
            let cache = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
            cache
                .set(
                    user_id,
                    &CachedUserData {
                        email: format!("{user_id}@test.com"),
                        organizations: vec![CachedOrg::new(org_pid, PlanTier::Free, OrgRole::Member)],
                    },
                )
                .await;
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_share_agent_with_another_user(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let provider = ctx.create_system_provider("share-basic", "ollama").await;
            ctx.create_api_key_secret("share-basic", provider.id).await;
            let model = ctx.create_model("share-basic", provider.id).await;
            let agent = ctx
                .create_org_agent("share-basic", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Owner shares with read permission
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.share_agent(request).await;
            assert!(result.is_ok(), "owner should be able to share: {:?}", result.err());

            // Verify share appears in list
            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_agent_shares(request).await.unwrap().into_inner();
            assert_eq!(response.shares.len(), 1);
            assert_eq!(response.shares[0].user_id, "recipient_user");
            assert_eq!(response.shares[0].permissions, 1);
            assert_eq!(response.shares[0].shared_by, "owner_user");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_upsert_permissions_on_reshare(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let provider = ctx.create_system_provider("share-upsert", "ollama").await;
            ctx.create_api_key_secret("share-upsert", provider.id).await;
            let model = ctx.create_model("share-upsert", provider.id).await;
            let agent = ctx
                .create_org_agent("share-upsert", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Re-share with RWX — should upsert, not create duplicate
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7, // RWX
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_agent_shares(request).await.unwrap().into_inner();
            assert_eq!(
                response.shares.len(),
                1,
                "should have exactly one share record, not two"
            );
            assert_eq!(
                response.shares[0].permissions, 7,
                "permissions should be updated to RWX"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_share_by_non_owner(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-deny", "ollama").await;
            ctx.create_api_key_secret("share-deny", provider.id).await;
            let model = ctx.create_model("share-deny", provider.id).await;
            let agent = ctx
                .create_org_agent("share-deny", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // Non-owner tries to share
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "third_user".to_string(),
                    permissions: 1,
                },
                "not_the_owner",
                Some("org_1"),
            );

            let result = service.share_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_share_with_non_org_member(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-non-member", "ollama").await;
            ctx.create_api_key_secret("share-non-member", provider.id).await;
            let model = ctx.create_model("share-non-member", provider.id).await;
            let agent = ctx
                .create_org_agent("share-non-member", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // Seed recipient as member of org_2, NOT org_1
            seed_recipient_cache(ctx, "outsider_user", "org_2").await;

            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "outsider_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.share_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::FailedPrecondition);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_unshare_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let provider = ctx.create_system_provider("unshare", "ollama").await;
            ctx.create_api_key_secret("unshare", provider.id).await;
            let model = ctx.create_model("unshare", provider.id).await;
            let agent = ctx.create_org_agent("unshare", model.id, "org_1", "owner_user").await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share first
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Unshare
            let request = authenticated_request(
                v1::UnshareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                },
                "owner_user",
                Some("org_1"),
            );

            let result = service.unshare_agent(request).await;
            assert!(result.is_ok(), "unshare should succeed: {:?}", result.err());

            // Verify share is gone
            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 0,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );

            let response = service.list_agent_shares(request).await.unwrap().into_inner();
            assert!(response.shares.is_empty(), "shares should be empty after unshare");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_shared_user_to_get_agent_with_read(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-read", "ollama").await;
            ctx.create_api_key_secret("share-read", provider.id).await;
            let model = ctx.create_model("share-read", provider.id).await;
            let agent = ctx
                .create_org_agent("share-read", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ to recipient in org_2
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Recipient accesses from org_2 (cross-org, but has share)
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "recipient_user",
                Some("org_2"),
            );

            let result = service.get_agent(request).await;
            assert!(
                result.is_ok(),
                "shared user should be able to read agent: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_unshared_user_from_getting_agent(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-deny-get", "ollama").await;
            ctx.create_api_key_secret("share-deny-get", provider.id).await;
            let model = ctx.create_model("share-deny-get", provider.id).await;
            let agent = ctx
                .create_org_agent("share-deny-get", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // No share exists — user from org_2 tries to get
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "random_user",
                Some("org_2"),
            );

            let result = service.get_agent(request).await;
            assert!(result.is_err(), "unshared user should not access agent");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_shared_user_to_update_agent_with_write(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-write", "ollama").await;
            ctx.create_api_key_secret("share-write", provider.id).await;
            let model = ctx.create_model("share-write", provider.id).await;
            let agent = ctx
                .create_org_agent("share-write", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "writer_user", "org_1").await;

            // Share with WRITE
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "writer_user".to_string(),
                    permissions: 2, // WRITE
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Writer updates the agent from org_2
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Updated by shared user".to_string()),
                    ..Default::default()
                },
                "writer_user",
                Some("org_2"),
            );

            let result = service.update_agent(request).await;
            assert!(result.is_ok(), "shared writer should update agent: {:?}", result.err());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_update_when_only_read_shared(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-read-no-write", "ollama").await;
            ctx.create_api_key_secret("share-read-no-write", provider.id).await;
            let model = ctx.create_model("share-read-no-write", provider.id).await;
            let agent = ctx
                .create_org_agent("share-read-no-write", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "reader_user", "org_1").await;

            // Share with READ only
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "reader_user".to_string(),
                    permissions: 1, // READ only
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Reader tries to update from org_2
            let request = authenticated_request(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Should not work".to_string()),
                    ..Default::default()
                },
                "reader_user",
                Some("org_2"),
            );

            let result = service.update_agent(request).await;
            assert!(result.is_err(), "read-only user should not be able to update");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_revoke_access_after_unshare(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-revoke", "ollama").await;
            ctx.create_api_key_secret("share-revoke", provider.id).await;
            let model = ctx.create_model("share-revoke", provider.id).await;
            let agent = ctx
                .create_org_agent("share-revoke", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Verify access works
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "recipient_user",
                Some("org_2"),
            );
            assert!(
                service.get_agent(request).await.is_ok(),
                "should have access before unshare"
            );

            // Unshare
            let request = authenticated_request(
                v1::UnshareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                },
                "owner_user",
                Some("org_1"),
            );
            service.unshare_agent(request).await.unwrap();

            // Verify access revoked
            let request = authenticated_request(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "recipient_user",
                Some("org_2"),
            );
            let result = service.get_agent(request).await;
            assert!(result.is_err(), "access should be revoked after unshare");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_include_shared_agent_in_listing(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-list", "ollama").await;
            ctx.create_api_key_secret("share-list", provider.id).await;
            let model = ctx.create_model("share-list", provider.id).await;

            // Owner creates agent in org_1
            let agent = ctx
                .create_org_agent("share-list", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share with READ
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Recipient lists agents in org_1 — shared agent should appear
            let request = authenticated_request(
                v1::ListAgentsRequest {
                    page: 1,
                    page_size: 50,
                    ..Default::default()
                },
                "recipient_user",
                Some("org_1"),
            );

            let response = service.list_agents(request).await.unwrap().into_inner();
            let agent_pids: Vec<_> = response
                .agents
                .iter()
                .filter_map(|a| match &a.identifier {
                    Some(proto_rs::rig::v1::agent::Identifier::Pid(pid)) => Some(pid.as_str()),
                    _ => None,
                })
                .collect();
            assert!(
                agent_pids.contains(&agent.pid.as_str()),
                "shared agent should appear in recipient's listing, got: {:?}",
                agent_pids
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_preserve_shares_after_soft_delete(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("share-softdel", "ollama").await;
            ctx.create_api_key_secret("share-softdel", provider.id).await;
            let model = ctx.create_model("share-softdel", provider.id).await;
            let agent = ctx
                .create_org_agent("share-softdel", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "recipient_user", "org_1").await;

            // Share
            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "recipient_user".to_string(),
                    permissions: 7,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            // Soft-delete the agent
            let request = authenticated_request(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "owner_user",
                Some("org_1"),
            );
            service.delete_agent(request).await.unwrap();

            // Share records should still exist in the DB
            let shares = agent_users::Entity::find()
                .filter(agent_users::Column::AgentId.eq(agent.id))
                .all(ctx.db.as_ref())
                .await
                .unwrap();
            assert_eq!(shares.len(), 1, "share records should survive soft-delete");
            assert_eq!(shares[0].user_id, "recipient_user");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_paginate_shares_with_cursor(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let provider = ctx.create_system_provider("share-paginate", "ollama").await;
            ctx.create_api_key_secret("share-paginate", provider.id).await;
            let model = ctx.create_model("share-paginate", provider.id).await;
            let agent = ctx
                .create_org_agent("share-paginate", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // Share with 3 users
            for i in 1..=3 {
                let user_id = format!("page_user_{i}");
                seed_recipient_cache(ctx, &user_id, "org_1").await;
                let request = authenticated_request(
                    v1::ShareAgentRequest {
                        agent_pid: agent.pid.clone(),
                        user_id,
                        permissions: 1,
                    },
                    "owner_user",
                    Some("org_1"),
                );
                service.share_agent(request).await.unwrap();
            }

            // Page 1: fetch 2 of 3
            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 2,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );
            let page1 = service.list_agent_shares(request).await.unwrap().into_inner();
            assert_eq!(page1.shares.len(), 2);
            assert_eq!(page1.total, 3);
            assert!(page1.next_cursor.is_some(), "should have cursor for next page");

            // Page 2: use cursor
            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 2,
                    cursor: page1.next_cursor,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );
            let page2 = service.list_agent_shares(request).await.unwrap().into_inner();
            assert_eq!(page2.shares.len(), 1, "last page should have remaining record");
            assert_eq!(page2.total, 3);
            assert!(page2.next_cursor.is_none(), "no cursor on last page");

            // No overlap
            let page1_ids: Vec<&str> = page1.shares.iter().map(|s| s.user_id.as_str()).collect();
            let page2_ids: Vec<&str> = page2.shares.iter().map(|s| s.user_id.as_str()).collect();
            for id in &page2_ids {
                assert!(!page1_ids.contains(id), "pages should not overlap");
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_no_cursor_when_shares_fit_in_page(ctx: &mut TestContext) {
            init_test_crypto();
            let tokens = ctx.zitadel.get_project_token_pair().await.unwrap();
            let provider = ctx.create_system_provider("share-no-cursor", "ollama").await;
            ctx.create_api_key_secret("share-no-cursor", provider.id).await;
            let model = ctx.create_model("share-no-cursor", provider.id).await;
            let agent = ctx
                .create_org_agent("share-no-cursor", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);
            seed_recipient_cache(ctx, "single_user", "org_1").await;

            let request = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "single_user".to_string(),
                    permissions: 1,
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(request).await.unwrap();

            let request = authenticated_request_with_token(
                v1::ListAgentSharesRequest {
                    agent_pid: agent.pid.clone(),
                    page_size: 10,
                    cursor: None,
                },
                "owner_user",
                Some("org_1"),
                Some(&tokens),
            );
            let response = service.list_agent_shares(request).await.unwrap().into_inner();
            assert_eq!(response.shares.len(), 1);
            assert_eq!(response.total, 1);
            assert!(response.next_cursor.is_none(), "no cursor when all results fit");
        }
    }

    mod permission_matrix {
        use super::*;
        use crate::consts::REDIS_USER_CACHE_KEY;
        use platform_rs::cache::{
            CachedOrg, CachedUserData, OrgRole, PlanTier, ResolvedPermissions, ResourcePermission, ResourceType,
            UserCache,
        };
        use platform_rs::middleware::organization::OrgContext;
        use std::collections::HashMap;

        /// Creates a request with OrgContext containing specific permissions.
        fn request_with_permissions<T>(
            inner: T,
            user_id: &str,
            org_pid: &str,
            permissions: ResolvedPermissions,
        ) -> tonic::Request<T> {
            let mut request = authenticated_request(inner, user_id, Some(org_pid));
            request.extensions_mut().insert(OrgContext {
                pid: org_pid.to_string(),
                role: OrgRole::Guest, // role doesn't matter — permissions are pre-resolved
                tier: PlanTier::Free,
                permissions,
            });
            request
        }

        /// Build permissions where a single resource has a specific permission.
        fn perms_with(resource: ResourceType, perm: ResourcePermission) -> ResolvedPermissions {
            let mut map = HashMap::new();
            map.insert(resource, perm);
            ResolvedPermissions::new(map)
        }

        /// Build permissions granting nothing.
        fn no_perms() -> ResolvedPermissions {
            ResolvedPermissions::default()
        }

        // -- get_agent --

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_get_agent_when_role_denies_read(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-get-deny", "ollama").await;
            ctx.create_api_key_secret("perm-get-deny", provider.id).await;
            let model = ctx.create_model("perm-get-deny", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-get-deny", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            let request = request_with_permissions(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "restricted_user",
                "org_1",
                no_perms(), // no read on agents
            );

            let result = service.get_agent(request).await;
            assert!(result.is_err(), "should deny when role has no agent read permission");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_get_agent_when_role_grants_read(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-get-allow", "ollama").await;
            ctx.create_api_key_secret("perm-get-allow", provider.id).await;
            let model = ctx.create_model("perm-get-allow", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-get-allow", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            let request = request_with_permissions(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "member_user",
                "org_1",
                perms_with(ResourceType::Agents, ResourcePermission::READ),
            );

            let result = service.get_agent(request).await;
            assert!(
                result.is_ok(),
                "should allow when role grants agent read: {:?}",
                result.err()
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_get_agent_via_share_when_role_denies_read(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-get-share", "ollama").await;
            ctx.create_api_key_secret("perm-get-share", provider.id).await;
            let model = ctx.create_model("perm-get-share", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-get-share", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // Seed recipient cache and share
            let cache = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
            cache
                .set(
                    "guest_user",
                    &CachedUserData {
                        email: "guest@test.com".into(),
                        organizations: vec![CachedOrg::new("org_1", PlanTier::Free, OrgRole::Guest)],
                    },
                )
                .await;

            // Owner shares with READ
            let share_req = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "guest_user".to_string(),
                    permissions: 1, // READ
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(share_req).await.unwrap();

            // Guest with no type-level agent perms tries to get — should succeed via share
            let request = request_with_permissions(
                v1::GetAgentRequest { pid: agent.pid.clone() },
                "guest_user",
                "org_1",
                no_perms(),
            );

            let result = service.get_agent(request).await;
            assert!(
                result.is_ok(),
                "share should bypass type-level denial: {:?}",
                result.err()
            );
        }

        // -- create_agent --

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_create_agent_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let service = create_service(ctx);

            let request = request_with_permissions(
                v1::CreateAgentRequest {
                    name: "Test".to_string(),
                    model_pid: "model_123".to_string(),
                    system_prompt: "Hello".to_string(),
                    ..Default::default()
                },
                "restricted_user",
                "org_1",
                perms_with(ResourceType::Agents, ResourcePermission::READ), // read only, no write
            );

            let result = service.create_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }

        // -- update_agent --

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_update_agent_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-update-deny", "ollama").await;
            ctx.create_api_key_secret("perm-update-deny", provider.id).await;
            let model = ctx.create_model("perm-update-deny", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-update-deny", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            let request = request_with_permissions(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Updated".to_string()),
                    ..Default::default()
                },
                "restricted_user",
                "org_1",
                perms_with(ResourceType::Agents, ResourcePermission::READ), // read only
            );

            let result = service.update_agent(request).await;
            assert!(result.is_err(), "should deny update when role has no write");
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_allow_update_agent_via_share_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-update-share", "ollama").await;
            ctx.create_api_key_secret("perm-update-share", provider.id).await;
            let model = ctx.create_model("perm-update-share", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-update-share", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            // Seed cache and share with WRITE
            let cache = UserCache::new(ctx.redis.clone(), REDIS_USER_CACHE_KEY);
            cache
                .set(
                    "writer_user",
                    &CachedUserData {
                        email: "writer@test.com".into(),
                        organizations: vec![CachedOrg::new("org_1", PlanTier::Free, OrgRole::Guest)],
                    },
                )
                .await;

            let share_req = authenticated_request(
                v1::ShareAgentRequest {
                    agent_pid: agent.pid.clone(),
                    user_id: "writer_user".to_string(),
                    permissions: 2, // WRITE
                },
                "owner_user",
                Some("org_1"),
            );
            service.share_agent(share_req).await.unwrap();

            // User with no type-level write but WRITE share → allowed
            let request = request_with_permissions(
                v1::UpdateAgentRequest {
                    pid: agent.pid.clone(),
                    name: Some("Updated via share".to_string()),
                    ..Default::default()
                },
                "writer_user",
                "org_1",
                no_perms(),
            );

            let result = service.update_agent(request).await;
            assert!(
                result.is_ok(),
                "write share should bypass type-level denial: {:?}",
                result.err()
            );
        }

        // -- delete_agent --

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deny_delete_agent_when_role_denies_write(ctx: &mut TestContext) {
            init_test_crypto();
            let provider = ctx.create_system_provider("perm-delete-deny", "ollama").await;
            ctx.create_api_key_secret("perm-delete-deny", provider.id).await;
            let model = ctx.create_model("perm-delete-deny", provider.id).await;
            let agent = ctx
                .create_org_agent("perm-delete-deny", model.id, "org_1", "owner_user")
                .await;

            let service = create_service(ctx);

            let request = request_with_permissions(
                v1::DeleteAgentRequest { pid: agent.pid.clone() },
                "restricted_user",
                "org_1",
                perms_with(ResourceType::Agents, ResourcePermission::READ),
            );

            let result = service.delete_agent(request).await;
            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::PermissionDenied);
        }
    }
}
