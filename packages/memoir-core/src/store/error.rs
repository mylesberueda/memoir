use crate::memory::{MemoryKind, ScopeError};

/// Failure modes for [`crate::store::MemoryStore`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("memory not found: {0}")]
    NotFound(String),

    #[error("invalid scope: {0}")]
    InvalidScope(String),

    /// `edit` was called on a memory whose kind does not support in-place
    /// edits. Today this is every non-Episodic kind; Semantic rows require
    /// the Override-conversion path that epic 0011 introduces, and editing
    /// them via this method would silently break the `source_pid`
    /// provenance contract.
    #[error("edit not supported for memory {pid} with kind {kind:?}")]
    UnsupportedEdit { pid: String, kind: MemoryKind },

    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("cache invariant violated: {0}")]
    CacheInvariant(String),
}

impl From<ScopeError> for StoreError {
    fn from(err: ScopeError) -> Self {
        Self::InvalidScope(err.to_string())
    }
}
