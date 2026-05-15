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
pub use types::{Job, JobKind, JobState};

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

    /// Atomically claims the oldest pending job and returns it, or `None`.
    ///
    /// Uses `SELECT FOR UPDATE SKIP LOCKED` (or equivalent) so concurrent
    /// callers never see the same row. The claim records `claimed_at` and
    /// optionally `claimed_by` for lease tracking.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn claim(
        &self,
        claimed_by: Option<&str>,
    ) -> impl Future<Output = Result<Option<Job>, JobsError>> + Send;

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
    fn fail(
        &self,
        id: i64,
        reason: String,
        max_attempts: i32,
    ) -> impl Future<Output = Result<(), JobsError>> + Send;

    /// Re-pends every claimed job whose lease has expired.
    ///
    /// Called by the worker on startup and periodically while idle, so a
    /// crashed worker's in-flight rows do not stay claimed forever. Returns
    /// the number of rows recovered.
    ///
    /// # Errors
    ///
    /// Returns [`JobsError::Database`] for database failures.
    fn reset_expired_leases(
        &self,
        lease: std::time::Duration,
    ) -> impl Future<Output = Result<u64, JobsError>> + Send;
}

// M-TYPES-SEND: public types must be `Send` so they compose with tokio.
const fn assert_send<T: Send>() {}
const _: () = {
    assert_send::<Job>();
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
            let Some(row) = rows.iter_mut().find(|r| r.id == id && r.state == JobState::Claimed)
            else {
                return Err(JobsError::NotFound(id.to_string()));
            };
            row.attempts += 1;
            row.failure_reason = Some(reason);
            row.claimed_at = None;
            row.claimed_by = None;
            row.state = if row.attempts >= max_attempts { JobState::Failed } else { JobState::Pending };
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
