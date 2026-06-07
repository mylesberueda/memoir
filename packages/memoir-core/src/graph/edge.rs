//! Reconciling a new relationship edge against the edges already in the graph.
//!
//! Once a triple's subject and object resolve to canonical nodes
//! ([`super::resolve`]), the *edge* between them must be reconciled against
//! existing edges. A new fact may **contradict** an existing one — "Alice works
//! at Acme" then later "Alice works at Globex" — and the graph must record the
//! change without losing history. [`EdgeResolver`] is the seam that decides what
//! happens to existing edges when a new one arrives.
//!
//! **Contradiction is not Forget.** A contradiction *invalidates* an edge (keeps
//! it, marks it closed) — it does not delete it. Deletion is the Forget path
//! (reference-counted, a separate ticket). These stay distinct operations.
//!
//! Two implementations ship: [`NaiveAppendResolver`] (the benchmark floor —
//! every edge is added, nothing invalidated) and [`TemporalEdgeResolver`] (the
//! production impl — a conflicting edge is closed and the new one opened,
//! preserving the chain). The temporal model echoes the row-level supersession
//! model ([`crate::memory::SupersessionInfo`]: a newer fact won, a normal
//! lifecycle event, not an error) at the edge level — it is not a reuse of that
//! type, because edges are graph elements, not memory rows.

use std::collections::HashSet;
use std::future::Future;

use chrono::{DateTime, FixedOffset};

use crate::memory::Scope;

/// Whether a subject may hold one relation to many objects at the same time.
///
/// The axis that decides whether a new edge *contradicts* an existing one or
/// merely *adds* to it. A subject works at one employer at a time
/// ([`Self::SingleValued`] — a new `works_at` supersedes the old), but deploys
/// to many environments over time and knows many people at once
/// ([`Self::MultiValued`] — every `deployed`/`knows` edge coexists).
///
/// This is *simultaneous* cardinality, not "does the relation ever repeat":
/// "we deployed last weekend" and "we deployed Monday" are two true events, so
/// `deployed` is multi-valued and neither supersedes the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationCardinality {
    /// One live object at a time; a newer edge supersedes the older.
    SingleValued,

    /// Many live objects at once; every edge coexists, none supersedes.
    MultiValued,
}

/// Classifies each relation's [`RelationCardinality`] for conflict detection.
///
/// Relations not in the single-valued set default to [`RelationCardinality::MultiValued`]:
/// appending a duplicate is recoverable (dedup later), whereas wrongly
/// superseding destroys a true fact, so the safe default is to append.
#[derive(Debug, Clone, Default)]
pub struct CardinalityPolicy {
    single_valued: HashSet<String>,
}

impl CardinalityPolicy {
    /// Builds a policy treating `relations` as single-valued, all others multi.
    ///
    /// Relations are matched case-insensitively against the lowercased relation
    /// label, so `"works at"` and `"Works At"` classify alike.
    pub fn with_single_valued<I, S>(relations: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            single_valued: relations.into_iter().map(|relation| relation.into().to_lowercase()).collect(),
        }
    }

    /// Returns the cardinality of `relation`.
    pub fn cardinality(&self, relation: &str) -> RelationCardinality {
        if self.single_valued.contains(&relation.to_lowercase()) {
            RelationCardinality::SingleValued
        } else {
            RelationCardinality::MultiValued
        }
    }
}

/// A new relationship edge to reconcile against the graph.
///
/// Subject and object are the *resolved* node keys ([`super::Resolution`]), not
/// raw entity strings. `valid_from` is the source memory's event time (when the
/// fact became true), so "newer" orders by when facts held, not when they were
/// processed — a backdated memory does not wrongly win over a current one.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    /// Resolved key of the subject node within the scope.
    pub subject_key: String,
    /// The relation label (open vocabulary, as the extractor produced it).
    pub relation: String,
    /// Resolved key of the object node within the scope.
    pub object_key: String,
    /// The extractor's confidence in this edge, on the 0.0-1.0 scale.
    pub confidence: f32,
    /// When the fact became true (the source memory's event time).
    pub valid_from: DateTime<FixedOffset>,
}

/// An edge already in the graph, as the resolver sees it for conflict checks.
///
/// `key` is the edge's stable identity (used to close it); `valid_to` is `None`
/// while the edge is current and `Some(t)` once it was superseded at `t`. Only
/// current edges (those with `valid_to == None`) take part in conflict
/// resolution; already-closed edges are history.
#[derive(Debug, Clone, PartialEq)]
pub struct ExistingEdge {
    /// The edge's stable identity within its scope.
    pub key: String,
    /// Resolved key of the subject node.
    pub subject_key: String,
    /// The relation label.
    pub relation: String,
    /// Resolved key of the object node.
    pub object_key: String,
    /// `None` while the edge is current; `Some(t)` once superseded at `t`.
    pub valid_to: Option<DateTime<FixedOffset>>,
}

/// The resolver's decision: which existing edges to close, and the edge to open.
///
/// Echoes the row-level supersession model at the edge level — closing an edge
/// records "a newer fact won," a normal lifecycle event, not an extraction
/// error. `close` lists the stable keys of existing edges to mark superseded
/// (their `valid_to` set to the new edge's `valid_from`); `open` is the new edge
/// to add with `valid_to == None`. The commit path (a later ticket) writes both
/// in one transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeResolution {
    /// Stable keys of current edges to close (mark superseded).
    pub close: Vec<String>,
    /// The new edge to open as current.
    pub open: Edge,
}

/// Yields the current edges a resolver must reconcile a new edge against.
///
/// A focused retrieval seam mirroring [`super::EntityCatalog`]: the resolver
/// needs only the *current* edges (those with `valid_to == None`) that share the
/// new edge's subject and relation within one [`Scope`], not the whole graph.
/// This is also where a FalkorDB-backed lookup later slots in behind the same
/// trait, without changing the resolvers.
pub trait EdgeCatalog: Send + Sync + 'static {
    /// Returns the current edges in `scope` with `subject_key` and `relation`.
    ///
    /// Implementations return only edges whose `valid_to` is `None` (closed
    /// edges are history and never reconsidered), confined to `scope`.
    ///
    /// # Errors
    ///
    /// Returns [`EdgeError::Catalog`] when the backing store cannot be read.
    fn current_edges(
        &self,
        scope: &Scope,
        subject_key: &str,
        relation: &str,
    ) -> impl Future<Output = Result<Vec<ExistingEdge>, EdgeError>> + Send;
}

/// Reconciles a new edge against the graph's existing edges.
///
/// Implementations decide which existing edges a new one invalidates (if any)
/// and return the [`EdgeResolution`] the commit path applies. Swapping one
/// implementation for another (naive-append, temporal-invalidate) requires no
/// caller change, which is what lets the benchmark compare them.
pub trait EdgeResolver: Send + Sync + 'static {
    /// Resolves `edge` within `scope` against existing edges.
    ///
    /// # Errors
    ///
    /// Returns [`EdgeError::Catalog`] when reading existing edges fails.
    fn resolve(&self, scope: &Scope, edge: Edge) -> impl Future<Output = Result<EdgeResolution, EdgeError>> + Send;
}

/// Failure modes for [`EdgeResolver`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
    /// Reading existing edges from the [`EdgeCatalog`] failed.
    #[error("edge catalog read failed: {0}")]
    Catalog(String),
}

/// Appends every new edge, invalidating nothing.
///
/// The benchmark floor: a contradiction leaves both the old and new edge
/// current, so the graph accumulates conflicting facts. Establishes the gap the
/// benchmark measures [`TemporalEdgeResolver`] against, and never reads the
/// graph — its resolution is the new edge alone.
#[derive(Debug, Default, Clone, Copy)]
pub struct NaiveAppendResolver;

impl NaiveAppendResolver {
    /// Creates a naive-append resolver.
    pub fn new() -> Self {
        Self
    }
}

impl EdgeResolver for NaiveAppendResolver {
    async fn resolve(&self, _scope: &Scope, edge: Edge) -> Result<EdgeResolution, EdgeError> {
        Ok(EdgeResolution {
            close: Vec::new(),
            open: edge,
        })
    }
}

/// Invalidates conflicting edges instead of deleting them, preserving history.
///
/// The production impl. A new edge conflicts with a current edge when they share
/// subject and relation *and* the relation is [`RelationCardinality::SingleValued`]
/// (one live object at a time). Conflicting edges are closed (added to
/// [`EdgeResolution::close`]); the new edge opens as current. Multi-valued
/// relations never conflict, so every edge coexists.
///
/// Winner is decided by recency alone — a newer fact supersedes an older one
/// regardless of confidence, matching the row-level supersession model where "a
/// newer fact won" is purely temporal. Confidence rides on the edge for the read
/// path but never gates invalidation.
#[derive(Debug, Clone)]
pub struct TemporalEdgeResolver<C> {
    catalog: C,
    policy: CardinalityPolicy,
}

impl<C: EdgeCatalog> TemporalEdgeResolver<C> {
    /// Builds a temporal resolver over `catalog` with the cardinality `policy`.
    pub fn new(catalog: C, policy: CardinalityPolicy) -> Self {
        Self { catalog, policy }
    }
}

impl<C: EdgeCatalog> EdgeResolver for TemporalEdgeResolver<C> {
    async fn resolve(&self, scope: &Scope, edge: Edge) -> Result<EdgeResolution, EdgeError> {
        if self.policy.cardinality(&edge.relation) == RelationCardinality::MultiValued {
            return Ok(EdgeResolution {
                close: Vec::new(),
                open: edge,
            });
        }

        let current = self.catalog.current_edges(scope, &edge.subject_key, &edge.relation).await?;
        let close = current
            .into_iter()
            .filter(|existing| existing.object_key != edge.object_key)
            .map(|existing| existing.key)
            .collect();

        Ok(EdgeResolution { close, open: edge })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::*;

    fn scope() -> Scope {
        Scope {
            agent_id: "agent".to_string(),
            org_id: "org".to_string(),
            user_id: "user".to_string(),
        }
    }

    fn at(day: u32) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(&format!("2026-06-{day:02}T00:00:00Z")).expect("valid test date")
    }

    fn edge(subject: &str, relation: &str, object: &str, day: u32) -> Edge {
        Edge {
            subject_key: subject.to_string(),
            relation: relation.to_string(),
            object_key: object.to_string(),
            confidence: 0.9,
            valid_from: at(day),
        }
    }

    /// In-memory [`EdgeCatalog`] returning only current edges (`valid_to == None`).
    #[derive(Default)]
    struct InMemoryEdgeCatalog {
        edges: Mutex<HashMap<String, ExistingEdge>>,
    }

    impl InMemoryEdgeCatalog {
        fn with(edges: Vec<ExistingEdge>) -> Self {
            let map = edges.into_iter().map(|existing| (existing.key.clone(), existing)).collect();
            Self { edges: Mutex::new(map) }
        }
    }

    impl EdgeCatalog for InMemoryEdgeCatalog {
        async fn current_edges(
            &self,
            _scope: &Scope,
            subject_key: &str,
            relation: &str,
        ) -> Result<Vec<ExistingEdge>, EdgeError> {
            Ok(self
                .edges
                .lock()
                .expect("edge catalog mutex poisoned")
                .values()
                .filter(|existing| {
                    existing.valid_to.is_none() && existing.subject_key == subject_key && existing.relation == relation
                })
                .cloned()
                .collect())
        }
    }

    fn existing(key: &str, subject: &str, relation: &str, object: &str) -> ExistingEdge {
        ExistingEdge {
            key: key.to_string(),
            subject_key: subject.to_string(),
            relation: relation.to_string(),
            object_key: object.to_string(),
            valid_to: None,
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_append_without_closing_under_naive_resolver() {
        let resolver = NaiveAppendResolver::new();

        let resolution = resolver
            .resolve(&scope(), edge("alice", "works at", "globex", 2))
            .await
            .unwrap();

        assert!(resolution.close.is_empty());
        assert_eq!(resolution.open.object_key, "globex");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_close_conflicting_single_valued_edge() {
        let catalog = InMemoryEdgeCatalog::with(vec![existing("e1", "alice", "works at", "acme")]);
        let policy = CardinalityPolicy::with_single_valued(["works at"]);
        let resolver = TemporalEdgeResolver::new(catalog, policy);

        let resolution = resolver
            .resolve(&scope(), edge("alice", "works at", "globex", 2))
            .await
            .unwrap();

        assert_eq!(resolution.close, vec!["e1".to_string()]);
        assert_eq!(resolution.open.object_key, "globex");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_not_close_multi_valued_edges() {
        // The "deploy" case: three deploy events to distinct objects are all
        // true and coexist; a multi-valued relation never supersedes.
        let catalog = InMemoryEdgeCatalog::with(vec![
            existing("e1", "team", "deployed", "weekend"),
            existing("e2", "team", "deployed", "monday"),
        ]);
        let policy = CardinalityPolicy::with_single_valued(["works at"]);
        let resolver = TemporalEdgeResolver::new(catalog, policy);

        let resolution = resolver
            .resolve(&scope(), edge("team", "deployed", "today", 6))
            .await
            .unwrap();

        assert!(resolution.close.is_empty());
        assert_eq!(resolution.open.object_key, "today");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_not_close_when_same_object_restated() {
        // Restating the current fact must not close it against itself.
        let catalog = InMemoryEdgeCatalog::with(vec![existing("e1", "alice", "works at", "acme")]);
        let policy = CardinalityPolicy::with_single_valued(["works at"]);
        let resolver = TemporalEdgeResolver::new(catalog, policy);

        let resolution = resolver
            .resolve(&scope(), edge("alice", "works at", "acme", 2))
            .await
            .unwrap();

        assert!(resolution.close.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_close_low_confidence_new_edge_over_high_confidence_old() {
        // Recency wins regardless of confidence (matches row supersession).
        let catalog = InMemoryEdgeCatalog::with(vec![existing("e1", "alice", "works at", "acme")]);
        let policy = CardinalityPolicy::with_single_valued(["works at"]);
        let resolver = TemporalEdgeResolver::new(catalog, policy);

        let mut hedged = edge("alice", "works at", "globex", 2);
        hedged.confidence = 0.3;
        let resolution = resolver.resolve(&scope(), hedged).await.unwrap();

        assert_eq!(resolution.close, vec!["e1".to_string()]);
    }

    #[test]
    fn should_default_unknown_relations_to_multi_valued() {
        let policy = CardinalityPolicy::with_single_valued(["works at"]);
        assert_eq!(policy.cardinality("knows"), RelationCardinality::MultiValued);
        assert_eq!(policy.cardinality("works at"), RelationCardinality::SingleValued);
    }

    #[test]
    fn should_classify_cardinality_case_insensitively() {
        let policy = CardinalityPolicy::with_single_valued(["Works At"]);
        assert_eq!(policy.cardinality("works at"), RelationCardinality::SingleValued);
    }

    #[test]
    fn should_treat_empty_policy_as_all_multi_valued() {
        let policy = CardinalityPolicy::default();
        assert_eq!(policy.cardinality("works at"), RelationCardinality::MultiValued);
    }

    #[test]
    fn should_carry_event_time_as_valid_from() {
        let backdated = edge("alice", "works at", "acme", 1);
        let current = edge("alice", "works at", "globex", 5);
        assert!(backdated.valid_from < current.valid_from);
    }
}
