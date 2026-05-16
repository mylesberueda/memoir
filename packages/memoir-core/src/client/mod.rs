//! High-level facade composing the embedder, store, and vector index.

mod admin;
mod embed;
mod error;
mod extract;
mod reconcile;
mod remember;
mod worker;

pub use admin::RetryBuilder;
pub use error::ClientError;
pub use reconcile::{ReconcileBuilder, ReconcileSummary};
pub use remember::{DEFAULT_SYSTEM_PROMPT, RememberBuilder};
pub use worker::{
    DEFAULT_DRAIN_TIMEOUT, DEFAULT_LEASE_DURATION, DEFAULT_MAX_ATTEMPTS, DEFAULT_POLL_INTERVAL,
    WorkerBuilder, WorkerHandle,
};

use std::sync::Arc;

use bon::bon;
use qdrant_client::Qdrant;
use sea_orm::DatabaseConnection;

use crate::embedding::{EmbeddingModel, OnnxEmbedding};
use crate::jobs::{MemoryJobsStore, PostgresJobsStore};
use crate::llm::{LlmConfig, LlmRegistry, LlmRole, RigLlmProvider};
use crate::memory::{ForgetTarget, Memory};
use crate::store::{MemoryStore, PostgresStore};
use crate::vector::{QdrantIndex, VectorIndex};

/// Shared internal state held by [`Client`] behind an `Arc`.
pub(crate) struct ClientInner {
    pub(crate) embedder: Arc<OnnxEmbedding>,
    pub(crate) store: PostgresStore,
    pub(crate) index: QdrantIndex,
    pub(crate) jobs: PostgresJobsStore,
    pub(crate) llms: LlmRegistry,
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
    /// Builds a [`Client`] from caller-owned Postgres + Qdrant handles.
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
    /// let db = sea_orm::Database::connect("postgres://...").await?;
    /// let qdrant = qdrant_client::Qdrant::from_url("http://localhost:6334").build()?;
    ///
    /// let client = Client::builder()
    ///     .db(db)
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
    /// Returns [`ClientError::Embedding`] if the embedder fails to initialize,
    /// [`ClientError::Vector`] if `ensure_collection` fails on Qdrant, and
    /// [`ClientError::Llm`] if a configured provider can't be constructed
    /// (e.g. malformed URL or api-key rejected by rig).
    #[builder(start_fn = builder, finish_fn = build)]
    pub async fn new(
        db: DatabaseConnection,
        qdrant: Qdrant,
        #[builder(into)] schema: Option<String>,
        #[builder(into)] system_prompt: Option<String>,
        #[builder(into)] collection: Option<String>,
        extraction_llm: Option<LlmConfig>,
        contradiction_llm: Option<LlmConfig>,
    ) -> Result<Client, ClientError> {
        let embedder = OnnxEmbedding::new()?;
        let store = PostgresStore::new(db.clone());
        let jobs = PostgresJobsStore::new(db);
        let index = match collection {
            Some(name) => QdrantIndex::new(qdrant).with_collection(name),
            None => QdrantIndex::new(qdrant),
        };

        index.ensure_collection(embedder.dimensions()).await?;

        let schema = schema.unwrap_or_else(|| memoir_core_migration::DEFAULT_SCHEMA.to_string());

        let mut llms = LlmRegistry::new();
        if let Some(config) = extraction_llm {
            install_llm(&mut llms, LlmRole::Extraction, config)?;
        }
        if let Some(config) = contradiction_llm {
            install_llm(&mut llms, LlmRole::Contradiction, config)?;
        }

        Ok(Client {
            inner: Arc::new(ClientInner {
                embedder: Arc::new(embedder),
                store,
                index,
                jobs,
                llms,
                schema,
                system_prompt,
            }),
        })
    }
}

fn install_llm(
    registry: &mut LlmRegistry,
    role: LlmRole,
    config: LlmConfig,
) -> Result<(), ClientError> {
    let kind = config.kind();
    let provider = RigLlmProvider::new(config)?;
    registry.insert(role, provider);

    tracing::event!(
        name: "memoir.client.llm_configured",
        tracing::Level::INFO,
        role = role.as_str(),
        provider = kind.as_str(),
        "configured {{provider}} provider for {{role}}",
    );

    Ok(())
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
        memoir_core_migration::bootstrap_and_migrate(self.inner.store.db(), &self.inner.schema).await?;
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

    /// Writes `prompt` as an episodic memory and retrieves related memories.
    ///
    /// Returns a per-call builder; await it to run the operation. The kind
    /// toggles on the builder filter retrieval — the write is always episodic.
    /// See [`RememberBuilder`] for builder methods.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use memoir_core::client::Client;
    /// # use memoir_core::memory::Scope;
    /// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
    /// let memories = client.remember("hello", scope).await?;
    /// for m in memories.list() {
    ///     println!("{}", m.content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn remember(&self, prompt: impl Into<String>, scope: crate::memory::Scope) -> RememberBuilder<'_> {
        RememberBuilder::new(self, prompt.into(), scope)
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
    pub async fn failed_jobs(
        &self,
        limit: usize,
    ) -> Result<Vec<crate::jobs::FailedJob>, ClientError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmKind;

    #[test]
    fn should_install_extraction_llm_into_empty_registry() {
        let mut registry = LlmRegistry::new();
        install_llm(
            &mut registry,
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
        install_llm(
            &mut registry,
            LlmRole::Extraction,
            LlmConfig::ollama("http://localhost:11434", "extraction-model"),
        )
        .unwrap();
        install_llm(
            &mut registry,
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
