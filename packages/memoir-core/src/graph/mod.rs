//! Property-graph abstraction for entity/relationship storage.
//!
//! Defines [`GraphStore`], memoir's optional secondary index for the knowledge
//! graph derived from extracted memories (epic 0012). Two implementations ship:
//! [`InMemoryGraphStore`] (always available — the test/benchmark boundary) and
//! [`FalkorGraphStore`] (behind the `knowledge-graph` feature — production,
//! backed by FalkorDB).
//!
//! This trait is deliberately thin: it covers *connectivity* (ensuring the
//! named graph is reachable) and a raw Cypher [`GraphStore::query`] escape
//! hatch. Typed upsert and traversal methods are layered on top of it.
//!
//! Like [`crate::vector::VectorIndex`], the graph is a *derived* index: Postgres
//! remains the source of truth, and the graph can be rebuilt from the episodic
//! memories. Absence of a graph store is a first-class, non-degraded state —
//! recall simply returns vector hits with no graph enrichment.

mod error;
mod extraction;
mod memory;
mod resolve;

pub use error::GraphError;
pub use extraction::{
    DEFAULT_TRIPLE_PROMPT, LlmExtractor, TRIPLE_REPLY_MAX_CHARS, Triple, TripleExtractor, TripleSet,
};
pub use memory::InMemoryGraphStore;
pub use resolve::{
    EmbeddingEntityResolver, EntityCatalog, EntityResolver, EntityVector, ExactStringResolver, InMemoryEntityCatalog,
    Resolution, ResolveError, MIN_ENTITY_SIMILARITY,
};

#[cfg(feature = "knowledge-graph")]
mod falkor;

#[cfg(feature = "knowledge-graph")]
pub use falkor::FalkorGraphStore;

use std::future::Future;

/// Default graph name memoir writes to within a shared FalkorDB instance.
///
/// FalkorDB hosts many named graphs in one process; memoir confines its writes
/// to this graph so it coexists with a host application's own graphs. Override
/// per deployment so two memoir instances never collide on one engine.
pub const DEFAULT_GRAPH_NAME: &str = "memoir";

/// One row of a Cypher result, mapping each returned column to a scalar value.
///
/// Scalars are rendered to `String` so the public surface never leaks a
/// backend-specific value type. Columns preserve the order of the `RETURN`
/// clause.
pub type GraphRow = Vec<(String, String)>;

/// The rows produced by a Cypher [`GraphStore::query`], in result order.
pub type GraphRows = Vec<GraphRow>;

/// Stores and queries an entity/relationship property graph.
///
/// Implementations own the graph-backend connection and confine their writes to
/// a single named graph (see [`DEFAULT_GRAPH_NAME`]). The trait methods are
/// async and `Send`-bound so callers can drive them from any tokio runtime,
/// mirroring [`crate::vector::VectorIndex`].
pub trait GraphStore: Send + Sync + 'static {
    /// Ensures the configured named graph is reachable.
    ///
    /// Idempotent: callers invoke this on startup to fail fast when the backend
    /// is unreachable or misconfigured, rather than on first write. FalkorDB
    /// creates a graph lazily on first write, so this is a connectivity probe,
    /// not a schema-creation step.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Connection`] if the backend is unreachable.
    fn ensure_graph(&self) -> impl Future<Output = Result<(), GraphError>> + Send;

    /// Runs a Cypher query against the configured graph and returns its rows.
    ///
    /// The raw escape hatch the write-path and read-path tickets build their
    /// typed operations on. Scalar result values are rendered to `String`;
    /// node/edge/path projections are out of scope until a consumer needs them.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Query`] when the backend rejects or fails the
    /// query, and [`GraphError::Connection`] when the backend is unreachable.
    fn query(&self, cypher: &str) -> impl Future<Output = Result<GraphRows, GraphError>> + Send;
}
