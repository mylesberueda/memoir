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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum IndexStatus {
    /// Row written to Postgres; embedding + vector upsert in flight.
    Pending,

    /// Vector upserted; memory is searchable.
    Indexed,

    /// Embedding or vector upsert failed; reconciliation will retry.
    Failed,
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
    /// `created_at`, and a `score` of `None`. `source_pid` is `None` for
    /// episodic rows and `Some(pid)` for semantic rows extracted from an
    /// episodic memory (the extract worker stage passes this through).
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
        source_pid: Option<String>,
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
    fn find_by_pids(&self, pids: &[&str]) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

    /// Deletes one memory or every memory in a scope, returning deleted pids.
    ///
    /// The returned pids let callers issue follow-up deletes against the
    /// vector index or graph store.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if a scope target has empty
    /// fields, [`StoreError::Database`] for database failures.
    fn forget(&self, target: ForgetTarget) -> impl Future<Output = Result<Vec<String>, StoreError>> + Send;

    /// Updates a memory's index lifecycle state.
    ///
    /// Called by the async embed substrate after vector upsert succeeds or
    /// fails, and by the reconciliation sweep.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn set_index_status(&self, pid: &str, status: IndexStatus) -> impl Future<Output = Result<(), StoreError>> + Send;

    /// Returns up to `limit` memories whose index lifecycle is `failed`.
    ///
    /// Used by the reconciliation sweep to retry embed + upsert. Returned in
    /// no specific order; the caller drives retry concurrency.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn find_failed(&self, limit: usize) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

    /// Returns every distinct scope tuple present in the store.
    ///
    /// Used by the reconciliation sweep's orphan-cleanup pass to know which
    /// scopes need a vector-index scroll. Expected to be cheap for typical
    /// tenant counts; very large deployments may need pagination later.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn list_scopes(&self) -> impl Future<Output = Result<Vec<Scope>, StoreError>> + Send;

    /// Returns every indexed pid for the given scope.
    ///
    /// Used by the reconciliation sweep's orphan-cleanup pass to compare
    /// against the vector index's scope contents. Only `indexed` rows are
    /// returned; `pending`/`failed` rows are not yet expected to have a
    /// vector index entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for database failures.
    fn indexed_pids_in_scope(&self, scope: &Scope) -> impl Future<Output = Result<Vec<String>, StoreError>> + Send;

    /// Marks `pid` as superseded by `by_pid`.
    ///
    /// Sets the `superseded_by` column to `by_pid` so search paths filter
    /// the row out. Idempotent: re-superseding an already-superseded row
    /// overwrites the pointer to the new winner. Internal API — callers
    /// must come from the contradiction-detection engine, not user code.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures (including FK
    /// violations when `by_pid` does not exist).
    fn supersede(&self, pid: &str, by_pid: &str) -> impl Future<Output = Result<(), StoreError>> + Send;

    /// Clears the supersession marker on `pid`, restoring it to active state.
    ///
    /// Used by the admin surface when an operator decides a supersession was
    /// wrong. No-ops at the SQL level if the row was already active
    /// (`superseded_by IS NULL`); still requires the row to exist.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn unsupersede(&self, pid: &str) -> impl Future<Output = Result<(), StoreError>> + Send;
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
            source_pid: Option<String>,
        ) -> Result<Memory, StoreError> {
            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
            let memory = Memory {
                pid: format!("test-{}", self.memories.lock().unwrap().len()),
                scope,
                content,
                metadata,
                kind,
                source_pid,
                supersession: None,
                created_at: now,
                updated_at: now,
                event_at: None,
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

        async fn find_failed(&self, _limit: usize) -> Result<Vec<Memory>, StoreError> {
            Ok(Vec::new())
        }

        async fn list_scopes(&self) -> Result<Vec<Scope>, StoreError> {
            let scopes: std::collections::HashSet<Scope> =
                self.memories.lock().unwrap().iter().map(|m| m.scope.clone()).collect();
            Ok(scopes.into_iter().collect())
        }

        async fn indexed_pids_in_scope(&self, scope: &Scope) -> Result<Vec<String>, StoreError> {
            Ok(self
                .memories
                .lock()
                .unwrap()
                .iter()
                .filter(|m| &m.scope == scope)
                .map(|m| m.pid.clone())
                .collect())
        }

        async fn supersede(&self, pid: &str, by_pid: &str) -> Result<(), StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let target = memories
                .iter_mut()
                .find(|m| m.pid == pid)
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;
            target.supersession = Some(crate::memory::SupersessionInfo {
                winner_pid: by_pid.to_string(),
                at: Utc::now().into(),
            });
            Ok(())
        }

        async fn unsupersede(&self, pid: &str) -> Result<(), StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let target = memories
                .iter_mut()
                .find(|m| m.pid == pid)
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;
            target.supersession = None;
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
                None,
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
        assert_eq!(IndexStatus::Pending.as_ref(), "pending");
        assert_eq!(IndexStatus::Indexed.as_ref(), "indexed");
        assert_eq!(IndexStatus::Failed.as_ref(), "failed");
    }

    async fn write(store: &StubStore, content: &str) -> Memory {
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "o".to_string(),
            user_id: "u".to_string(),
        };
        store
            .remember(
                scope,
                content.to_string(),
                serde_json::json!({}),
                MemoryKind::Semantic,
                None,
            )
            .await
            .unwrap()
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_set_superseded_by_when_supersede_called() {
        let store = StubStore::default();
        let loser = write(&store, "old fact").await;
        let winner = write(&store, "new fact").await;

        store.supersede(&loser.pid, &winner.pid).await.unwrap();

        let after = store.recall(&loser.pid).await.unwrap();
        let supersession = after.supersession.as_ref().expect("supersession set");
        assert_eq!(supersession.winner_pid, winner.pid);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_clear_superseded_by_when_unsupersede_called() {
        let store = StubStore::default();
        let loser = write(&store, "old fact").await;
        let winner = write(&store, "new fact").await;
        store.supersede(&loser.pid, &winner.pid).await.unwrap();

        store.unsupersede(&loser.pid).await.unwrap();

        let after = store.recall(&loser.pid).await.unwrap();
        assert_eq!(after.supersession, None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_not_found_when_supersede_targets_missing_pid() {
        let store = StubStore::default();
        let winner = write(&store, "fact").await;

        let result = store.supersede("does-not-exist", &winner.pid).await;

        assert!(matches!(result, Err(StoreError::NotFound(_))));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_not_found_when_unsupersede_targets_missing_pid() {
        let store = StubStore::default();

        let result = store.unsupersede("does-not-exist").await;

        assert!(matches!(result, Err(StoreError::NotFound(_))));
    }
}
