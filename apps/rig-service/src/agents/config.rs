use proto_rs::rig::v1;

#[derive(Debug, Clone, bon::Builder)]
pub(crate) struct AgentConfig {
    #[builder(default = 50)]
    pub(crate) history_length: u32,
    #[builder(default = 1800)]
    pub(crate) session_ttl_seconds: u32,
    #[builder(default = true)]
    pub(crate) streaming_enabled: bool,
    #[builder(default = true)]
    pub(crate) thinking_enabled: bool,
    #[builder(default = 1200)]
    pub(crate) timeout_seconds: u32,
    #[builder(default = 120)]
    pub(crate) idle_timeout_seconds: u32,
    #[builder(default = true)]
    pub(crate) memory_enabled: bool,
    #[builder(default = 10)]
    pub(crate) memory_result_count: u32,
    #[builder(default = 0.7)]
    pub(crate) memory_similarity_threshold: f32,
    #[builder(default = 5)]
    pub(crate) document_result_count: u32,
    #[builder(default = 1.0)]
    pub(crate) compaction_threshold: f32,
    #[builder(default = 0.2)]
    pub(crate) compaction_keep_ratio: f32,
    pub(crate) max_tokens: Option<u32>,
    #[builder(default = true)]
    pub(crate) use_system_providers_on_creation: bool,
}

impl From<v1::AgentConfig> for AgentConfig {
    fn from(proto: v1::AgentConfig) -> Self {
        let defaults = Self::builder().build();
        let base = proto.base.unwrap_or_default();
        let startup = match proto.kind_config {
            Some(v1::agent_config::KindConfig::Startup(s)) => s,
            _ => v1::StartupAgentConfig::default(),
        };

        Self {
            history_length: base.history_length.unwrap_or(defaults.history_length),
            session_ttl_seconds: base.session_ttl_seconds.unwrap_or(defaults.session_ttl_seconds),
            streaming_enabled: base.streaming_enabled.unwrap_or(defaults.streaming_enabled),
            thinking_enabled: base.thinking_enabled.unwrap_or(defaults.thinking_enabled),
            timeout_seconds: base.timeout_seconds.unwrap_or(defaults.timeout_seconds),
            idle_timeout_seconds: base.idle_timeout_seconds.unwrap_or(defaults.idle_timeout_seconds),
            memory_enabled: base.memory_enabled.unwrap_or(defaults.memory_enabled),
            memory_result_count: base.memory_result_count.unwrap_or(defaults.memory_result_count),
            memory_similarity_threshold: base
                .memory_similarity_threshold
                .unwrap_or(defaults.memory_similarity_threshold),
            document_result_count: base.document_result_count.unwrap_or(defaults.document_result_count),
            compaction_threshold: base.compaction_threshold.unwrap_or(defaults.compaction_threshold),
            compaction_keep_ratio: base.compaction_keep_ratio.unwrap_or(defaults.compaction_keep_ratio),
            max_tokens: base.max_tokens,
            use_system_providers_on_creation: startup
                .use_system_providers_on_creation
                .unwrap_or(defaults.use_system_providers_on_creation),
        }
    }
}

impl From<AgentConfig> for v1::AgentConfig {
    fn from(config: AgentConfig) -> Self {
        v1::AgentConfig {
            base: Some(v1::BaseAgentConfig {
                history_length: Some(config.history_length),
                session_ttl_seconds: Some(config.session_ttl_seconds),
                streaming_enabled: Some(config.streaming_enabled),
                thinking_enabled: Some(config.thinking_enabled),
                timeout_seconds: Some(config.timeout_seconds),
                idle_timeout_seconds: Some(config.idle_timeout_seconds),
                memory_enabled: Some(config.memory_enabled),
                memory_result_count: Some(config.memory_result_count),
                memory_similarity_threshold: Some(config.memory_similarity_threshold),
                document_result_count: Some(config.document_result_count),
                compaction_threshold: Some(config.compaction_threshold),
                compaction_keep_ratio: Some(config.compaction_keep_ratio),
                max_tokens: config.max_tokens,
            }),
            kind_config: Some(v1::agent_config::KindConfig::Startup(v1::StartupAgentConfig {
                use_system_providers_on_creation: Some(config.use_system_providers_on_creation),
            })),
        }
    }
}
