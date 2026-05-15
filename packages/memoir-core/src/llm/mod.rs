//! Text-completion abstraction for extraction and adjacent LLM tasks.
//!
//! Defines [`LlmProvider`], implemented by [`RigLlmProvider`] (a wrapper
//! over the `rig-core` crate's per-provider clients). The trait stays
//! provider-agnostic; the wrapper dispatches via [`InnerLlm`] under the
//! hood. Tasks that need different models for different work look up
//! providers in an [`LlmRegistry`] keyed by [`LlmRole`].

mod anthropic;
mod config;
mod error;
pub mod extraction;
mod inner;
mod ollama;
mod openai;
mod role;

pub use config::{
    DEFAULT_ANTHROPIC_MODEL, DEFAULT_OLLAMA_MODEL, DEFAULT_OLLAMA_URL, DEFAULT_OPENAI_MODEL,
    LlmConfig, LlmKind,
};
pub use error::LlmError;
pub use extraction::{
    DEFAULT_EXTRACTION_PROMPT, ExtractionOutput, Fact, MAX_CONTENT_CHARS, parse_extraction,
};
pub use role::{LlmRegistry, LlmRole};

use std::future::Future;
use std::sync::Arc;

use rig::client::CompletionClient;
use rig::completion::Prompt;

use inner::InnerLlm;

/// Sends a preamble + content pair to an LLM and returns its raw text reply.
///
/// Implementations are responsible for transport, retries, and provider-side
/// error mapping. Parsing the reply is the caller's job; the trait stays
/// neutral on whether the reply is JSON, prose, or anything else.
pub trait LlmProvider: Send + Sync + 'static {
    /// Sends `preamble` as the system message and `content` as the user
    /// message, returning the model's text reply.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError::Connection`] when the backend is unreachable and
    /// [`LlmError::Provider`] for provider-side errors (rate limits, model
    /// not found, invalid request shape).
    fn extract(
        &self,
        preamble: &str,
        content: &str,
    ) -> impl Future<Output = Result<String, LlmError>> + Send;
}

/// Default [`LlmProvider`] backed by `rig-core`'s per-provider clients.
///
/// Constructed via [`Self::new`] from an [`LlmConfig`]; the variant
/// determines which underlying rig provider runs the call. The constructed
/// rig client is held behind an `Arc` so cloning [`RigLlmProvider`] is
/// cheap and the underlying connection pool is shared.
#[derive(Clone)]
pub struct RigLlmProvider {
    inner: InnerLlm,
    model: String,
}

impl std::fmt::Debug for RigLlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Deliberately omit api_key, base_url, and any provider-specific
        // connection details. Operators may run these against private
        // endpoints with credentials baked into the rig client.
        f.debug_struct("RigLlmProvider")
            .field("kind", &self.inner.kind())
            .field("model", &self.model)
            .finish_non_exhaustive()
    }
}

impl RigLlmProvider {
    /// Builds a provider from `config`.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError::Connection`] if the rig client can't be built
    /// (typically because the URL is malformed or the api-key shape is
    /// rejected by rig). The remote service itself is not contacted until
    /// [`Self::extract`] runs.
    pub fn new(config: LlmConfig) -> Result<Self, LlmError> {
        let (inner, model) = match config {
            LlmConfig::Ollama { url, model } => {
                let client = ollama::build_client(&url)?;
                (InnerLlm::Ollama(Arc::new(client)), model)
            }
            LlmConfig::OpenAI {
                api_key,
                model,
                base_url,
            } => {
                let client = openai::build_client(&api_key, base_url.as_deref())?;
                (InnerLlm::OpenAI(Arc::new(client)), model)
            }
            LlmConfig::Anthropic { api_key, model } => {
                let client = anthropic::build_client(&api_key)?;
                (InnerLlm::Anthropic(Arc::new(client)), model)
            }
        };

        Ok(Self { inner, model })
    }

    /// Returns the variant discriminator for this provider.
    #[must_use]
    pub fn kind(&self) -> LlmKind {
        self.inner.kind()
    }

    /// Returns the configured model name.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }
}

impl LlmProvider for RigLlmProvider {
    async fn extract(&self, preamble: &str, content: &str) -> Result<String, LlmError> {
        // Per-call agent construction: rig builds an `Agent<M>` cheaply
        // (it's a thin wrapper around the cloned client + preamble), so we
        // construct one per `extract` to let callers vary the preamble.
        match &self.inner {
            InnerLlm::Ollama(client) => {
                let agent = client.agent(&self.model).preamble(preamble).build();
                agent
                    .prompt(content)
                    .await
                    .map_err(|err| LlmError::Provider(err.to_string()))
            }
            InnerLlm::OpenAI(client) => {
                let agent = client.agent(&self.model).preamble(preamble).build();
                agent
                    .prompt(content)
                    .await
                    .map_err(|err| LlmError::Provider(err.to_string()))
            }
            InnerLlm::Anthropic(client) => {
                let agent = client.agent(&self.model).preamble(preamble).build();
                agent
                    .prompt(content)
                    .await
                    .map_err(|err| LlmError::Provider(err.to_string()))
            }
        }
    }
}

// M-TYPES-SEND: public types must be `Send` so they compose with tokio
// runtimes and held across `.await` points without infecting futures `!Send`.
const fn assert_send<T: Send>() {}
const _: () = {
    assert_send::<RigLlmProvider>();
    assert_send::<LlmConfig>();
    assert_send::<LlmError>();
    assert_send::<LlmKind>();
    assert_send::<LlmRegistry>();
    assert_send::<LlmRole>();
    assert_send::<Fact>();
    assert_send::<ExtractionOutput>();
};

#[cfg(test)]
mod tests {
    use super::*;

    struct StubLlmProvider {
        reply: String,
    }

    impl LlmProvider for StubLlmProvider {
        async fn extract(&self, _preamble: &str, _content: &str) -> Result<String, LlmError> {
            Ok(self.reply.clone())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_implement_trait_with_in_test_stub() {
        let provider = StubLlmProvider {
            reply: "ok".to_string(),
        };
        let reply = provider.extract("preamble", "content").await.unwrap();
        assert_eq!(reply, "ok");
    }

    #[test]
    fn should_construct_rig_provider_from_ollama_config() {
        let provider =
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "llama3.2")).unwrap();
        assert_eq!(provider.kind(), LlmKind::Ollama);
        assert_eq!(provider.model(), "llama3.2");
    }

    #[test]
    fn should_construct_rig_provider_from_openai_config() {
        let provider = RigLlmProvider::new(LlmConfig::openai("sk-test", "gpt-4o-mini")).unwrap();
        assert_eq!(provider.kind(), LlmKind::OpenAI);
        assert_eq!(provider.model(), "gpt-4o-mini");
    }

    #[test]
    fn should_construct_rig_provider_from_anthropic_config() {
        let provider =
            RigLlmProvider::new(LlmConfig::anthropic("sk-ant-test", "claude-haiku-4-5")).unwrap();
        assert_eq!(provider.kind(), LlmKind::Anthropic);
        assert_eq!(provider.model(), "claude-haiku-4-5");
    }

    #[test]
    fn should_redact_credentials_in_debug_output() {
        // Ollama URL
        let provider =
            RigLlmProvider::new(LlmConfig::ollama("http://internal-host:11434", "llama3.2"))
                .unwrap();
        let debug = format!("{provider:?}");
        assert!(!debug.contains("internal-host"), "Debug must NOT leak the URL; got {debug}");

        // OpenAI api_key
        let provider = RigLlmProvider::new(LlmConfig::openai("sk-secret-key", "gpt-4o")).unwrap();
        let debug = format!("{provider:?}");
        assert!(!debug.contains("sk-secret-key"), "Debug must NOT leak api_key; got {debug}");

        // Anthropic api_key
        let provider =
            RigLlmProvider::new(LlmConfig::anthropic("sk-ant-secret", "claude-haiku-4-5")).unwrap();
        let debug = format!("{provider:?}");
        assert!(!debug.contains("sk-ant-secret"), "Debug must NOT leak api_key; got {debug}");
    }

    #[test]
    fn should_debug_show_kind_and_model() {
        let provider =
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "llama3.2")).unwrap();
        let debug = format!("{provider:?}");
        // Debug renders the LlmKind enum variant verbatim (e.g. `Ollama`),
        // while Display lowercases it. Either is fine for operators; the
        // test just confirms kind information surfaces somewhere in Debug.
        assert!(
            debug.contains("Ollama") || debug.contains("ollama"),
            "Debug should include kind; got {debug}"
        );
        assert!(debug.contains("llama3.2"), "Debug should include model; got {debug}");
    }

    // ---- LlmRegistry tests ----

    #[test]
    fn should_registry_default_is_empty() {
        let registry = LlmRegistry::default();
        assert!(registry.is_empty());
        assert!(registry.get(LlmRole::Extraction).is_none());
    }

    #[test]
    fn should_registry_get_return_inserted_provider() {
        let mut registry = LlmRegistry::default();
        let provider =
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "llama3.2")).unwrap();
        registry.insert(LlmRole::Extraction, provider);

        let fetched = registry.get(LlmRole::Extraction).expect("should be present");
        assert_eq!(fetched.kind(), LlmKind::Ollama);
        assert!(!registry.is_empty());
    }

    #[test]
    fn should_registry_get_with_fallback_prefer_primary() {
        let mut registry = LlmRegistry::default();
        registry.insert(
            LlmRole::Extraction,
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "extraction-model"))
                .unwrap(),
        );
        registry.insert(
            LlmRole::Contradiction,
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "contradiction-model"))
                .unwrap(),
        );

        let chosen = registry
            .get_with_fallback(LlmRole::Contradiction, LlmRole::Extraction)
            .expect("primary should win");
        assert_eq!(chosen.model(), "contradiction-model");
    }

    #[test]
    fn should_registry_get_with_fallback_use_fallback_when_primary_missing() {
        let mut registry = LlmRegistry::default();
        registry.insert(
            LlmRole::Extraction,
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "extraction-model"))
                .unwrap(),
        );

        let chosen = registry
            .get_with_fallback(LlmRole::Contradiction, LlmRole::Extraction)
            .expect("fallback should resolve");
        assert_eq!(chosen.model(), "extraction-model");
    }

    #[test]
    fn should_registry_get_with_fallback_return_none_when_both_missing() {
        let registry = LlmRegistry::default();
        assert!(
            registry
                .get_with_fallback(LlmRole::Contradiction, LlmRole::Extraction)
                .is_none()
        );
    }

    #[test]
    fn should_registry_insert_replace_existing_provider() {
        let mut registry = LlmRegistry::default();
        registry.insert(
            LlmRole::Extraction,
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "first")).unwrap(),
        );
        registry.insert(
            LlmRole::Extraction,
            RigLlmProvider::new(LlmConfig::ollama("http://localhost:11434", "second")).unwrap(),
        );

        let chosen = registry.get(LlmRole::Extraction).unwrap();
        assert_eq!(chosen.model(), "second", "insert should replace");
    }

    #[test]
    fn should_render_role_as_lowercase_string() {
        assert_eq!(LlmRole::Extraction.as_str(), "extraction");
        assert_eq!(LlmRole::Contradiction.as_str(), "contradiction");
    }
}
