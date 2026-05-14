//! Postgres-backed source-of-truth storage for memories.
//!
//! Defines [`MemoryStore`], implemented by [`PostgresStore`] (the default) and
//! by callers who want to plug in a different backend or a test mock.
//! Vector search is a separate concern handled by [`crate::vector::VectorIndex`];
//! this trait covers only the source-of-truth row operations.

mod error;
pub mod postgres;

pub use error::StoreError;
pub use postgres::PostgresStore;

use std::future::Future;

use crate::memory::{ForgetTarget, Memory, MemoryKind, Scope};

/// Lifecycle state of a memory's vector index.
///
/// Persisted as the `qdrant_status` column on the memories table. The column
/// name is historical; the state is generic over which vector backend an
/// implementation uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexStatus {
    /// Row written to Postgres; embedding + vector upsert in flight.
    Pending,

    /// Vector upserted; memory is searchable.
    Indexed,

    /// Embedding or vector upsert failed; reconciliation will retry.
    Failed,
}

impl IndexStatus {
    /// Returns the canonical lowercase string used in storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Indexed => "indexed",
            Self::Failed => "failed",
        }
    }
}

/// Persists and retrieves memory rows from the source-of-truth store.
///
/// Implementations own the database connection. The trait methods are async
/// and `Send`-bound so callers can drive them from any tokio runtime,
/// including across `spawn` boundaries.
pub trait MemoryStore: Send + Sync + 'static {
    /// Inserts a new memory and returns the persisted row.
    ///
    /// The returned [`Memory`] carries the server-generated `pid`,
    /// `created_at`, and a `score` of `None`.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for database failures.
    fn remember(
        &self,
        scope: Scope,
        content: String,
        metadata: serde_json::Value,
        kind: MemoryKind,
    ) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Looks up a single memory by pid, returning all lifecycle states.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn recall(&self, pid: &str) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Fetches multiple memories by pid, returning only indexed rows.
    ///
    /// Used by the client facade to hydrate vector-search hits into full
    /// [`Memory`] values. Pids whose rows are in non-indexed lifecycle states
    /// (`pending`, `failed`) are silently omitted.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn find_by_pids(
        &self,
        pids: &[&str],
    ) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

    /// Deletes one memory or every memory in a scope, returning deleted pids.
    ///
    /// The returned pids let callers issue follow-up deletes against the
    /// vector index or graph store.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if a scope target has empty
    /// fields, [`StoreError::Database`] for database failures.
    fn forget(
        &self,
        target: ForgetTarget,
    ) -> impl Future<Output = Result<Vec<String>, StoreError>> + Send;

    /// Updates a memory's index lifecycle state.
    ///
    /// Called by the async embed substrate after vector upsert succeeds or
    /// fails, and by the reconciliation sweep.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn set_index_status(
        &self,
        pid: &str,
        status: IndexStatus,
    ) -> impl Future<Output = Result<(), StoreError>> + Send;
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

        async fn find_by_pids(&self, pids: &[&str]) -> Result<Vec<Memory>, StoreError> {
            let memories = self.memories.lock().unwrap();
            Ok(pids
                .iter()
                .filter_map(|pid| memories.iter().find(|m| m.pid == *pid).cloned())
                .collect())
        }

        async fn forget(&self, target: ForgetTarget) -> Result<Vec<String>, StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let mut deleted = Vec::new();
            match target {
                ForgetTarget::Pid(pid) => {
                    memories.retain(|m| {
                        if m.pid == pid {
                            deleted.push(m.pid.clone());
                            false
                        } else {
                            true
                        }
                    });
                }
                ForgetTarget::Scope(scope) => {
                    memories.retain(|m| {
                        if m.scope == scope {
                            deleted.push(m.pid.clone());
                            false
                        } else {
                            true
                        }
                    });
                }
            }
            Ok(deleted)
        }

        async fn set_index_status(&self, _pid: &str, _status: IndexStatus) -> Result<(), StoreError> {
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
        assert_eq!(deleted, vec![memory.pid.clone()]);

        let not_found = store.recall(&memory.pid).await;
        assert!(matches!(not_found, Err(StoreError::NotFound(_))));
    }

    #[test]
    fn should_render_index_status_as_lowercase_string() {
        assert_eq!(IndexStatus::Pending.as_str(), "pending");
        assert_eq!(IndexStatus::Indexed.as_str(), "indexed");
        assert_eq!(IndexStatus::Failed.as_str(), "failed");
    }
}
