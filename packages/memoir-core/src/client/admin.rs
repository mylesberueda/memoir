//! Admin surface for inspecting and triaging the write-behind queue.
//!
//! These methods exist on [`super::Client`] (top-level, not behind a
//! sub-handle) because v0.1 ships only a small number. If the admin surface
//! grows past ~5 methods, group them under a `Client::admin()` sub-handle
//! to keep the primary API uncluttered.
//!
//! ## Trust boundary
//!
//! memoir-core treats the caller as the trust boundary. These methods
//! perform privileged operations (mass retry, deletion) with no caller
//! identity check. Service-mode consumers (epic 0007) gate access via
//! their own auth layer before reaching these methods.

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use tracing::{Level, event};

use crate::jobs::{JobKind, MemoryJobsStore};

use super::{Client, ClientError};

/// Per-call builder returned by [`Client::retry_failed_jobs`].
///
/// Awaiting the builder runs the bulk-retry operation against the configured
/// filters. Returns the number of jobs that were affected (or, with
/// `.dry_run(true)`, would have been affected).
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # use memoir_core::jobs::JobKind;
/// # async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// // Retry every failed extract job.
/// let n = client.retry_failed_jobs().of_kind(JobKind::Extract).await?;
/// println!("retried {n} extract jobs");
///
/// // Dry-run: preview the count without enqueueing anything.
/// let n = client.retry_failed_jobs().dry_run().await?;
/// println!("would retry {n} failed jobs");
/// # Ok(())
/// # }
/// ```
#[must_use = "retry_failed_jobs() returns a builder that must be awaited"]
pub struct RetryBuilder<'a> {
    client: &'a Client,
    kind: Option<JobKind>,
    dry_run: bool,
}

impl<'a> RetryBuilder<'a> {
    pub(super) fn new(client: &'a Client) -> Self {
        Self {
            client,
            kind: None,
            dry_run: false,
        }
    }

    /// Restricts the bulk retry to one job kind. Default: all kinds.
    pub fn of_kind(mut self, kind: JobKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Returns the affected count without modifying any rows.
    ///
    /// Useful for previewing how large a bulk retry will be before firing
    /// it — a wide retry against many failed extract jobs can DoS the LLM.
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }
}

impl<'a> IntoFuture for RetryBuilder<'a> {
    type Output = Result<u64, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: RetryBuilder<'_>) -> Result<u64, ClientError> {
    let RetryBuilder { client, kind, dry_run } = builder;

    let affected = client
        .inner
        .jobs
        .bulk_retry(kind, dry_run)
        .await?;

    event!(
        name: "memoir.admin.bulk_retry",
        Level::INFO,
        affected = affected,
        dry_run = dry_run,
        kind = kind.as_ref().map(|k| k.as_ref()).unwrap_or("any"),
        "bulk retry affected={{affected}} dry_run={{dry_run}} kind={{kind}}",
    );

    Ok(affected)
}
