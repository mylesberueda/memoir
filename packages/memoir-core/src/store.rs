//! Storage abstraction for memories.
//!
//! Defines [`MemoryStore`], implemented by `PostgresQdrantStore` (ticket 0005)
//! and by callers who want to plug in a different backend or a test mock.

use std::future::Future;

use crate::types::{ForgetTarget, Memory, MemoryKind, MemoryKindFilter, Scope};

/// Failure modes for [`MemoryStore`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("memory not found: {0}")]
    NotFound(String),

    #[error("invalid scope: {0}")]
    InvalidScope(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("vector store error: {0}")]
    VectorStore(String),
}

/// Lifecycle state of a memory's vector index.
///
/// Persisted as the `qdrant_status` column on the memories table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QdrantStatus {
    /// Written to Postgres; embedding + Qdrant upsert in flight.
    Pending,

    /// Embedding upserted to Qdrant; memory is searchable.
    Indexed,

    /// Embedding or Qdrant upsert failed; reconciliation will retry.
    Failed,
}

impl QdrantStatus {
    /// Returns the canonical lowercase string used in storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Indexed => "indexed",
            Self::Failed => "failed",
        }
    }
}

/// Persists and retrieves memories across the source-of-truth + vector index.
///
/// Implementations own the Postgres + vector-store connections. The trait
/// methods are async and `Send`-bound so callers can drive them from any
/// tokio runtime, including across `spawn` boundaries.
pub trait MemoryStore: Send + Sync + 'static {
    /// Inserts a new memory and returns the persisted row.
    ///
    /// The returned [`Memory`] carries the server-generated `pid`,
    /// `created_at`, and a `score` of `None` (no similarity match occurred).
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for Postgres failures.
    fn remember(
        &self,
        scope: Scope,
        content: String,
        metadata: serde_json::Value,
        kind: MemoryKind,
    ) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Looks up a single memory by its public id, returning all lifecycle states.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for Postgres failures.
    fn recall(&self, pid: &str) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Performs vector similarity search within `scope` and returns the top hits.
    ///
    /// Only memories whose vectors have been indexed are returned; pending and
    /// failed memories are excluded. Results are ordered by descending score.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::VectorStore`] for Qdrant failures, [`StoreError::Database`]
    /// for the Postgres fetch step.
    fn search(
        &self,
        scope: Scope,
        query_embedding: Vec<f32>,
        limit: usize,
        kind: MemoryKindFilter,
    ) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

    /// Deletes one memory or every memory in a scope.
    ///
    /// Returns the count of memories deleted. Postgres deletion is
    /// authoritative; vector-store deletion is best-effort and orphans are
    /// reconciled later (ticket 0012).
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if a scope target has empty
    /// fields, [`StoreError::Database`] for Postgres failures.
    fn forget(&self, target: ForgetTarget) -> impl Future<Output = Result<usize, StoreError>> + Send;

    /// Updates a memory's vector-index lifecycle state.
    ///
    /// Called by the async embed substrate (ticket 0010) and by the
    /// reconciliation sweep (ticket 0012).
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for Postgres failures.
    fn set_qdrant_status(&self, pid: &str, status: QdrantStatus)
    -> impl Future<Output = Result<(), StoreError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::sync::Mutex;

    #[derive(Default)]
    struct StubStore {
        memories: Mutex<Vec<Memory>>,
    }

    impl MemoryStore for StubStore {
        async fn remember(
            &self,
            scope: Scope,
            content: String,
            metadata: serde_json::Value,
            kind: MemoryKind,
        ) -> Result<Memory, StoreError> {
            let memory = Memory {
                pid: format!("test-{}", self.memories.lock().unwrap().len()),
                scope,
                content,
                metadata,
                kind,
                created_at: Utc::now().into(),
                score: None,
            };
            self.memories.lock().unwrap().push(memory.clone());
            Ok(memory)
        }

        async fn recall(&self, pid: &str) -> Result<Memory, StoreError> {
            self.memories
                .lock()
                .unwrap()
                .iter()
                .find(|m| m.pid == pid)
                .cloned()
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))
        }

        async fn search(
            &self,
            _scope: Scope,
            _query_embedding: Vec<f32>,
            limit: usize,
            _kind: MemoryKindFilter,
        ) -> Result<Vec<Memory>, StoreError> {
            Ok(self.memories.lock().unwrap().iter().take(limit).cloned().collect())
        }

        async fn forget(&self, target: ForgetTarget) -> Result<usize, StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let before = memories.len();
            match target {
                ForgetTarget::Pid(pid) => memories.retain(|m| m.pid != pid),
                ForgetTarget::Scope(scope) => memories.retain(|m| m.scope != scope),
            }
            Ok(before - memories.len())
        }

        async fn set_qdrant_status(&self, _pid: &str, _status: QdrantStatus) -> Result<(), StoreError> {
            Ok(())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_implement_trait_with_in_test_stub() {
        let store = StubStore::default();
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "o".to_string(),
            user_id: "u".to_string(),
        };

        let memory = store
            .remember(
                scope.clone(),
                "content".to_string(),
                serde_json::json!({}),
                MemoryKind::Episodic,
            )
            .await
            .unwrap();
        assert_eq!(memory.content, "content");

        let recalled = store.recall(&memory.pid).await.unwrap();
        assert_eq!(recalled.pid, memory.pid);

        let deleted = store.forget(ForgetTarget::Pid(memory.pid.clone())).await.unwrap();
        assert_eq!(deleted, 1);

        let not_found = store.recall(&memory.pid).await;
        assert!(matches!(not_found, Err(StoreError::NotFound(_))));
    }

    #[test]
    fn should_render_qdrant_status_as_lowercase_string() {
        assert_eq!(QdrantStatus::Pending.as_str(), "pending");
        assert_eq!(QdrantStatus::Indexed.as_str(), "indexed");
        assert_eq!(QdrantStatus::Failed.as_str(), "failed");
    }
}
