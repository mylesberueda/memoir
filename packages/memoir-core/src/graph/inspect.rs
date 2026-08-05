//! Admin read-path: a whole-scope snapshot of the graph.
//!
//! The admin "Knowledge graph view" (`.tasks/README.md:587`): every entity and
//! relationship in a scope, for an operator to inspect or render. Unlike
//! read-path enrichment ([`super::enrich`]), this is *scope-anchored*, not
//! *seed-anchored* — there are no seed memories, the whole (possibly partial)
//! scope is dumped — and it returns **both** current and superseded edges, each
//! flagged by `valid_to`, so the admin UI can render the temporal history.
//!
//! Scope is *partial*: any of `agent_id` / `org_id` / `user_id` may be absent,
//! and an absent dimension imposes no filter. This is the one cross-scope read
//! in memoir (an admin can view across agents/users/orgs); it is read-only and
//! gated by the caller's auth layer (memoir-service's `require_admin`). The
//! write, forget, and enrichment paths keep full-scope-tuple isolation.
//!
//! The snapshot carries richer per-element provenance than the flat enrichment
//! [`GraphContext`](super::GraphContext): nodes carry `memory_pids` and
//! `first_seen_at`, edges carry `valid_from`, `valid_to`, and `memory_pids`.
//! Source memory *content* is not hydrated here — the consumer resolves
//! `memory_pids` against Postgres if it needs the underlying utterances.

use std::collections::HashMap;

use super::{GraphParam, GraphRow};

/// Default cap on the nodes and on the edges a single inspection returns.
///
/// A scope's full graph can be large; an unbounded dump risks an enormous
/// payload and a heavy backend scan. The cap applies independently to nodes and
/// to edges (each limited to this many), and the snapshot flags when either was
/// truncated so the UI knows the view is partial.
pub const DEFAULT_INSPECTION_LIMIT: usize = 500;

/// Hard upper bound on a caller-supplied inspection limit.
///
/// Clamps an over-large request so an admin cannot ask for an unbounded scan;
/// mirrors the failed-jobs limit discipline (`services/admin.rs`).
pub const MAX_INSPECTION_LIMIT: usize = 5_000;

/// A partial scope selecting which slice of the graph to inspect.
///
/// Each dimension is independently optional, and an absent one imposes no
/// filter — the admin read is the one cross-scope path in memoir. Contrast
/// [`Scope`](crate::memory::Scope), which models a complete write-side tuple.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScopeFilter {
    agent_id: Option<String>,
    org_id: Option<String>,
    user_id: Option<String>,
}

impl ScopeFilter {
    /// Narrows the filter to one agent id.
    #[must_use]
    pub fn agent(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    /// Narrows the filter to one org id.
    #[must_use]
    pub fn org(mut self, org_id: impl Into<String>) -> Self {
        self.org_id = Some(org_id.into());
        self
    }

    /// Narrows the filter to one user id.
    #[must_use]
    pub fn user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// The agent id this filter selects, if any.
    pub fn agent_id(&self) -> Option<&str> {
        self.agent_id.as_deref()
    }

    /// The org id this filter selects, if any.
    pub fn org_id(&self) -> Option<&str> {
        self.org_id.as_deref()
    }

    /// The user id this filter selects, if any.
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    /// Binds the present dimensions as query parameters and returns the `WHERE`
    /// clauses that reference them, one set per alias.
    ///
    /// Scope values bind as parameters rather than interpolating into Cypher.
    pub(super) fn bind(&self, node_alias: &str, edge_alias: &str) -> (HashMap<String, GraphParam>, String, String) {
        let mut params = HashMap::new();
        let mut node_terms: Vec<String> = Vec::new();
        let mut edge_terms: Vec<String> = Vec::new();

        for (column, value) in [
            ("agent_id", self.agent_id()),
            ("org_id", self.org_id()),
            ("user_id", self.user_id()),
        ] {
            let Some(value) = value else { continue };
            params.insert(column.to_string(), value.into());
            node_terms.push(format!("{node_alias}.{column} = ${column}"));
            edge_terms.push(format!("{edge_alias}.{column} = ${column}"));
        }

        (params, Self::where_clause(&node_terms), Self::where_clause(&edge_terms))
    }

    /// Joins scope predicates into a `WHERE` clause, or empty when unconstrained.
    fn where_clause(terms: &[String]) -> String {
        if terms.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", terms.join(" AND "))
        }
    }
}

/// An entity node in an admin graph snapshot.
///
/// Untyped in v1 (`:Entity`, ticket 0005) — carries the canonical `name` for
/// identity, plus the provenance the admin view wants: which memories
/// contributed it and when it first appeared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNode {
    /// The entity node's canonical name (its identity within a scope).
    pub name: String,
    /// Public ids of the memories that contributed this entity.
    pub memory_pids: Vec<String>,
    /// When this entity was first seen, RFC 3339 (the commit's `first_seen_at`).
    pub first_seen_at: Option<String>,
}

/// A relationship edge in an admin graph snapshot.
///
/// Carries the full temporal state: `valid_from` (when the fact became true) and
/// `valid_to` (`None` = current, `Some` = superseded at that time). Both current
/// and closed edges are returned so the admin UI can render history — the reason
/// this type carries `valid_to` where the enrichment
/// [`GraphRelationship`](super::GraphRelationship) does not.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphEdge {
    /// The subject entity's name.
    pub subject: String,
    /// The relation label (open vocabulary, the original extracted string).
    pub relation: String,
    /// The object entity's name.
    pub object: String,
    /// The extractor's confidence in this relationship, 0.0-1.0.
    pub confidence: f32,
    /// When the fact became true, RFC 3339.
    pub valid_from: Option<String>,
    /// When the fact was superseded, RFC 3339; `None` for a current edge.
    pub valid_to: Option<String>,
    /// Public ids of the memories that contributed this relationship.
    pub memory_pids: Vec<String>,
}

/// A whole-scope snapshot of the graph for admin inspection.
///
/// Every node and edge in the (possibly partial) scope, up to the inspection
/// limit. `truncated` is set when either list hit the cap, so the consumer knows
/// the view is incomplete rather than the scope being small.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphSnapshot {
    /// Every entity in scope, ordered by `first_seen_at` then `name`.
    pub nodes: Vec<GraphNode>,
    /// Every relationship in scope — current and superseded.
    pub edges: Vec<GraphEdge>,
    /// Whether the node or edge list was capped at the inspection limit.
    pub truncated: bool,
}

impl GraphSnapshot {
    /// Returns whether the snapshot holds no nodes or edges.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }

    /// Returns whether any entity's name contains `name`, ignoring case.
    ///
    /// Substring rather than equality because extraction picks the canonical
    /// name: a caller looking for "Alice" should find a committed "Alice Smith".
    pub fn has_entity(&self, name: &str) -> bool {
        self.entities(name).next().is_some()
    }

    /// Counts the entities whose name contains `name`, ignoring case.
    ///
    /// A count above one means entity resolution left duplicates that should
    /// have merged into a single node.
    pub fn count_entities(&self, name: &str) -> usize {
        self.entities(name).count()
    }

    /// Returns whether `a` and `b` are related by any edge, in either direction.
    ///
    /// Direction- and label-agnostic: the extractor chooses relation phrasing
    /// and triple direction, so "related at all" is the strongest claim a
    /// caller can make about two names without pinning model output.
    pub fn has_edge_between(&self, a: &str, b: &str) -> bool {
        self.edges_between(a, b).next().is_some()
    }

    /// Returns every edge spanning `a` and `b`, in either direction.
    ///
    /// Includes superseded edges — check [`GraphEdge::valid_to`] to filter to
    /// current relationships.
    pub fn edges_between<'a>(&'a self, a: &'a str, b: &'a str) -> impl Iterator<Item = &'a GraphEdge> {
        let a = a.to_lowercase();
        let b = b.to_lowercase();
        self.edges.iter().filter(move |edge| {
            let subject = edge.subject.to_lowercase();
            let object = edge.object.to_lowercase();
            (subject.contains(&a) && object.contains(&b)) || (subject.contains(&b) && object.contains(&a))
        })
    }

    fn entities<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a GraphNode> {
        let name = name.to_lowercase();
        self.nodes
            .iter()
            .filter(move |node| node.name.to_lowercase().contains(&name))
    }
}

impl GraphNode {
    /// Parses a node from a result row, or `None` when it carries no `name`.
    ///
    /// One malformed node is skipped rather than failing the whole snapshot.
    pub(super) fn from_row(row: &GraphRow) -> Option<Self> {
        Some(Self {
            name: row.column("name")?.to_string(),
            memory_pids: row.memory_pids(),
            first_seen_at: row.present("first_seen_at"),
        })
    }
}

impl GraphEdge {
    /// Parses an edge from a result row, or `None` when subject, relation, or
    /// object is missing.
    pub(super) fn from_row(row: &GraphRow) -> Option<Self> {
        Some(Self {
            subject: row.column("subject")?.to_string(),
            relation: row.column("relation")?.to_string(),
            object: row.column("object")?.to_string(),
            confidence: row.column("confidence").and_then(|c| c.parse().ok()).unwrap_or(1.0),
            valid_from: row.present("valid_from"),
            valid_to: row.present("valid_to"),
            memory_pids: row.memory_pids(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::graph::{GraphError, GraphRows, GraphStore};

    fn row(pairs: &[(&str, &str)]) -> GraphRow {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    /// Returns staged node/edge rows in turn, recording each (cypher, params) call.
    ///
    /// `inspect_scope` issues the node query first, then the edge query, so the
    /// staged responses are drained in that order.
    struct StagedStore {
        responses: Mutex<Vec<GraphRows>>,
        calls: Mutex<Vec<(String, HashMap<String, GraphParam>)>>,
    }

    impl StagedStore {
        fn new(responses: Vec<GraphRows>) -> Self {
            Self {
                responses: Mutex::new(responses),
                calls: Mutex::default(),
            }
        }

        fn empty() -> Self {
            Self::new(vec![vec![], vec![]])
        }

        fn calls(&self) -> Vec<(String, HashMap<String, GraphParam>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl GraphStore for StagedStore {
        async fn ensure_graph(&self) -> Result<(), GraphError> {
            Ok(())
        }

        async fn query(&self, cypher: &str, params: &HashMap<String, GraphParam>) -> Result<GraphRows, GraphError> {
            self.calls.lock().unwrap().push((cypher.to_string(), params.clone()));
            let mut responses = self.responses.lock().unwrap();
            Ok(if responses.is_empty() {
                Vec::new()
            } else {
                responses.remove(0)
            })
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bind_full_scope_as_params() {
        let store = StagedStore::empty();
        store
            .inspect_scope(&ScopeFilter::default().agent("a").org("o").user("u"), 100)
            .await
            .unwrap();

        let (node_cypher, params) = &store.calls()[0];
        assert!(!node_cypher.contains("\"a\""), "scope must not be interpolated");
        assert_eq!(params.get("agent_id"), Some(&GraphParam::Str("a".to_string())));
        assert_eq!(params.get("org_id"), Some(&GraphParam::Str("o".to_string())));
        assert_eq!(params.get("user_id"), Some(&GraphParam::Str("u".to_string())));
        assert!(node_cypher.contains("n.agent_id = $agent_id"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_omit_absent_scope_dimensions() {
        let store = StagedStore::empty();
        store
            .inspect_scope(&ScopeFilter::default().org("o"), 100)
            .await
            .unwrap();

        let (node_cypher, params) = &store.calls()[0];
        assert!(node_cypher.contains("n.org_id = $org_id"));
        assert!(!node_cypher.contains("agent_id"), "absent dimension imposes no filter");
        assert!(!node_cypher.contains("user_id"));
        assert!(!params.contains_key("agent_id"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_emit_no_where_clause_for_empty_scope() {
        let store = StagedStore::empty();
        store.inspect_scope(&ScopeFilter::default(), 100).await.unwrap();

        let (node_cypher, _) = &store.calls()[0];
        assert!(!node_cypher.contains("WHERE"), "no scope -> whole-graph dump");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_read_both_current_and_superseded_edges() {
        let store = StagedStore::empty();
        store
            .inspect_scope(&ScopeFilter::default().agent("a").org("o").user("u"), 100)
            .await
            .unwrap();

        let edge_cypher = &store.calls()[1].0;
        assert!(
            !edge_cypher.contains("valid_to IS NULL"),
            "admin view must include superseded edges, not filter to current",
        );
        assert!(edge_cypher.contains("r.valid_to AS valid_to"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_clamp_limit_to_max() {
        let store = StagedStore::empty();
        store
            .inspect_scope(&ScopeFilter::default(), MAX_INSPECTION_LIMIT * 10)
            .await
            .unwrap();
        assert_eq!(
            store.calls()[0].1.get("lim"),
            Some(&GraphParam::Int(MAX_INSPECTION_LIMIT as i64)),
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_build_snapshot_from_node_and_edge_rows() {
        let store = StagedStore::new(vec![
            vec![row(&[
                ("name", "Alice"),
                ("memory_pids", "[\"mem1\",\"mem2\"]"),
                ("first_seen_at", "2026-06-01T00:00:00+00:00"),
            ])],
            vec![row(&[
                ("subject", "Alice"),
                ("relation", "works at"),
                ("object", "Acme"),
                ("confidence", "0.9"),
                ("valid_from", "2026-06-01T00:00:00+00:00"),
                ("valid_to", "null"),
                ("memory_pids", "[\"mem1\"]"),
            ])],
        ]);

        let snapshot = store.inspect_scope(&ScopeFilter::default(), 100).await.unwrap();

        assert_eq!(snapshot.nodes.len(), 1);
        assert_eq!(snapshot.nodes[0].name, "Alice");
        assert_eq!(snapshot.nodes[0].memory_pids, vec!["mem1", "mem2"]);
        assert_eq!(snapshot.edges.len(), 1);
        assert_eq!(snapshot.edges[0].object, "Acme");
        assert!(snapshot.edges[0].valid_to.is_none(), "null valid_to -> current edge");
        assert!(!snapshot.truncated);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_surface_superseded_edge_valid_to() {
        let store = StagedStore::new(vec![
            vec![],
            vec![row(&[
                ("subject", "Alice"),
                ("relation", "works at"),
                ("object", "Globex"),
                ("confidence", "0.8"),
                ("valid_from", "2026-05-01T00:00:00+00:00"),
                ("valid_to", "2026-06-01T00:00:00+00:00"),
                ("memory_pids", "[\"mem0\"]"),
            ])],
        ]);

        let snapshot = store.inspect_scope(&ScopeFilter::default(), 100).await.unwrap();
        assert_eq!(snapshot.edges[0].valid_to.as_deref(), Some("2026-06-01T00:00:00+00:00"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_flag_truncated_when_limit_reached() {
        let store = StagedStore::new(vec![vec![row(&[("name", "Alice")]), row(&[("name", "Bob")])], vec![]]);

        let snapshot = store.inspect_scope(&ScopeFilter::default(), 2).await.unwrap();
        assert!(snapshot.truncated, "node count == limit -> truncated");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_skip_node_missing_name() {
        let store = StagedStore::new(vec![vec![row(&[("memory_pids", "[\"mem1\"]")])], vec![]]);
        let snapshot = store.inspect_scope(&ScopeFilter::default(), 100).await.unwrap();
        assert!(snapshot.nodes.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_default_pids_empty_when_unparseable() {
        let store = StagedStore::new(vec![
            vec![row(&[("name", "Alice"), ("memory_pids", "not json")])],
            vec![],
        ]);
        let snapshot = store.inspect_scope(&ScopeFilter::default(), 100).await.unwrap();
        assert!(snapshot.nodes[0].memory_pids.is_empty());
    }
}
