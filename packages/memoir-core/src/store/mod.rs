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

use crate::memory::{ForgetTarget, Memory, MemoryKind, Scope, SupersessionEvent};

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

/// The attributes of a new memory row for [`MemoryStore::remember`].
///
/// Groups the row's write-time attributes into one value so the insert path
/// has a single self-documenting parameter rather than a long positional list
/// (M-INIT-CASCADED). Every field is stated explicitly by the caller — there
/// are no silent defaults at this layer; the episodic and extract write paths
/// each supply their own `kind`, `confidence`, etc.
#[derive(Debug, Clone)]
pub struct NewMemory {
    /// Tenant + agent + user partition.
    pub scope: Scope,

    /// Raw text of the memory.
    pub content: String,

    /// Arbitrary JSON attached at write time; round-trips unchanged.
    pub metadata: serde_json::Value,

    /// Episodic (raw utterance) or semantic (extracted fact).
    pub kind: MemoryKind,

    /// Originating episodic pid for semantic rows; `None` for episodic.
    pub source_pid: Option<String>,

    /// Event-time of the remembered thing; `None` when unknown.
    pub event_at: Option<DateTime<FixedOffset>>,

    /// How sure memoir is of this memory: `MAX` for episodic, the scaled
    /// extraction score for semantic.
    pub confidence: crate::memory::Confidence,
}

/// Field-level patch for [`MemoryStore::edit`].
///
/// Each field is `Option`-tracked so callers update only what they pass.
/// `None` means "leave this field untouched"; `Some(value)` means "overwrite
/// with this value." `event_at = Some(None)` is reachable via the nested
/// `Option` and means "clear the event-time"; the outer wrapper distinguishes
/// "untouched" from "explicitly cleared."
#[derive(Debug, Clone, Default)]
pub struct EditPatch {
    /// New content. `None` leaves it unchanged.
    pub content: Option<String>,

    /// New metadata blob. `None` leaves it unchanged.
    pub metadata: Option<serde_json::Value>,

    /// New event-time. Outer `None` means "untouched"; `Some(None)` clears.
    pub event_at: Option<Option<DateTime<FixedOffset>>>,
}

impl EditPatch {
    /// Returns `true` when no field is set — the patch is a no-op.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.content.is_none() && self.metadata.is_none() && self.event_at.is_none()
    }
}

/// Direction in which [`MemoryStore::timeline`] orders rows by `created_at`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimelineDirection {
    /// Newest-first — `ORDER BY created_at DESC`.
    #[default]
    Descending,

    /// Oldest-first — `ORDER BY created_at ASC`.
    Ascending,
}

/// Default page size for [`MemoryStore::timeline`].
pub const DEFAULT_TIMELINE_LIMIT: usize = 50;

/// Parameters for [`MemoryStore::timeline`].
///
/// `None` on a window bound means "no bound on that side." `include_superseded`
/// defaults to `true` because timeline is the audit view; consumers wanting
/// "current truth only" pass `false`.
#[derive(Debug, Clone)]
pub struct TimelineParams {
    pub kinds: crate::memory::KindSelector,
    pub created_after: Option<DateTime<FixedOffset>>,
    pub created_before: Option<DateTime<FixedOffset>>,
    pub event_at_after: Option<DateTime<FixedOffset>>,
    pub event_at_before: Option<DateTime<FixedOffset>>,
    pub include_superseded: bool,
    pub limit: usize,
    pub direction: TimelineDirection,
}

impl Default for TimelineParams {
    fn default() -> Self {
        Self {
            kinds: crate::memory::KindSelector::default(),
            created_after: None,
            created_before: None,
            event_at_after: None,
            event_at_before: None,
            include_superseded: true,
            limit: DEFAULT_TIMELINE_LIMIT,
            direction: TimelineDirection::Descending,
        }
    }
}

/// Parameters for [`MemoryStore::memories_as_of`].
///
/// Returns memories that *existed* (`created_at <= as_of`) and were *active*
/// (not yet superseded as of `as_of`). `kinds` filter mirrors timeline's.
#[derive(Debug, Clone)]
pub struct AsOfParams {
    pub as_of: DateTime<FixedOffset>,
    pub kinds: crate::memory::KindSelector,
    pub limit: usize,
}

impl AsOfParams {
    /// Builds a default `AsOfParams` for `as_of` — all kinds, default limit.
    pub fn new(as_of: DateTime<FixedOffset>) -> Self {
        Self {
            as_of,
            kinds: crate::memory::KindSelector::default(),
            limit: DEFAULT_TIMELINE_LIMIT,
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
    /// `created_at`, `updated_at` (equal to `created_at` on insert), and a
    /// `score` of `None`. See [`NewMemory`] for the write-time attributes;
    /// `source_pid` is `None` for episodic rows and `Some(pid)` for semantic
    /// rows, and `confidence` is [`crate::memory::Confidence::MAX`] for
    /// episodic rows (the user said it) or the scaled extraction score for
    /// semantic rows.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for database failures.
    fn remember(&self, new: NewMemory) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Looks up a single memory by pid, returning all lifecycle states.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn recall(&self, pid: &str) -> impl Future<Output = Result<Memory, StoreError>> + Send;

    /// Returns memories in `scope` ordered by `created_at`, with optional filters.
    ///
    /// Postgres-only read; does not consult the vector index. Includes
    /// superseded rows by default — pass [`TimelineParams::include_superseded`]
    /// = `false` to filter them out. The `kinds` selector mirrors search's
    /// kind toggles. Hydrated rows carry `score = None` (no similarity was
    /// computed).
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for database failures.
    fn timeline(
        &self,
        scope: Scope,
        params: TimelineParams,
    ) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

    /// Returns memories that were active in `scope` as of `params.as_of`.
    ///
    /// A memory is included when `created_at <= as_of` AND, considering only
    /// `supersession_events` rows with `decided_at <= as_of`, the memory is
    /// not currently superseded (either no such events exist or the most
    /// recent one was an unsupersede with `winner_pid IS NULL`). Ordering
    /// is newest-first by `created_at`. Hydrated rows carry `score = None`.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::InvalidScope`] if any scope field is empty,
    /// [`StoreError::Database`] for database failures.
    fn memories_as_of(
        &self,
        scope: Scope,
        params: AsOfParams,
    ) -> impl Future<Output = Result<Vec<Memory>, StoreError>> + Send;

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

    /// Returns the active semantic rows derived from `source_pid` (epic 0011 Track B).
    ///
    /// "Active" means not yet superseded and not yet retired: the rows the
    /// reprocess engine must retire before re-deriving fresh ones. Episodic
    /// sources own zero or more semantic rows via `source_pid`; this is that
    /// set, filtered to the live ones. An unknown or episodic-only source
    /// yields an empty vector, not an error. Index lifecycle is ignored —
    /// a still-`pending` derived row is just as much in need of retirement.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn active_semantics_for_source(
        &self,
        source_pid: &str,
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

    /// Sets a memory's category label (epic 0011 ticket 0005).
    ///
    /// Called by the categorize worker after NLI classification. Overwrites
    /// any prior category. The value set is the caller's responsibility; the
    /// column is unconstrained `TEXT` (the taxonomy lives in the worker).
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn set_category(&self, pid: &str, category: &str) -> impl Future<Output = Result<(), StoreError>> + Send;

    /// Retires a memory with the given reason (epic 0011 Track B).
    ///
    /// Sets `retirement_reason`, hiding the row from all active-row reads.
    /// The row is NOT deleted — it stays recall-reachable by pid for audit
    /// and is the reprocess "don't re-derive this" guard + accuracy-metric
    /// record. The caller is responsible for evicting the row's vector (the
    /// store has no vector index); see [`crate::client::Client::reject`] /
    /// `mark_stale`, which orchestrate both.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures.
    fn retire(
        &self,
        pid: &str,
        reason: crate::memory::RetirementReason,
    ) -> impl Future<Output = Result<(), StoreError>> + Send;

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

    /// Returns the distinct agent ids that have memories in the given
    /// org + user scope, sorted ascending.
    ///
    /// Powers caller-scoped agent discovery (e.g. the UI's agent picker): a
    /// user sees only the agents under their own org and user, never another
    /// tenant's. Returns an empty vec when the scope has no memories yet.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn list_agent_ids(
        &self,
        org_id: &str,
        user_id: &str,
    ) -> impl Future<Output = Result<Vec<String>, StoreError>> + Send;

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

    /// Mutates a memory in place. See [`EditPatch`] for the field semantics.
    ///
    /// Distinct from [`Self::supersede`]: edit *overwrites* the original row
    /// because it was wrong (a correction), while supersede preserves it
    /// because new information obsoletes — but does not invalidate — old.
    /// `created_at` is unchanged; `updated_at` is bumped by the database
    /// trigger. The caller is responsible for re-embedding the row after a
    /// content change (enqueue a `JobKind::Embed` job; the worker handles
    /// the upsert) and for flipping `qdrant_status` back to `pending` so
    /// the row falls out of search until re-embedding completes.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::UnsupportedEdit`] when the target row's kind does not
    /// support in-place edits (currently every kind except `Episodic`),
    /// [`StoreError::Database`] for database failures.
    fn edit(&self, pid: &str, patch: EditPatch) -> impl Future<Output = Result<Memory, StoreError>> + Send;

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

    /// Returns every supersession decision against `pid` in chronological order.
    ///
    /// Reads the `supersession_events` audit table for `pid` and returns
    /// each event ascending by `decided_at`. An event with `winner_pid =
    /// None` is an unsupersede. Used by the supersession-audit view to
    /// render the full trail (supersede → unsupersede → re-supersede), in
    /// contrast to [`Self::supersession_at`] which collapses the trail to
    /// a single point-in-time answer.
    ///
    /// A `pid` with no events — whether it was never superseded or does
    /// not exist — returns an empty vec, not an error. The events table is
    /// the source of truth here.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::Database`] for database failures.
    fn supersession_history(
        &self,
        pid: &str,
    ) -> impl Future<Output = Result<Vec<SupersessionEvent>, StoreError>> + Send;
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
        async fn remember(&self, new: NewMemory) -> Result<Memory, StoreError> {
            let NewMemory {
                scope,
                content,
                metadata,
                kind,
                source_pid,
                event_at,
                confidence,
            } = new;
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
                event_at,
                score: None,
                status: IndexStatus::Pending,
                confidence,
                category: None,
                retirement: None,
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

        async fn active_semantics_for_source(&self, source_pid: &str) -> Result<Vec<Memory>, StoreError> {
            let memories = self.memories.lock().unwrap();
            Ok(memories
                .iter()
                .filter(|m| m.kind == MemoryKind::Semantic)
                .filter(|m| m.source_pid.as_deref() == Some(source_pid))
                .filter(|m| m.supersession.is_none() && m.retirement.is_none())
                .cloned()
                .collect())
        }

        async fn timeline(&self, scope: Scope, params: TimelineParams) -> Result<Vec<Memory>, StoreError> {
            scope.validate()?;
            let memories = self.memories.lock().unwrap();

            let mut filtered: Vec<Memory> = memories
                .iter()
                .filter(|m| m.scope == scope)
                .filter(|m| match m.kind {
                    MemoryKind::Episodic => params.kinds.episodic,
                    MemoryKind::Semantic => params.kinds.semantic,
                })
                .filter(|m| params.created_after.is_none_or(|t| m.created_at >= t))
                .filter(|m| params.created_before.is_none_or(|t| m.created_at < t))
                .filter(|m| {
                    params
                        .event_at_after
                        .is_none_or(|t| m.event_at.is_some_and(|ev| ev >= t))
                })
                .filter(|m| {
                    params
                        .event_at_before
                        .is_none_or(|t| m.event_at.is_some_and(|ev| ev < t))
                })
                .filter(|m| params.include_superseded || m.supersession.is_none())
                .cloned()
                .collect();

            filtered.sort_by(|a, b| match params.direction {
                TimelineDirection::Descending => b.created_at.cmp(&a.created_at),
                TimelineDirection::Ascending => a.created_at.cmp(&b.created_at),
            });
            filtered.truncate(params.limit);
            Ok(filtered)
        }

        async fn memories_as_of(&self, scope: Scope, params: AsOfParams) -> Result<Vec<Memory>, StoreError> {
            scope.validate()?;
            let memories = self.memories.lock().unwrap();
            let events = self.events.lock().unwrap();

            let mut filtered: Vec<Memory> = memories
                .iter()
                .filter(|m| m.scope == scope)
                .filter(|m| m.created_at <= params.as_of)
                .filter(|m| match m.kind {
                    MemoryKind::Episodic => params.kinds.episodic,
                    MemoryKind::Semantic => params.kinds.semantic,
                })
                .filter(|m| {
                    let latest = events
                        .iter()
                        .filter(|e| e.loser_pid == m.pid && e.decided_at <= params.as_of)
                        .max_by_key(|e| e.decided_at);
                    match latest {
                        None => true,
                        Some(e) => e.winner_pid.is_none(),
                    }
                })
                .cloned()
                .collect();

            filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            filtered.truncate(params.limit);
            Ok(filtered)
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

        async fn set_category(&self, pid: &str, category: &str) -> Result<(), StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let memory = memories
                .iter_mut()
                .find(|m| m.pid == pid)
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;
            memory.category = Some(category.to_string());
            Ok(())
        }

        async fn retire(&self, pid: &str, reason: crate::memory::RetirementReason) -> Result<(), StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let memory = memories
                .iter_mut()
                .find(|m| m.pid == pid)
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;
            memory.retirement = Some(reason);
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

        async fn list_agent_ids(&self, org_id: &str, user_id: &str) -> Result<Vec<String>, StoreError> {
            let agent_ids: std::collections::BTreeSet<String> = self
                .memories
                .lock()
                .unwrap()
                .iter()
                .filter(|m| m.scope.org_id == org_id && m.scope.user_id == user_id)
                .map(|m| m.scope.agent_id.clone())
                .collect();
            Ok(agent_ids.into_iter().collect())
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

        async fn edit(&self, pid: &str, patch: EditPatch) -> Result<Memory, StoreError> {
            let mut memories = self.memories.lock().unwrap();
            let memory = memories
                .iter_mut()
                .find(|m| m.pid == pid)
                .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;
            if memory.kind != MemoryKind::Episodic {
                return Err(StoreError::UnsupportedEdit {
                    pid: pid.to_string(),
                    kind: memory.kind,
                });
            }
            if let Some(content) = patch.content {
                memory.content = content;
            }
            if let Some(metadata) = patch.metadata {
                memory.metadata = metadata;
            }
            if let Some(event_at) = patch.event_at {
                memory.event_at = event_at;
            }
            memory.updated_at = Utc::now().into();
            Ok(memory.clone())
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

        async fn supersession_history(&self, pid: &str) -> Result<Vec<SupersessionEvent>, StoreError> {
            let events = self.events.lock().unwrap();
            let mut trail: Vec<SupersessionEvent> = events
                .iter()
                .filter(|e| e.loser_pid == pid)
                .map(|e| SupersessionEvent {
                    winner_pid: e.winner_pid.clone(),
                    decided_at: e.decided_at,
                })
                .collect();
            trail.sort_by_key(|e| e.decided_at);
            Ok(trail)
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
            .remember(NewMemory {
                scope: scope.clone(),
                content: "content".to_string(),
                metadata: serde_json::json!({}),
                kind: MemoryKind::Episodic,
                source_pid: None,
                event_at: None,
                confidence: crate::memory::Confidence::MAX,
            })
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

    #[tokio::test]
    async fn should_list_distinct_sorted_agent_ids_for_matching_org_and_user() {
        let store = StubStore::default();
        let remember = async |agent: &str, org: &str, user: &str| {
            store
                .remember(NewMemory {
                    scope: Scope {
                        agent_id: agent.to_string(),
                        org_id: org.to_string(),
                        user_id: user.to_string(),
                    },
                    content: "c".to_string(),
                    metadata: serde_json::json!({}),
                    kind: MemoryKind::Episodic,
                    source_pid: None,
                    event_at: None,
                    confidence: crate::memory::Confidence::MAX,
                })
                .await
                .unwrap();
        };

        remember("zeta", "o", "u").await;
        remember("alpha", "o", "u").await;
        remember("alpha", "o", "u").await; // duplicate agent — should collapse
        remember("other-org", "o2", "u").await; // wrong org — excluded
        remember("other-user", "o", "u2").await; // wrong user — excluded

        let agents = store.list_agent_ids("o", "u").await.unwrap();

        assert_eq!(agents, vec!["alpha".to_string(), "zeta".to_string()]);
    }

    #[tokio::test]
    async fn should_return_empty_agent_ids_when_scope_has_no_memories() {
        let store = StubStore::default();
        let agents = store.list_agent_ids("empty-org", "empty-user").await.unwrap();
        assert!(agents.is_empty());
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
            .remember(NewMemory {
                scope,
                content: content.to_string(),
                metadata: serde_json::json!({}),
                kind: MemoryKind::Semantic,
                source_pid: None,
                event_at: None,
                confidence: crate::memory::Confidence::MAX,
            })
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

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_empty_supersession_history_when_pid_has_no_events() {
        let store = StubStore::default();
        let solo = write(&store, "never superseded").await;

        let trail = store.supersession_history(&solo.pid).await.unwrap();

        assert!(trail.is_empty(), "no events = empty trail, not NotFound");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_supersession_history_in_ascending_order() {
        let store = StubStore::default();
        let loser = write(&store, "old").await;
        let first = write(&store, "first").await;
        let second = write(&store, "second").await;
        store.supersede(&loser.pid, &first.pid).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        store.supersede(&loser.pid, &second.pid).await.unwrap();

        let trail = store.supersession_history(&loser.pid).await.unwrap();

        assert_eq!(trail.len(), 2, "both events present");
        assert_eq!(trail[0].winner_pid.as_deref(), Some(first.pid.as_str()));
        assert_eq!(trail[1].winner_pid.as_deref(), Some(second.pid.as_str()));
        assert!(trail[0].decided_at <= trail[1].decided_at, "ascending by decided_at");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_include_unsupersede_events_in_supersession_history() {
        let store = StubStore::default();
        let loser = write(&store, "old").await;
        let winner = write(&store, "winner").await;
        store.supersede(&loser.pid, &winner.pid).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        store.unsupersede(&loser.pid).await.unwrap();

        let trail = store.supersession_history(&loser.pid).await.unwrap();

        assert_eq!(trail.len(), 2);
        assert_eq!(
            trail[0].winner_pid.as_deref(),
            Some(winner.pid.as_str()),
            "supersede first"
        );
        assert!(
            trail[1].winner_pid.is_none(),
            "unsupersede represented as winner_pid=None"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_reject_edit_when_kind_is_not_episodic() {
        let store = StubStore::default();
        let semantic = write(&store, "derived fact").await;

        let result = store
            .edit(
                &semantic.pid,
                EditPatch {
                    content: Some("hand edit".to_string()),
                    ..EditPatch::default()
                },
            )
            .await;

        match result {
            Err(StoreError::UnsupportedEdit { pid, kind }) => {
                assert_eq!(pid, semantic.pid);
                assert_eq!(kind, MemoryKind::Semantic);
            }
            other => panic!("expected UnsupportedEdit for semantic kind; got {other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_not_found_when_editing_missing_pid() {
        let store = StubStore::default();

        let result = store
            .edit(
                "no-such-pid",
                EditPatch {
                    content: Some("anything".to_string()),
                    ..EditPatch::default()
                },
            )
            .await;

        match result {
            Err(StoreError::NotFound(pid)) => assert_eq!(pid, "no-such-pid"),
            other => panic!("expected NotFound for missing pid; got {other:?}"),
        }
    }
}
