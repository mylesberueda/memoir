//! Per-task LLM dispatch registry.
//!
//! Different memoir-core jobs may want different models — extraction is
//! often fine on a cheaper/faster model, while contradiction detection (if
//! ever implemented via an LLM rather than NLP math) may benefit from a
//! stronger one. [`LlmRegistry`] keys [`super::RigLlmProvider`] instances
//! by [`LlmRole`] so each call site reaches for the right provider.

use std::collections::HashMap;

use super::{LlmConfig, LlmError, RigLlmProvider};

/// What a given LLM call is being used for.
///
/// Adding a new role is purely additive: existing call sites that look up
/// other roles keep working, and operators who haven't configured the new
/// role get [`LlmRegistry::get`] returning `None` (which downstream code
/// handles as "skip this step gracefully").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum LlmRole {
    /// Calls extracting structured facts from episodic content (ticket 0006).
    Extraction,

    /// Calls deciding whether two semantic memories contradict.
    ///
    /// Memoir v0.1 expects to do contradiction detection via NLP math, not
    /// an LLM call. This variant exists so the LLM path is available if the
    /// math doesn't pan out — no consumer wires it today.
    Contradiction,
}

/// Map of [`LlmRole`] → [`RigLlmProvider`].
///
/// Constructed empty via [`Self::default`]; the builder (ticket 0010) calls
/// [`Self::insert`] for each role the caller configures. Lookup is via
/// [`Self::get`] or [`Self::get_with_fallback`].
#[derive(Debug, Clone, Default)]
pub struct LlmRegistry {
    providers: HashMap<LlmRole, RigLlmProvider>,
}

impl LlmRegistry {
    /// Builds an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the provider configured for `role`, if any.
    #[must_use]
    pub fn get(&self, role: LlmRole) -> Option<&RigLlmProvider> {
        self.providers.get(&role)
    }

    /// Returns the provider configured for `primary`, or falls back to
    /// `fallback` if `primary` is unset.
    ///
    /// Used by call sites that have a preferred role but accept a less-ideal
    /// alternative — e.g. contradiction detection prefers
    /// [`LlmRole::Contradiction`] but accepts [`LlmRole::Extraction`] if no
    /// dedicated contradiction LLM is configured.
    #[must_use]
    pub fn get_with_fallback(
        &self,
        primary: LlmRole,
        fallback: LlmRole,
    ) -> Option<&RigLlmProvider> {
        self.providers.get(&primary).or_else(|| self.providers.get(&fallback))
    }

    /// Installs `provider` at `role`, replacing any prior entry.
    pub fn insert(&mut self, role: LlmRole, provider: RigLlmProvider) {
        self.providers.insert(role, provider);
    }

    /// Builds a provider from `config` and installs it at `role`.
    ///
    /// Equivalent to `self.insert(role, RigLlmProvider::new(config)?)` plus a
    /// structured `INFO`-level trace recording the role and provider kind.
    /// Replaces any prior entry at `role`.
    ///
    /// # Errors
    ///
    /// Propagates [`LlmError`] from [`RigLlmProvider::new`].
    pub fn install(&mut self, role: LlmRole, config: LlmConfig) -> Result<(), LlmError> {
        let kind = config.kind();
        let provider = RigLlmProvider::new(config)?;
        self.insert(role, provider);

        tracing::event!(
            name: "memoir.llm.configured",
            tracing::Level::INFO,
            role = role.as_ref(),
            provider = kind.as_ref(),
            "configured {{provider}} provider for {{role}}",
        );

        Ok(())
    }

    /// Returns `true` when no roles are configured.
    ///
    /// The worker uses this to skip extract-job dispatch entirely when no
    /// LLM is wired up.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}
