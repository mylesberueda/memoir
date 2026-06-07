//! Read-path enrichment: graph neighborhoods around search hits.
//!
//! After a vector search, a consumer can opt into a graph traversal that
//! surfaces entities and relationships *related* to the hits — facts the vector
//! search alone would miss. [`GraphStore::neighbors`](super::GraphStore::neighbors)
//! seeds from the hit memories' entities (those whose `memory_pids` contains a
//! hit pid) and walks current edges out to a bounded depth, returning a flat
//! [`GraphContext`]. Traversal is scope-confined and reads only current edges
//! (`valid_to = null`); superseded edges are history, not "related now".

use std::collections::HashMap;

use crate::memory::Scope;

use super::{GraphError, GraphRow, GraphStore};

/// Maximum traversal depth — the manifesto's "1-2 hop" upper bound.
///
/// Caps how far enrichment walks from a hit's entities. Beyond two hops the
/// related set grows fast and its relevance to the original hit thins; the cap
/// keeps an opt-in enrichment from turning into an unbounded graph scan.
pub const MAX_ENRICHMENT_DEPTH: usize = 2;

/// Default traversal depth when a consumer opts in without specifying one.
///
/// One hop — a hit's directly-related entities — is the high-value case and the
/// cheapest. Deeper traversal is opt-in via the depth knob.
pub const DEFAULT_ENRICHMENT_DEPTH: usize = 1;

/// An entity surfaced by read-path graph enrichment.
///
/// Untyped in v1 (`:Entity`, ticket 0005), so it carries only the canonical
/// `name`; a type field can be added later without breaking the struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEntity {
    /// The entity node's canonical name.
    pub name: String,
}

/// A relationship surfaced by read-path graph enrichment.
///
/// A current (non-superseded) edge between two entities, with the extractor's
/// confidence carried through for the consumer to weigh.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphRelationship {
    /// The subject entity's name.
    pub subject: String,
    /// The relation label.
    pub relation: String,
    /// The object entity's name.
    pub object: String,
    /// The extractor's confidence in this relationship, 0.0-1.0.
    pub confidence: f32,
}

/// The graph neighborhood around a search's hits.
///
/// A property of *one* enriched search (attached to the result wrapper, not to
/// any [`Memory`](crate::memory::Memory)). Flat, deduplicated lists; empty when
/// enrichment was not requested or no graph is configured. Fields are public so
/// later additions (entity type, edge validity) are additive via struct-update.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphContext {
    /// Distinct related entities, including the seed entities.
    pub entities: Vec<GraphEntity>,
    /// Distinct current relationships among the neighborhood.
    pub relationships: Vec<GraphRelationship>,
}

impl GraphContext {
    /// Returns whether the context holds no entities or relationships.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty() && self.relationships.is_empty()
    }
}

/// Backs [`GraphStore::neighbors`]; see that method for semantics.
pub(super) async fn neighbors<G: GraphStore + ?Sized>(
    store: &G,
    seed_pids: &[&str],
    scope: &Scope,
    depth: usize,
) -> Result<GraphContext, GraphError> {
    if seed_pids.is_empty() {
        return Ok(GraphContext::default());
    }
    let depth = depth.clamp(1, MAX_ENRICHMENT_DEPTH);

    let mut params = HashMap::from([
        ("agent_id".to_string(), scope.agent_id.clone()),
        ("org_id".to_string(), scope.org_id.clone()),
        ("user_id".to_string(), scope.user_id.clone()),
    ]);
    for (i, pid) in seed_pids.iter().enumerate() {
        params.insert(format!("pid{i}"), (*pid).to_string());
    }
    let pid_list = (0..seed_pids.len())
        .map(|i| format!("$pid{i}"))
        .collect::<Vec<_>>()
        .join(", ");

    // Seed = entities in scope whose memory_pids intersects the hit pids. Walk
    // current edges (valid_to null) out to `depth` hops, returning each edge's
    // endpoints + properties. The depth is interpolated (it is a clamped
    // integer, never user text), the rest binds as parameters.
    let cypher = format!(
        "MATCH (seed:Entity {{agent_id: $agent_id, org_id: $org_id, user_id: $user_id}}) \
         WHERE any(p IN seed.memory_pids WHERE p IN [{pid_list}]) \
         MATCH (seed)-[r*1..{depth}]-(related:Entity) \
         WITH seed, related, r \
         UNWIND r AS edge \
         WITH seed, related, edge WHERE edge.valid_to IS NULL \
         RETURN startNode(edge).name AS subject, edge.relation AS relation, \
                endNode(edge).name AS object, edge.confidence AS confidence, related.name AS related_name"
    );

    let rows = store.query(&cypher, &params).await?;
    Ok(build_context(&rows))
}

/// Assembles a deduplicated [`GraphContext`] from traversal result rows.
fn build_context(rows: &[GraphRow]) -> GraphContext {
    let mut entities: Vec<GraphEntity> = Vec::new();
    let mut relationships: Vec<GraphRelationship> = Vec::new();

    for row in rows {
        if let Some(name) = column(row, "related_name") {
            let entity = GraphEntity { name: name.to_string() };
            if !entities.contains(&entity) {
                entities.push(entity);
            }
        }

        let (Some(subject), Some(relation), Some(object)) =
            (column(row, "subject"), column(row, "relation"), column(row, "object"))
        else {
            continue;
        };
        let confidence = column(row, "confidence").and_then(|c| c.parse().ok()).unwrap_or(1.0);
        let relationship = GraphRelationship {
            subject: subject.to_string(),
            relation: relation.to_string(),
            object: object.to_string(),
            confidence,
        };
        if !relationships.contains(&relationship) {
            relationships.push(relationship);
        }
    }

    GraphContext { entities, relationships }
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

    fn scope() -> Scope {
        Scope {
            agent_id: "agent".to_string(),
            org_id: "org".to_string(),
            user_id: "user".to_string(),
        }
    }

    fn row(pairs: &[(&str, &str)]) -> GraphRow {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    /// Returns staged rows for any query, recording the (cypher, params) call.
    #[derive(Default)]
    struct StagedStore {
        rows: Mutex<GraphRows>,
        calls: Mutex<Vec<(String, HashMap<String, String>)>>,
    }

    impl StagedStore {
        fn with_rows(rows: GraphRows) -> Self {
            Self {
                rows: Mutex::new(rows),
                calls: Mutex::default(),
            }
        }

        fn calls(&self) -> Vec<(String, HashMap<String, String>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl GraphStore for StagedStore {
        async fn ensure_graph(&self) -> Result<(), GraphError> {
            Ok(())
        }

        async fn query(&self, cypher: &str, params: &HashMap<String, String>) -> Result<GraphRows, GraphError> {
            self.calls.lock().unwrap().push((cypher.to_string(), params.clone()));
            Ok(self.rows.lock().unwrap().clone())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_empty_for_no_seeds() {
        let store = StagedStore::default();
        let ctx = neighbors(&store, &[], &scope(), 1).await.unwrap();
        assert!(ctx.is_empty());
        assert!(store.calls().is_empty(), "no seeds -> no query");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bind_seeds_and_scope_as_params() {
        let store = StagedStore::default();
        neighbors(&store, &["mem1", "mem2"], &scope(), 1).await.unwrap();

        let (cypher, params) = &store.calls()[0];
        assert!(!cypher.contains("mem1"), "pids must not be interpolated");
        assert_eq!(params.get("pid0").map(String::as_str), Some("mem1"));
        assert_eq!(params.get("pid1").map(String::as_str), Some("mem2"));
        assert_eq!(params.get("agent_id").map(String::as_str), Some("agent"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_filter_current_edges_only() {
        let store = StagedStore::default();
        neighbors(&store, &["mem1"], &scope(), 1).await.unwrap();
        assert!(store.calls()[0].0.contains("edge.valid_to IS NULL"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_clamp_depth_into_range() {
        let store = StagedStore::default();
        neighbors(&store, &["mem1"], &scope(), 99).await.unwrap();
        assert!(
            store.calls()[0].0.contains(&format!("*1..{MAX_ENRICHMENT_DEPTH}")),
            "depth clamps to the max",
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_build_deduped_context_from_rows() {
        let store = StagedStore::with_rows(vec![
            row(&[
                ("subject", "Alice"),
                ("relation", "works at"),
                ("object", "Acme"),
                ("confidence", "0.9"),
                ("related_name", "Acme"),
            ]),
            // duplicate relationship + entity — must dedup
            row(&[
                ("subject", "Alice"),
                ("relation", "works at"),
                ("object", "Acme"),
                ("confidence", "0.9"),
                ("related_name", "Acme"),
            ]),
        ]);

        let ctx = neighbors(&store, &["mem1"], &scope(), 1).await.unwrap();

        assert_eq!(ctx.relationships.len(), 1);
        assert_eq!(ctx.relationships[0].object, "Acme");
        assert_eq!(ctx.entities.len(), 1);
        assert_eq!(ctx.entities[0].name, "Acme");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_default_confidence_when_unparseable() {
        let store = StagedStore::with_rows(vec![row(&[
            ("subject", "Alice"),
            ("relation", "knows"),
            ("object", "Bob"),
            ("confidence", "null"),
            ("related_name", "Bob"),
        ])]);

        let ctx = neighbors(&store, &["mem1"], &scope(), 1).await.unwrap();
        assert_eq!(ctx.relationships[0].confidence, 1.0);
    }
}
