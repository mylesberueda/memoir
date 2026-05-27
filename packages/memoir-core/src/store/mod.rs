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

use chrono::{DateTime, FixedOffset};

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
    /// Records a row in the `supersession_events` audit table; a database
    /// trigger maintains the cached `memories.superseded_by` column so
    /// search paths continue to filter superseded rows out. Idempotent in
    /// effect (the cache reflects the latest event), but every call is
    /// recorded in history. Internal API — callers must come from the
    /// contradiction-detection engine, not user code.
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
    /// wrong. Records an unsupersede event in the audit log; the trigger
    /// clears the cache. The audit row is always recorded, even when the
    /// row was already active — operator intent is preserved in history.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn unsupersede(&self, pid: &str) -> impl Future<Output = Result<(), StoreError>> + Send;

    /// Returns the winner pid `pid` was superseded by as of `as_of`, or `None`.
    ///
    /// Walks the `supersession_events` audit table for `pid`, returning the
    /// `winner_pid` of the most recent event whose `decided_at <= as_of`.
    /// `None` covers three cases: the pid has no supersession events at all,
    /// the events all occurred after `as_of`, or the latest event before
    /// `as_of` was an unsupersede (a row with `winner_pid IS NULL`).
    ///
    /// Used by point-in-time reads (`Client::recall_as_of`, ticket 0009) to
    /// answer "was this memory active at T?" The cached
    /// `memories.superseded_by` column is the present-time answer; this
    /// method answers the same question for arbitrary past timestamps.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn supersession_at(
        &self,
        pid: &str,
        as_of: DateTime<FixedOffset>,
    ) -> impl Future<Output = Result<Option<String>, StoreError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::sync::Mutex;

    /// One row of the in-memory supersession event log used by `StubStore`.
    ///
    /// Mirrors the Postgres `supersession_events` table shape. `winner_pid`
    /// is `None` for unsupersede events, matching the SQL `NULL` semantics.
    #[derive(Debug, Clone)]
    struct StubEvent {
        loser_pid: String,
        winner_pid: Option<String>,
        decided_at: DateTime<FixedOffset>,
    }

    #[derive(Default)]
    struct StubStore {
        memories: Mutex<Vec<Memory>>,
        events: Mutex<Vec<StubEvent>>,
    }

    impl StubStore {
        /// Recomputes a memory's `supersession` field from the event log.
        ///
        /// Replicates the Postgres trigger: latest event wins, `winner_pid IS
        /// NULL` clears the cache. Called after every supersede/unsupersede
        /// so reads see a consistent cached view without consulting the log.
        fn refresh_cache(&self, pid: &str) {
            let events = self.events.lock().unwrap();
            let latest = events
                .iter()
                .filter(|e| e.loser_pid == pid)
                .max_by_key(|e| e.decided_at);
            let supersession = latest.and_then(|e| {
                e.winner_pid.clone().map(|winner_pid| crate::memory::SupersessionInfo {
                    winner_pid,
                    at: e.decided_at,
                })
            });
            drop(events);
            let mut memories = self.memories.lock().unwrap();
            if let Some(m) = memories.iter_mut().find(|m| m.pid == pid) {
                m.supersession = supersession;
            }
        }
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
            // EXISTS-guarded behavior mirrored from Postgres: if the loser
            // pid doesn't exist, return NotFound without writing anything.
            {
                let memories = self.memories.lock().unwrap();
                if !memories.iter().any(|m| m.pid == pid) {
                    return Err(StoreError::NotFound(pid.to_string()));
                }
            }
            self.events.lock().unwrap().push(StubEvent {
                loser_pid: pid.to_string(),
                winner_pid: Some(by_pid.to_string()),
                decided_at: Utc::now().into(),
            });
            self.refresh_cache(pid);
            Ok(())
        }

        async fn unsupersede(&self, pid: &str) -> Result<(), StoreError> {
            {
                let memories = self.memories.lock().unwrap();
                if !memories.iter().any(|m| m.pid == pid) {
                    return Err(StoreError::NotFound(pid.to_string()));
                }
            }
            // Per DP2: always insert, even when already active.
            self.events.lock().unwrap().push(StubEvent {
                loser_pid: pid.to_string(),
                winner_pid: None,
                decided_at: Utc::now().into(),
            });
            self.refresh_cache(pid);
            Ok(())
        }

        async fn supersession_at(&self, pid: &str, as_of: DateTime<FixedOffset>) -> Result<Option<String>, StoreError> {
            let events = self.events.lock().unwrap();
            let latest = events
                .iter()
                .filter(|e| e.loser_pid == pid && e.decided_at <= as_of)
                .max_by_key(|e| e.decided_at);
            Ok(latest.and_then(|e| e.winner_pid.clone()))
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

    #[tokio::test(flavor = "current_thread")]
    async fn should_resolve_to_latest_winner_when_resuperseded() {
        let store = StubStore::default();
        let loser = write(&store, "old").await;
        let first_winner = write(&store, "first").await;
        let second_winner = write(&store, "second").await;

        store.supersede(&loser.pid, &first_winner.pid).await.unwrap();
        store.supersede(&loser.pid, &second_winner.pid).await.unwrap();

        let after = store.recall(&loser.pid).await.unwrap();
        let supersession = after.supersession.as_ref().expect("supersession set");
        assert_eq!(
            supersession.winner_pid, second_winner.pid,
            "latest event wins the cache"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_winner_pid_from_supersession_at_for_past_timestamp() {
        let store = StubStore::default();
        let loser = write(&store, "loser").await;
        let winner = write(&store, "winner").await;
        store.supersede(&loser.pid, &winner.pid).await.unwrap();
        let now: DateTime<FixedOffset> = Utc::now().into();

        let result = store.supersession_at(&loser.pid, now).await.unwrap();

        assert_eq!(result.as_deref(), Some(winner.pid.as_str()));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_none_from_supersession_at_when_as_of_predates_event() {
        let store = StubStore::default();
        let loser = write(&store, "loser").await;
        let winner = write(&store, "winner").await;
        let before: DateTime<FixedOffset> = Utc::now().into();
        // Sleep just enough that the event's decided_at is strictly after `before`.
        // current_thread runtime + this short sleep is reliable in CI.
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        store.supersede(&loser.pid, &winner.pid).await.unwrap();

        let result = store.supersession_at(&loser.pid, before).await.unwrap();

        assert!(result.is_none(), "events after as_of must not count");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_none_from_supersession_at_when_latest_event_was_unsupersede() {
        let store = StubStore::default();
        let loser = write(&store, "loser").await;
        let winner = write(&store, "winner").await;
        store.supersede(&loser.pid, &winner.pid).await.unwrap();
        store.unsupersede(&loser.pid).await.unwrap();
        let now: DateTime<FixedOffset> = Utc::now().into();

        let result = store.supersession_at(&loser.pid, now).await.unwrap();

        assert!(result.is_none(), "unsupersede event clears the as-of answer");
    }
}
