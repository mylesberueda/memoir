use std::collections::HashMap;

use falkordb::{FalkorAsyncClient, FalkorClientBuilder, FalkorConnectionInfo, FalkorValue, LazyResultSet};

use super::{GraphError, GraphRows, GraphStore};

/// FalkorDB-backed [`GraphStore`] for the production knowledge graph.
///
/// Owns an async FalkorDB connection and confines every query to a single named
/// graph so memoir coexists with a host application's own graphs in one shared
/// instance. Construct it with [`FalkorGraphStore::connect`], passing the
/// caller-owned connection string and the graph name to confine writes to.
///
/// Only compiled with the `knowledge-graph` feature.
pub struct FalkorGraphStore {
    client: FalkorAsyncClient,
    graph_name: String,
}

impl FalkorGraphStore {
    /// Connects to FalkorDB at `connection_url`, confined to `graph_name`.
    ///
    /// `connection_url` is a FalkorDB/Redis-protocol endpoint (e.g.
    /// `redis://localhost:63792`). Pass [`super::DEFAULT_GRAPH_NAME`] for `graph_name`
    /// unless a deployment overrides it. The connection is established eagerly so
    /// a bad endpoint fails here rather than on first query.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Connection`] if the connection string is malformed
    /// or the backend is unreachable.
    pub async fn connect(connection_url: impl Into<String>, graph_name: impl Into<String>) -> Result<Self, GraphError> {
        let connection_url = connection_url.into();
        let info: FalkorConnectionInfo = connection_url
            .as_str()
            .try_into()
            .map_err(|e| GraphError::Connection(format!("invalid FalkorDB connection url: {e}")))?;

        let client = FalkorClientBuilder::new_async()
            .with_connection_info(info)
            .build()
            .await
            .map_err(|e| GraphError::Connection(e.to_string()))?;

        Ok(Self {
            client,
            graph_name: graph_name.into(),
        })
    }

    /// Renders a single FalkorDB scalar value to a `String`.
    ///
    /// Keeps backend value types from leaking past [`GraphStore`]'s public
    /// surface. Non-scalar values (node, edge, path) render to their debug form.
    fn render_value(value: &FalkorValue) -> String {
        match value {
            FalkorValue::String(s) => s.clone(),
            FalkorValue::I64(n) => n.to_string(),
            FalkorValue::Bool(b) => b.to_string(),
            FalkorValue::F64(f) => f.to_string(),
            other => format!("{other:?}"),
        }
    }
}

impl GraphStore for FalkorGraphStore {
    async fn ensure_graph(&self) -> Result<(), GraphError> {
        self.client
            .list_graphs()
            .await
            .map(|_| ())
            .map_err(|e| GraphError::Connection(e.to_string()))
    }

    async fn query(&self, cypher: &str, params: &HashMap<String, String>) -> Result<GraphRows, GraphError> {
        let mut graph = self.client.select_graph(&self.graph_name);
        let builder = graph.query(cypher);
        let builder = if params.is_empty() {
            builder
        } else {
            builder.with_params(params)
        };
        let mut result = builder.execute().await.map_err(|e| GraphError::Query(e.to_string()))?;

        let header: Vec<String> = result.header.iter().map(ToString::to_string).collect();

        let rows = collect_rows(&header, &mut result.data);
        Ok(rows)
    }
}

/// Drains a [`LazyResultSet`] into owned column-labeled rows.
fn collect_rows(header: &[String], data: &mut LazyResultSet<'_>) -> GraphRows {
    let mut rows = GraphRows::new();
    for record in data.by_ref() {
        let row = header
            .iter()
            .cloned()
            .zip(record.iter().map(FalkorGraphStore::render_value))
            .collect();
        rows.push(row);
    }
    rows
}
