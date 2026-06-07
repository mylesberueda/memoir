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

mod commit;
mod cosine;
mod edge;
mod enrich;
mod error;
mod extraction;
mod forget;
mod memory;
mod resolve;
mod synthesis;

pub use commit::{CommitContext, CommitError};
pub use enrich::{
    DEFAULT_ENRICHMENT_DEPTH, GraphContext, GraphEntity, GraphRelationship, MAX_ENRICHMENT_DEPTH,
};
pub use edge::{
    CardinalityPolicy, Edge, EdgeCatalog, EdgeError, EdgeResolution, EdgeResolver, ExistingEdge, NaiveAppendResolver,
    RelationCardinality, TemporalEdgeResolver,
};
pub use error::GraphError;
pub use extraction::{
    DEFAULT_TRIPLE_PROMPT, LlmExtractor, TRIPLE_REPLY_MAX_CHARS, Triple, TripleExtractor, TripleSet,
};
pub use memory::InMemoryGraphStore;
pub use resolve::{
    EmbeddingEntityResolver, EntityCatalog, EntityResolver, EntityVector, ExactStringResolver, InMemoryEntityCatalog,
    Resolution, ResolveError, MIN_ENTITY_SIMILARITY,
};
pub use synthesis::{
    EmbeddingSynthesizer, MIN_CORROBORATION_SIMILARITY, PassthroughSynthesizer, SemanticFact, SynthesisError,
    Synthesizer,
};

#[cfg(feature = "knowledge-graph")]
mod falkor;

#[cfg(feature = "knowledge-graph")]
mod falkor_catalog;

#[cfg(feature = "knowledge-graph")]
mod staging;

#[cfg(feature = "knowledge-graph")]
pub use falkor::FalkorGraphStore;

#[cfg(feature = "knowledge-graph")]
pub use falkor_catalog::{FalkorEdgeCatalog, FalkorEntityCatalog};

#[cfg(feature = "knowledge-graph")]
pub use staging::TripleStaging;

use std::collections::HashMap;
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

    /// Runs a parameterized Cypher query against the graph, returning its rows.
    ///
    /// The raw escape hatch the write-path and read-path build their operations
    /// on. `params` binds query parameters by name, referenced as `$name` in the
    /// `cypher` body — the only injection-safe way to embed values drawn from
    /// user content (entity names, memory ids), since the values never enter the
    /// query string. Relationship *types* and labels cannot be parameterized by
    /// Cypher and must be sanitized by the caller. Pass an empty map for a query
    /// with no parameters. Scalar result values are rendered to `String`;
    /// node/edge/path projections are out of scope until a consumer needs them.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Query`] when the backend rejects or fails the
    /// query, and [`GraphError::Connection`] when the backend is unreachable.
    fn query(
        &self,
        cypher: &str,
        params: &HashMap<String, String>,
    ) -> impl Future<Output = Result<GraphRows, GraphError>> + Send;

    /// Removes each forgotten pid from the graph, reference-counted.
    ///
    /// For each pid: strips it from every edge's and node's `memory_pids`,
    /// deletes edges whose array empties, then deletes nodes whose array empties
    /// *and* that have no surviving edges (a node still joined by an other-pid
    /// edge is kept). Edges are processed before nodes so a node is never deleted
    /// out from under a surviving edge. A pid is a globally-unique memory id, so
    /// matching needs no scope guard; the pid binds as a parameter. Idempotent —
    /// re-forgetting an absent pid changes nothing.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if the backend rejects a statement.
    fn forget_pids(&self, pids: &[&str]) -> impl Future<Output = Result<(), GraphError>> + Send {
        forget::forget_pids(self, pids)
    }

    /// Deletes every node and edge in `scope` — a whole-tenant forget.
    ///
    /// The entire scoped subgraph is removed regardless of `memory_pids`, so
    /// this needs no pid list. `DETACH DELETE` removes each node together with
    /// its edges.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if the backend rejects the statement.
    fn forget_scope(&self, scope: &crate::memory::Scope) -> impl Future<Output = Result<(), GraphError>> + Send {
        forget::forget_scope(self, scope)
    }

    /// Commits a source's resolved triples to the graph, returning the count.
    ///
    /// Resolves each triple's entities ([`EntityResolver`]) and edge
    /// ([`EdgeResolver`]), embeds new nodes ([`EmbeddingModel`]), then `MERGE`s
    /// the nodes and the (possibly supersession-closing) edge — tagging every
    /// element with the source's pid and scope from `ctx`. Writes are idempotent,
    /// so retrying a partially-failed batch does not double-write. Triples whose
    /// subject and object resolve to the same node are skipped.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError`] on the first resolution or write failure.
    fn commit_triples<EM, ER, EdgeR>(
        &self,
        embedder: &EM,
        entities: &ER,
        edges: &EdgeR,
        ctx: &CommitContext,
        triples: &TripleSet,
    ) -> impl Future<Output = Result<usize, CommitError>> + Send
    where
        EM: crate::embedding::EmbeddingModel,
        ER: EntityResolver,
        EdgeR: EdgeResolver,
    {
        commit::commit_triples(self, embedder, entities, edges, ctx, triples)
    }

    /// Returns the graph neighborhood around a set of seed memories.
    ///
    /// Seeds from the entities whose `memory_pids` contains any of `seed_pids`,
    /// then walks current edges (`valid_to = null`) out to `depth` hops
    /// (clamped to [`MAX_ENRICHMENT_DEPTH`]), scope-confined. Returns a flat,
    /// deduplicated [`GraphContext`]; an empty `seed_pids` yields an empty
    /// context with no query. The read-path enrichment behind `.with_graph()`.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if the backend rejects the traversal.
    fn neighbors(
        &self,
        seed_pids: &[&str],
        scope: &crate::memory::Scope,
        depth: usize,
    ) -> impl Future<Output = Result<GraphContext, GraphError>> + Send {
        enrich::neighbors(self, seed_pids, scope, depth)
    }
}
