//! FalkorDB-backed reads for the entity and edge resolution seams.
//!
//! [`FalkorEntityCatalog`] and [`FalkorEdgeCatalog`] implement the
//! [`EntityCatalog`](super::EntityCatalog) and [`EdgeCatalog`](super::EdgeCatalog)
//! traits by reading the nodes and edges the commit path
//! ([`super::commit_triples`]) writes. They are the production backings the
//! resolvers (ticket 0005/0006) run against, replacing the in-memory catalogs
//! used in tests.
//!
//! Both share one [`FalkorGraphStore`] and issue parameterized reads through it,
//! confined to a [`Scope`]. Entity embeddings are stored as JSON strings on the
//! node (see [`super::commit`]); this module parses them back to `Vec<f32>` for
//! cosine matching. Only compiled with the `knowledge-graph` feature.

use std::collections::HashMap;
use std::sync::Arc;

use crate::memory::Scope;

use super::{
    EdgeCatalog, EdgeError, EntityCatalog, EntityVector, ExistingEdge, FalkorGraphStore, GraphRow, GraphStore,
    ResolveError,
};

/// Reads candidate entity nodes from FalkorDB for [`EntityCatalog`].
///
/// Returns every `:Entity` node in a scope with its name and parsed embedding,
/// which the embedding resolver scores by cosine. The node's canonical name is
/// its key (unique within a scope by the commit's `MERGE`-on-properties).
#[derive(Clone)]
pub struct FalkorEntityCatalog {
    store: Arc<FalkorGraphStore>,
}

impl FalkorEntityCatalog {
    /// Builds an entity catalog reading from `store`.
    pub fn new(store: Arc<FalkorGraphStore>) -> Self {
        Self { store }
    }
}

impl EntityCatalog for FalkorEntityCatalog {
    async fn candidates_in_scope(&self, scope: &Scope) -> Result<Vec<EntityVector>, ResolveError> {
        let cypher = "MATCH (e:Entity {agent_id: $agent_id, org_id: $org_id, user_id: $user_id}) \
             RETURN e.name AS name, e.embedding AS embedding";
        let rows = self
            .store
            .query(cypher, &scope_params(scope))
            .await
            .map_err(|err| ResolveError::Catalog(err.to_string()))?;

        Ok(rows.iter().filter_map(entity_from_row).collect())
    }
}

/// Reads current relationship edges from FalkorDB for [`EdgeCatalog`].
///
/// Returns only edges whose `valid_to` is null (current, not superseded) sharing
/// the given subject and relation within a scope — the set the temporal resolver
/// reconciles a new edge against.
#[derive(Clone)]
pub struct FalkorEdgeCatalog {
    store: Arc<FalkorGraphStore>,
}

impl FalkorEdgeCatalog {
    /// Builds an edge catalog reading from `store`.
    pub fn new(store: Arc<FalkorGraphStore>) -> Self {
        Self { store }
    }
}

impl EdgeCatalog for FalkorEdgeCatalog {
    async fn current_edges(
        &self,
        scope: &Scope,
        subject_key: &str,
        relation: &str,
    ) -> Result<Vec<ExistingEdge>, EdgeError> {
        let cypher = "MATCH (s:Entity {agent_id: $agent_id, org_id: $org_id, user_id: $user_id, name: $subject}) \
             -[r {relation: $relation, valid_to: null}]->(o:Entity) \
             RETURN id(r) AS key, s.name AS subject, r.relation AS relation, o.name AS object";

        let mut params = scope_params(scope);
        params.insert("subject".to_string(), subject_key.to_string());
        params.insert("relation".to_string(), relation.to_string());

        let rows = self
            .store
            .query(cypher, &params)
            .await
            .map_err(|err| EdgeError::Catalog(err.to_string()))?;

        Ok(rows.iter().filter_map(existing_edge_from_row).collect())
    }
}

/// Parses an `EntityVector` from a `(name, embedding)` result row.
///
/// Rows missing either column, or whose embedding is not the JSON array the
/// commit wrote, are skipped rather than failing the whole read — one malformed
/// node should not break resolution for the rest of the scope.
fn entity_from_row(row: &GraphRow) -> Option<EntityVector> {
    let name = column(row, "name")?.to_string();
    let embedding_json = column(row, "embedding")?;
    let embedding: Vec<f32> = serde_json::from_str(embedding_json).ok()?;
    Some(EntityVector {
        key: name.clone(),
        name,
        embedding,
    })
}

/// Parses an `ExistingEdge` from a current-edge result row.
///
/// `valid_to` is `None` by construction — the query matches only current edges.
fn existing_edge_from_row(row: &GraphRow) -> Option<ExistingEdge> {
    Some(ExistingEdge {
        key: column(row, "key")?.to_string(),
        subject_key: column(row, "subject")?.to_string(),
        relation: column(row, "relation")?.to_string(),
        object_key: column(row, "object")?.to_string(),
        valid_to: None,
    })
}

/// Returns the value of the column named `name` in a result row.
fn column<'a>(row: &'a GraphRow, name: &str) -> Option<&'a str> {
    row.iter()
        .find(|(column, _)| column == name)
        .map(|(_, value)| value.as_str())
}

/// Builds the scope parameter map shared by the catalog reads.
fn scope_params(scope: &Scope) -> HashMap<String, String> {
    HashMap::from([
        ("agent_id".to_string(), scope.agent_id.clone()),
        ("org_id".to_string(), scope.org_id.clone()),
        ("user_id".to_string(), scope.user_id.clone()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pairs: &[(&str, &str)]) -> GraphRow {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn should_parse_entity_with_json_embedding() {
        let parsed = entity_from_row(&row(&[("name", "Alice"), ("embedding", "[0.1,0.2,0.3]")])).unwrap();
        assert_eq!(parsed.name, "Alice");
        assert_eq!(parsed.key, "Alice");
        assert_eq!(parsed.embedding, vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn should_skip_entity_with_malformed_embedding() {
        assert!(entity_from_row(&row(&[("name", "Alice"), ("embedding", "not json")])).is_none());
    }

    #[test]
    fn should_skip_entity_missing_a_column() {
        assert!(entity_from_row(&row(&[("name", "Alice")])).is_none());
    }

    #[test]
    fn should_parse_current_edge_with_null_valid_to() {
        let parsed = existing_edge_from_row(&row(&[
            ("key", "42"),
            ("subject", "Alice"),
            ("relation", "works at"),
            ("object", "Acme"),
        ]))
        .unwrap();
        assert_eq!(parsed.key, "42");
        assert_eq!(parsed.object_key, "Acme");
        assert!(parsed.valid_to.is_none());
    }
}
