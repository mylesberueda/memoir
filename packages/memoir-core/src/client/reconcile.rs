//! Per-call builder for [`Client::reconcile`].

use std::collections::HashSet;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

use tracing::{Level, event};

use crate::store::MemoryStore;
use crate::vector::VectorIndex;

use super::{Client, ClientError};

/// Default page size for the failed-row retry pass.
pub const DEFAULT_FAILED_BATCH: usize = 100;

/// Default Qdrant scroll page size for the orphan-cleanup pass.
pub const DEFAULT_SCROLL_PAGE_SIZE: usize = 256;

/// Summary of one reconciliation invocation.
///
/// Returned by awaiting [`ReconcileBuilder`]. Counts reflect work performed,
/// not work attempted — a row that failed to embed twice in a row counts
/// once in `failed_retried` (the retry was attempted) and zero times in
/// `failed_recovered` (the retry did not succeed).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReconcileSummary {
    /// Number of `failed` rows the retry pass attempted to re-embed.
    pub failed_retried: usize,

    /// Number of `failed` rows the retry pass moved into `indexed`.
    pub failed_recovered: usize,

    /// Number of orphan vectors deleted from the index.
    pub orphans_deleted: usize,
}

/// Per-call builder returned by [`Client::reconcile`].
///
/// Awaiting the builder runs the configured passes. By default both passes
/// run; toggle methods narrow the work.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
/// let summary = client.reconcile().await?;
/// println!(
///     "retried {} failed, recovered {}, deleted {} orphans",
///     summary.failed_retried, summary.failed_recovered, summary.orphans_deleted,
/// );
/// # Ok(())
/// # }
/// ```
#[must_use = "reconcile() returns a builder that must be awaited"]
pub struct ReconcileBuilder<'a> {
    client: &'a Client,
    retry_failed: bool,
    clean_orphans: bool,
    failed_batch: usize,
    scroll_page_size: usize,
}

impl<'a> ReconcileBuilder<'a> {
    pub(super) fn new(client: &'a Client) -> Self {
        Self {
            client,
            retry_failed: true,
            clean_orphans: true,
            failed_batch: DEFAULT_FAILED_BATCH,
            scroll_page_size: DEFAULT_SCROLL_PAGE_SIZE,
        }
    }

    /// Runs only the failed-row retry pass; skips orphan cleanup.
    pub fn only_retry_failed(mut self) -> Self {
        self.retry_failed = true;
        self.clean_orphans = false;
        self
    }

    /// Runs only the orphan-cleanup pass; skips failed-row retry.
    pub fn only_clean_orphans(mut self) -> Self {
        self.retry_failed = false;
        self.clean_orphans = true;
        self
    }

    /// Caps the number of `failed` rows considered in one invocation.
    pub fn failed_batch(mut self, batch: usize) -> Self {
        self.failed_batch = batch;
        self
    }

    /// Sets the Qdrant scroll page size for the orphan-cleanup pass.
    pub fn scroll_page_size(mut self, page_size: usize) -> Self {
        self.scroll_page_size = page_size;
        self
    }
}

impl<'a> IntoFuture for ReconcileBuilder<'a> {
    type Output = Result<ReconcileSummary, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: ReconcileBuilder<'_>) -> Result<ReconcileSummary, ClientError> {
    let ReconcileBuilder {
        client,
        retry_failed,
        clean_orphans,
        failed_batch,
        scroll_page_size,
    } = builder;

    let inner = client.inner.clone();
    let mut summary = ReconcileSummary::default();

    if retry_failed {
        let rows = inner.store.find_failed(failed_batch).await?;
        summary.failed_retried = rows.len();
        for row in rows {
            if matches!(
                inner.embed_and_index(row).await,
                super::embed::EmbedOutcome::Indexed
            ) {
                summary.failed_recovered += 1;
            }
        }
        event!(
            name: "memoir.reconcile.retry_failed_complete",
            Level::INFO,
            retried = summary.failed_retried,
            recovered = summary.failed_recovered,
            "retried {{retried}}, recovered {{recovered}}",
        );
    }

    if clean_orphans {
        let scopes = inner.store.list_scopes().await?;
        for scope in scopes {
            let postgres_pids: HashSet<String> = inner
                .store
                .indexed_pids_in_scope(&scope)
                .await?
                .into_iter()
                .collect();
            let index_pids = inner
                .index
                .list_pids_in_scope(scope.clone(), scroll_page_size)
                .await?;
            let orphans: Vec<&str> = index_pids
                .iter()
                .filter(|pid| !postgres_pids.contains(pid.as_str()))
                .map(String::as_str)
                .collect();

            if orphans.is_empty() {
                continue;
            }

            let count = orphans.len();
            if let Err(err) = inner.index.delete_by_pids(&orphans).await {
                event!(
                    name: "memoir.reconcile.orphan_delete_failed",
                    Level::WARN,
                    count = count,
                    error = %err,
                    "orphan delete failed for {{count}} pid(s) — will retry on next sweep",
                );
            } else {
                summary.orphans_deleted += count;
            }
        }
        event!(
            name: "memoir.reconcile.orphans_complete",
            Level::INFO,
            deleted = summary.orphans_deleted,
            "deleted {{deleted}} orphans",
        );
    }

    Ok(summary)
}
