//! Dispatch core for [`super::RigLlmProvider`].
//!
//! Wraps each rig provider's `Client` in a variant of [`InnerLlm`] so the
//! enclosing provider's methods can dispatch via a single `match` rather
//! than carrying a trait object. The pattern mirrors rig-service's
//! `InnerAgent` enum (apps/rig-service/src/agents/rig.rs:36-40 in
//! ~/dev/startup-ai).

use std::sync::Arc;

use rig_core::providers::{anthropic, ollama, openai};

use super::LlmKind;

/// Per-provider rig client held behind an `Arc` for cheap cloning.
///
/// Each variant wraps a fully-built rig `Client` — building a per-call
/// `Agent` is the responsibility of [`super::RigLlmProvider::extract`],
/// which constructs the agent fresh for each call so the preamble can
/// vary.
#[derive(Clone)]
pub(super) enum InnerLlm {
    Ollama(Arc<ollama::Client>),
    OpenAI(Arc<openai::Client>),
    Anthropic(Arc<anthropic::Client>),
}

impl InnerLlm {
    /// Returns the discriminator for this provider.
    pub(super) fn kind(&self) -> LlmKind {
        match self {
            Self::Ollama(_) => LlmKind::Ollama,
            Self::OpenAI(_) => LlmKind::OpenAI,
            Self::Anthropic(_) => LlmKind::Anthropic,
        }
    }
}

impl std::fmt::Debug for InnerLlm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't leak any field of the underlying client. Operators may run
        // these against private endpoints with credentials in the builder.
        f.debug_tuple("InnerLlm").field(&self.kind()).finish()
    }
}
