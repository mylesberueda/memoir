//! High-level facade composing the embedder, store, and vector index.

mod embed;
mod error;
mod remember;

pub use error::ClientError;
pub use remember::{DEFAULT_SYSTEM_PROMPT, RememberBuilder};

use std::sync::Arc;

use bon::bon;
use qdrant_client::Qdrant;
use sea_orm::DatabaseConnection;

use crate::embedding::{EmbeddingModel, OnnxEmbedding};
use crate::memory::Memory;
use crate::store::{MemoryStore, PostgresStore};
use crate::vector::{QdrantIndex, VectorIndex};

/// Shared internal state held by [`Client`] behind an `Arc`.
pub(crate) struct ClientInner {
    pub(crate) embedder: Arc<OnnxEmbedding>,
    pub(crate) store: PostgresStore,
    pub(crate) index: QdrantIndex,
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
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use memoir_core::client::Client;
    ///
    /// let db = sea_orm::Database::connect("postgres://...").await?;
    /// let qdrant = qdrant_client::Qdrant::from_url("http://localhost:6334").build()?;
    ///
    /// let client = Client::builder()
    ///     .db(db)
    ///     .qdrant(qdrant)
    ///     .schema("memoir")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Embedding`] if the embedder fails to initialize
    /// and [`ClientError::Vector`] if `ensure_collection` fails on Qdrant.
    #[builder(start_fn = builder, finish_fn = build)]
    pub async fn new(
        db: DatabaseConnection,
        qdrant: Qdrant,
        #[builder(into)] schema: Option<String>,
        #[builder(into)] system_prompt: Option<String>,
        #[builder(into)] collection: Option<String>,
    ) -> Result<Client, ClientError> {
        let embedder = OnnxEmbedding::new()?;
        let store = PostgresStore::new(db);
        let index = match collection {
            Some(name) => QdrantIndex::new(qdrant).with_collection(name),
            None => QdrantIndex::new(qdrant),
        };

        index.ensure_collection(embedder.dimensions()).await?;

        let schema = schema.unwrap_or_else(|| memoir_core_migration::DEFAULT_SCHEMA.to_string());

        Ok(Client {
            inner: Arc::new(ClientInner {
                embedder: Arc::new(embedder),
                store,
                index,
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
}
