//! Postgres-backed write-behind queue for memoir-core's worker.
//!
//! Defines [`MemoryJobsStore`], implemented by [`PostgresJobsStore`] (the
//! default) and by callers who want to plug in a different queue substrate
//! or a test mock. Stage handlers (embed, extract) live on `ClientInner`
//! and consume the trait via the worker loop.

mod error;
pub mod postgres;
mod types;

pub use error::JobsError;
pub use postgres::PostgresJobsStore;
pub use types::{FailedJob, Job, JobKind, JobState};

use std::future::Future;

/// Claims, completes, and fails rows in the `memory_jobs` table.
///
/// Implementations own the database connection. The trait methods are async
/// and `Send`-bound so callers can drive them from any tokio runtime,
/// including across `spawn` boundaries.
///
/// Concurrent claim safety: implementations MUST guarantee that two callers
/// invoking [`Self::claim`] never receive the same job. The Postgres impl
/// uses `SELECT FOR UPDATE SKIP LOCKED`; other backends provide equivalent
/// semantics.
pub trait MemoryJobsStore: Send + Sync + 'static {
    /// Enqueues a new job in `pending` state and returns its id.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn enqueue(
        &self,
        kind: JobKind,
        source_pid: String,
        payload: serde_json::Value,
    ) -> impl Future<Output = Result<i64, JobsError>> + Send;

    /// Atomically enqueues a `synthesize` job for `source_pid`, but only once
    /// both LLM-derived siblings have succeeded.
    ///
    /// The two-parent fan-in of the knowledge-graph pipeline, resolved without a
    /// job-dependency system: a `synthesize` row is inserted iff no `extract` or
    /// `relational_extract` row for `source_pid` still exists. Because a
    /// succeeded job deletes its row ([`Self::complete`]) while a failed or
    /// in-flight one retains it, "no sibling row" means "both siblings
    /// succeeded." The check and the insert MUST share one atomic statement so
    /// two concurrent sibling completions cannot both insert. Returns whether a
    /// row was inserted (`false` when a sibling is still present, or a
    /// `synthesize` row already exists).
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn enqueue_synthesis_if_ready(&self, source_pid: &str) -> impl Future<Output = Result<bool, JobsError>> + Send;

    /// Atomically claims the oldest pending job and returns it, or `None`.
    ///
    /// Uses `SELECT FOR UPDATE SKIP LOCKED` (or equivalent) so concurrent
    /// callers never see the same row. The claim records `claimed_at` and
    /// optionally `claimed_by` for lease tracking.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn claim(&self, claimed_by: Option<&str>) -> impl Future<Output = Result<Option<Job>, JobsError>> + Send;

    /// Marks a claimed job as completed (deletes the row).
    ///
    /// The job MUST currently be in `claimed` state held by the caller;
    /// completing a row another worker holds is a logic error and surfaces
    /// as [`JobsError::NotFound`] (no row matched the id-and-state predicate).
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::NotFound`] when no claimed job matches `id`,
    /// [`JobsError::Database`] for database failures.
    fn complete(&self, id: i64) -> impl Future<Output = Result<(), JobsError>> + Send;

    /// Flips a claimed job back to `pending` with an incremented attempt
    /// counter; if `attempts` reaches `max_attempts`, the job lands in
    /// `failed` instead.
    ///
    /// `reason` is recorded on the row's `failure_reason` column. The
    /// `updated_at` timestamp is bumped by the trigger.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::NotFound`] when no claimed job matches `id`,
    /// [`JobsError::Database`] for database failures.
    fn fail(&self, id: i64, reason: String, max_attempts: i32) -> impl Future<Output = Result<(), JobsError>> + Send;

    /// Re-pends every claimed job whose lease has expired.
    ///
    /// Called by the worker on startup and periodically while idle, so a
    /// crashed worker's in-flight rows do not stay claimed forever. Returns
    /// the number of rows recovered.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn reset_expired_leases(&self, lease: std::time::Duration) -> impl Future<Output = Result<u64, JobsError>> + Send;

    /// Returns up to `limit` failed jobs, newest-first by `updated_at`.
    ///
    /// Excludes job payloads and any related memory content; the returned
    /// [`FailedJob`] carries only metadata operators need to triage.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn list_failed(&self, limit: usize) -> impl Future<Output = Result<Vec<FailedJob>, JobsError>> + Send;

    /// Flips one failed job back to `pending` and clears the attempt counter.
    ///
    /// "Cleared counter" is the deliberate semantic for operator-initiated
    /// retries: a human has decided the prior failures shouldn't count
    /// against the new attempt budget.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::NotFound`] when no failed job matches `id`,
    /// [`JobsError::Database`] for database failures.
    fn retry_job(&self, id: i64) -> impl Future<Output = Result<(), JobsError>> + Send;

    /// Flips every failed job matching `kind` back to `pending` with cleared
    /// attempts. Returns the number of rows that would be (or were) affected.
    ///
    /// When `dry_run` is `true`, returns the count without modifying any
    /// rows — useful for previewing how big a bulk retry will be before
    /// firing it.
    ///
    /// Passing `kind = None` matches all kinds.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn bulk_retry(&self, kind: Option<JobKind>, dry_run: bool) -> impl Future<Output = Result<u64, JobsError>> + Send;

    /// Permanently deletes one failed job. The referenced memory is untouched.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::NotFound`] when no failed job matches `id`,
    /// [`JobsError::Database`] for database failures.
    fn delete_failed(&self, id: i64) -> impl Future<Output = Result<(), JobsError>> + Send;

    /// Returns the count of jobs currently in `pending` state.
    ///
    /// Cheap observation for operators monitoring queue depth.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn pending_count(&self) -> impl Future<Output = Result<u64, JobsError>> + Send;
}

// M-TYPES-SEND: public types must be `Send` so they compose with tokio.
const fn assert_send<T: Send>() {}
const _: () = {
    assert_send::<Job>();
    assert_send::<FailedJob>();
    assert_send::<JobKind>();
    assert_send::<JobState>();
    assert_send::<JobsError>();
    assert_send::<PostgresJobsStore>();
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::sync::Mutex;
    use std::time::Duration;

    #[derive(Default)]
    struct StubJobsStore {
        rows: Mutex<Vec<Job>>,
        next_id: Mutex<i64>,
    }

    impl StubJobsStore {
        fn alloc_id(&self) -> i64 {
            let mut guard = self.next_id.lock().unwrap();
            *guard += 1;
            *guard
        }
    }

    impl MemoryJobsStore for StubJobsStore {
        async fn enqueue(
            &self,
            kind: JobKind,
            source_pid: String,
            payload: serde_json::Value,
        ) -> Result<i64, JobsError> {
            let id = self.alloc_id();
            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
            let job = Job {
                id,
                source_pid,
                kind,
                state: JobState::Pending,
                payload,
                attempts: 0,
                failure_reason: None,
                claimed_at: None,
                claimed_by: None,
                created_at: now,
                updated_at: now,
            };
            self.rows.lock().unwrap().push(job);
            Ok(id)
        }

        async fn enqueue_synthesis_if_ready(&self, source_pid: &str) -> Result<bool, JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let blocked = rows.iter().any(|r| {
                r.source_pid == source_pid
                    && matches!(
                        r.kind,
                        JobKind::Extract | JobKind::RelationalExtract | JobKind::Synthesize
                    )
            });
            if blocked {
                return Ok(false);
            }
            let id = {
                let mut guard = self.next_id.lock().unwrap();
                *guard += 1;
                *guard
            };
            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
            rows.push(Job {
                id,
                source_pid: source_pid.to_owned(),
                kind: JobKind::Synthesize,
                state: JobState::Pending,
                payload: serde_json::json!({}),
                attempts: 0,
                failure_reason: None,
                claimed_at: None,
                claimed_by: None,
                created_at: now,
                updated_at: now,
            });
            Ok(true)
        }

        async fn claim(&self, claimed_by: Option<&str>) -> Result<Option<Job>, JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
            let Some(row) = rows.iter_mut().find(|r| r.state == JobState::Pending) else {
                return Ok(None);
            };
            row.state = JobState::Claimed;
            row.claimed_at = Some(now);
            row.claimed_by = claimed_by.map(str::to_owned);
            row.updated_at = now;
            Ok(Some(row.clone()))
        }

        async fn complete(&self, id: i64) -> Result<(), JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let before = rows.len();
            rows.retain(|r| !(r.id == id && r.state == JobState::Claimed));
            if rows.len() == before {
                return Err(JobsError::NotFound(id.to_string()));
            }
            Ok(())
        }

        async fn fail(&self, id: i64, reason: String, max_attempts: i32) -> Result<(), JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let Some(row) = rows.iter_mut().find(|r| r.id == id && r.state == JobState::Claimed) else {
                return Err(JobsError::NotFound(id.to_string()));
            };
            row.attempts += 1;
            row.failure_reason = Some(reason);
            row.claimed_at = None;
            row.claimed_by = None;
            row.state = if row.attempts >= max_attempts {
                JobState::Failed
            } else {
                JobState::Pending
            };
            row.updated_at = Utc::now().into();
            Ok(())
        }

        async fn reset_expired_leases(&self, lease: Duration) -> Result<u64, JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let now = Utc::now();
            let cutoff = now - chrono::Duration::from_std(lease).unwrap_or_default();
            let mut recovered = 0u64;
            for row in rows.iter_mut() {
                if row.state != JobState::Claimed {
                    continue;
                }
                let Some(claimed_at) = row.claimed_at else { continue };
                if claimed_at < cutoff {
                    row.state = JobState::Pending;
                    row.claimed_at = None;
                    row.claimed_by = None;
                    row.updated_at = now.into();
                    recovered += 1;
                }
            }
            Ok(recovered)
        }

        async fn list_failed(&self, limit: usize) -> Result<Vec<FailedJob>, JobsError> {
            let rows = self.rows.lock().unwrap();
            let mut out: Vec<FailedJob> = rows
                .iter()
                .filter(|r| r.state == JobState::Failed)
                .map(|r| FailedJob {
                    id: r.id,
                    source_pid: r.source_pid.clone(),
                    kind: r.kind,
                    attempts: r.attempts,
                    failure_reason: r.failure_reason.clone(),
                    updated_at: r.updated_at,
                })
                .collect();
            // Newest first by updated_at.
            out.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
            out.truncate(limit);
            Ok(out)
        }

        async fn retry_job(&self, id: i64) -> Result<(), JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let Some(row) = rows.iter_mut().find(|r| r.id == id && r.state == JobState::Failed) else {
                return Err(JobsError::NotFound(id.to_string()));
            };
            row.state = JobState::Pending;
            row.attempts = 0;
            row.failure_reason = None;
            row.claimed_at = None;
            row.claimed_by = None;
            row.updated_at = Utc::now().into();
            Ok(())
        }

        async fn bulk_retry(&self, kind: Option<JobKind>, dry_run: bool) -> Result<u64, JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
            let mut affected = 0u64;
            for row in rows.iter_mut() {
                if row.state != JobState::Failed {
                    continue;
                }
                if let Some(k) = kind
                    && row.kind != k
                {
                    continue;
                }
                affected += 1;
                if dry_run {
                    continue;
                }
                row.state = JobState::Pending;
                row.attempts = 0;
                row.failure_reason = None;
                row.claimed_at = None;
                row.claimed_by = None;
                row.updated_at = now;
            }
            Ok(affected)
        }

        async fn delete_failed(&self, id: i64) -> Result<(), JobsError> {
            let mut rows = self.rows.lock().unwrap();
            let before = rows.len();
            rows.retain(|r| !(r.id == id && r.state == JobState::Failed));
            if rows.len() == before {
                return Err(JobsError::NotFound(id.to_string()));
            }
            Ok(())
        }

        async fn pending_count(&self) -> Result<u64, JobsError> {
            let rows = self.rows.lock().unwrap();
            Ok(rows.iter().filter(|r| r.state == JobState::Pending).count() as u64)
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_enqueue_then_claim_the_same_row() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let claimed = store.claim(None).await.unwrap().expect("row to claim");
        assert_eq!(claimed.id, id);
        assert_eq!(claimed.state, JobState::Claimed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_none_when_queue_is_empty() {
        let store = StubJobsStore::default();
        assert!(store.claim(None).await.unwrap().is_none());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_complete_remove_the_row() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Extract, "pid_y".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.complete(id).await.unwrap();
        assert!(store.claim(None).await.unwrap().is_none());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_fail_bump_attempts_and_return_to_pending_when_under_max() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid_z".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id, "boom".to_string(), 3).await.unwrap();
        let next = store.claim(None).await.unwrap().expect("row should be claimable again");
        assert_eq!(next.attempts, 1);
        assert_eq!(next.failure_reason.as_deref(), Some("boom"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_fail_terminal_at_max_attempts() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid_w".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        for _ in 0..3 {
            store.claim(None).await.unwrap();
            store.fail(id, "boom".to_string(), 3).await.unwrap();
        }
        assert!(store.claim(None).await.unwrap().is_none());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_list_failed_return_only_failed_rows_newest_first() {
        let store = StubJobsStore::default();
        // One row that reaches failed terminal at max_attempts=1.
        let id_a = store
            .enqueue(JobKind::Extract, "pid_a".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id_a, "boom-a".to_string(), 1).await.unwrap();

        // Another failed row.
        let id_b = store
            .enqueue(JobKind::Extract, "pid_b".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id_b, "boom-b".to_string(), 1).await.unwrap();

        // One pending row (should not appear in list_failed).
        let _ = store
            .enqueue(JobKind::Embed, "pid_pending".to_string(), serde_json::json!({}))
            .await
            .unwrap();

        let failed = store.list_failed(10).await.unwrap();
        assert_eq!(failed.len(), 2);
        // Newest first: id_b was failed after id_a.
        assert_eq!(failed[0].id, id_b);
        assert_eq!(failed[0].source_pid, "pid_b");
        assert_eq!(failed[0].failure_reason.as_deref(), Some("boom-b"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_list_failed_respect_limit() {
        let store = StubJobsStore::default();
        for i in 0..3 {
            let id = store
                .enqueue(JobKind::Embed, format!("pid_{i}"), serde_json::json!({}))
                .await
                .unwrap();
            store.claim(None).await.unwrap();
            store.fail(id, "boom".to_string(), 1).await.unwrap();
        }
        let failed = store.list_failed(2).await.unwrap();
        assert_eq!(failed.len(), 2);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_retry_job_clear_attempts_and_pend() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id, "boom".to_string(), 1).await.unwrap();

        store.retry_job(id).await.unwrap();
        let claimed = store
            .claim(None)
            .await
            .unwrap()
            .expect("retried job should be claimable");
        assert_eq!(claimed.id, id);
        assert_eq!(claimed.attempts, 0, "retry resets attempts to zero");
        assert!(claimed.failure_reason.is_none(), "retry clears failure reason");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_retry_job_return_not_found_when_id_missing() {
        let store = StubJobsStore::default();
        let err = store.retry_job(999).await.unwrap_err();
        assert!(matches!(err, JobsError::NotFound(_)));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bulk_retry_with_kind_filter() {
        let store = StubJobsStore::default();
        let id_e = store
            .enqueue(JobKind::Embed, "pid_e".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id_e, "boom".to_string(), 1).await.unwrap();

        let id_x = store
            .enqueue(JobKind::Extract, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id_x, "boom".to_string(), 1).await.unwrap();

        let affected = store.bulk_retry(Some(JobKind::Extract), false).await.unwrap();
        assert_eq!(affected, 1, "only the extract row should be affected");

        // Embed row stays failed; extract row is pending.
        let failed = store.list_failed(10).await.unwrap();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].id, id_e);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bulk_retry_dry_run_count_without_modifying() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id, "boom".to_string(), 1).await.unwrap();

        let affected = store.bulk_retry(None, true).await.unwrap();
        assert_eq!(affected, 1, "dry_run should still report the count");

        // Row should still be in failed state.
        let failed = store.list_failed(10).await.unwrap();
        assert_eq!(failed.len(), 1, "dry_run must NOT modify rows");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_delete_failed_remove_row() {
        let store = StubJobsStore::default();
        let id = store
            .enqueue(JobKind::Embed, "pid".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store.claim(None).await.unwrap();
        store.fail(id, "boom".to_string(), 1).await.unwrap();

        store.delete_failed(id).await.unwrap();
        let failed = store.list_failed(10).await.unwrap();
        assert!(failed.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_delete_failed_return_not_found_when_id_missing() {
        let store = StubJobsStore::default();
        let err = store.delete_failed(999).await.unwrap_err();
        assert!(matches!(err, JobsError::NotFound(_)));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_pending_count_reflect_queue_state() {
        let store = StubJobsStore::default();
        assert_eq!(store.pending_count().await.unwrap(), 0);

        let _ = store
            .enqueue(JobKind::Embed, "pid_a".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let _ = store
            .enqueue(JobKind::Embed, "pid_b".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        assert_eq!(store.pending_count().await.unwrap(), 2);

        store.claim(None).await.unwrap();
        // After claim: 1 pending + 1 claimed.
        assert_eq!(store.pending_count().await.unwrap(), 1);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_enqueue_synthesis_when_no_siblings_remain() {
        let store = StubJobsStore::default();
        let inserted = store.enqueue_synthesis_if_ready("pid_done").await.unwrap();
        assert!(inserted, "no sibling rows -> synthesis fires");
        let claimed = store.claim(None).await.unwrap().expect("synthesize row");
        assert_eq!(claimed.kind, JobKind::Synthesize);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_not_enqueue_synthesis_while_extract_sibling_present() {
        let store = StubJobsStore::default();
        store
            .enqueue(JobKind::Extract, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let inserted = store.enqueue_synthesis_if_ready("pid_x").await.unwrap();
        assert!(!inserted, "extract sibling still present -> no synthesis");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_not_enqueue_synthesis_while_relational_sibling_present() {
        let store = StubJobsStore::default();
        store
            .enqueue(JobKind::RelationalExtract, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let inserted = store.enqueue_synthesis_if_ready("pid_x").await.unwrap();
        assert!(!inserted, "relational sibling still present -> no synthesis");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_enqueue_synthesis_exactly_once_regardless_of_order() {
        // Two sibling completions both call the guard; exactly one inserts.
        let store = StubJobsStore::default();
        let first = store.enqueue_synthesis_if_ready("pid_x").await.unwrap();
        let second = store.enqueue_synthesis_if_ready("pid_x").await.unwrap();
        assert!(first, "first call inserts");
        assert!(!second, "second call sees the existing synthesize row -> no duplicate");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_ignore_unrelated_kinds_when_checking_readiness() {
        // A pending embed/categorize for the same source must NOT block synthesis.
        let store = StubJobsStore::default();
        store
            .enqueue(JobKind::Embed, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        store
            .enqueue(JobKind::Categorize, "pid_x".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let inserted = store.enqueue_synthesis_if_ready("pid_x").await.unwrap();
        assert!(inserted, "embed/categorize are not synthesis parents -> synthesis fires");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_reset_expired_leases_re_pend_old_claims() {
        let store = StubJobsStore::default();
        let _ = store
            .enqueue(JobKind::Embed, "pid_a".to_string(), serde_json::json!({}))
            .await
            .unwrap();
        let _ = store.claim(None).await.unwrap();

        // Simulate a stale claim by rewinding claimed_at.
        {
            let mut rows = store.rows.lock().unwrap();
            let claimed_at: chrono::DateTime<chrono::FixedOffset> =
                (Utc::now() - chrono::Duration::seconds(120)).into();
            rows[0].claimed_at = Some(claimed_at);
        }

        let recovered = store.reset_expired_leases(Duration::from_secs(30)).await.unwrap();
        assert_eq!(recovered, 1);
        let next = store.claim(None).await.unwrap().expect("row should be claimable again");
        assert_eq!(next.state, JobState::Claimed);
    }
}
