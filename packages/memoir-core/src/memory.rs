//! Memory domain types.

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

/// Selects which memory kinds a read includes.
///
/// Each field gates inclusion of one kind. Default ([`Self::default`]) has
/// every field `true` — retrieve all kinds. A field set to `false` filters
/// that kind out. Constructing with all fields `false` is legal and yields an
/// empty result.
///
/// Designed so that adding a new kind later is additive: a new `pub bool`
/// field with default `true` does not break existing constructors that use
/// `..Default::default()` or named-field init.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KindSelector {
    pub episodic: bool,
    pub semantic: bool,
}

impl Default for KindSelector {
    fn default() -> Self {
        Self { episodic: true, semantic: true }
    }
}

impl KindSelector {
    /// Returns the kinds this selector includes, in canonical order.
    pub fn included_kinds(&self) -> Vec<MemoryKind> {
        let mut out = Vec::with_capacity(2);
        if self.episodic {
            out.push(MemoryKind::Episodic);
        }
        if self.semantic {
            out.push(MemoryKind::Semantic);
        }
        out
    }

    /// Returns `true` when every defined kind is included.
    pub fn includes_all(&self) -> bool {
        self.episodic && self.semantic
    }

    /// Returns `true` when no kind is included.
    pub fn is_empty(&self) -> bool {
        !self.episodic && !self.semantic
    }
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

/// A list of memories and an optional LLM-facing system prompt section.
///
/// Returned by [`crate::client::Client::remember`]. Implements [`Display`]
/// for direct injection into a system prompt and [`Deref`] to `[Memory]`
/// for iteration.
///
/// When `system_prompt` is `Some`, [`Display`] emits the prompt followed by
/// a bullet list of memory content. When `None`, only the bullet list is
/// emitted — the caller takes responsibility for instructing the LLM.
///
/// [`Display`]: std::fmt::Display
/// [`Deref`]: std::ops::Deref
#[derive(Debug, Clone)]
pub struct Memories {
    list: Vec<Memory>,
    system_prompt: Option<String>,
}

impl Memories {
    /// Builds a `Memories` from a list and an optional system prompt section.
    pub fn new(list: Vec<Memory>, system_prompt: Option<String>) -> Self {
        Self { list, system_prompt }
    }

    /// Returns the contained memories as a slice.
    pub fn list(&self) -> &[Memory] {
        &self.list
    }

    /// Returns the configured system-prompt section, if any.
    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }
}

impl std::fmt::Display for Memories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prompt) = &self.system_prompt {
            writeln!(f, "{prompt}")?;
        }
        for memory in &self.list {
            writeln!(f, "- {}", memory.content)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Memories {
    type Target = [Memory];

    fn deref(&self) -> &[Memory] {
        &self.list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn fixture(content: &str) -> Memory {
        Memory {
            pid: "test".into(),
            scope: Scope {
                agent_id: "a".into(),
                org_id: "o".into(),
                user_id: "u".into(),
            },
            content: content.into(),
            metadata: serde_json::json!({}),
            kind: MemoryKind::Episodic,
            created_at: Utc::now().into(),
            score: None,
        }
    }

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

    #[test]
    fn should_display_memories_with_system_prompt_and_bullets() {
        let memories = Memories::new(
            vec![fixture("first"), fixture("second")],
            Some("Context:".into()),
        );

        assert_eq!(memories.to_string(), "Context:\n- first\n- second\n");
    }

    #[test]
    fn should_display_memories_without_system_prompt_as_bullets_only() {
        let memories = Memories::new(vec![fixture("only")], None);

        assert_eq!(memories.to_string(), "- only\n");
    }

    #[test]
    fn should_display_empty_memories_as_empty_string() {
        let memories = Memories::new(Vec::new(), None);
        assert_eq!(memories.to_string(), "");
    }

    #[test]
    fn should_deref_memories_to_slice() {
        let memories = Memories::new(vec![fixture("a"), fixture("b")], None);
        assert_eq!(memories.len(), 2);
        assert_eq!(memories[0].content, "a");
    }
}
