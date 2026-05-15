//! Provider-agnostic configuration for memoir-core's LLM integration.
//!
//! Each variant of [`LlmConfig`] carries the connection-and-model fields one
//! provider needs. The set of variants is closed by what `rig-core` supports
//! today (Ollama, OpenAI, Anthropic); extending it means adding a variant
//! plus a per-provider factory in the sibling files (`ollama.rs`,
//! `openai.rs`, `anthropic.rs`).

/// Default Ollama endpoint when callers don't specify one.
pub const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

/// Default Ollama model.
///
/// Picks a small Llama-family model that's commonly pulled, runs on a CPU,
/// and produces decent JSON when prompted with examples. Operators with
/// stronger hardware should override this.
pub const DEFAULT_OLLAMA_MODEL: &str = "llama3.2";

/// Default OpenAI model.
///
/// `gpt-4o-mini` is the cost/quality sweet spot for extraction-class tasks
/// at the time of writing.
pub const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

/// Default Anthropic model.
///
/// `claude-haiku-4-5` is the cheap/fast Claude tier; good enough for
/// extraction in most cases, and significantly cheaper than Sonnet.
pub const DEFAULT_ANTHROPIC_MODEL: &str = "claude-haiku-4-5";

/// Discriminator for [`LlmConfig`] variants.
///
/// Useful when a caller wants to log "which provider is configured" without
/// pattern-matching the full enum, and as the cross-reference for any
/// provider-specific dashboards or routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LlmKind {
    Ollama,
    OpenAI,
    Anthropic,
}

impl LlmKind {
    /// Returns the canonical lowercase string used in logs and storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ollama => "ollama",
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
        }
    }
}

impl std::fmt::Display for LlmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Connection + model selection for memoir-core's LLM integration.
///
/// Constructed via [`Self::ollama`], [`Self::openai`], [`Self::anthropic`].
/// Construct-via-struct-literal is intentionally NOT supported on a stable
/// surface — callers go through the constructors so future variant additions
/// stay non-breaking.
#[derive(Debug, Clone)]
pub enum LlmConfig {
    Ollama {
        url: String,
        model: String,
    },
    OpenAI {
        api_key: String,
        model: String,
        /// `None` defaults to the public OpenAI API endpoint. `Some` allows
        /// pointing at an enterprise proxy or a self-hosted compatible endpoint.
        base_url: Option<String>,
    },
    Anthropic {
        api_key: String,
        model: String,
    },
}

impl LlmConfig {
    /// Builds a config for a local Ollama instance.
    pub fn ollama(url: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Ollama {
            url: url.into(),
            model: model.into(),
        }
    }

    /// Builds a config for the OpenAI API.
    ///
    /// Uses the public OpenAI endpoint. For enterprise/proxy setups, use
    /// [`Self::openai_with_base_url`].
    pub fn openai(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::OpenAI {
            api_key: api_key.into(),
            model: model.into(),
            base_url: None,
        }
    }

    /// Builds a config for an OpenAI-compatible endpoint at a custom URL.
    pub fn openai_with_base_url(
        api_key: impl Into<String>,
        model: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        Self::OpenAI {
            api_key: api_key.into(),
            model: model.into(),
            base_url: Some(base_url.into()),
        }
    }

    /// Builds a config for the Anthropic API.
    pub fn anthropic(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::Anthropic {
            api_key: api_key.into(),
            model: model.into(),
        }
    }

    /// Returns the variant discriminator.
    pub fn kind(&self) -> LlmKind {
        match self {
            Self::Ollama { .. } => LlmKind::Ollama,
            Self::OpenAI { .. } => LlmKind::OpenAI,
            Self::Anthropic { .. } => LlmKind::Anthropic,
        }
    }

    /// Returns the configured model identifier.
    pub fn model(&self) -> &str {
        match self {
            Self::Ollama { model, .. }
            | Self::OpenAI { model, .. }
            | Self::Anthropic { model, .. } => model,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_ollama_from_str_literals() {
        let config = LlmConfig::ollama("http://localhost:11434", "llama3.2");
        match config {
            LlmConfig::Ollama { url, model } => {
                assert_eq!(url, "http://localhost:11434");
                assert_eq!(model, "llama3.2");
            }
            other => panic!("expected Ollama, got {other:?}"),
        }
    }

    #[test]
    fn should_build_openai_default_base_url_as_none() {
        let config = LlmConfig::openai("sk-test", "gpt-4o-mini");
        match config {
            LlmConfig::OpenAI {
                api_key,
                model,
                base_url,
            } => {
                assert_eq!(api_key, "sk-test");
                assert_eq!(model, "gpt-4o-mini");
                assert!(base_url.is_none());
            }
            other => panic!("expected OpenAI, got {other:?}"),
        }
    }

    #[test]
    fn should_build_openai_with_custom_base_url() {
        let config =
            LlmConfig::openai_with_base_url("sk-test", "gpt-4o-mini", "https://proxy.example.com");
        match config {
            LlmConfig::OpenAI { base_url, .. } => {
                assert_eq!(base_url.as_deref(), Some("https://proxy.example.com"));
            }
            other => panic!("expected OpenAI, got {other:?}"),
        }
    }

    #[test]
    fn should_build_anthropic_from_str_literals() {
        let config = LlmConfig::anthropic("sk-ant-test", "claude-haiku-4-5");
        match config {
            LlmConfig::Anthropic { api_key, model } => {
                assert_eq!(api_key, "sk-ant-test");
                assert_eq!(model, "claude-haiku-4-5");
            }
            other => panic!("expected Anthropic, got {other:?}"),
        }
    }

    #[test]
    fn should_report_kind_per_variant() {
        assert_eq!(LlmConfig::ollama("u", "m").kind(), LlmKind::Ollama);
        assert_eq!(LlmConfig::openai("k", "m").kind(), LlmKind::OpenAI);
        assert_eq!(LlmConfig::anthropic("k", "m").kind(), LlmKind::Anthropic);
    }

    #[test]
    fn should_report_model_per_variant() {
        assert_eq!(LlmConfig::ollama("u", "llama").model(), "llama");
        assert_eq!(LlmConfig::openai("k", "gpt-4o").model(), "gpt-4o");
        assert_eq!(LlmConfig::anthropic("k", "claude").model(), "claude");
    }

    #[test]
    fn should_render_kind_as_lowercase_string() {
        assert_eq!(LlmKind::Ollama.as_str(), "ollama");
        assert_eq!(LlmKind::OpenAI.as_str(), "openai");
        assert_eq!(LlmKind::Anthropic.as_str(), "anthropic");
    }

    #[test]
    fn should_display_kind_matching_as_str() {
        assert_eq!(LlmKind::Ollama.to_string(), "ollama");
    }
}
