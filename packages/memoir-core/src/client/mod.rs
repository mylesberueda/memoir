//! High-level facade composing the embedder, store, and vector index.

mod admin;
mod categorize;
mod edit;
mod embed;
mod error;
mod feedback;
mod extract;
mod query;
mod recall_as_of;
mod reconcile;
mod remember;
mod reprocess;
mod search;
mod timeline;
mod worker;

pub use admin::{ExtractionStatsBuilder, RetryBuilder};
pub use edit::EditBuilder;
pub use error::ClientError;
pub use feedback::FeedbackBuilder;
pub use query::{
    BlendWeights, DEFAULT_HYBRID_ALPHA, DEFAULT_HYBRID_HALF_LIFE_DAYS, DEFAULT_QUERY_LIMIT, DecayFn, MemoryContext,
    QueryBuilder, RankingStrategy,
};
pub use recall_as_of::RecallAsOfBuilder;
pub use reconcile::{ReconcileBuilder, ReconcileSummary};
pub use remember::{DEFAULT_SYSTEM_PROMPT, RememberBuilder};
pub use search::{DEFAULT_LIMIT, SearchBuilder};
pub use timeline::TimelineBuilder;
pub use worker::{
    DEFAULT_DRAIN_TIMEOUT, DEFAULT_LEASE_DURATION, DEFAULT_MAX_ATTEMPTS, DEFAULT_POLL_INTERVAL, WorkerBuilder,
    WorkerHandle,
};

use std::sync::Arc;

use bon::bon;
use qdrant_client::Qdrant;
use sea_orm::{ConnectOptions, Database};

use crate::embedding::{EmbeddingModel, OnnxEmbedding};
use crate::jobs::{MemoryJobsStore, PostgresJobsStore};
use crate::llm::{LlmConfig, LlmRegistry, LlmRole};
use crate::memory::{ForgetTarget, Memory, SupersessionEvent};
use crate::store::{MemoryStore, PostgresStore};
use crate::vector::{QdrantIndex, VectorIndex};

/// Shared internal state held by [`Client`] behind an `Arc`.
pub(crate) struct ClientInner {
    pub(crate) embedder: Arc<OnnxEmbedding>,
    pub(crate) store: PostgresStore,
    pub(crate) index: QdrantIndex,
    pub(crate) jobs: PostgresJobsStore,
    pub(crate) llms: LlmRegistry,
    /// Optional NLI classifier for the categorize stage (epic 0011).
    ///
    /// `None` when no classifier is configured — categorization is then a
    /// no-op and the extract stage skips enqueuing categorize jobs. Behind
    /// `Arc` because the classifier is `Send + Sync` and shared into the
    /// `spawn_blocking` inference task.
    pub(crate) nli: Option<Arc<crate::nli::NliClassifier>>,
    pub(crate) schema: String,
    pub(crate) system_prompt: Option<String>,
}

/// High-level facade composing the embedder, store, and vector index.
///
/// Constructed via [`Client::builder`]. Cheap to clone — internally backed by
/// `Arc` so multiple call sites can share one configured Memoir instance.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("schema", &self.inner.schema)
            .field("collection", &self.inner.index.collection_name())
            .finish_non_exhaustive()
    }
}

#[bon]
impl Client {
    /// Builds a [`Client`] from a Postgres connection string + Qdrant handle.
    ///
    /// memoir-core owns its own connection pool. The pool's `search_path` is
    /// pinned to the configured schema so memoir-core's tables and
    /// migration ledger never collide with the consumer's other Postgres
    /// state. The consumer never sees a [`sea_orm::DatabaseConnection`] — this is a
    /// deliberate boundary so the library can manage its own connection
    /// lifecycle (search_path, pool sizing, future read-replica routing)
    /// without each consumer reinventing the same plumbing.
    ///
    /// LLM providers are configured per [`LlmRole`] via the `extraction_llm`
    /// and `contradiction_llm` setters. A role left unconfigured produces a
    /// registry with no entry for that role; downstream call sites
    /// (e.g. the extraction worker stage) skip gracefully when their
    /// preferred role is absent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use memoir_core::client::Client;
    /// use memoir_core::llm::LlmConfig;
    ///
    /// let qdrant = qdrant_client::Qdrant::from_url("http://localhost:6334").build()?;
    ///
    /// let client = Client::builder()
    ///     .database_url("postgres://postgres:postgres@localhost:54321/my_app")
    ///     .qdrant(qdrant)
    ///     .schema("memoir")
    ///     .extraction_llm(LlmConfig::ollama("http://localhost:11434", "llama3.2"))
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Database`] when the pool cannot connect,
    /// [`ClientError::Embedding`] if the embedder fails to initialize,
    /// [`ClientError::Vector`] if `ensure_collection` fails on Qdrant, and
    /// [`ClientError::Llm`] if a configured provider can't be constructed
    /// (e.g. malformed URL or api-key rejected by rig).
    #[builder(start_fn = builder, finish_fn = build)]
    pub async fn new(
        #[builder(into)] database_url: String,
        qdrant: Qdrant,
        #[builder(into)] schema: Option<String>,
        #[builder(into)] system_prompt: Option<String>,
        #[builder(into)] collection: Option<String>,
        extraction_llm: Option<LlmConfig>,
        contradiction_llm: Option<LlmConfig>,
        categorize_model: Option<crate::nli::NliConfig>,
    ) -> Result<Client, ClientError> {
        let schema = schema.unwrap_or_else(|| crate::migration::DEFAULT_SCHEMA.to_string());

        // Pin the pool to memoir-core's schema. Every connection the pool
        // hands out — including ones sea-orm-migration grabs for
        // `Migrator::up` — resolves unqualified table names against this
        // path. Listing `public` second lets shared extensions (pgcrypto,
        // etc.) resolve.
        let search_path = format!("{schema},public");
        let options = ConnectOptions::new(database_url)
            .set_schema_search_path(search_path)
            .to_owned();
        let db = Database::connect(options).await.map_err(ClientError::Database)?;

        let embedder = OnnxEmbedding::new()?;
        let store = PostgresStore::new(db.clone());
        let jobs = PostgresJobsStore::new(db);
        let index = match collection {
            Some(name) => QdrantIndex::new(qdrant).with_collection(name),
            None => QdrantIndex::new(qdrant),
        };

        index.ensure_collection(embedder.dimensions()).await?;

        let mut llms = LlmRegistry::new();
        if let Some(config) = extraction_llm {
            llms.install(LlmRole::Extraction, config)?;
        }
        if let Some(config) = contradiction_llm {
            llms.install(LlmRole::Contradiction, config)?;
        }

        // Build the NLI classifier only when a model is configured — it
        // downloads an ~87MB model on first construction, so consumers who
        // don't want categorization shouldn't pay for it. `new()` is
        // sync-blocking (HF download), so it runs on the blocking pool to
        // avoid stalling the async runtime. Pass `NliConfig::default()` for
        // the model memoir ships with.
        let nli = if let Some(config) = categorize_model {
            let classifier = tokio::task::spawn_blocking(move || crate::nli::NliClassifier::new(config))
                .await
                .map_err(|join_err| ClientError::Nli(format!("classifier init task panicked: {join_err}")))?
                .map_err(|nli_err| ClientError::Nli(nli_err.to_string()))?;
            Some(Arc::new(classifier))
        } else {
            None
        };

        Ok(Client {
            inner: Arc::new(ClientInner {
                embedder: Arc::new(embedder),
                store,
                index,
                jobs,
                llms,
                nli,
                schema,
                system_prompt,
            }),
        })
    }
}

impl Client {
    /// Applies memoir-core's migrations in the configured Postgres schema.
    ///
    /// Idempotent — safe to call on every startup. Creates the schema if
    /// missing and applies all pending migrations.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Migration`] if the schema name is invalid or
    /// if any migration fails to apply.
    pub async fn migrate(&self) -> Result<(), ClientError> {
        crate::migration::bootstrap_and_migrate(self.inner.store.db(), &self.inner.schema).await?;
        Ok(())
    }

    /// Returns the Postgres schema this client writes its tables into.
    pub fn schema(&self) -> &str {
        &self.inner.schema
    }

    /// Returns the configured system-prompt section, if any.
    pub fn system_prompt(&self) -> Option<&str> {
        self.inner.system_prompt.as_deref()
    }

    /// Returns the Qdrant collection name configured for vector storage.
    pub fn collection_name(&self) -> &str {
        self.inner.index.collection_name()
    }

    /// Returns the embedding model used by this client.
    pub fn embedder(&self) -> &OnnxEmbedding {
        &self.inner.embedder
    }

    /// Returns the source-of-truth store used by this client.
    pub fn store(&self) -> &PostgresStore {
        &self.inner.store
    }

    /// Returns the vector index used by this client.
    pub fn index(&self) -> &QdrantIndex {
        &self.inner.index
    }

    /// Returns the registry of LLM providers configured on this client.
    pub fn llms(&self) -> &LlmRegistry {
        &self.inner.llms
    }

    /// Writes `prompt` as an episodic memory under `scope`.
    ///
    /// Returns a per-call builder; await it to persist the row and enqueue
    /// its embed (and, if extraction is configured, extract) job. The
    /// returned [`Memory`] reflects the source-of-truth row; its vector
    /// index entry is `pending` until the worker drains the embed job.
    /// Use [`Client::search`] to retrieve memories — `remember` is
    /// write-only.
    ///
    /// Attach optional JSON metadata via [`RememberBuilder::metadata`];
    /// without it the column defaults to `{}`. Attach a parsed event-time
    /// via [`RememberBuilder::event_at`] when the content references a
    /// specific moment (e.g. "the deployment happened Friday"); without it,
    /// the memory has no event-time and is excluded from event-time range
    /// filters at search time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # use memoir_core::memory::Scope;
    /// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
    /// let written = client
    ///     .remember("hello", scope)
    ///     .metadata(serde_json::json!({ "source": "chat" }))
    ///     .await?;
    /// println!("wrote pid={}", written.pid);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping a database failure when the
    /// row cannot be inserted, and [`ClientError::Jobs`] when the embed or
    /// extract job cannot be enqueued.
    pub fn remember(&self, prompt: impl Into<String>, scope: crate::memory::Scope) -> RememberBuilder<'_> {
        RememberBuilder::new(self, prompt.into(), scope)
    }

    /// Mutates an existing memory in place — a correction, not a supersession.
    ///
    /// Returns a per-call builder; await it to apply the patch. Use this when
    /// the original row was *wrong* and the caller is overwriting it (typo,
    /// misheard utterance, wrong parsed date). When the original was *true at
    /// the time* but new information now obsoletes it, use the contradiction
    /// path that calls `MemoryStore::supersede` instead — `edit` discards the
    /// old text, supersede preserves it.
    ///
    /// `created_at` is preserved; `updated_at` bumps automatically via the
    /// database trigger. Content changes flip the row's vector-index state
    /// back to `pending` and enqueue a re-embed job, so the row drops out of
    /// search hits until the worker drains the queue — same lifecycle as a
    /// fresh `remember()`. See [`EditBuilder`] for the builder methods.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::NotFound`] when no memory matches `pid`,
    /// [`crate::store::StoreError::UnsupportedEdit`] when the target row's
    /// kind does not support in-place edits (today: every non-Episodic
    /// kind), [`ClientError::ReservedMetadataKey`] when metadata contains a
    /// reserved payload key, and [`ClientError::Jobs`] when the re-embed
    /// job cannot be enqueued.
    pub fn edit(&self, pid: impl Into<String>) -> EditBuilder<'_> {
        EditBuilder::new(self, pid.into())
    }

    /// Searches indexed memories in `scope` by vector similarity to `query`.
    ///
    /// Returns a per-call builder; await it to embed the query, run the
    /// vector search, and assemble the matching [`Memory`] rows. The kind
    /// toggles on the builder filter retrieval. See [`SearchBuilder`] for
    /// builder methods.
    ///
    /// Only memories whose vector index entry has reached `indexed` are
    /// eligible. Rows still in `pending` (recently written via
    /// [`Client::remember`], not yet drained by the worker) are filtered
    /// out — they can still be inspected by pid via [`Client::recall`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # use memoir_core::memory::Scope;
    /// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
    /// let memories = client.search("hello", scope).limit(5).await?;
    /// for m in memories.list() {
    ///     println!("{}", m.content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Embedding`] if the query cannot be embedded,
    /// [`ClientError::Vector`] if the vector index search fails, and
    /// [`ClientError::Store`] wrapping a database failure when the matched
    /// pids cannot be hydrated to full rows.
    pub fn search(&self, query: impl Into<String>, scope: crate::memory::Scope) -> SearchBuilder<'_> {
        SearchBuilder::new(self, query.into(), scope)
    }

    /// Returns memories in `scope` ordered chronologically — the event log.
    ///
    /// Postgres-only read; no embedding, no Qdrant. Includes superseded rows
    /// by default (this is the audit view). Default order is newest-first,
    /// default limit is [`crate::store::DEFAULT_TIMELINE_LIMIT`]. See
    /// [`TimelineBuilder`] for the builder methods.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::InvalidScope`] when a `Scope` target has
    /// empty fields, or wrapping
    /// [`crate::store::StoreError::Database`] for database failures.
    pub fn timeline(&self, scope: crate::memory::Scope) -> TimelineBuilder<'_> {
        TimelineBuilder::new(self, scope)
    }

    /// Retrieves memories in `scope` ranked by hybrid cosine-and-recency, as
    /// a prompt-shaped [`MemoryContext`].
    ///
    /// Mirrors [`Client::search`]'s candidate-retrieval primitives but
    /// re-ranks the top-K candidates by combining vector similarity with
    /// recency before returning. Default strategy is
    /// [`RankingStrategy::default_hybrid`]; override via
    /// [`QueryBuilder::ranking`]. **The default strategy's parameter values
    /// are explicitly allowed to drift pre-1.0** — callers depending on a
    /// specific ranking must pass an explicit `RankingStrategy::Hybrid {
    /// .. }`.
    ///
    /// Returns a [`MemoryContext`] suitable for dropping into a system
    /// prompt via [`Display`]. See [`MemoryContext`] for the rendering
    /// shape and the staleness caveat for cached output.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Embedding`] if the query cannot be embedded,
    /// [`ClientError::Vector`] if the vector index search fails, and
    /// [`ClientError::Store`] wrapping a database failure when the matched
    /// pids cannot be hydrated to full rows.
    ///
    /// [`Display`]: std::fmt::Display
    pub fn query(&self, query: impl Into<String>, scope: crate::memory::Scope) -> QueryBuilder<'_> {
        QueryBuilder::new(self, query.into(), scope)
    }

    /// Returns memoir's state of knowledge in `scope` as of `as_of`.
    ///
    /// A memory is included when it was written on or before `as_of` and was
    /// not yet superseded then. Pure Postgres read; no Qdrant, no embedder.
    /// Newest-first by `created_at`, default limit
    /// [`crate::store::DEFAULT_TIMELINE_LIMIT`]. See [`RecallAsOfBuilder`]
    /// for the builder methods.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::InvalidScope`] when a `Scope` target has
    /// empty fields, or wrapping
    /// [`crate::store::StoreError::Database`] for database failures.
    pub fn recall_as_of(
        &self,
        scope: crate::memory::Scope,
        as_of: impl Into<chrono::DateTime<chrono::FixedOffset>>,
    ) -> RecallAsOfBuilder<'_> {
        RecallAsOfBuilder::new(self, scope, as_of.into())
    }

    /// Looks up a single memory by its public id, at any lifecycle state.
    ///
    /// Returns the memory regardless of whether its vector index entry is
    /// `pending`, `indexed`, or `failed` — callers using this for direct
    /// lookups see the source-of-truth row from Postgres.
    ///
    /// No scope check is performed: any caller holding a `pid` can retrieve
    /// the corresponding memory. The library treats its caller as the trust
    /// boundary; service-mode callers (epic 0007) gate access via their own
    /// auth layer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let memory = client.recall("AbCdEfGhIjKlMnOpQrStU").await?;
    /// println!("{}", memory.content);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping [`crate::store::StoreError::NotFound`]
    /// when no memory matches `pid`, and [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::Database`] for database failures.
    pub async fn recall(&self, pid: &str) -> Result<Memory, ClientError> {
        Ok(self.inner.store.recall(pid).await?)
    }

    /// Deletes one memory by pid, or every memory matching a scope tuple.
    ///
    /// The Postgres delete is authoritative — returned pids reflect what was
    /// actually removed from the source of truth. The Qdrant delete is
    /// best-effort: on failure the source-of-truth row is already gone and
    /// the orphaned vector becomes the reconciliation sweep's problem
    /// (ticket 0012). Failure does not propagate; it emits
    /// `memoir.forget.index_delete_failed` at WARN.
    ///
    /// Returns the pids that were deleted. An empty vector means the target
    /// matched no rows — not an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # use memoir_core::memory::{ForgetTarget, Scope};
    /// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
    /// let deleted = client.forget(ForgetTarget::Pid("abc123".to_string())).await?;
    /// let cleared = client.forget(ForgetTarget::Scope(scope)).await?;
    /// # let _ = (deleted, cleared);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::InvalidScope`] when a `Scope` target has
    /// any empty field, and [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::Database`] for database failures.
    pub async fn forget(&self, target: ForgetTarget) -> Result<Vec<String>, ClientError> {
        let deleted = self.inner.store.forget(target).await?;

        if deleted.is_empty() {
            return Ok(deleted);
        }

        let pid_refs: Vec<&str> = deleted.iter().map(String::as_str).collect();
        if let Err(err) = self.inner.index.delete_by_pids(&pid_refs).await {
            tracing::event!(
                name: "memoir.forget.index_delete_failed",
                tracing::Level::WARN,
                pid_count = deleted.len(),
                error.message = %err,
                "vector delete failed for {{pid_count}} pid(s): {{error.message}} — reconciliation will clean up orphans",
            );
        } else {
            tracing::event!(
                name: "memoir.forget.success",
                tracing::Level::INFO,
                pid_count = deleted.len(),
                "{{pid_count}} memories forgotten",
            );
        }

        Ok(deleted)
    }

    /// Rejects a memory: a wrong extraction the user corrected (epic 0011).
    ///
    /// Marks the row `retirement_reason = 'rejected'` and evicts its vector,
    /// so it disappears from every read and can no longer pollute search or
    /// reprocessing. The row is kept (not deleted) — it is the reprocess
    /// "don't re-derive this" guard and counts toward the extraction-accuracy
    /// metric. Rejection is the extraction-error case; for a source that
    /// merely changed, use [`Self::mark_stale`].
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::NotFound`] when no memory matches `pid`,
    /// or [`crate::store::StoreError::Database`] for database failures. A
    /// vector-eviction failure is logged at WARN (reconciliation cleans the
    /// orphan) and does not fail the call once the row is marked.
    pub async fn reject(&self, pid: &str) -> Result<(), ClientError> {
        self.retire(pid, crate::memory::RetirementReason::Rejected).await
    }

    /// Marks a memory stale: its episodic source changed (epic 0011).
    ///
    /// Marks the row `retirement_reason = 'stale'` and evicts its vector. Like
    /// [`Self::reject`] the row is hidden everywhere and kept, but stale is
    /// NOT an extraction error (the model was right; the source moved), so it
    /// does not count against the accuracy metric.
    ///
    /// # Errors
    ///
    /// See [`Self::reject`].
    pub async fn mark_stale(&self, pid: &str) -> Result<(), ClientError> {
        self.retire(pid, crate::memory::RetirementReason::Stale).await
    }

    /// Corrects a wrong extraction by teaching, not editing (epic 0011).
    ///
    /// `pid` is the wrong *semantic* memory the user saw in recall. Awaiting
    /// the returned builder enqueues a reprocess of that fact's episodic
    /// source: the derived rows are retired as `rejected` and re-derived with
    /// the correction in context, so a corrected fact replaces the wrong one.
    /// The user never hand-writes a semantic row — semantic memory stays
    /// always-derived. Fire-and-forget: returns once the job is enqueued.
    ///
    /// To correct the episodic record itself, use [`Self::edit`]; that is a
    /// different correction (the source changed, not a wrong extraction). See
    /// [`FeedbackBuilder`] for the builder methods.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::NotFound`] when no memory matches `pid`,
    /// [`ClientError::NotCorrectable`] when the target is not a semantic row
    /// or has no episodic source, and [`ClientError::Jobs`] when the reprocess
    /// job cannot be enqueued.
    pub fn feedback(&self, pid: impl Into<String>) -> FeedbackBuilder<'_> {
        FeedbackBuilder::new(self, pid.into())
    }

    /// Retires `pid` with `reason`: marks the column, then evicts the vector.
    ///
    /// Shared by [`Self::reject`] and [`Self::mark_stale`]. Mirrors
    /// [`Self::forget`]'s store-then-index ordering and its
    /// WARN-on-evict-failure resilience: the Postgres mark is the source of
    /// truth, and a transient Qdrant failure leaves a searchable orphan that
    /// reconciliation removes — it does not roll back the retirement.
    async fn retire(&self, pid: &str, reason: crate::memory::RetirementReason) -> Result<(), ClientError> {
        self.inner.retire_and_evict(pid, reason).await
    }

    /// Runs reconciliation: retries `failed` rows and cleans Qdrant orphans.
    ///
    /// Returns a per-call builder. Awaiting it runs the configured passes
    /// and returns a [`ReconcileSummary`]. Both passes run by default;
    /// narrow with [`ReconcileBuilder::only_retry_failed`] /
    /// [`ReconcileBuilder::only_clean_orphans`]. Idempotent: running against
    /// a clean store does nothing and exits cleanly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # async fn example(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let summary = client.reconcile().await?;
    /// let _ = summary;
    /// # Ok(())
    /// # }
    /// ```
    pub fn reconcile(&self) -> ReconcileBuilder<'_> {
        ReconcileBuilder::new(self)
    }

    /// Configures the background queue worker; call `.start().await` to launch.
    ///
    /// Returns a per-call builder. The worker polls the `memory_jobs` queue,
    /// dispatches each row to its stage handler (embed in ticket 0007,
    /// extract in ticket 0006), and runs lease recovery when the queue is
    /// idle. Cooperative shutdown via [`WorkerHandle::shutdown`].
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
    pub fn spawn_worker(&self) -> WorkerBuilder<'_> {
        WorkerBuilder::new(self)
    }

    /// Lists failed jobs newest-first, capped at `limit`.
    ///
    /// Returns metadata only (id, kind, source pid, attempts, failure
    /// reason, last update); content from the referenced memory is NOT
    /// included. Operators who need to inspect the memory's content can
    /// follow up with [`Self::recall`] against the source pid.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Jobs`] wrapping any database failure.
    pub async fn failed_jobs(&self, limit: usize) -> Result<Vec<crate::jobs::FailedJob>, ClientError> {
        Ok(self.inner.jobs.list_failed(limit).await?)
    }

    /// Retries one failed job, clearing the attempt counter.
    ///
    /// The attempt counter is reset to zero on operator-initiated retry: a
    /// human has decided prior failures shouldn't count against the new
    /// attempt budget. Reconciliation-driven retries leave the counter
    /// alone (see [`Self::reconcile`]).
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Jobs`] wrapping [`crate::jobs::JobsError::NotFound`]
    /// when no failed job matches `id`, or wrapping a database failure.
    pub async fn retry_job(&self, id: i64) -> Result<(), ClientError> {
        self.inner.jobs.retry_job(id).await?;
        tracing::event!(
            name: "memoir.admin.retry_succeeded",
            tracing::Level::INFO,
            job_id = id,
            "retried failed job {{job_id}}",
        );
        Ok(())
    }

    /// Configures a bulk retry. Awaiting the returned builder runs it.
    ///
    /// See [`RetryBuilder`] for filter and dry-run options. Returns the
    /// number of affected (or for `dry_run`, would-affect) rows.
    pub fn retry_failed_jobs(&self) -> RetryBuilder<'_> {
        RetryBuilder::new(self)
    }

    /// Permanently deletes one failed job. The referenced memory is untouched.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Jobs`] wrapping [`crate::jobs::JobsError::NotFound`]
    /// when no failed job matches `id`, or wrapping a database failure.
    pub async fn delete_failed_job(&self, id: i64) -> Result<(), ClientError> {
        self.inner.jobs.delete_failed(id).await?;
        tracing::event!(
            name: "memoir.admin.delete_failed",
            tracing::Level::INFO,
            job_id = id,
            "deleted failed job {{job_id}}",
        );
        Ok(())
    }

    /// Returns the number of jobs currently in `pending` state.
    ///
    /// Cheap observation for operators monitoring queue depth.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Jobs`] wrapping any database failure.
    pub async fn pending_jobs_count(&self) -> Result<u64, ClientError> {
        Ok(self.inner.jobs.pending_count().await?)
    }

    /// Clears the supersession marker on `pid`, restoring it to active state.
    ///
    /// Admin-only counterpart to the internal supersede path. Use when an
    /// operator decides a future contradiction-detection pass wrongly
    /// marked a row as outdated. Idempotent at the SQL level for rows that
    /// were already active, but still errors if no row matches `pid`.
    ///
    /// There is no symmetric public `Client::supersede`: supersession is a
    /// decision made by the (forthcoming) detection engine against verified
    /// contradicting facts, not by operator hand. Hand-rolled supersession
    /// would defeat the audit trail the column is meant to preserve.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping
    /// [`crate::store::StoreError::NotFound`] when no memory matches `pid`,
    /// or wrapping a database failure.
    pub async fn unsupersede(&self, pid: &str) -> Result<(), ClientError> {
        self.inner.store.unsupersede(pid).await?;
        tracing::event!(
            name: "memoir.admin.unsuperseded",
            tracing::Level::INFO,
            pid = pid,
            "unsuperseded memory {{pid}}",
        );
        Ok(())
    }

    /// Returns the full supersede/unsupersede event trail for `pid`.
    ///
    /// Each [`SupersessionEvent`] is one decision against the memory,
    /// chronological (oldest first). An event with `winner_pid = None` is
    /// an unsupersede. A pid with no events — never superseded, or simply
    /// not present in the store — returns an empty vec, not an error.
    ///
    /// Surfaces the audit trail behind a row's current `Memory.supersession`
    /// marker, for the supersession-audit UI and rig-service introspection
    /// of contradiction-detection decisions over time.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping a database failure.
    pub async fn supersession_history(&self, pid: &str) -> Result<Vec<SupersessionEvent>, ClientError> {
        Ok(self.inner.store.supersession_history(pid).await?)
    }

    /// Lists the distinct agent ids with memories under `org_id` + `user_id`.
    ///
    /// Caller-scoped agent discovery: returns only the agents within the given
    /// org and user, sorted ascending, so a tenant never sees another tenant's
    /// agents. A scope with no memories yet returns an empty vec, not an error.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Store`] wrapping a database failure.
    pub async fn list_agents(&self, org_id: &str, user_id: &str) -> Result<Vec<String>, ClientError> {
        Ok(self.inner.store.list_agent_ids(org_id, user_id).await?)
    }

    /// Computes extraction accuracy per `(provider, model)` over a scope slice.
    ///
    /// Returns an [`ExtractionStatsBuilder`]; its scope setters narrow the slice
    /// before awaiting. A read-only aggregate proving extraction quality to a
    /// consumer — `accuracy = 1 − rejected/total`, where `rejected` counts only
    /// wrong extractions the user corrected (not `Stale` or superseded rows).
    /// No LLM call. See [`ExtractionStatsBuilder`] for the builder methods.
    pub fn extraction_stats(&self) -> ExtractionStatsBuilder<'_> {
        ExtractionStatsBuilder::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmKind;

    #[test]
    fn should_install_extraction_llm_into_empty_registry() {
        let mut registry = LlmRegistry::new();
        registry
            .install(
                LlmRole::Extraction,
                LlmConfig::ollama("http://localhost:11434", "llama3.2"),
            )
            .unwrap();

        let provider = registry.get(LlmRole::Extraction).expect("extraction provider present");
        assert_eq!(provider.kind(), LlmKind::Ollama);
        assert_eq!(provider.model(), "llama3.2");
    }

    #[test]
    fn should_install_both_extraction_and_contradiction_llms_independently() {
        let mut registry = LlmRegistry::new();
        registry
            .install(
                LlmRole::Extraction,
                LlmConfig::ollama("http://localhost:11434", "extraction-model"),
            )
            .unwrap();
        registry
            .install(
                LlmRole::Contradiction,
                LlmConfig::ollama("http://localhost:11434", "contradiction-model"),
            )
            .unwrap();

        assert_eq!(registry.get(LlmRole::Extraction).unwrap().model(), "extraction-model");
        assert_eq!(
            registry.get(LlmRole::Contradiction).unwrap().model(),
            "contradiction-model"
        );
    }

    #[test]
    fn should_leave_registry_empty_when_no_llms_installed() {
        let registry = LlmRegistry::new();
        assert!(registry.is_empty());
        assert!(registry.get(LlmRole::Extraction).is_none());
        assert!(registry.get(LlmRole::Contradiction).is_none());
    }
}
