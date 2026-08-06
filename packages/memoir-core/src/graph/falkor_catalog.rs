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

use chrono::DateTime;

use crate::memory::Scope;

use super::{
    EdgeCatalog, EdgeError, EntityCatalog, EntityVector, ExistingEdge, FalkorGraphStore, GraphParam, GraphRow,
    GraphStore, ResolveError,
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

        Ok(rows.iter().filter_map(EntityVector::from_row).collect())
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
             -[r {relation: $relation}]->(o:Entity {agent_id: $agent_id, org_id: $org_id, user_id: $user_id}) \
             WHERE r.valid_to IS NULL \
             RETURN s.name AS subject, r.relation AS relation, o.name AS object, r.valid_from AS valid_from";

        let mut params = scope_params(scope);
        params.insert("subject".to_string(), subject_key.into());
        params.insert("relation".to_string(), relation.into());

        let rows = self
            .store
            .query(cypher, &params)
            .await
            .map_err(|err| EdgeError::Catalog(err.to_string()))?;

        Ok(rows.iter().filter_map(ExistingEdge::from_row).collect())
    }
}

impl EntityVector {
    /// Parses an entity from a `(name, embedding)` result row.
    ///
    /// Rows missing either column, or whose embedding is not the JSON array
    /// the commit wrote, yield `None` — one malformed node should not break
    /// resolution for the rest of the scope.
    fn from_row(row: &GraphRow) -> Option<Self> {
        let name = row.column("name")?.to_string();
        Some(Self {
            key: name.clone(),
            name,
            embedding: serde_json::from_str(row.column("embedding")?).ok()?,
        })
    }
}

impl ExistingEdge {
    /// Parses a current edge from a result row.
    ///
    /// `valid_to` is `None` by construction — the query matches only current
    /// edges. A missing column or unparseable `valid_from` yields `None`.
    fn from_row(row: &GraphRow) -> Option<Self> {
        Some(Self {
            subject_key: row.column("subject")?.to_string(),
            relation: row.column("relation")?.to_string(),
            object_key: row.column("object")?.to_string(),
            valid_from: DateTime::parse_from_rfc3339(row.column("valid_from")?).ok()?,
            valid_to: None,
        })
    }
}

/// Builds the scope parameter map shared by the catalog reads.
fn scope_params(scope: &Scope) -> HashMap<String, GraphParam> {
    HashMap::from([
        ("agent_id".to_string(), scope.agent_id_or_sentinel().into()),
        ("org_id".to_string(), scope.org_id_or_sentinel().into()),
        ("user_id".to_string(), scope.user_id().into()),
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
        let parsed = EntityVector::from_row(&row(&[("name", "Alice"), ("embedding", "[0.1,0.2,0.3]")])).unwrap();
        assert_eq!(parsed.name, "Alice");
        assert_eq!(parsed.key, "Alice");
        assert_eq!(parsed.embedding, vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn should_skip_entity_with_malformed_embedding() {
        assert!(EntityVector::from_row(&row(&[("name", "Alice"), ("embedding", "not json")])).is_none());
    }

    #[test]
    fn should_skip_entity_missing_a_column() {
        assert!(EntityVector::from_row(&row(&[("name", "Alice")])).is_none());
    }

    #[test]
    fn should_parse_current_edge_with_identity_tuple() {
        let parsed = ExistingEdge::from_row(&row(&[
            ("subject", "Alice"),
            ("relation", "works at"),
            ("object", "Acme"),
            ("valid_from", "2026-06-01T00:00:00+00:00"),
        ]))
        .unwrap();
        assert_eq!(parsed.subject_key, "Alice");
        assert_eq!(parsed.object_key, "Acme");
        assert_eq!(parsed.valid_from.to_rfc3339(), "2026-06-01T00:00:00+00:00");
        assert!(parsed.valid_to.is_none());
    }

    #[test]
    fn should_skip_edge_with_unparseable_valid_from() {
        assert!(
            ExistingEdge::from_row(&row(&[
                ("subject", "Alice"),
                ("relation", "works at"),
                ("object", "Acme"),
                ("valid_from", "null"),
            ]))
            .is_none()
        );
    }
}
