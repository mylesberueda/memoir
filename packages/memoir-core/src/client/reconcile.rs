//! Per-call builder for [`Client::reconcile`].

use std::collections::HashSet;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

use tracing::{Level, event};

use crate::memory::Scope;
use crate::store::MemoryStore;
use crate::vector::VectorIndex;

use super::{Client, ClientError};

/// Default page size for the failed-row retry pass.
pub const DEFAULT_FAILED_BATCH: usize = 100;

/// Default Qdrant scroll page size for the orphan-cleanup pass.
pub const DEFAULT_SCROLL_PAGE_SIZE: usize = 256;

/// Default page size for the episodic scan of the graph-rebuild pass.
pub const DEFAULT_REBUILD_PAGE_SIZE: usize = 256;

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

    /// Number of relational-extract jobs the graph-rebuild pass enqueued.
    ///
    /// Counts jobs *enqueued*, not triples committed: the rebuild re-feeds the
    /// worker pipeline (extract → synthesize → commit), which drains
    /// asynchronously after this call returns. A repopulated graph requires the
    /// worker to be running. Zero unless [`ReconcileBuilder::rebuild_graph`] was
    /// set (the pass is opt-in).
    pub graph_rebuild_enqueued: usize,
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
    rebuild_scope: Option<Scope>,
    rebuild_page_size: usize,
}

impl<'a> ReconcileBuilder<'a> {
    pub(super) fn new(client: &'a Client) -> Self {
        Self {
            client,
            retry_failed: true,
            clean_orphans: true,
            failed_batch: DEFAULT_FAILED_BATCH,
            scroll_page_size: DEFAULT_SCROLL_PAGE_SIZE,
            rebuild_scope: None,
            rebuild_page_size: DEFAULT_REBUILD_PAGE_SIZE,
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

    /// Rebuilds `scope`'s knowledge graph from its episodic memories.
    ///
    /// Opt-in (off by default): a plain [`Client::reconcile`] runs only the
    /// vector passes and never touches the graph. This pass **wipes** the named
    /// scope's subgraph, then re-enqueues relational extraction for each
    /// episodic memory so the worker pipeline repopulates it — the recovery path
    /// after FalkorDB data loss. It is asynchronous: the summary counts jobs
    /// enqueued, and the graph is whole only once the worker drains them.
    ///
    /// Requires the `knowledge-graph` feature and a configured graph; without a
    /// graph it is a no-op. Confined to the one named scope — there is no
    /// all-scope rebuild, since a wipe across every tenant would be too blunt a
    /// default.
    #[cfg(feature = "knowledge-graph")]
    pub fn rebuild_graph(mut self, scope: Scope) -> Self {
        self.rebuild_scope = Some(scope);
        self
    }

    /// Sets the episodic-scan page size for the graph-rebuild pass.
    #[cfg(feature = "knowledge-graph")]
    pub fn rebuild_page_size(mut self, page_size: usize) -> Self {
        self.rebuild_page_size = page_size;
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
        rebuild_scope,
        rebuild_page_size,
    } = builder;

    // The rebuild fields are only consumed under the knowledge-graph feature;
    // bind them as used in vector-only builds so they carry no dead-code weight.
    #[cfg(not(feature = "knowledge-graph"))]
    let _ = (&rebuild_scope, rebuild_page_size);

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
                    error.message = %err,
                    "orphan delete failed for {{count}} pid(s): {{error.message}} — will retry on next sweep",
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

    #[cfg(feature = "knowledge-graph")]
    if let Some(scope) = rebuild_scope {
        summary.graph_rebuild_enqueued = rebuild_graph(&inner, scope, rebuild_page_size).await?;
    }

    Ok(summary)
}

/// Wipes `scope`'s graph and re-enqueues relational extraction per episodic memory.
///
/// Returns the number of `RelationalExtract` jobs enqueued. A no-op (returns 0)
/// when no graph is configured. The wipe is best-effort-logged like
/// [`Client::forget`]'s graph block — a wipe failure does not abort the
/// re-enqueue, since `commit_triples` is idempotent and a stale edge is less
/// harmful than skipping the rebuild entirely.
#[cfg(feature = "knowledge-graph")]
async fn rebuild_graph(
    inner: &std::sync::Arc<super::ClientInner>,
    scope: Scope,
    page_size: usize,
) -> Result<usize, ClientError> {
    use crate::graph::GraphStore;
    use crate::jobs::{JobKind, MemoryJobsStore};
    use crate::memory::KindSelector;
    use crate::store::TimelineParams;

    let Some(graph) = inner.graph.as_deref() else {
        return Ok(0);
    };

    if let Err(err) = graph.forget_scope(&scope).await {
        event!(
            name: "memoir.reconcile.rebuild_wipe_failed",
            Level::WARN,
            error.message = %err,
            "graph wipe before rebuild failed: {{error.message}} — rebuilding over existing graph (commit is idempotent)",
        );
    }

    // Page through episodic memories newest-first. `created_before` is an
    // exclusive upper bound, so advancing the cursor to the oldest row seen would
    // *skip* any same-timestamp rows that fell on the next page. Instead the
    // cursor steps to just past the oldest — by one MICROSECOND, the precision
    // Postgres `timestamptz` actually stores and the wire protocol encodes; a
    // finer step truncates back to the same instant and the next page would
    // exclude that timestamp's remaining rows. The re-included rows dedup via a
    // seen-pid set. Re-enqueue is idempotent anyway, but skipping a memory would
    // silently drop its triples — so the bias is deliberately toward overlap,
    // never skip.
    let mut seen = HashSet::new();
    let mut cursor = None;
    let mut limit = page_size;
    loop {
        let params = TimelineParams {
            kinds: KindSelector {
                episodic: true,
                semantic: false,
            },
            created_before: cursor,
            include_superseded: true,
            limit,
            ..TimelineParams::default()
        };

        let page = inner.store.timeline(scope.clone(), params).await?;
        if page.is_empty() {
            break;
        }

        let page_len = page.len();
        let oldest = page.last().map(|memory| memory.created_at);
        for memory in page {
            if !seen.insert(memory.pid.clone()) {
                continue;
            }
            inner
                .jobs
                .enqueue(
                    JobKind::RelationalExtract,
                    memory.pid,
                    serde_json::json!({ "origin": "reconcile" }),
                )
                .await?;
        }

        // A short page is the last one. Otherwise step the cursor to just past
        // the oldest row so the next page re-includes that timestamp's rows; the
        // seen-set dedups them. A stalled cursor (a full page within one
        // timestamp) cannot advance past the tie group — and tie ordering has no
        // secondary sort, so a same-size refetch could return a different subset
        // and silently skip rows. Escalate the fetch limit instead, until one
        // page provably covers the whole group (page < limit), then stop.
        let next_cursor = oldest.map(|ts| ts + chrono::Duration::microseconds(1));
        if page_len < limit {
            break;
        }
        if next_cursor == cursor {
            limit *= 2;
            continue;
        }
        cursor = next_cursor;
        limit = page_size;
    }
    let enqueued = seen.len();

    event!(
        name: "memoir.reconcile.rebuild_complete",
        Level::INFO,
        enqueued = enqueued,
        "graph rebuild enqueued {{enqueued}} relational-extract job(s)",
    );

    Ok(enqueued)
}
