use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    models::{agents, conversations, language_models, messages, providers, tools},
};
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};
use sea_orm::{ColumnTrait as _, Condition, EntityTrait as _, QueryFilter as _, QueryOrder as _, QuerySelect as _};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Maximum number of results to return from a query
const DEFAULT_LIMIT: u64 = 100;

/// Tool name constant — accessible without resolving the generic parameter.
pub(crate) const DB_QUERY_TOOL_NAME: &str = "db_query";

/// Tool that allows agents to query conversation and message data
/// scoped to the current user and organization context.
#[derive(Debug)]
pub(crate) struct DbQueryTool<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
    user_id: String,
    organization_pid: Option<String>,
}

impl<EM> DbQueryTool<EM>
where
    EM: EmbeddingModel,
{
    pub(crate) fn new(ctx: Arc<AppContext<EM>>, user_id: String, organization_pid: Option<String>) -> Self {
        Self {
            ctx,
            user_id,
            organization_pid,
        }
    }

    /// Build the condition for filtering by organization context.
    /// - Personal context (None): agent.organization_pid IS NULL
    /// - Org context (Some): agent.organization_pid = $org_pid
    fn org_condition(&self) -> Condition {
        match &self.organization_pid {
            Some(org_pid) => Condition::all().add(agents::Column::OrganizationPid.eq(org_pid)),
            None => Condition::all().add(agents::Column::OrganizationPid.is_null()),
        }
    }

    async fn query_conversations(&self, limit: u64) -> Result<QueryResult, DbQueryError> {
        let results: Vec<(conversations::Model, Option<agents::Model>)> = conversations::Entity::find()
            .filter(conversations::Column::UserId.eq(&self.user_id))
            .filter(conversations::Column::IsDeleted.eq(false))
            .find_also_related(agents::Entity)
            .filter(self.org_condition())
            .order_by_desc(conversations::Column::LastMessageAt)
            .limit(limit)
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let items: Vec<ConversationSummary> = results
            .into_iter()
            .map(|(conv, agent)| ConversationSummary {
                pid: conv.pid,
                title: conv.title,
                agent_name: agent.map(|a| a.name),
                message_count: conv.message_count,
                last_message_at: conv.last_message_at.map(|dt| dt.to_string()),
                created_at: conv.created_at.to_string(),
            })
            .collect();

        let count = items.len();
        Ok(QueryResult::Conversations { items, count })
    }

    async fn query_messages(&self, conversation_pid: &str, limit: u64) -> Result<QueryResult, DbQueryError> {
        // First verify the conversation belongs to this user in this context
        let result: Option<(conversations::Model, Option<agents::Model>)> = conversations::Entity::find()
            .filter(conversations::Column::Pid.eq(conversation_pid))
            .filter(conversations::Column::UserId.eq(&self.user_id))
            .filter(conversations::Column::IsDeleted.eq(false))
            .find_also_related(agents::Entity)
            .filter(self.org_condition())
            .one(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let (conversation, _agent) =
            result.ok_or_else(|| DbQueryError::NotFound("conversation not found".to_string()))?;

        let msgs: Vec<messages::Model> = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(conversation.id))
            .filter(messages::Column::IsDeleted.eq(false))
            .order_by_desc(messages::Column::CreatedAt)
            .limit(limit)
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let items: Vec<MessageSummary> = msgs
            .into_iter()
            .rev() // Return in chronological order
            .map(|msg| MessageSummary {
                pid: msg.pid,
                role: msg.role,
                content: msg.content,
                created_at: msg.created_at.to_string(),
            })
            .collect();

        let count = items.len();
        Ok(QueryResult::Messages { items, count })
    }

    async fn query_stats(&self) -> Result<QueryResult, DbQueryError> {
        // Fetch conversations in this context to count and sum message counts
        let results: Vec<(conversations::Model, Option<agents::Model>)> = conversations::Entity::find()
            .filter(conversations::Column::UserId.eq(&self.user_id))
            .filter(conversations::Column::IsDeleted.eq(false))
            .find_also_related(agents::Entity)
            .filter(self.org_condition())
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let conversation_count = results.len() as i64;
        let message_count: i64 = results.iter().map(|(c, _)| c.message_count as i64).sum();

        Ok(QueryResult::Stats {
            conversation_count,
            message_count,
        })
    }

    async fn query_agents(&self, limit: u64) -> Result<QueryResult, DbQueryError> {
        let results: Vec<agents::Model> = agents::Entity::find()
            .filter(self.org_condition())
            .filter(agents::Column::CreatedBy.eq(&self.user_id))
            .order_by_desc(agents::Column::CreatedAt)
            .limit(limit)
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let items: Vec<AgentSummary> = results
            .into_iter()
            .map(|agent| AgentSummary {
                pid: agent.pid,
                name: agent.name,
                description: agent.description,
                is_active: agent.is_active,
            })
            .collect();

        let count = items.len();
        Ok(QueryResult::Agents { items, count })
    }

    async fn query_models(&self, limit: u64) -> Result<QueryResult, DbQueryError> {
        // Query models from providers accessible in this context
        // For assistants: system providers (no owner, no org)
        // For regular users: personal/org providers + system providers
        let results: Vec<(language_models::Model, Option<providers::Model>)> = language_models::Entity::find()
            .find_also_related(providers::Entity)
            .filter(providers::Column::IsActive.eq(true))
            .filter(providers::Column::IsDeprecated.eq(false))
            .filter(language_models::Column::IsActive.eq(true))
            .filter(match &self.organization_pid {
                Some(org_pid) => {
                    // Org context: org providers + system providers
                    Condition::any()
                        .add(providers::Column::OrganizationPid.eq(org_pid))
                        .add(
                            Condition::all()
                                .add(providers::Column::CreatedBy.is_null())
                                .add(providers::Column::OrganizationPid.is_null()),
                        )
                }
                None => {
                    // Personal context: user's providers + system providers
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(providers::Column::CreatedBy.eq(&self.user_id))
                                .add(providers::Column::OrganizationPid.is_null()),
                        )
                        .add(
                            Condition::all()
                                .add(providers::Column::CreatedBy.is_null())
                                .add(providers::Column::OrganizationPid.is_null()),
                        )
                }
            })
            .limit(limit)
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let items: Vec<ModelSummary> = results
            .into_iter()
            .filter_map(|(model, provider)| {
                provider.map(|p| ModelSummary {
                    model_id: model.model_id,
                    name: model.name,
                    provider_name: p.name,
                    context_length: model.context_window,
                    is_active: model.is_active,
                })
            })
            .collect();

        let count = items.len();
        Ok(QueryResult::Models { items, count })
    }

    async fn query_tools(&self, limit: u64) -> Result<QueryResult, DbQueryError> {
        let results: Vec<tools::Model> = tools::Entity::find()
            .filter(tools::Column::IsActive.eq(true))
            .order_by_asc(tools::Column::DisplayName)
            .limit(limit)
            .all(&self.ctx.db)
            .await
            .map_err(DbQueryError::Database)?;

        let items: Vec<ToolSummary> = results
            .into_iter()
            .map(|tool| ToolSummary {
                name: tool.name,
                display_name: tool.display_name,
                description: tool.description,
                is_active: tool.is_active,
            })
            .collect();

        let count = items.len();
        Ok(QueryResult::Tools { items, count })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QueryType {
    /// List recent conversations
    Conversations,
    /// List messages from a specific conversation
    Messages,
    /// Get statistics (conversation count, message count)
    Stats,
    /// List agents in the current context
    Agents,
    /// List available models for creating agents
    Models,
    /// List available tools for attaching to agents
    Tools,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct DbQueryToolArgs {
    /// The type of query to perform: 'stats' for counts, 'conversations' for recent conversations,
    /// 'messages' for messages in a conversation, 'agents' to list agents, 'models' to list
    /// available models for creating agents, 'tools' to list tools that can be attached to agents
    query_type: QueryType,
    /// Required for 'messages' query: the conversation PID to fetch messages from
    conversation_pid: Option<String>,
    /// Maximum results to return (default: 100, max: 100)
    limit: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum QueryResult {
    Conversations {
        items: Vec<ConversationSummary>,
        count: usize,
    },
    Messages {
        items: Vec<MessageSummary>,
        count: usize,
    },
    Stats {
        conversation_count: i64,
        message_count: i64,
    },
    Agents {
        items: Vec<AgentSummary>,
        count: usize,
    },
    Models {
        items: Vec<ModelSummary>,
        count: usize,
    },
    Tools {
        items: Vec<ToolSummary>,
        count: usize,
    },
}

#[derive(Debug, Serialize)]
struct ConversationSummary {
    pid: String,
    title: Option<String>,
    agent_name: Option<String>,
    message_count: i32,
    last_message_at: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct MessageSummary {
    pid: String,
    role: String,
    content: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize)]
struct AgentSummary {
    pid: String,
    name: String,
    description: Option<String>,
    is_active: bool,
}

#[derive(Debug, Serialize)]
struct ModelSummary {
    /// The model_id to use when creating agents (e.g., "llama3.2:3b")
    model_id: String,
    name: String,
    provider_name: String,
    context_length: Option<i32>,
    is_active: bool,
}

#[derive(Debug, Serialize)]
struct ToolSummary {
    /// The tool name to use when attaching to agents
    name: String,
    display_name: String,
    description: String,
    is_active: bool,
}

impl<EM> Tool for DbQueryTool<EM>
where
    EM: EmbeddingModel,
{
    const NAME: &'static str = DB_QUERY_TOOL_NAME;
    type Error = DbQueryError;
    type Args = DbQueryToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Query your data including conversations, messages, agents, models, and tools. \
                Use 'models' and 'tools' before creating agents to see available options."
                .to_string(),
            parameters: serde_json::to_value(schema_for!(DbQueryToolArgs))
                .expect("schema serialization should not fail"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let limit = args.limit.unwrap_or(DEFAULT_LIMIT).min(DEFAULT_LIMIT);

        let result = match args.query_type {
            QueryType::Conversations => self.query_conversations(limit).await?,
            QueryType::Messages => {
                let conversation_pid = args.conversation_pid.ok_or_else(|| {
                    DbQueryError::InvalidArgs("conversation_pid is required for messages query".to_string())
                })?;
                self.query_messages(&conversation_pid, limit).await?
            }
            QueryType::Stats => self.query_stats().await?,
            QueryType::Agents => self.query_agents(limit).await?,
            QueryType::Models => self.query_models(limit).await?,
            QueryType::Tools => self.query_tools(limit).await?,
        };

        serde_json::to_string(&result).map_err(|e| DbQueryError::Serialization(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum DbQueryError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid arguments: {0}")]
    InvalidArgs(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}
