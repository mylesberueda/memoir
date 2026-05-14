//! Shared value types for memory operations.
//!
//! Defines [`Scope`], [`Memory`], [`MemoryKind`], [`MemoryKindFilter`], and
//! [`ForgetTarget`] — used across the storage trait, the client facade, and
//! the return types consumers handle directly.

use chrono::{DateTime, FixedOffset};

/// Tenant + agent + user partition for a memory.
///
/// Memories written under one scope are never returned under another. All
/// fields must be non-empty; callers that violate this get a runtime error
/// from the storage layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub agent_id: String,
    pub org_id: String,
    pub user_id: String,
}

/// Kind of memory written to or read from storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryKind {
    /// Conversational memory; written by `Client::remember`.
    Episodic,

    /// Structured fact extracted from episodic memory by an LLM (epic 0006).
    Semantic,
}

impl MemoryKind {
    /// Returns the canonical lowercase string used in storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
        }
    }
}

impl std::fmt::Display for MemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Filter applied when reading memories.
///
/// [`MemoryKindFilter::Both`] is the union of episodic + semantic and has no
/// direct write counterpart in [`MemoryKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryKindFilter {
    Episodic,
    Semantic,
    Both,
}

/// A stored memory and (if from a vector search) its similarity score.
///
/// `score` is `Some` only for memories returned by a similarity search;
/// memories returned by direct lookup (`Client::recall`) have `score = None`.
#[derive(Debug, Clone)]
pub struct Memory {
    pub pid: String,
    pub scope: Scope,
    pub content: String,
    pub metadata: serde_json::Value,
    pub kind: MemoryKind,
    pub created_at: DateTime<FixedOffset>,
    pub score: Option<f32>,
}

/// Target of a forget operation: a single memory or a whole scope.
#[derive(Debug, Clone)]
pub enum ForgetTarget {
    /// Forget exactly one memory by its public id.
    Pid(String),

    /// Forget every memory matching the scope tuple.
    Scope(Scope),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_memory_kind_as_lowercase_string() {
        assert_eq!(MemoryKind::Episodic.as_str(), "episodic");
        assert_eq!(MemoryKind::Semantic.as_str(), "semantic");
    }

    #[test]
    fn should_display_memory_kind_matching_as_str() {
        assert_eq!(MemoryKind::Episodic.to_string(), "episodic");
        assert_eq!(MemoryKind::Semantic.to_string(), "semantic");
    }
}
