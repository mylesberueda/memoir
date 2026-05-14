//! [`VectorIndex`] implementation backed by Qdrant.

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, DeletePointsBuilder, Distance, Filter, PointId, PointStruct,
    QueryPointsBuilder, UpsertPointsBuilder, Value, VectorParamsBuilder, point_id::PointIdOptions,
};

use super::{VectorError, VectorIndex};
use crate::memory::{KindSelector, MemoryKind, Scope};

const DEFAULT_COLLECTION: &str = "memoir_memories";

/// Default [`VectorIndex`] backed by Qdrant.
///
/// Constructed via [`Self::new`]. Collection name defaults to
/// `memoir_memories`; override with [`Self::with_collection`].
#[derive(Clone)]
pub struct QdrantIndex {
    qdrant: Qdrant,
    collection: String,
}

impl std::fmt::Debug for QdrantIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QdrantIndex")
            .field("collection", &self.collection)
            .finish_non_exhaustive()
    }
}

impl QdrantIndex {
    /// Builds an index from an existing Qdrant client.
    pub fn new(qdrant: Qdrant) -> Self {
        Self {
            qdrant,
            collection: DEFAULT_COLLECTION.to_string(),
        }
    }

    /// Sets the Qdrant collection name used for vector storage.
    pub fn with_collection(mut self, collection: impl Into<String>) -> Self {
        self.collection = collection.into();
        self
    }

    /// Returns the Qdrant collection name configured for this index.
    pub fn collection_name(&self) -> &str {
        &self.collection
    }
}

impl VectorIndex for QdrantIndex {
    async fn ensure_collection(&self, vector_dim: usize) -> Result<(), VectorError> {
        let exists = self
            .qdrant
            .collection_exists(&self.collection)
            .await
            .map_err(connection)?;
        if exists {
            return Ok(());
        }

        self.qdrant
            .create_collection(
                CreateCollectionBuilder::new(&self.collection)
                    .vectors_config(VectorParamsBuilder::new(vector_dim as u64, Distance::Cosine)),
            )
            .await
            .map_err(connection)?;
        Ok(())
    }

    async fn upsert(
        &self,
        pid: &str,
        scope: &Scope,
        kind: MemoryKind,
        vector: Vec<f32>,
    ) -> Result<(), VectorError> {
        let payload = [
            ("agent_id", Value::from(scope.agent_id.clone())),
            ("org_id", Value::from(scope.org_id.clone())),
            ("user_id", Value::from(scope.user_id.clone())),
            ("kind", Value::from(kind.as_str().to_string())),
        ];

        let point = PointStruct::new(pid.to_string(), vector, payload);

        self.qdrant
            .upsert_points(UpsertPointsBuilder::new(&self.collection, vec![point]))
            .await
            .map_err(connection)?;

        Ok(())
    }

    async fn search(
        &self,
        scope: Scope,
        query_embedding: Vec<f32>,
        limit: usize,
        kinds: KindSelector,
    ) -> Result<Vec<(String, f32)>, VectorError> {
        if kinds.is_empty() {
            return Ok(Vec::new());
        }

        let mut conditions = vec![
            Condition::matches("agent_id", scope.agent_id),
            Condition::matches("org_id", scope.org_id),
            Condition::matches("user_id", scope.user_id),
        ];
        if !kinds.includes_all() {
            let names: Vec<String> = kinds
                .included_kinds()
                .into_iter()
                .map(|k| k.as_str().to_string())
                .collect();
            conditions.push(Condition::matches("kind", names));
        }
        let filter = Filter::must(conditions);

        let response = self
            .qdrant
            .query(
                QueryPointsBuilder::new(&self.collection)
                    .query(query_embedding)
                    .limit(limit as u64)
                    .filter(filter)
                    .with_payload(false),
            )
            .await
            .map_err(connection)?;

        let mut hits = Vec::with_capacity(response.result.len());
        for scored in response.result {
            if let Some(id) = scored.id.and_then(point_id_to_string) {
                hits.push((id, scored.score));
            }
        }
        Ok(hits)
    }

    async fn delete_by_pids(&self, pids: &[&str]) -> Result<(), VectorError> {
        if pids.is_empty() {
            return Ok(());
        }

        let point_ids: Vec<PointId> = pids.iter().map(|p| PointId::from((*p).to_string())).collect();
        self.qdrant
            .delete_points(DeletePointsBuilder::new(&self.collection).points(point_ids))
            .await
            .map_err(connection)?;
        Ok(())
    }
}

fn connection<E: std::fmt::Display>(err: E) -> VectorError {
    VectorError::Connection(err.to_string())
}

fn point_id_to_string(id: PointId) -> Option<String> {
    match id.point_id_options? {
        PointIdOptions::Uuid(s) => Some(s),
        PointIdOptions::Num(n) => Some(n.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_uuid_point_id_to_string() {
        let id = PointId {
            point_id_options: Some(PointIdOptions::Uuid("abc123".to_string())),
        };
        assert_eq!(point_id_to_string(id), Some("abc123".to_string()));
    }

    #[test]
    fn should_convert_numeric_point_id_to_string() {
        let id = PointId {
            point_id_options: Some(PointIdOptions::Num(42)),
        };
        assert_eq!(point_id_to_string(id), Some("42".to_string()));
    }
}
