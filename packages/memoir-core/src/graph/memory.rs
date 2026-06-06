use std::sync::Mutex;

use super::{GraphError, GraphRows, GraphStore};

/// In-memory [`GraphStore`] for tests and benchmarks, with no live backend.
///
/// This is the trait's test/benchmark boundary, letting callers exercise the
/// graph seam without a running FalkorDB. It does not interpret Cypher — there
/// is no graph engine here — so [`GraphStore::query`] returns whatever rows
/// were staged via [`InMemoryGraphStore::stage_rows`].
///
/// # Examples
///
/// ```
/// use memoir_core::graph::{GraphStore, InMemoryGraphStore};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let store = InMemoryGraphStore::new();
/// store.ensure_graph().await?;
/// store.stage_rows(vec![vec![("n".to_string(), "Alice".to_string())]]);
/// let rows = store.query("MATCH (n) RETURN n").await?;
/// assert_eq!(rows[0][0].1, "Alice");
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct InMemoryGraphStore {
    rows: Mutex<GraphRows>,
}

impl InMemoryGraphStore {
    /// Creates an empty in-memory graph store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Stages the rows that the next [`GraphStore::query`] call returns.
    ///
    /// A test affordance: since there is no Cypher engine, this lets a test
    /// arrange the result a query observes. Replaces any previously staged rows.
    pub fn stage_rows(&self, rows: GraphRows) {
        *self.rows.lock().expect("graph store mutex poisoned") = rows;
    }
}

impl GraphStore for InMemoryGraphStore {
    async fn ensure_graph(&self) -> Result<(), GraphError> {
        Ok(())
    }

    async fn query(&self, _cypher: &str) -> Result<GraphRows, GraphError> {
        Ok(self.rows.lock().expect("graph store mutex poisoned").clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_staged_rows_from_query() {
        let store = InMemoryGraphStore::new();
        store.ensure_graph().await.unwrap();
        store.stage_rows(vec![vec![("n".to_string(), "Alice".to_string())]]);

        let rows = store.query("MATCH (n) RETURN n").await.unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], ("n".to_string(), "Alice".to_string()));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_return_empty_when_nothing_staged() {
        let store = InMemoryGraphStore::new();
        let rows = store.query("MATCH (n) RETURN n").await.unwrap();
        assert!(rows.is_empty());
    }
}
