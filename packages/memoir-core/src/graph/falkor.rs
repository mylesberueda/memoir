use std::collections::HashMap;

use falkordb::{FalkorAsyncClient, FalkorClientBuilder, FalkorConnectionInfo, FalkorValue, LazyResultSet};

use super::{GraphError, GraphParam, GraphRows, GraphStore};

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

    /// Renders a single FalkorDB value to the string result surface.
    ///
    /// The encoding contract for everything crossing [`GraphStore::query`]'s
    /// string-scalar rows: scalars render plain, a null property renders as the
    /// `"null"` sentinel readers test for, and a list property (the shape
    /// `memory_pids` is stored in) renders as a JSON array so it parses back
    /// losslessly. Other shapes (node, edge, path) render to their debug form;
    /// no read projects them today.
    fn render_value(value: &FalkorValue) -> String {
        match value {
            FalkorValue::String(s) => s.clone(),
            FalkorValue::I64(n) => n.to_string(),
            FalkorValue::Bool(b) => b.to_string(),
            FalkorValue::F64(f) => f.to_string(),
            FalkorValue::None => "null".to_string(),
            FalkorValue::Array(items) => {
                let rendered: Vec<String> = items.iter().map(Self::render_value).collect();
                serde_json::to_string(&rendered).expect("serializing Vec<String> to JSON cannot fail")
            }
            other => format!("{other:?}"),
        }
    }

    /// Substitutes each `$name` reference in `cypher` with its bound literal.
    ///
    /// falkordb 0.2.1's `with_params` builds a space-delimited `CYPHER k=v`
    /// preamble that FalkorDB splits on whitespace, so any value containing a
    /// space (an embedding's JSON array, a multi-word name, an RFC 3339
    /// timestamp) corrupts the parse. Binding instead happens here, inlining each
    /// value as an escaped Cypher literal ([`GraphParam::to_cypher_literal`]) so
    /// Cypher's own parser handles it — the escaping is the injection guard.
    ///
    /// Keys are applied longest-first so a `$relation` binding never partially
    /// overwrites a `$relation_key` reference, and each match requires a word
    /// boundary after the key so `$pid` does not match inside `$pid0`.
    fn bind_params(cypher: &str, params: &HashMap<String, GraphParam>) -> String {
        let mut keys: Vec<&String> = params.keys().collect();
        keys.sort_by_key(|k| std::cmp::Reverse(k.len()));

        let mut bound = cypher.to_string();
        for key in keys {
            let literal = params[key].to_cypher_literal();
            bound = replace_param(&bound, key, &literal);
        }
        bound
    }
}

/// Replaces every `$key` token in `cypher` with `literal`, respecting boundaries.
///
/// A `$key` matches only when the character following the key is not an
/// identifier character, so `$pid` does not match the `$pid0` prefix.
fn replace_param(cypher: &str, key: &str, literal: &str) -> String {
    let token = format!("${key}");
    let mut out = String::with_capacity(cypher.len());
    let mut rest = cypher;
    while let Some(at) = rest.find(&token) {
        let after = at + token.len();
        let boundary = rest[after..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_ascii_alphanumeric() && c != '_');
        out.push_str(&rest[..at]);
        if boundary {
            out.push_str(literal);
        } else {
            out.push_str(&rest[at..after]);
        }
        rest = &rest[after..];
    }
    out.push_str(rest);
    out
}

impl GraphStore for FalkorGraphStore {
    async fn ensure_graph(&self) -> Result<(), GraphError> {
        self.client
            .list_graphs()
            .await
            .map(|_| ())
            .map_err(|e| GraphError::Connection(e.to_string()))
    }

    async fn query(&self, cypher: &str, params: &HashMap<String, GraphParam>) -> Result<GraphRows, GraphError> {
        let bound = Self::bind_params(cypher, params);
        let mut graph = self.client.select_graph(&self.graph_name);
        let mut result = graph
            .query(&bound)
            .execute()
            .await
            .map_err(|e| {
                tracing::debug!(bound_query = %bound, "falkordb query failed");
                GraphError::Query(e.to_string())
            })?;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn params(pairs: &[(&str, GraphParam)]) -> HashMap<String, GraphParam> {
        pairs.iter().map(|(k, v)| ((*k).to_string(), v.clone())).collect()
    }

    #[test]
    fn should_inline_string_value_as_quoted_literal() {
        let bound = FalkorGraphStore::bind_params(
            "MERGE (e {name: $name})",
            &params(&[("name", GraphParam::Str("Alice".to_string()))]),
        );
        assert_eq!(bound, "MERGE (e {name: 'Alice'})");
    }

    #[test]
    fn should_inline_space_bearing_value_without_corrupting_query() {
        // The bug: a space in the value broke falkordb's CYPHER-preamble split.
        // Inlined as a quoted literal, the space is harmless to Cypher's parser.
        let bound = FalkorGraphStore::bind_params(
            "MERGE (e {name: $name})",
            &params(&[("name", GraphParam::Str("Alice Smith".to_string()))]),
        );
        assert_eq!(bound, "MERGE (e {name: 'Alice Smith'})");
    }

    #[test]
    fn should_not_let_longer_key_be_clobbered_by_a_prefix_key() {
        // $relation must not partially overwrite $relation_key.
        let bound = FalkorGraphStore::bind_params(
            "SET r.relation = $relation WHERE k = $relation_key",
            &params(&[
                ("relation", GraphParam::Str("works at".to_string())),
                ("relation_key", GraphParam::Str("e123".to_string())),
            ]),
        );
        assert_eq!(bound, "SET r.relation = 'works at' WHERE k = 'e123'");
    }

    #[test]
    fn should_not_match_key_inside_a_longer_reference() {
        // $pid must not bind inside $pid0.
        let bound = FalkorGraphStore::bind_params(
            "WHERE p IN [$pid0, $pid]",
            &params(&[
                ("pid", GraphParam::Str("mem_b".to_string())),
                ("pid0", GraphParam::Str("mem_a".to_string())),
            ]),
        );
        assert_eq!(bound, "WHERE p IN ['mem_a', 'mem_b']");
    }

    #[test]
    fn should_inline_int_param_bare_for_limit() {
        let bound = FalkorGraphStore::bind_params("RETURN n LIMIT $lim", &params(&[("lim", GraphParam::Int(500))]));
        assert_eq!(bound, "RETURN n LIMIT 500");
    }

    #[test]
    fn should_replace_every_occurrence_of_a_key() {
        let bound = FalkorGraphStore::bind_params(
            "SET a = $pid WHERE b = $pid",
            &params(&[("pid", GraphParam::Str("mem1".to_string()))]),
        );
        assert_eq!(bound, "SET a = 'mem1' WHERE b = 'mem1'");
    }

    #[test]
    fn should_render_null_value_as_the_null_sentinel() {
        // A current edge's absent valid_to comes back as FalkorValue::None and
        // must hit the "null" sentinel present()-style readers test for.
        assert_eq!(FalkorGraphStore::render_value(&FalkorValue::None), "null");
    }

    #[test]
    fn should_render_list_value_as_json_array() {
        // memory_pids is stored as a native Cypher list; it must read back as
        // JSON so parse_pids round-trips it.
        let value = FalkorValue::Array(vec![
            FalkorValue::String("mem1".to_string()),
            FalkorValue::String("mem2".to_string()),
        ]);
        assert_eq!(FalkorGraphStore::render_value(&value), r#"["mem1","mem2"]"#);
    }

    #[test]
    fn should_bind_every_param_in_the_real_node_merge() {
        // The production node MERGE: every $ref must resolve, with no $token left
        // unbound (an unbound $ref is null to FalkorDB -> "merge node using null
        // property value"). Mirrors graph/commit.rs upsert_node.
        let cypher = "MERGE (e:Entity {agent_id: $agent_id, org_id: $org_id, user_id: $user_id, name: $name}) \
             ON CREATE SET e.first_seen_at = $now, e.embedding = $embedding, e.memory_pids = [$pid] \
             ON MATCH SET e.memory_pids = \
               CASE WHEN $pid IN e.memory_pids THEN e.memory_pids ELSE e.memory_pids + $pid END";
        let bound = FalkorGraphStore::bind_params(
            cypher,
            &params(&[
                ("agent_id", GraphParam::Str("agent_x".to_string())),
                ("org_id", GraphParam::Str("org_x".to_string())),
                ("user_id", GraphParam::Str("user_x".to_string())),
                ("name", GraphParam::Str("Alice Smith".to_string())),
                ("now", GraphParam::Str("2026-06-10T01:00:00+00:00".to_string())),
                ("embedding", GraphParam::Str("[0.1, 0.2, 0.3]".to_string())),
                ("pid", GraphParam::Str("mem1".to_string())),
            ]),
        );
        assert!(!bound.contains('$'), "no unbound $ref may remain: {bound}");
        assert!(bound.contains("name: 'Alice Smith'"), "name bound as a quoted literal: {bound}");
    }
}
