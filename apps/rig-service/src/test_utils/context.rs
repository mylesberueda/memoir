use super::MockEmbeddingModel;
use crate::{
    AppContext,
    api::{crypto, hooks::seed},
    clients::{ApiServiceClient, QdrantClient, StorageClient},
    models::{
        agent_tools, agents, conversation_documents, conversations, document_group_memberships, document_groups,
        documents, language_models, messages, provider_secrets, providers, secrets, tool_secrets, tools,
        user_assistants,
    },
};
use common_rs::crypto::LocalCrypto;
use fred::{clients::Client as RedisClient, interfaces::ClientLike, types::config::Config as RedisConfig};
use migration::MigratorTrait as _;
use platform_rs::test_utils::ZitadelTestClient;
use proto_rs::rig::v1::{MessagePart, MessagePartKind, MessagePartStatus};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait as _, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use test_context::AsyncTestContext;
use tokio::sync::OnceCell;

static SEEDED: OnceCell<()> = OnceCell::const_new();

/// Build a minimal AppContext for seed/test setup. Qdrant, storage, and embedding
/// are dummies — only the database connection is used during seeding.
fn seed_app_ctx(db: &Arc<DatabaseConnection>, redis: &Arc<RedisClient>) -> Arc<AppContext<MockEmbeddingModel>> {
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

    let api_service_url = std::env::var("API_SERVICE_URL").expect("API_SERVICE_URL must be set");
    let api_service = ApiServiceClient::new(&api_service_url).expect("test api-service client");

    Arc::new(AppContext {
        db: DatabaseConnection::clone(db),
        redis: redis.clone(),
        qdrant: QdrantClient::new(qdrant_inner),
        embedding: Arc::new(MockEmbeddingModel),
        storage: StorageClient::new(aws_sdk_s3::Client::from_conf(s3_config), "test".into()),
        api_service,
    })
}

/// Initialize test crypto with deterministic passphrase/salt.
/// Safe to call multiple times (crypto::init is idempotent).
pub fn init_test_crypto() {
    let crypto =
        LocalCrypto::from_passphrase("test-passphrase", "test-salt").expect("static test credentials are valid");
    crypto::init(crypto);
}

/// Test context for integration tests
///
/// Automatically sets up database connection and tracks created test data
/// for cleanup. Use with #[test_context(TestContext)] on test functions.
pub struct TestContext {
    pub db: Arc<DatabaseConnection>,
    pub redis: Arc<RedisClient>,
    pub zitadel: Arc<ZitadelTestClient>,
    pub created_providers: Vec<i64>,
    pub created_models: Vec<i64>,
    pub created_agents: Vec<i64>,
    pub created_conversations: Vec<i64>,
    pub created_messages: Vec<i64>,
    pub created_secrets: Vec<i64>,
    pub created_tools: Vec<i64>,
    pub created_documents: Vec<i64>,
    pub created_document_groups: Vec<i64>,
    pub created_zitadel_users: Vec<String>,
    test_prefix: String,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        let db = Arc::new(
            sea_orm::Database::connect(&db_url)
                .await
                .expect("Failed to connect to test database"),
        );

        // Run migrations (idempotent - only applies pending migrations)
        migration::Migrator::up(db.as_ref(), None)
            .await
            .expect("Failed to run migrations on test database");

        // Connect to Redis
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_config = RedisConfig::from_url(&redis_url).expect("Invalid REDIS_URL");
        let redis = RedisClient::new(redis_config, None, None, None);
        redis.init().await.expect("Failed to connect to Redis");
        let redis = Arc::new(redis);

        // Generate unique prefix for this test run to avoid collisions
        let test_prefix = format!(
            "test:{}:",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        SEEDED
            .get_or_init(|| async {
                init_test_crypto();
                seed::Seed::init(seed_app_ctx(&db, &redis))
                    .await
                    .expect("Failed to seed system providers");
            })
            .await;

        let zitadel = Arc::new(
            ZitadelTestClient::from_env()
                .await
                .expect("Failed to initialize Zitadel test client"),
        );

        Self {
            db,
            redis,
            zitadel,
            created_providers: Vec::new(),
            created_models: Vec::new(),
            created_agents: Vec::new(),
            created_conversations: Vec::new(),
            created_messages: Vec::new(),
            created_secrets: Vec::new(),
            created_tools: Vec::new(),
            created_documents: Vec::new(),
            created_document_groups: Vec::new(),
            created_zitadel_users: Vec::new(),
            test_prefix,
        }
    }

    async fn teardown(self) {
        // Clean up in reverse dependency order

        // 0. Zitadel users (external, clean up first)
        for user_id in &self.created_zitadel_users {
            let _ = self.zitadel.delete_user(user_id).await;
        }

        // 0b. Document group memberships cascade from group/document deletes,
        //    but delete documents before groups to avoid FK issues
        for id in &self.created_documents {
            let _ = documents::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }
        for id in &self.created_document_groups {
            let _ = document_groups::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 1. Messages (depends on conversations)
        for id in &self.created_messages {
            let _ = messages::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 2. Conversations (depends on agents)
        for id in &self.created_conversations {
            let _ = conversations::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 3. Tools (agent_tools and tool_secrets cascade delete)
        for id in &self.created_tools {
            let _ = tools::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 4. Secrets (depends on providers via provider_secrets, tools via tool_secrets)
        for id in &self.created_secrets {
            let _ = secrets::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 5. Agents (depends on models)
        for id in &self.created_agents {
            let _ = agents::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 6. Models (depends on providers)
        for id in &self.created_models {
            let _ = language_models::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }

        // 7. Providers
        for id in &self.created_providers {
            let _ = providers::Entity::delete_by_id(*id).exec(self.db.as_ref()).await;
        }
    }
}

impl TestContext {
    /// Build a minimal AppContext backed by this test's database, Redis, and dummy
    /// Qdrant/storage/embedding clients. Suitable for constructing tools and services
    /// that need an `Arc<AppContext<_>>`.
    pub fn app_ctx(&self) -> Arc<AppContext<MockEmbeddingModel>> {
        seed_app_ctx(&self.db, &self.redis)
    }

    /// Creates a test user in Zitadel and tracks it for cleanup on teardown.
    ///
    /// Returns the Zitadel user ID.
    pub async fn create_zitadel_user(&mut self, suffix: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let email = format!("test-{}-{}@example.com", suffix, timestamp);

        let user_id = self
            .zitadel
            .create_user(&email, "Test", suffix, "TestPassword123!")
            .await
            .expect("Failed to create Zitadel test user");

        self.created_zitadel_users.push(user_id.clone());
        user_id
    }

    /// Creates a test provider (system scope by default — accessible from any org)
    pub async fn create_provider(&mut self, suffix: &str, provider_type: &str) -> providers::Model {
        self.create_system_provider(suffix, provider_type).await
    }

    /// Creates a provider with explicit scope configuration
    ///
    /// - `organization_pid`: None for personal/system, Some(org_pid) for org scope
    /// - `created_by`: None for system provider, Some(user_id) for user-created
    pub async fn create_provider_with_scope(
        &mut self,
        suffix: &str,
        provider_type: &str,
        organization_pid: Option<&str>,
        created_by: Option<&str>,
    ) -> providers::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let base_url = std::env::var("OLLAMA_BASE_URL").expect("OLLAMA_BASE_URL should exist");

        let provider = providers::ActiveModel {
            pid: Set(format!("provider-{suffix}-{timestamp}")),
            organization_pid: Set(organization_pid.map(String::from)),
            created_by: Set(created_by.map(String::from)),
            name: Set(format!("Test Provider {suffix}")),
            provider_type: Set(provider_type.to_string()),
            base_url: Set(Some(base_url)),
            is_active: Set(true),
            is_deprecated: Set(false),
            ..Default::default()
        };

        let provider = provider
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test provider");

        self.created_providers.push(provider.id);
        provider
    }

    /// Creates a system provider (accessible from everywhere)
    pub async fn create_system_provider(&mut self, suffix: &str, provider_type: &str) -> providers::Model {
        self.create_provider_with_scope(suffix, provider_type, None, None).await
    }

    /// Creates a personal provider for a specific user
    pub async fn create_personal_provider(
        &mut self,
        suffix: &str,
        provider_type: &str,
        user_id: &str,
    ) -> providers::Model {
        self.create_provider_with_scope(suffix, provider_type, None, Some(user_id))
            .await
    }

    /// Creates an org provider
    pub async fn create_org_provider(
        &mut self,
        suffix: &str,
        provider_type: &str,
        org_pid: &str,
        created_by: &str,
    ) -> providers::Model {
        self.create_provider_with_scope(suffix, provider_type, Some(org_pid), Some(created_by))
            .await
    }

    /// Creates a test language model (Ollama)
    pub async fn create_model(&mut self, suffix: &str, provider_id: i64) -> language_models::Model {
        let model_id = std::env::var("OLLAMA_TEST_MODEL").expect("OLLAMA_TEST_MODEL should exist");
        self.create_model_with_id(suffix, provider_id, &model_id).await
    }

    /// Creates a test language model for OpenAI provider
    pub async fn create_openai_model(&mut self, suffix: &str, provider_id: i64) -> language_models::Model {
        // Use a valid OpenAI model ID - doesn't need to be real since we're testing auth failure
        self.create_model_with_id(suffix, provider_id, "gpt-4o-mini").await
    }

    /// Creates a test language model with a specific model_id
    pub async fn create_model_with_id(
        &mut self,
        suffix: &str,
        provider_id: i64,
        model_id: &str,
    ) -> language_models::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let model = language_models::ActiveModel {
            pid: Set(format!("model-{suffix}-{timestamp}")),
            provider_id: Set(provider_id),
            model_id: Set(model_id.to_string()),
            name: Set(format!("Test Model {suffix}")),
            is_active: Set(true),
            ..Default::default()
        };

        let model = model
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test model");

        self.created_models.push(model.id);
        model
    }

    /// Creates a test agent (personal scope for user_test by default)
    pub async fn create_agent(&mut self, suffix: &str, model_id: i64) -> agents::Model {
        self.create_agent_in_org(suffix, model_id, "org_test", "user_test")
            .await
    }

    /// Creates an agent in a specific org
    pub async fn create_agent_in_org(
        &mut self,
        suffix: &str,
        model_id: i64,
        organization_pid: &str,
        created_by: &str,
    ) -> agents::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let agent = agents::ActiveModel {
            pid: Set(format!("agent-{suffix}-{timestamp}")),
            organization_pid: Set(organization_pid.to_string()),
            created_by: Set(created_by.to_string()),
            name: Set(format!("Test Agent {suffix}")),
            slug: Set(format!("test-agent-{suffix}-{timestamp}")),
            kind: Set("startup".to_string()),
            model_id: Set(model_id),
            description: Set(None),
            temperature: Set(0.7),
            system_prompt: Set(Some("You are a helpful assistant.".to_string())),
            config: Set(serde_json::json!({"history_length": 10})),
            is_active: Set(true),
            ..Default::default()
        };

        let agent = agent
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test agent");

        self.created_agents.push(agent.id);
        agent
    }

    /// Creates an agent with a custom config JSON (personal scope for user_test by default).
    pub async fn create_agent_with_config(
        &mut self,
        suffix: &str,
        model_id: i64,
        config: serde_json::Value,
    ) -> agents::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let agent = agents::ActiveModel {
            pid: Set(format!("agent-{suffix}-{timestamp}")),
            organization_pid: Set("org_test".to_string()),
            created_by: Set("user_test".to_string()),
            name: Set(format!("Test Agent {suffix}")),
            slug: Set(format!("test-agent-{suffix}-{timestamp}")),
            kind: Set("startup".to_string()),
            model_id: Set(model_id),
            description: Set(None),
            temperature: Set(0.7),
            system_prompt: Set(Some("You are a helpful assistant.".to_string())),
            config: Set(config),
            is_active: Set(true),
            ..Default::default()
        };

        let agent = agent
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test agent");

        self.created_agents.push(agent.id);
        agent
    }

    /// Creates an agent in "personal_org_{user_id}" for a specific user
    pub async fn create_personal_agent(&mut self, suffix: &str, model_id: i64, user_id: &str) -> agents::Model {
        self.create_agent_in_org(suffix, model_id, &format!("personal_org_{user_id}"), user_id)
            .await
    }

    /// Creates an org agent
    pub async fn create_org_agent(
        &mut self,
        suffix: &str,
        model_id: i64,
        org_pid: &str,
        created_by: &str,
    ) -> agents::Model {
        self.create_agent_in_org(suffix, model_id, org_pid, created_by).await
    }

    /// Creates a test conversation (owned by user_test by default)
    pub async fn create_conversation(&mut self, suffix: &str, agent_id: i64) -> conversations::Model {
        self.create_conversation_for_user(suffix, agent_id, "user_test").await
    }

    /// Creates a conversation owned by a specific user in org_test
    pub async fn create_conversation_for_user(
        &mut self,
        suffix: &str,
        agent_id: i64,
        user_id: &str,
    ) -> conversations::Model {
        self.create_conversation_in_org(suffix, agent_id, user_id, "org_test")
            .await
    }

    /// Creates a conversation owned by a specific user in a specific org
    pub async fn create_conversation_in_org(
        &mut self,
        suffix: &str,
        agent_id: i64,
        user_id: &str,
        organization_pid: &str,
    ) -> conversations::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let conversation = conversations::ActiveModel {
            pid: Set(format!("conv-{suffix}-{timestamp}")),
            user_id: Set(user_id.to_string()),
            organization_pid: Set(organization_pid.to_string()),
            agent_id: Set(agent_id),
            title: Set(Some(format!("Test Conversation {suffix}"))),
            message_count: Set(0),
            ..Default::default()
        };

        let conversation = conversation
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test conversation");

        self.created_conversations.push(conversation.id);
        conversation
    }

    /// Creates a test secret (API key) for a provider
    ///
    /// Note: The secret value is automatically encrypted by the `before_save` hook
    /// in the secrets model.
    pub async fn create_api_key_secret(&mut self, suffix: &str, provider_id: i64) -> secrets::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Create the secret - encrypted_value will be encrypted by before_save hook
        let secret = secrets::ActiveModel {
            pid: Set(format!("secret-{suffix}-{timestamp}")),
            secret_type: Set(secrets::SecretKind::ApiKey.to_string()),
            encrypted_value: Set(b"test-api-key-value".to_vec()),
            ..Default::default()
        };

        let secret = secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test secret");

        // Link secret to provider
        let provider_secret = provider_secrets::ActiveModel {
            provider_id: Set(provider_id),
            secret_id: Set(secret.id),
        };

        provider_secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to link secret to provider");

        self.created_secrets.push(secret.id);
        secret
    }

    /// Creates a test message in a conversation
    pub async fn create_message(
        &mut self,
        suffix: &str,
        conversation_id: i64,
        role: &str,
        content: &str,
    ) -> messages::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Create a TEXT part with the content
        let parts = vec![MessagePart {
            id: format!("part-{suffix}-{timestamp}"),
            kind: MessagePartKind::Text.into(),
            status: MessagePartStatus::Complete.into(),
            content: Some(content.to_string()),
            tool_call: None,
            tool_result: None,
            media: None,
            summary: None,
        }];

        let message = messages::ActiveModel {
            pid: Set(format!("msg-{suffix}-{timestamp}")),
            conversation_id: Set(conversation_id),
            role: Set(role.to_string()),
            content: Set(Some(content.to_string())),
            parts: Set(serde_json::to_value(&parts).expect("parts serialization")),
            status: Set("complete".to_string()),
            ..Default::default()
        };

        let message = message
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test message");

        self.created_messages.push(message.id);
        message
    }

    /// Creates a malformed API key secret that will fail decryption.
    ///
    /// The value has the encryption marker prefix but contains garbage data
    /// that cannot be decrypted, causing `CryptoError::Decrypt` on access.
    pub async fn create_malformed_secret(&mut self, suffix: &str, provider_id: i64) -> secrets::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Value starts with marker (so before_save won't re-encrypt) but contains garbage
        let malformed_value = b"\xf0\x9f\x94\x91x0garbage-not-valid-ciphertext".to_vec();

        let secret = secrets::ActiveModel {
            pid: Set(format!("secret-malformed-{suffix}-{timestamp}")),
            secret_type: Set(secrets::SecretKind::ApiKey.to_string()),
            encrypted_value: Set(malformed_value),
            ..Default::default()
        };

        let secret = secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create malformed secret");

        // Link secret to provider
        let provider_secret = provider_secrets::ActiveModel {
            provider_id: Set(provider_id),
            secret_id: Set(secret.id),
        };

        provider_secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to link malformed secret to provider");

        self.created_secrets.push(secret.id);
        secret
    }

    /// Creates a complete test setup: provider → model → agent → conversation
    /// Returns (provider, model, agent, conversation)
    pub async fn create_full_setup(
        &mut self,
        suffix: &str,
    ) -> (
        providers::Model,
        language_models::Model,
        agents::Model,
        conversations::Model,
    ) {
        let provider = self.create_provider(suffix, "ollama").await;
        self.create_api_key_secret(suffix, provider.id).await;
        let model = self.create_model(suffix, provider.id).await;
        let agent = self.create_agent(suffix, model.id).await;
        let conversation = self.create_conversation(suffix, agent.id).await;

        (provider, model, agent, conversation)
    }

    /// Creates a test tool
    ///
    /// - `name`: The tool identifier (e.g., "current_time", "web_search")
    /// - `tool_type`: Either "system" or "user_defined"
    pub async fn create_tool(&mut self, suffix: &str, name: &str, tool_type: &str) -> tools::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let tool = tools::ActiveModel {
            pid: Set(format!("tool-{suffix}-{timestamp}")),
            name: Set(format!("{name}-{timestamp}")),
            display_name: Set(format!("Test Tool {suffix}")),
            description: Set(format!("Test tool description for {name}")),
            tool_type: Set(tool_type.to_string()),
            parameters_schema: Set(serde_json::json!({"type": "object", "properties": {}, "required": []})),
            is_active: Set(true),
            ..Default::default()
        };

        let tool = tool.insert(self.db.as_ref()).await.expect("Failed to create test tool");

        self.created_tools.push(tool.id);
        tool
    }

    /// Creates a system tool (tool_type = "system")
    pub async fn create_system_tool(&mut self, suffix: &str, name: &str) -> tools::Model {
        self.create_tool(suffix, name, "system").await
    }

    /// Creates an inactive tool
    pub async fn create_inactive_tool(&mut self, suffix: &str, name: &str) -> tools::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let tool = tools::ActiveModel {
            pid: Set(format!("tool-inactive-{suffix}-{timestamp}")),
            name: Set(format!("{name}-inactive-{timestamp}")),
            display_name: Set(format!("Inactive Tool {suffix}")),
            description: Set("Inactive test tool".to_string()),
            tool_type: Set("system".to_string()),
            parameters_schema: Set(serde_json::json!({"type": "object", "properties": {}, "required": []})),
            is_active: Set(false),
            ..Default::default()
        };

        let tool = tool
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create inactive test tool");

        self.created_tools.push(tool.id);
        tool
    }

    /// Links a tool to an agent via the agent_tools junction table
    pub async fn link_tool_to_agent(&mut self, agent_id: i64, tool_id: i64) {
        let agent_tool = agent_tools::ActiveModel {
            agent_id: Set(agent_id),
            tool_id: Set(tool_id),
            config: Set(serde_json::json!({})),
        };

        agent_tool
            .insert(self.db.as_ref())
            .await
            .expect("Failed to link tool to agent");
    }

    /// Links a secret to a tool via the tool_secrets junction table
    pub async fn link_secret_to_tool(&mut self, tool_id: i64, secret_id: i64) {
        let tool_secret = tool_secrets::ActiveModel {
            tool_id: Set(tool_id),
            secret_id: Set(secret_id),
        };

        tool_secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to link secret to tool");
    }

    /// Creates an API key secret and links it to a tool
    pub async fn create_tool_api_key_secret(&mut self, suffix: &str, tool_id: i64) -> secrets::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let secret = secrets::ActiveModel {
            pid: Set(format!("tool-secret-{suffix}-{timestamp}")),
            secret_type: Set(secrets::SecretKind::ApiKey.to_string()),
            encrypted_value: Set(b"test-tool-api-key-value".to_vec()),
            ..Default::default()
        };

        let secret = secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create tool secret");

        self.link_secret_to_tool(tool_id, secret.id).await;

        self.created_secrets.push(secret.id);
        secret
    }

    /// Creates a malformed secret and links it to a tool (for testing decryption failures)
    pub async fn create_malformed_tool_secret(&mut self, suffix: &str, tool_id: i64) -> secrets::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Value starts with marker (so before_save won't re-encrypt) but contains garbage
        let malformed_value = b"\xf0\x9f\x94\x91x0garbage-not-valid-ciphertext".to_vec();

        let secret = secrets::ActiveModel {
            pid: Set(format!("tool-secret-malformed-{suffix}-{timestamp}")),
            secret_type: Set(secrets::SecretKind::ApiKey.to_string()),
            encrypted_value: Set(malformed_value),
            ..Default::default()
        };

        let secret = secret
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create malformed tool secret");

        self.link_secret_to_tool(tool_id, secret.id).await;

        self.created_secrets.push(secret.id);
        secret
    }

    /// Links a user to an assistant agent via the user_assistants junction table (upsert)
    pub async fn link_user_to_assistant(&self, user_id: &str, agent_id: i64) {
        use sea_orm::sea_query::OnConflict;

        user_assistants::Entity::insert(user_assistants::ActiveModel {
            user_id: Set(user_id.to_string()),
            agent_id: Set(agent_id),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::column(user_assistants::Column::UserId)
                .update_column(user_assistants::Column::AgentId)
                .to_owned(),
        )
        .exec(self.db.as_ref())
        .await
        .expect("Failed to link user to assistant");
    }

    /// Creates an assistant agent for a user with the junction table link
    ///
    /// This creates both the agent and the user_assistants junction record.
    pub async fn create_user_assistant(&mut self, suffix: &str, model_id: i64, user_id: &str) -> agents::Model {
        let agent = self.create_personal_agent(suffix, model_id, user_id).await;
        self.link_user_to_assistant(user_id, agent.id).await;
        agent
    }

    /// Gets the system provider and its model (seeded by lifecycle)
    ///
    /// Uses ASSISTANT_PROVIDER and ASSISTANT_MODEL env vars to find the correct
    /// system provider and model. Panics if seed hasn't run or model not found.
    pub async fn get_system_provider_and_model(&self) -> (providers::Model, language_models::Model) {
        use sea_orm::QueryFilter;

        let assistant_provider = std::env::var("ASSISTANT_PROVIDER").expect("ASSISTANT_PROVIDER should be set");
        let assistant_model_id = std::env::var("ASSISTANT_MODEL").expect("ASSISTANT_MODEL should be set");

        let provider = providers::Entity::find()
            .filter(providers::Column::CreatedBy.is_null())
            .filter(providers::Column::OrganizationPid.is_null())
            .filter(providers::Column::ProviderType.eq(&assistant_provider))
            .one(self.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("System provider should exist (run seed first)");

        let model = language_models::Entity::find()
            .filter(language_models::Column::ProviderId.eq(provider.id))
            .filter(language_models::Column::ModelId.eq(&assistant_model_id))
            .one(self.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("System model should exist (run seed first)");

        (provider, model)
    }

    /// Creates an assistant agent backed by the system provider
    ///
    /// This is the correct way to create test assistants that will be found
    /// by get_user_assistant (which only looks for system-provider-backed agents).
    pub async fn create_system_user_assistant(&mut self, suffix: &str, user_id: &str) -> agents::Model {
        let (_provider, model) = self.get_system_provider_and_model().await;
        let agent = self.create_personal_agent(suffix, model.id, user_id).await;
        self.link_user_to_assistant(user_id, agent.id).await;
        agent
    }

    /// Generate a unique user ID for rate limit testing.
    ///
    /// Returns a user ID that will create unique rate limit keys,
    /// avoiding collisions between concurrent test runs.
    pub fn unique_user_id(&self, suffix: &str) -> String {
        format!("{}user_{}", self.test_prefix, suffix)
    }

    /// Creates a document record in the given status.
    pub async fn create_document_with_status(
        &mut self,
        suffix: &str,
        user_id: &str,
        organization_pid: &str,
        status: &str,
    ) -> documents::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let doc = documents::ActiveModelEx::new()
            .set_user_id(user_id)
            .set_organization_pid(organization_pid)
            .set_filename(format!("test-{suffix}.txt"))
            .set_content_type("text/plain")
            .set_size_bytes(1024)
            .set_storage_path(format!("{organization_pid}/doc-{suffix}-{timestamp}/test-{suffix}.txt"))
            .set_status(status.parse().unwrap_or(documents::DocStatus::Pending))
            .set_summary(None)
            .set_error_message(None);

        let doc = doc
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test document");

        self.created_documents.push(doc.id);
        doc.into()
    }

    /// Creates a document in `pending` status.
    pub async fn create_document(&mut self, suffix: &str, user_id: &str, organization_pid: &str) -> documents::Model {
        self.create_document_with_status(suffix, user_id, organization_pid, "pending")
            .await
    }

    /// Creates a document in `ready` status (for tests that don't care about upload flow).
    pub async fn create_ready_document(
        &mut self,
        suffix: &str,
        user_id: &str,
        organization_pid: &str,
    ) -> documents::Model {
        self.create_document_with_status(suffix, user_id, organization_pid, "ready")
            .await
    }

    /// Creates a document group.
    pub async fn create_document_group(
        &mut self,
        suffix: &str,
        user_id: &str,
        organization_pid: &str,
        is_org_shared: bool,
    ) -> document_groups::Model {
        let group = document_groups::ActiveModelEx::new()
            .set_user_id(user_id)
            .set_organization_pid(organization_pid)
            .set_name(format!("Test Group {suffix}"))
            .set_description(None)
            .set_is_org_shared(is_org_shared);

        let group = group
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test document group");

        self.created_document_groups.push(group.id);
        group.into()
    }

    /// Creates a document in `ready` status with optional org scope.
    pub async fn create_ready_document_with_org(
        &mut self,
        suffix: &str,
        user_id: &str,
        organization_pid: &str,
    ) -> documents::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let doc = documents::ActiveModelEx::new()
            .set_user_id(user_id)
            .set_organization_pid(organization_pid)
            .set_filename(format!("test-{suffix}.txt"))
            .set_content_type("text/plain")
            .set_size_bytes(1024)
            .set_storage_path(format!("{organization_pid}/doc-{suffix}-{timestamp}/test-{suffix}.txt"))
            .set_status(documents::DocStatus::Ready)
            .set_summary(None)
            .set_error_message(None);

        let doc = doc
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test document");

        self.created_documents.push(doc.id);
        doc.into()
    }

    pub async fn create_document_group_with_org(
        &mut self,
        suffix: &str,
        user_id: &str,
        organization_pid: &str,
        is_org_shared: bool,
    ) -> document_groups::Model {
        let group = document_groups::ActiveModelEx::new()
            .set_user_id(user_id)
            .set_organization_pid(organization_pid)
            .set_name(format!("Test Group {suffix}"))
            .set_description(None)
            .set_is_org_shared(is_org_shared);

        let group = group
            .insert(self.db.as_ref())
            .await
            .expect("Failed to create test document group");

        self.created_document_groups.push(group.id);
        group.into()
    }

    /// Links a document to a conversation via the conversation_documents join table.
    pub async fn link_document_to_conversation(&self, document_id: i64, conversation_id: i64) {
        let link = conversation_documents::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            conversation_id: Set(conversation_id),
            document_id: Set(document_id),
            created_at: sea_orm::ActiveValue::NotSet,
        };

        link.insert(self.db.as_ref())
            .await
            .expect("Failed to link document to conversation");
    }

    /// Adds a document to a group via the membership junction table.
    pub async fn add_document_to_group(&self, document_id: i64, group_id: i64) {
        let membership = document_group_memberships::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            document_id: Set(document_id),
            group_id: Set(group_id),
            created_at: sea_orm::ActiveValue::NotSet,
        };

        membership
            .insert(self.db.as_ref())
            .await
            .expect("Failed to add document to group");
    }
}
