//! In-library worker that drains the `memory_jobs` queue.
//!
//! The worker is a detached tokio task launched via
//! [`super::Client::spawn_worker`]. It polls the queue, dispatches each job
//! to its stage handler, and completes-or-fails the job in the store. The
//! handler dispatch is a no-op in this scaffolding pass; tickets 0006
//! (extract) and 0007 (embed) replace the placeholder with real work.
//!
//! Shutdown is cooperative. Sending the shutdown signal lets the worker
//! finish its current job before exiting. A drain timeout caps how long the
//! caller is willing to wait.

use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, Level, event, info_span};

use crate::jobs::{Job, JobKind, JobState, MemoryJobsStore};

use super::{Client, ClientError, ClientInner};

/// Default interval between empty-queue polls.
///
/// One second balances responsiveness against idle CPU. Lower values make
/// newly-enqueued work pick up faster but waste CPU when the queue is idle;
/// higher values are friendlier to laptops + cheaper hosts.
pub const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Default lease duration; claims older than this get recovered.
///
/// Sixty seconds is long enough that a healthy worker's claim won't be
/// stolen mid-work, short enough that a crashed worker's claim recovers
/// quickly. Tune up if extraction LLM calls regularly exceed this; tune
/// down if rapid recovery matters more than tolerating slow operations.
pub const DEFAULT_LEASE_DURATION: Duration = Duration::from_secs(60);

/// Default max retry count before a job moves to `failed`.
///
/// Three attempts catches transient failures (network blips, momentary LLM
/// rate limits) without amplifying systemic ones. Operators raise this only
/// when working with provider tiers that have heavy throttling.
pub const DEFAULT_MAX_ATTEMPTS: i32 = 3;

/// Default cap on graceful drain after `.shutdown()` is called.
///
/// Long enough to let a typical extraction job finish (LLM + DB writes
/// usually <10s), short enough that a hung worker doesn't block server
/// shutdown indefinitely.
pub const DEFAULT_DRAIN_TIMEOUT: Duration = Duration::from_secs(30);

/// Per-call builder returned by [`Client::spawn_worker`].
///
/// Configure via toggle methods, then call [`Self::start`] to spawn the
/// worker task. Returns a [`WorkerHandle`] the caller uses to observe and
/// shut down the worker.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// let worker = client.spawn_worker().start().await?;
/// // ... server runs ...
/// worker.shutdown().await;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
#[must_use = "spawn_worker() returns a builder; call .start() to launch the task"]
pub struct WorkerBuilder<'a> {
    client: &'a Client,
    poll_interval: Duration,
    lease_duration: Duration,
    max_attempts: i32,
    drain_timeout: Duration,
    claimed_by: Option<String>,
}

impl<'a> WorkerBuilder<'a> {
    pub(super) fn new(client: &'a Client) -> Self {
        Self {
            client,
            poll_interval: DEFAULT_POLL_INTERVAL,
            lease_duration: DEFAULT_LEASE_DURATION,
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            drain_timeout: DEFAULT_DRAIN_TIMEOUT,
            claimed_by: None,
        }
    }

    /// Interval between polls when the queue is empty. Default 1 second.
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Lease duration for in-flight claims. Default 60 seconds.
    ///
    /// A worker that crashes before completing a job leaves the row in
    /// `claimed` state with stale `claimed_at`. The lease-recovery sweep
    /// re-pends rows older than this duration.
    pub fn lease_duration(mut self, lease: Duration) -> Self {
        self.lease_duration = lease;
        self
    }

    /// Maximum failed attempts before a job moves to terminal `failed`.
    /// Default 3.
    pub fn max_attempts(mut self, max: i32) -> Self {
        self.max_attempts = max;
        self
    }

    /// Maximum time `.shutdown()` waits for the current job to finish.
    /// Default 30 seconds. After this, the task is aborted.
    pub fn drain_timeout(mut self, timeout: Duration) -> Self {
        self.drain_timeout = timeout;
        self
    }

    /// Identifier persisted on each claim's `claimed_by` column.
    ///
    /// Default `None`. Useful when multiple workers share a queue — e.g.
    /// `hostname-pid` lets operators identify which process holds a stale
    /// lease.
    pub fn claimed_by(mut self, id: impl Into<String>) -> Self {
        self.claimed_by = Some(id.into());
        self
    }

    /// Spawns the worker task and returns a handle.
    ///
    /// # Errors
    ///
    /// Currently infallible (returns `Ok` unconditionally); the `Result`
    /// signature reserves room for startup-time validation that downstream
    /// tickets (0010 LLM config) may add.
    pub async fn start(self) -> Result<WorkerHandle, ClientError> {
        let WorkerBuilder {
            client,
            poll_interval,
            lease_duration,
            max_attempts,
            drain_timeout,
            claimed_by,
        } = self;

        let token = CancellationToken::new();
        let inner = client.inner.clone();
        let config = WorkerConfig {
            poll_interval,
            lease_duration,
            max_attempts,
            claimed_by,
        };

        let span = info_span!("memoir.worker");
        let token_for_task = token.clone();
        let join = tokio::spawn(
            async move { run_worker(inner, config, token_for_task).await }.instrument(span),
        );

        Ok(WorkerHandle {
            inner: Arc::new(WorkerHandleInner {
                join: tokio::sync::Mutex::new(Some(join)),
                token,
                drain_timeout,
            }),
        })
    }
}

/// Handle to a running worker task.
///
/// Cheap to clone — internally `Arc`'d so multiple call sites can hold
/// references. Dropping the last clone does NOT trigger shutdown; callers
/// should explicitly invoke [`Self::shutdown`] on graceful-stop paths.
#[derive(Clone)]
pub struct WorkerHandle {
    inner: Arc<WorkerHandleInner>,
}

impl std::fmt::Debug for WorkerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkerHandle")
            .field("is_shutting_down", &self.inner.token.is_cancelled())
            .field("drain_timeout", &self.inner.drain_timeout)
            .finish_non_exhaustive()
    }
}

struct WorkerHandleInner {
    join: tokio::sync::Mutex<Option<JoinHandle<()>>>,
    token: CancellationToken,
    drain_timeout: Duration,
}

impl WorkerHandle {
    /// Returns `true` if the worker has been signaled to stop.
    #[must_use]
    pub fn is_shutting_down(&self) -> bool {
        self.inner.token.is_cancelled()
    }

    /// Returns a child [`CancellationToken`] tied to the worker's lifecycle.
    ///
    /// Child tokens are cancelled when the worker itself is shut down. Useful
    /// when downstream subtasks want to share the same shutdown semantics.
    ///
    /// `CancellationToken` is leaked here from `tokio-util` deliberately:
    /// it is the de-facto-standard cooperative cancellation primitive in the
    /// tokio ecosystem, and exposing it gives consumers direct
    /// interoperability with the rest of their async code.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.inner.token.child_token()
    }

    /// Signals the worker to stop and waits for it to drain.
    ///
    /// The worker finishes its current job (if any), declines to claim a new
    /// one, and exits. If the drain timeout elapses first, the task is
    /// aborted and any in-flight claim leaks until the lease expires.
    ///
    /// Calling `shutdown` more than once is safe — subsequent calls observe
    /// the already-shut-down state and return immediately.
    pub async fn shutdown(&self) {
        self.inner.token.cancel();

        let mut guard = self.inner.join.lock().await;
        let Some(join) = guard.take() else {
            return;
        };

        match timeout(self.inner.drain_timeout, join).await {
            Ok(Ok(())) => {
                event!(
                    name: "memoir.worker.shutdown",
                    Level::INFO,
                    outcome = "drained",
                    "worker shutdown {{outcome}}",
                );
            }
            Ok(Err(err)) => {
                event!(
                    name: "memoir.worker.shutdown",
                    Level::WARN,
                    outcome = "join_failed",
                    error.message = %err,
                    "worker shutdown {{outcome}}: {{error.message}}",
                );
            }
            Err(_) => {
                event!(
                    name: "memoir.worker.shutdown",
                    Level::WARN,
                    outcome = "timeout",
                    "worker shutdown {{outcome}} (drain deadline exceeded; task continues until natural exit)",
                );
                // Note: we can't abort here because we already took the
                // JoinHandle out of the Option. The task continues until it
                // naturally exits or the runtime drops. Consumers who need
                // hard-abort semantics should call `.abort()` explicitly.
            }
        }
    }

    /// Aborts the worker task without waiting for graceful drain.
    ///
    /// In-flight claims leak until their lease expires. Prefer
    /// [`Self::shutdown`] except in emergency shutdown paths.
    pub async fn abort(&self) {
        self.inner.token.cancel();
        let mut guard = self.inner.join.lock().await;
        if let Some(join) = guard.take() {
            join.abort();
            event!(
                name: "memoir.worker.aborted",
                Level::WARN,
                outcome = "aborted",
                "worker {{outcome}}",
            );
        }
    }
}

#[derive(Clone)]
struct WorkerConfig {
    poll_interval: Duration,
    lease_duration: Duration,
    max_attempts: i32,
    claimed_by: Option<String>,
}

async fn run_worker(inner: Arc<ClientInner>, config: WorkerConfig, token: CancellationToken) {
    // `as_millis()` returns u128; cap at u64::MAX since tracing event fields
    // accept u64 and durations beyond ~584 million years aren't a real concern.
    let poll_interval_ms = u64::try_from(config.poll_interval.as_millis()).unwrap_or(u64::MAX);
    event!(
        name: "memoir.worker.started",
        Level::INFO,
        poll_interval_ms = poll_interval_ms,
        lease_secs = config.lease_duration.as_secs(),
        max_attempts = config.max_attempts,
        "worker started: poll_interval={{poll_interval_ms}}ms lease={{lease_secs}}s max_attempts={{max_attempts}}",
    );

    while !token.is_cancelled() {
        let claimed_by = config.claimed_by.as_deref();
        let claim_result = inner.jobs.claim(claimed_by).await;

        match claim_result {
            Ok(Some(job)) => {
                dispatch(&inner, job, config.max_attempts).await;
            }
            Ok(None) => {
                // Queue empty: recover expired leases, then wait.
                match inner.jobs.reset_expired_leases(config.lease_duration).await {
                    Ok(0) => {}
                    Ok(n) => {
                        event!(
                            name: "memoir.worker.lease_recovered",
                            Level::INFO,
                            count = n,
                            "recovered {{count}} expired lease(s)",
                        );
                    }
                    Err(err) => {
                        event!(
                            name: "memoir.worker.lease_recovery_failed",
                            Level::WARN,
                            error.message = %err,
                            "lease recovery failed: {{error.message}}",
                        );
                    }
                }

                wait_or_cancel(&token, config.poll_interval).await;
            }
            Err(err) => {
                event!(
                    name: "memoir.worker.claim_failed",
                    Level::WARN,
                    error.message = %err,
                    "claim failed: {{error.message}}; backing off",
                );
                wait_or_cancel(&token, config.poll_interval).await;
            }
        }
    }

    event!(
        name: "memoir.worker.exited",
        Level::INFO,
        outcome = "exited",
        "worker loop {{outcome}}",
    );
}

/// Sleeps for `dur` or returns immediately when the token is cancelled.
async fn wait_or_cancel(token: &CancellationToken, dur: Duration) {
    tokio::select! {
        _ = sleep(dur) => {}
        _ = token.cancelled() => {}
    }
}

/// Dispatches one claimed job. No-op in this ticket — completes immediately.
///
/// Tickets 0006 (extract) and 0007 (embed) replace this body with real
/// stage handlers nested under per-job spans.
async fn dispatch(inner: &Arc<ClientInner>, job: Job, max_attempts: i32) {
    debug_assert_eq!(job.state, JobState::Claimed);

    let job_span = info_span!(
        "memoir.worker.job",
        job_id = job.id,
        kind = %job.kind,
        source_pid = %job.source_pid,
    );
    let _enter = job_span.enter();

    event!(
        name: "memoir.worker.job_started",
        Level::DEBUG,
        outcome = "claimed",
        "job {{outcome}}",
    );

    let result: Result<(), String> = match job.kind {
        JobKind::Extract => inner
            .run_extract(job.clone())
            .await
            .map_err(|err| err.to_string()),
        // Embed dispatch is the substrate-swap target. Until ticket 0007 lands
        // the swap, embed jobs are a no-op (the in-process spawn at
        // `Client::remember` continues to handle freshly-written episodic
        // rows). Newly-extracted semantic rows enqueue embed jobs that this
        // arm currently drains without doing real work.
        JobKind::Embed => Ok(()),
    };

    match result {
        Ok(()) => match inner.jobs.complete(job.id).await {
            Ok(()) => event!(
                name: "memoir.worker.job_succeeded",
                Level::INFO,
                outcome = "succeeded",
                "job {{outcome}}",
            ),
            Err(err) => event!(
                name: "memoir.worker.complete_failed",
                Level::WARN,
                error.message = %err,
                "complete failed after successful dispatch: {{error.message}}",
            ),
        },
        Err(reason) => {
            if let Err(fail_err) = inner.jobs.fail(job.id, reason.clone(), max_attempts).await {
                event!(
                    name: "memoir.worker.fail_failed",
                    Level::WARN,
                    error.message = %fail_err,
                    "fail call itself failed: {{error.message}}",
                );
            } else {
                event!(
                    name: "memoir.worker.job_failed",
                    Level::WARN,
                    error.message = %reason,
                    "job failed: {{error.message}}",
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // M-TYPES-SEND: public types must be `Send` so they compose with tokio.
    const fn assert_send<T: Send>() {}
    const _: () = assert_send::<WorkerHandle>();

    #[test]
    fn should_use_default_constants_for_builder() {
        // Sanity-check the defaults are sensible — fast enough for tests,
        // not so fast they pin a CPU.
        assert_eq!(DEFAULT_POLL_INTERVAL, Duration::from_secs(1));
        assert_eq!(DEFAULT_LEASE_DURATION, Duration::from_secs(60));
        assert_eq!(DEFAULT_MAX_ATTEMPTS, 3);
        assert_eq!(DEFAULT_DRAIN_TIMEOUT, Duration::from_secs(30));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_wait_or_cancel_complete_when_uncancelled() {
        let token = CancellationToken::new();
        let start = std::time::Instant::now();
        wait_or_cancel(&token, Duration::from_millis(10)).await;
        assert!(
            start.elapsed() >= Duration::from_millis(10),
            "expected ~10ms sleep without cancellation"
        );
        assert!(!token.is_cancelled());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_wait_or_cancel_return_immediately_when_cancelled() {
        let token = CancellationToken::new();
        token.cancel();

        let start = std::time::Instant::now();
        // Long timeout: would block forever if cancellation wasn't observed.
        wait_or_cancel(&token, Duration::from_secs(60)).await;
        assert!(
            start.elapsed() < Duration::from_millis(100),
            "cancellation should wake us nearly instantly"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_worker_handle_track_shutdown_state() {
        let token = CancellationToken::new();
        let join = tokio::spawn(async {});
        let handle = WorkerHandle {
            inner: Arc::new(WorkerHandleInner {
                join: tokio::sync::Mutex::new(Some(join)),
                token: token.clone(),
                drain_timeout: Duration::from_secs(1),
            }),
        };

        assert!(!handle.is_shutting_down());
        token.cancel();
        assert!(handle.is_shutting_down());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_child_token_inherit_cancellation_from_parent() {
        let token = CancellationToken::new();
        let join = tokio::spawn(async {});
        let handle = WorkerHandle {
            inner: Arc::new(WorkerHandleInner {
                join: tokio::sync::Mutex::new(Some(join)),
                token: token.clone(),
                drain_timeout: Duration::from_secs(1),
            }),
        };

        let child = handle.cancellation_token();
        assert!(!child.is_cancelled());
        token.cancel();
        assert!(child.is_cancelled(), "child should observe parent cancellation");
    }
}
