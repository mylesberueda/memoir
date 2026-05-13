use crate::{
    AppContext,
    api::embedding::{DefaultEmbedding, EmbeddingModel},
    services::AgentOps,
};
use proto_rs::rig::v1;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(crate) const CREATE_AGENT_TOOL_NAME: &str = "create_agent";

#[derive(Debug)]
pub(crate) struct CreateAgentTool<EM = DefaultEmbedding>
where
    EM: EmbeddingModel,
{
    ctx: Arc<AppContext<EM>>,
    user_id: String,
    organization_pid: Option<String>,
}

impl<EM> CreateAgentTool<EM>
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
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub(crate) struct CreateAgentToolArgs {
    /// Display name for the agent (e.g., 'Code Reviewer', 'Research Assistant')
    name: String,
    /// The system prompt that defines the agent's behavior, personality, and capabilities
    system_prompt: String,
    /// Model PID to use. Use db_query with query_type='models' to list available models and their PIDs.
    model_pid: String,
    /// Creativity level 0-100 (0=deterministic, 100=creative). Default: 70
    temperature: Option<i32>,
    /// List of tool PIDs to attach. Use db_query with query_type='tools' to list available tools and their PIDs.
    tool_pids: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct CreateAgentResult {
    /// The created agent's public ID
    pid: String,
    /// The agent's name
    name: String,
    /// The model PID used
    model_pid: String,
    /// Number of tools attached
    tools_attached: usize,
}

impl<EM> Tool for CreateAgentTool<EM>
where
    EM: EmbeddingModel,
{
    const NAME: &'static str = CREATE_AGENT_TOOL_NAME;
    type Error = CreateAgentError;
    type Args = CreateAgentToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Create a new AI agent for the user. IMPORTANT: You must first use \
                db_query with query_type='models' to get the model PIDs, and optionally \
                query_type='tools' to get tool PIDs. Do not use model names like 'gpt-4' directly - \
                use the PID values returned by db_query."
                .to_string(),
            parameters: serde_json::to_value(schema_for!(CreateAgentToolArgs))
                .expect("schema serialization should not fail"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        tracing::debug!(
            name = %args.name,
            model_pid = %args.model_pid,
            tool_count = args.tool_pids.as_ref().map(|t| t.len()).unwrap_or(0),
            "starting create_agent"
        );

        if args.name.trim().is_empty() {
            tracing::trace!("rejecting empty name");
            return Err(CreateAgentError::InvalidArgs("name cannot be empty".to_string()));
        }
        if args.system_prompt.trim().is_empty() {
            tracing::trace!("rejecting empty system_prompt");
            return Err(CreateAgentError::InvalidArgs(
                "system_prompt cannot be empty".to_string(),
            ));
        }
        if args.model_pid.trim().is_empty() {
            tracing::trace!("rejecting empty model_pid");
            return Err(CreateAgentError::InvalidArgs(
                "model_pid is required. Use db_query with query_type='models' to list available models.".to_string(),
            ));
        }

        let request = v1::CreateAgentRequest {
            name: args.name,
            model_pid: args.model_pid,
            temperature: args.temperature.unwrap_or(70).clamp(0, 100),
            system_prompt: args.system_prompt,
            config: None,
            provider_pid: None,
            tool_pids: args.tool_pids.unwrap_or_default(),
            kind: proto_rs::rig::v1::AgentKind::Startup.into(),
        };

        let org_pid = self.organization_pid.as_deref().ok_or_else(|| {
            CreateAgentError::ServiceError("Organization context required to create an agent".to_string())
        })?;

        // TODO: refactor AgentOps::create to accept &DatabaseConnection
        let res = AgentOps::create(Arc::new(self.ctx.db.clone()), &self.user_id, org_pid, request)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "AgentOps::create failed");
                CreateAgentError::ServiceError(e.to_string())
            })?;

        tracing::debug!(
            agent_pid = %res.agent.pid,
            tools_attached = res.tools.len(),
            "create_agent complete"
        );

        let result = CreateAgentResult {
            pid: res.agent.pid,
            name: res.agent.name,
            model_pid: res.model.pid,
            tools_attached: res.tools.len(),
        };

        serde_json::to_string(&result).map_err(|e| CreateAgentError::Serialization(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CreateAgentError {
    #[error("invalid arguments: {0}")]
    InvalidArgs(String),
    #[error("service error: {0}")]
    ServiceError(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::{
        api::embedding::{BoxFuture, EmbeddingError},
        clients::{QdrantClient, StorageClient},
        models::agents,
        test_utils::{TestContext, init_test_crypto},
    };
    use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};
    use serial_test::serial;
    use test_context::test_context;

    struct MockEmbeddingModel;

    impl EmbeddingModel for MockEmbeddingModel {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Ok(vec![0.0; 384]) })
        }
        fn dimensions(&self) -> usize {
            384
        }
    }

    fn mock_app_ctx(ctx: &TestContext) -> Arc<AppContext<MockEmbeddingModel>> {
        let qdrant_url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".into());
        let qdrant_inner = qdrant_client::Qdrant::from_url(&qdrant_url)
            .build()
            .expect("test qdrant client");

        let s3_config = aws_sdk_s3::Config::builder()
            .endpoint_url("http://localhost:9000")
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new("test", "test", None, None, "test"))
            .force_path_style(true)
            .behavior_version_latest()
            .build();

        Arc::new(AppContext {
            db: (*ctx.db).clone(),
            redis: ctx.redis.clone(),
            qdrant: QdrantClient::new(qdrant_inner),
            embedding: Arc::new(MockEmbeddingModel),
            storage: StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into()),
            api_service: crate::clients::ApiServiceClient::new(
                &std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set"),
            )
            .unwrap(),
        })
    }

    /// Helper to create args with model PID
    fn args_with_model_pid(name: &str, system_prompt: &str, model_pid: &str) -> CreateAgentToolArgs {
        CreateAgentToolArgs {
            name: name.to_string(),
            system_prompt: system_prompt.to_string(),
            model_pid: model_pid.to_string(),
            temperature: None,
            tool_pids: None,
        }
    }

    /// Helper to track created agent for cleanup
    async fn track_agent_for_cleanup(ctx: &mut TestContext, pid: &str) {
        if let Some(agent) = agents::Entity::find()
            .filter(agents::Column::Pid.eq(pid))
            .one(ctx.db.as_ref())
            .await
            .unwrap()
        {
            ctx.created_agents.push(agent.id);
        }
    }

    mod call {
        use super::*;

        mod validation {
            use super::*;

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_reject_empty_name(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool.call(args_with_model_pid("", "You are helpful.", &model.pid)).await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::InvalidArgs(msg) => {
                        assert!(msg.contains("name"), "error should mention name: {msg}");
                    }
                    other => panic!("expected InvalidArgs, got {:?}", other),
                }
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_reject_whitespace_only_name(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("   ", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::InvalidArgs(msg) => {
                        assert!(msg.contains("name"), "error should mention name: {msg}");
                    }
                    other => panic!("expected InvalidArgs, got {:?}", other),
                }
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_reject_empty_system_prompt(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool.call(args_with_model_pid("My Agent", "", &model.pid)).await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::InvalidArgs(msg) => {
                        assert!(
                            msg.contains("system_prompt"),
                            "error should mention system_prompt: {msg}"
                        );
                    }
                    other => panic!("expected InvalidArgs, got {:?}", other),
                }
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_reject_whitespace_only_system_prompt(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("My Agent", "   \n\t  ", &model.pid))
                    .await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::InvalidArgs(msg) => {
                        assert!(
                            msg.contains("system_prompt"),
                            "error should mention system_prompt: {msg}"
                        );
                    }
                    other => panic!("expected InvalidArgs, got {:?}", other),
                }
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_reject_empty_model_pid(ctx: &mut TestContext) {
                init_test_crypto();
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool.call(args_with_model_pid("My Agent", "You are helpful.", "")).await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::InvalidArgs(msg) => {
                        assert!(msg.contains("model_pid"), "error should mention model_pid: {msg}");
                    }
                    other => panic!("expected InvalidArgs, got {:?}", other),
                }
            }
        }

        mod model_resolution {
            use super::*;

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_find_model_by_pid(ctx: &mut TestContext) {
                init_test_crypto();
                let provider = ctx.create_system_provider("create-agent-model", "ollama").await;
                let model = ctx.create_model("create-agent-model", provider.id).await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("Test Agent", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should find model: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                assert_eq!(output["model_pid"], model.pid);
                track_agent_for_cleanup(ctx, output["pid"].as_str().unwrap()).await;
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_return_error_for_missing_model_pid(ctx: &mut TestContext) {
                init_test_crypto();
                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid(
                        "Test Agent",
                        "You are helpful.",
                        "nonexistent-model-pid",
                    ))
                    .await;

                assert!(result.is_err());
                match result.unwrap_err() {
                    CreateAgentError::ServiceError(msg) => {
                        assert!(
                            msg.contains("not found") || msg.contains("NotFound"),
                            "error should indicate model not found: {msg}"
                        );
                    }
                    other => panic!("expected ServiceError, got {:?}", other),
                }
            }
        }

        mod agent_creation {
            use super::*;

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_create_agent_with_all_fields(ctx: &mut TestContext) {
                init_test_crypto();
                let provider = ctx.create_system_provider("create-agent-full", "ollama").await;
                let model = ctx.create_model("create-agent-full", provider.id).await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let create_args = CreateAgentToolArgs {
                    name: "Full Agent".to_string(),
                    system_prompt: "You are a complete agent.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: Some(50),
                    tool_pids: None,
                };

                let result = tool.call(create_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                // Verify agent was created with correct fields
                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert_eq!(agent.name, "Full Agent");
                assert_eq!(agent.system_prompt, Some("You are a complete agent.".to_string()));
                assert!((agent.temperature - 0.5).abs() < 0.01, "temperature should be 0.5");

                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_set_organization_pid_from_context(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_456".to_string()));

                let result = tool
                    .call(args_with_model_pid("Org Agent", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert_eq!(agent.organization_pid, "org_456");
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_set_created_by_to_user_id(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(
                    mock_app_ctx(ctx),
                    "specific_user_id".into(),
                    Some("org_test".to_string()),
                );

                let result = tool
                    .call(args_with_model_pid("Owner Agent", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert_eq!(agent.created_by, "specific_user_id");
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_generate_slug_from_name(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("My Awesome Agent!", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert_eq!(agent.slug, "my-awesome-agent");
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_set_kind_to_startup(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("Kind Agent", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                // AgentOps::create sets kind to "startup" (AgentKind::Startup)
                assert_eq!(agent.kind, "startup");
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_use_default_temperature_when_not_provided(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid(
                        "Temp Default Agent",
                        "You are helpful.",
                        &model.pid,
                    ))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                // Default is 70, scaled to 0.7
                assert!(
                    (agent.temperature - 0.7).abs() < 0.01,
                    "temperature should default to 0.7, got {}",
                    agent.temperature
                );
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_clamp_temperature_to_valid_range(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                // Test with value above 100
                let high_args = CreateAgentToolArgs {
                    name: "High Temp Agent".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: Some(150),
                    tool_pids: None,
                };

                let result = tool.call(high_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert!(
                    (agent.temperature - 1.0).abs() < 0.01,
                    "temperature above 100 should clamp to 1.0, got {}",
                    agent.temperature
                );
                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_clamp_negative_temperature_to_zero(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let neg_args = CreateAgentToolArgs {
                    name: "Negative Temp Agent".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: Some(-50),
                    tool_pids: None,
                };

                let result = tool.call(neg_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                assert!(
                    agent.temperature.abs() < 0.01,
                    "negative temperature should clamp to 0.0, got {}",
                    agent.temperature
                );
                ctx.created_agents.push(agent.id);
            }
        }

        mod tool_attachment {
            use super::*;
            use crate::models::agent_tools;

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_attach_tools_by_pid(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                // Create some tools
                let tool1 = ctx.create_system_tool("attach-pid-1", "test_tool_1").await;
                let tool2 = ctx.create_system_tool("attach-pid-2", "test_tool_2").await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let create_args = CreateAgentToolArgs {
                    name: "Agent With Tools".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: None,
                    tool_pids: Some(vec![tool1.pid.clone(), tool2.pid.clone()]),
                };

                let result = tool.call(create_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                let pid = output["pid"].as_str().unwrap();

                let agent = agents::Entity::find()
                    .filter(agents::Column::Pid.eq(pid))
                    .one(ctx.db.as_ref())
                    .await
                    .unwrap()
                    .expect("agent should exist");

                // Verify junction records
                let links = agent_tools::Entity::find()
                    .filter(agent_tools::Column::AgentId.eq(agent.id))
                    .all(ctx.db.as_ref())
                    .await
                    .unwrap();

                assert_eq!(links.len(), 2, "should have 2 tool links");
                let tool_ids: Vec<i64> = links.iter().map(|l| l.tool_id).collect();
                assert!(tool_ids.contains(&tool1.id));
                assert!(tool_ids.contains(&tool2.id));

                ctx.created_agents.push(agent.id);
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_count_attached_tools_correctly(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool1 = ctx.create_system_tool("count-pid-1", "count_tool_1").await;
                let tool2 = ctx.create_system_tool("count-pid-2", "count_tool_2").await;
                let tool3 = ctx.create_system_tool("count-pid-3", "count_tool_3").await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let create_args = CreateAgentToolArgs {
                    name: "Agent Count Tools".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: None,
                    tool_pids: Some(vec![tool1.pid.clone(), tool2.pid.clone(), tool3.pid.clone()]),
                };

                let result = tool.call(create_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                assert_eq!(output["tools_attached"], 3);

                track_agent_for_cleanup(ctx, output["pid"].as_str().unwrap()).await;
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_skip_nonexistent_tool_pids(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let existing_tool = ctx.create_system_tool("skip-pid-1", "existing_tool").await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let create_args = CreateAgentToolArgs {
                    name: "Agent Skip Tools".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: None,
                    tool_pids: Some(vec![existing_tool.pid.clone(), "nonexistent-tool-pid".to_string()]),
                };

                let result = tool.call(create_args).await;
                assert!(
                    result.is_ok(),
                    "should succeed even with missing tools: {:?}",
                    result.err()
                );

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                assert_eq!(output["tools_attached"], 1, "should only attach existing tool");

                track_agent_for_cleanup(ctx, output["pid"].as_str().unwrap()).await;
            }

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_skip_inactive_tools(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let active_tool = ctx.create_system_tool("active-pid-1", "active_tool").await;
                let inactive_tool = ctx.create_inactive_tool("inactive-pid-1", "inactive_tool").await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let create_args = CreateAgentToolArgs {
                    name: "Agent Inactive Tools".to_string(),
                    system_prompt: "You are helpful.".to_string(),
                    model_pid: model.pid.clone(),
                    temperature: None,
                    tool_pids: Some(vec![active_tool.pid.clone(), inactive_tool.pid.clone()]),
                };

                let result = tool.call(create_args).await;
                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
                assert_eq!(output["tools_attached"], 1, "should only attach active tool");

                track_agent_for_cleanup(ctx, output["pid"].as_str().unwrap()).await;
            }
        }

        mod output {
            use super::*;

            #[test_context(TestContext)]
            #[tokio::test]
            #[serial]
            async fn should_return_json_with_pid_name_model_pid_and_tools_count(ctx: &mut TestContext) {
                init_test_crypto();
                let (_provider, model) = ctx.get_system_provider_and_model().await;

                let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));

                let result = tool
                    .call(args_with_model_pid("Output Agent", "You are helpful.", &model.pid))
                    .await;

                assert!(result.is_ok(), "should succeed: {:?}", result.err());

                let output: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

                assert!(output["pid"].is_string(), "should have pid");
                assert!(!output["pid"].as_str().unwrap().is_empty(), "pid should not be empty");
                assert_eq!(output["name"], "Output Agent");
                assert_eq!(output["model_pid"], model.pid);
                assert!(output["tools_attached"].is_number(), "should have tools_attached count");

                track_agent_for_cleanup(ctx, output["pid"].as_str().unwrap()).await;
            }
        }
    }

    mod definition {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_have_name_create_agent(ctx: &mut TestContext) {
            init_test_crypto();
            assert_eq!(CREATE_AGENT_TOOL_NAME, "create_agent");

            let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));
            let definition = tool.definition("".to_string()).await;
            assert_eq!(definition.name, "create_agent");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_require_name_system_prompt_and_model_pid_in_definition(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));
            let definition = tool.definition("".to_string()).await;

            let required = definition.parameters.get("required").unwrap();
            assert!(required.is_array(), "required should be an array");

            let required_arr = required.as_array().unwrap();
            assert!(required_arr.iter().any(|v| v == "name"), "name should be required");
            assert!(
                required_arr.iter().any(|v| v == "system_prompt"),
                "system_prompt should be required"
            );
            assert!(
                required_arr.iter().any(|v| v == "model_pid"),
                "model_pid should be required"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_have_description_suggesting_db_query(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));
            let definition = tool.definition("".to_string()).await;

            assert!(
                definition.description.contains("db_query"),
                "description should mention db_query: {}",
                definition.description
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_define_optional_parameters(ctx: &mut TestContext) {
            init_test_crypto();
            let tool = CreateAgentTool::new(mock_app_ctx(ctx), "user_test".into(), Some("org_test".to_string()));
            let definition = tool.definition("".to_string()).await;

            let properties = definition.parameters.get("properties").unwrap();

            assert!(properties.get("temperature").is_some(), "should have temperature");
            assert!(properties.get("tool_pids").is_some(), "should have tool_pids");
        }
    }
}
