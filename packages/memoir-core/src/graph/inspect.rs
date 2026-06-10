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

use super::{GraphError, GraphParam, GraphRow, GraphStore};

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
}

/// Backs [`GraphStore::inspect_scope`]; see that method for semantics.
///
/// Nodes and edges are read in two separate queries rather than one path-match:
/// an edge-anchored traversal would drop isolated entities, which are real graph
/// data the admin view must show.
pub(super) async fn inspect_scope<G: GraphStore + ?Sized>(
    store: &G,
    agent_id: Option<&str>,
    org_id: Option<&str>,
    user_id: Option<&str>,
    limit: usize,
) -> Result<GraphSnapshot, GraphError> {
    let limit = limit.clamp(1, MAX_INSPECTION_LIMIT);

    let mut params = HashMap::new();
    let mut node_terms: Vec<&str> = Vec::new();
    let mut edge_terms: Vec<&str> = Vec::new();
    if let Some(agent_id) = agent_id {
        params.insert("agent_id".to_string(), agent_id.into());
        node_terms.push("n.agent_id = $agent_id");
        edge_terms.push("s.agent_id = $agent_id");
    }
    if let Some(org_id) = org_id {
        params.insert("org_id".to_string(), org_id.into());
        node_terms.push("n.org_id = $org_id");
        edge_terms.push("s.org_id = $org_id");
    }
    if let Some(user_id) = user_id {
        params.insert("user_id".to_string(), user_id.into());
        node_terms.push("n.user_id = $user_id");
        edge_terms.push("s.user_id = $user_id");
    }
    params.insert("lim".to_string(), GraphParam::Int(limit as i64));

    let node_where = where_clause(&node_terms);
    let node_cypher = format!(
        "MATCH (n:Entity){node_where} \
         RETURN n.name AS name, n.memory_pids AS memory_pids, n.first_seen_at AS first_seen_at \
         ORDER BY n.first_seen_at, n.name \
         LIMIT $lim"
    );

    let edge_where = where_clause(&edge_terms);
    let edge_cypher = format!(
        "MATCH (s:Entity)-[r]->(o:Entity){edge_where} \
         RETURN s.name AS subject, r.relation AS relation, o.name AS object, \
                r.confidence AS confidence, r.valid_from AS valid_from, r.valid_to AS valid_to, \
                r.memory_pids AS memory_pids \
         ORDER BY r.valid_from \
         LIMIT $lim"
    );

    let node_rows = store.query(&node_cypher, &params).await?;
    let edge_rows = store.query(&edge_cypher, &params).await?;

    let nodes: Vec<GraphNode> = node_rows.iter().filter_map(node_from_row).collect();
    let edges: Vec<GraphEdge> = edge_rows.iter().filter_map(edge_from_row).collect();
    let truncated = nodes.len() >= limit || edges.len() >= limit;

    Ok(GraphSnapshot { nodes, edges, truncated })
}

/// Joins scope predicates into a `WHERE` clause, or empty when unconstrained.
fn where_clause(terms: &[&str]) -> String {
    if terms.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", terms.join(" AND "))
    }
}

/// Parses a [`GraphNode`] from a node result row.
///
/// A row missing its `name` is skipped — one malformed node should not break the
/// whole snapshot. `memory_pids` parses from the JSON array the commit writes;
/// an unparseable value yields an empty list rather than dropping the node.
fn node_from_row(row: &GraphRow) -> Option<GraphNode> {
    let name = column(row, "name")?.to_string();
    Some(GraphNode {
        name,
        memory_pids: parse_pids(column(row, "memory_pids")),
        first_seen_at: present(column(row, "first_seen_at")),
    })
}

/// Parses a [`GraphEdge`] from an edge result row.
///
/// A row missing subject, relation, or object is skipped. `valid_to` carries
/// through as `None` for a current edge (the backend renders null as the absent
/// sentinel), `Some` for a superseded one.
fn edge_from_row(row: &GraphRow) -> Option<GraphEdge> {
    let subject = column(row, "subject")?.to_string();
    let relation = column(row, "relation")?.to_string();
    let object = column(row, "object")?.to_string();
    let confidence = column(row, "confidence").and_then(|c| c.parse().ok()).unwrap_or(1.0);
    Some(GraphEdge {
        subject,
        relation,
        object,
        confidence,
        valid_from: present(column(row, "valid_from")),
        valid_to: present(column(row, "valid_to")),
        memory_pids: parse_pids(column(row, "memory_pids")),
    })
}

/// Parses the `memory_pids` JSON array, defaulting to empty on any other shape.
fn parse_pids(value: Option<&str>) -> Vec<String> {
    value.and_then(|v| serde_json::from_str(v).ok()).unwrap_or_default()
}

/// Maps a backend null sentinel (absent column or `"null"`) to `None`.
fn present(value: Option<&str>) -> Option<String> {
    match value {
        None => None,
        Some(v) if v == "null" || v.is_empty() => None,
        Some(v) => Some(v.to_string()),
    }
}

/// Returns the value of the column named `name` in a result row.
fn column<'a>(row: &'a GraphRow, name: &str) -> Option<&'a str> {
    row.iter()
        .find(|(column, _)| column == name)
        .map(|(_, value)| value.as_str())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::graph::GraphRows;

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
        inspect_scope(&store, Some("a"), Some("o"), Some("u"), 100).await.unwrap();

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
        inspect_scope(&store, None, Some("o"), None, 100).await.unwrap();

        let (node_cypher, params) = &store.calls()[0];
        assert!(node_cypher.contains("n.org_id = $org_id"));
        assert!(!node_cypher.contains("agent_id"), "absent dimension imposes no filter");
        assert!(!node_cypher.contains("user_id"));
        assert!(!params.contains_key("agent_id"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_emit_no_where_clause_for_empty_scope() {
        let store = StagedStore::empty();
        inspect_scope(&store, None, None, None, 100).await.unwrap();

        let (node_cypher, _) = &store.calls()[0];
        assert!(!node_cypher.contains("WHERE"), "no scope -> whole-graph dump");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_read_both_current_and_superseded_edges() {
        let store = StagedStore::empty();
        inspect_scope(&store, Some("a"), Some("o"), Some("u"), 100).await.unwrap();

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
        inspect_scope(&store, None, None, None, MAX_INSPECTION_LIMIT * 10)
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

        let snapshot = inspect_scope(&store, None, None, None, 100).await.unwrap();

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

        let snapshot = inspect_scope(&store, None, None, None, 100).await.unwrap();
        assert_eq!(snapshot.edges[0].valid_to.as_deref(), Some("2026-06-01T00:00:00+00:00"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_flag_truncated_when_limit_reached() {
        let store = StagedStore::new(vec![vec![row(&[("name", "Alice")]), row(&[("name", "Bob")])], vec![]]);

        let snapshot = inspect_scope(&store, None, None, None, 2).await.unwrap();
        assert!(snapshot.truncated, "node count == limit -> truncated");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_skip_node_missing_name() {
        let store = StagedStore::new(vec![vec![row(&[("memory_pids", "[\"mem1\"]")])], vec![]]);
        let snapshot = inspect_scope(&store, None, None, None, 100).await.unwrap();
        assert!(snapshot.nodes.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_default_pids_empty_when_unparseable() {
        let store = StagedStore::new(vec![vec![row(&[("name", "Alice"), ("memory_pids", "not json")])], vec![]]);
        let snapshot = inspect_scope(&store, None, None, None, 100).await.unwrap();
        assert!(snapshot.nodes[0].memory_pids.is_empty());
    }
}
