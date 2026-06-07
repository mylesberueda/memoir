//! Reference-counted removal of forgotten memories from the graph.
//!
//! When a memory is forgotten its Postgres row and Qdrant vector are deleted;
//! the graph must stay consistent too. But a node or edge can be referenced by
//! many memories (the commit accumulates a pid per memory into `memory_pids`),
//! so removal is reference-counted: [`forget_pids`] strips the forgotten pid
//! from every node and edge and deletes only those whose `memory_pids` empties.
//! A whole-tenant forget instead wipes the scope outright ([`forget_scope`]).
//!
//! This is the Forget path, distinct from contradiction (which *invalidates* an
//! edge via `valid_to`, [`super::edge`]). Forget operates on `memory_pids`
//! regardless of an edge's `valid_to`, so a superseded edge is removed like any
//! other once its last referencing pid is gone.
//!
//! Removal is best-effort at the call site (a failure is logged, not surfaced —
//! the source-of-truth row is already gone and reconciliation is the backstop)
//! and idempotent here (re-forgetting an absent pid changes nothing).

//! The bodies here back the [`GraphStore::forget_pids`] / [`GraphStore::forget_scope`]
//! default methods; callers reach them as methods on the store, not directly.

use std::collections::HashMap;

use crate::memory::Scope;

use super::{GraphError, GraphStore};

/// Backs [`GraphStore::forget_pids`]; see that method for semantics.
pub(super) async fn forget_pids<G: GraphStore + ?Sized>(store: &G, pids: &[&str]) -> Result<(), GraphError> {
    for pid in pids {
        forget_one_pid(store, pid).await?;
    }
    Ok(())
}

/// Removes a single pid's references, edges first then isolated empty nodes.
async fn forget_one_pid<G: GraphStore + ?Sized>(store: &G, pid: &str) -> Result<(), GraphError> {
    let params = HashMap::from([("pid".to_string(), pid.to_string())]);

    // Strip the pid from matching edges, then delete those left with no pids.
    let edge_cypher = "MATCH ()-[r]->() WHERE $pid IN r.memory_pids \
         SET r.memory_pids = [p IN r.memory_pids WHERE p <> $pid] \
         WITH r WHERE size(r.memory_pids) = 0 \
         DELETE r";
    store.query(edge_cypher, &params).await?;

    // Strip the pid from matching nodes, then delete those left with no pids
    // AND no surviving edges — a node still joined by an other-pid edge stays.
    let node_cypher = "MATCH (n:Entity) WHERE $pid IN n.memory_pids \
         SET n.memory_pids = [p IN n.memory_pids WHERE p <> $pid] \
         WITH n WHERE size(n.memory_pids) = 0 AND NOT (n)--() \
         DELETE n";
    store.query(node_cypher, &params).await?;

    Ok(())
}

/// Backs [`GraphStore::forget_scope`]; see that method for semantics.
pub(super) async fn forget_scope<G: GraphStore + ?Sized>(store: &G, scope: &Scope) -> Result<(), GraphError> {
    let cypher = "MATCH (n:Entity {agent_id: $agent_id, org_id: $org_id, user_id: $user_id}) DETACH DELETE n";
    store.query(cypher, &scope_params(scope)).await?;
    Ok(())
}

/// Builds the scope parameter map shared by every forget statement.
fn scope_params(scope: &Scope) -> HashMap<String, String> {
    HashMap::from([
        ("agent_id".to_string(), scope.agent_id.clone()),
        ("org_id".to_string(), scope.org_id.clone()),
        ("user_id".to_string(), scope.user_id.clone()),
    ])
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::graph::{GraphRows, GraphStore};

    fn scope() -> Scope {
        Scope {
            agent_id: "agent".to_string(),
            org_id: "org".to_string(),
            user_id: "user".to_string(),
        }
    }

    /// Records every (cypher, params) call so tests assert what forget issues.
    #[derive(Default)]
    struct RecordingStore {
        calls: Mutex<Vec<(String, HashMap<String, String>)>>,
    }

    impl RecordingStore {
        fn calls(&self) -> Vec<(String, HashMap<String, String>)> {
            self.calls.lock().expect("recording store poisoned").clone()
        }
    }

    impl GraphStore for RecordingStore {
        async fn ensure_graph(&self) -> Result<(), GraphError> {
            Ok(())
        }

        async fn query(&self, cypher: &str, params: &HashMap<String, String>) -> Result<GraphRows, GraphError> {
            self.calls
                .lock()
                .expect("recording store poisoned")
                .push((cypher.to_string(), params.clone()));
            Ok(GraphRows::new())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_issue_edge_then_node_statements_per_pid() {
        let store = RecordingStore::default();

        forget_pids(&store, &["mem1"]).await.unwrap();

        let calls = store.calls();
        assert_eq!(calls.len(), 2);
        assert!(calls[0].0.contains("-[r]->"), "edges decremented first");
        assert!(calls[1].0.contains("(n:Entity"), "nodes decremented second");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bind_pid_as_param_without_scope_guard() {
        let store = RecordingStore::default();

        forget_pids(&store, &["mem1"]).await.unwrap();

        for (cypher, params) in store.calls() {
            assert!(!cypher.contains("mem1"), "pid must not be interpolated");
            assert_eq!(params.get("pid").map(String::as_str), Some("mem1"));
            // The pid is globally unique, so the pid path carries no scope.
            assert!(!cypher.contains("agent_id"), "pid path is not scope-confined");
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_guard_node_delete_on_empty_pids_and_no_edges() {
        let store = RecordingStore::default();

        forget_pids(&store, &["mem1"]).await.unwrap();

        let node_call = &store.calls()[1].0;
        assert!(node_call.contains("size(n.memory_pids) = 0"));
        assert!(
            node_call.contains("NOT (n)--()"),
            "node kept if a surviving edge joins it"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_issue_two_statements_per_pid_for_multiple_pids() {
        let store = RecordingStore::default();

        forget_pids(&store, &["mem1", "mem2"]).await.unwrap();

        assert_eq!(store.calls().len(), 4, "edge+node per pid");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_no_op_for_empty_pid_list() {
        let store = RecordingStore::default();

        forget_pids(&store, &[]).await.unwrap();

        assert!(store.calls().is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_detach_delete_whole_scope_on_scope_forget() {
        let store = RecordingStore::default();

        forget_scope(&store, &scope()).await.unwrap();

        let calls = store.calls();
        assert_eq!(calls.len(), 1);
        assert!(calls[0].0.contains("DETACH DELETE"));
        assert_eq!(calls[0].1.get("agent_id").map(String::as_str), Some("agent"));
    }
}
