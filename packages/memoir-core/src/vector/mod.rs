//! Vector-index abstraction for similarity search.
//!
//! Defines [`VectorIndex`], implemented by [`QdrantIndex`] (the default) and
//! by callers who want to plug in a different vector backend. Memoir's
//! source-of-truth row storage is a separate concern handled by
//! [`crate::store::MemoryStore`]; this trait covers only the vector index.

mod error;
mod filter;
pub mod qdrant;

pub use error::VectorError;
pub use filter::{FilterCondition, MatchValue, MatchValues, MemoryFilter, NumericRange};
pub use qdrant::QdrantIndex;

use std::future::Future;

use crate::memory::{KindSelector, MemoryKind, Scope};

/// Stores and queries vectors keyed by memory pid.
///
/// Implementations own the vector-backend connection. The trait methods are
/// async and `Send`-bound so callers can drive them from any tokio runtime.
pub trait VectorIndex: Send + Sync + 'static {
    /// Ensures the backing collection exists with the configured dimension.
    ///
    /// Idempotent: callers invoke this on startup; second-call is a no-op.
    /// `vector_dim` must match the dimension produced by the embedding model
    /// the consumer pairs with this index.
    ///
    /// # Errors
    ///
    /// Returns [`VectorError::Connection`] if the backend is unreachable,
    /// [`VectorError::BadRequest`] if the collection exists with a
    /// different vector dimension than requested.
    fn ensure_collection(
        &self,
        vector_dim: usize,
    ) -> impl Future<Output = Result<(), VectorError>> + Send;

    /// Upserts a single memory's vector + scope payload.
    ///
    /// The pid is the same value persisted in the source-of-truth store; the
    /// caller is responsible for ensuring the corresponding row exists before
    /// the upsert completes (the [`crate::store::IndexStatus::Pending`]
    /// lifecycle state covers the gap).
    ///
    /// # Errors
    ///
    /// Returns [`VectorError::Connection`] on backend errors and
    /// [`VectorError::BadRequest`] when the vector's dimension does not
    /// match the collection's.
    fn upsert(
        &self,
        pid: &str,
        scope: &Scope,
        kind: MemoryKind,
        vector: Vec<f32>,
    ) -> impl Future<Output = Result<(), VectorError>> + Send;

    /// Returns the top similarity hits within `scope`, filtered by kind.
    ///
    /// Returns pid+score tuples ordered by descending score. The caller
    /// hydrates these into full [`crate::memory::Memory`] values via
    /// [`crate::store::MemoryStore::find_by_pids`].
    ///
    /// `extra_filter` AND-joins with the scope + kind filter — caller-supplied
    /// conditions cannot widen scope. An empty filter (or `None`) is inert.
    /// `min_similarity` sets a score floor; hits below it are dropped by the
    /// backend before they reach the result. `None` applies no floor.
    ///
    /// # Errors
    ///
    /// Returns [`VectorError::Connection`] on backend errors and
    /// [`VectorError::BadRequest`] when the query vector's dimension does
    /// not match the collection's.
    fn search(
        &self,
        scope: Scope,
        query_embedding: Vec<f32>,
        limit: usize,
        kinds: KindSelector,
        extra_filter: Option<MemoryFilter>,
        min_similarity: Option<f32>,
    ) -> impl Future<Output = Result<Vec<(String, f32)>, VectorError>> + Send;

    /// Deletes vectors for the given pids.
    ///
    /// Best-effort: failures are not propagated up to user-facing requests
    /// in the canonical Forget flow. The caller decides whether to surface
    /// errors (e.g. reconciliation propagates; user-facing Forget logs).
    ///
    /// # Errors
    ///
    /// Returns [`VectorError::Connection`] on backend errors.
    fn delete_by_pids(
        &self,
        pids: &[&str],
    ) -> impl Future<Output = Result<(), VectorError>> + Send;

    /// Returns every pid in the index that matches `scope`.
    ///
    /// Used by the reconciliation sweep's orphan-cleanup pass. Implementations
    /// paginate internally using `page_size` and concatenate the result.
    ///
    /// # Errors
    ///
    /// Returns [`VectorError::Connection`] on backend errors.
    fn list_pids_in_scope(
        &self,
        scope: Scope,
        page_size: usize,
    ) -> impl Future<Output = Result<Vec<String>, VectorError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Default)]
    struct StubIndex {
        points: Mutex<HashMap<String, (Scope, MemoryKind, Vec<f32>)>>,
    }

    impl VectorIndex for StubIndex {
        async fn ensure_collection(&self, _vector_dim: usize) -> Result<(), VectorError> {
            Ok(())
        }

        async fn upsert(
            &self,
            pid: &str,
            scope: &Scope,
            kind: MemoryKind,
            vector: Vec<f32>,
        ) -> Result<(), VectorError> {
            self.points
                .lock()
                .unwrap()
                .insert(pid.to_string(), (scope.clone(), kind, vector));
            Ok(())
        }

        async fn search(
            &self,
            _scope: Scope,
            _query_embedding: Vec<f32>,
            limit: usize,
            _kinds: KindSelector,
            _extra_filter: Option<MemoryFilter>,
            _min_similarity: Option<f32>,
        ) -> Result<Vec<(String, f32)>, VectorError> {
            Ok(self
                .points
                .lock()
                .unwrap()
                .keys()
                .take(limit)
                .map(|pid| (pid.clone(), 0.5))
                .collect())
        }

        async fn delete_by_pids(&self, pids: &[&str]) -> Result<(), VectorError> {
            let mut points = self.points.lock().unwrap();
            for pid in pids {
                points.remove(*pid);
            }
            Ok(())
        }

        async fn list_pids_in_scope(
            &self,
            scope: Scope,
            _page_size: usize,
        ) -> Result<Vec<String>, VectorError> {
            Ok(self
                .points
                .lock()
                .unwrap()
                .iter()
                .filter(|(_, (s, _, _))| s == &scope)
                .map(|(pid, _)| pid.clone())
                .collect())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_implement_trait_with_in_test_stub() {
        let index = StubIndex::default();
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "o".to_string(),
            user_id: "u".to_string(),
        };

        index.ensure_collection(4).await.unwrap();
        index
            .upsert("pid1", &scope, MemoryKind::Episodic, vec![0.1, 0.2, 0.3, 0.4])
            .await
            .unwrap();

        let hits = index
            .search(scope, vec![0.1, 0.2, 0.3, 0.4], 5, KindSelector::default(), None, None)
            .await
            .unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].0, "pid1");

        index.delete_by_pids(&["pid1"]).await.unwrap();
    }
}
