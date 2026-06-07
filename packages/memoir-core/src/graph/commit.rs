//! Committing resolved triples to the graph as nodes and edges.
//!
//! The write tier's payoff: [`commit_triples`] takes the triples a
//! [`TripleExtractor`](super::TripleExtractor) produced for one episodic source
//! and persists them, wiring the two resolution seams ([`EntityResolver`],
//! [`EdgeResolver`]) into [`GraphStore`] writes. Each triple's subject and
//! object resolve to canonical nodes, its edge is reconciled against existing
//! edges, and both are written with the source memory's pid and scope so Forget
//! and reconciliation can find them.
//!
//! Writes are idempotent (`MERGE`, not `CREATE`) and committed per triple, so a
//! job that fails partway and is retried (the worker's at-least-once lifecycle)
//! re-runs cleanly: already-written triples merge to no-ops. Every value drawn
//! from user content rides as a bound Cypher parameter, never interpolated into
//! the query string; relation labels — which Cypher cannot parameterize — are
//! sanitized to a safe form by [`sanitize_relation_label`].

use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};

use crate::embedding::{EmbeddingError, EmbeddingModel};
use crate::memory::Scope;

use super::{Edge, EdgeResolver, EntityResolver, GraphError, GraphStore, Resolution, ResolveError, Triple, TripleSet};

/// The label every entity node carries in v1 (untyped — see ticket 0005).
const ENTITY_LABEL: &str = "Entity";

/// Fallback relationship label when a relation sanitizes to the empty string.
///
/// An open-vocabulary relation made only of punctuation (which strips to
/// nothing) still needs a valid Cypher label; `RELATED_TO` keeps the edge
/// queryable rather than dropping the fact.
const FALLBACK_RELATION_LABEL: &str = "RELATED_TO";

/// What [`commit_triples`] needs to know about the originating memory.
///
/// `memory_pid` tags every node and edge so a single entity referenced by many
/// memories is one node carrying many pids — the precondition for
/// reference-counted Forget. `valid_from` is the source's event time (when the
/// facts became true), stamped on every edge so temporal ordering reflects when
/// facts held, not when they were processed.
#[derive(Debug, Clone)]
pub struct CommitContext {
    /// The scope every written node and edge is confined to.
    pub scope: Scope,
    /// The originating memory's public id, tagged onto every element.
    pub memory_pid: String,
    /// The source's event time, stamped as each new edge's `valid_from`.
    pub valid_from: DateTime<FixedOffset>,
}

/// Failure modes for the triple-commit path.
#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    /// Entity resolution failed for a triple's subject or object.
    #[error("entity resolution failed: {0}")]
    EntityResolution(#[from] ResolveError),

    /// Edge resolution failed for a triple's relationship.
    #[error("edge resolution failed: {0}")]
    EdgeResolution(#[from] super::EdgeError),

    /// Embedding a node's canonical name failed.
    #[error("node embedding failed: {0}")]
    Embed(#[from] EmbeddingError),

    /// Writing a node or edge to the graph failed.
    #[error("graph write failed: {0}")]
    Write(#[from] GraphError),
}

/// Commits every triple in `triples` to `store` for one source.
///
/// Resolves each triple's entities and edge, then writes the nodes and the
/// (possibly supersession-closing) edge. Returns the number of triples
/// committed. Triples whose subject and object resolve to the same node are
/// skipped (a self-loop is not a useful relationship).
///
/// # Errors
///
/// Returns [`CommitError`] on the first resolution or write failure. Because
/// every write is an idempotent `MERGE`, a caller that retries the whole batch
/// after a mid-way failure does not double-write.
/// Backs [`GraphStore::commit_triples`]; see that method for semantics.
pub(super) async fn commit_triples<G, EM, ER, EdgeR>(
    store: &G,
    embedder: &EM,
    entities: &ER,
    edges: &EdgeR,
    ctx: &CommitContext,
    triples: &TripleSet,
) -> Result<usize, CommitError>
where
    G: GraphStore + ?Sized,
    EM: EmbeddingModel,
    ER: EntityResolver,
    EdgeR: EdgeResolver,
{
    let mut committed = 0;
    for triple in triples.iter() {
        if commit_one(store, embedder, entities, edges, ctx, triple).await? {
            committed += 1;
        }
    }
    Ok(committed)
}

/// Commits one triple; returns whether an edge was written (`false` = skipped).
async fn commit_one<G, EM, ER, EdgeR>(
    store: &G,
    embedder: &EM,
    entities: &ER,
    edges: &EdgeR,
    ctx: &CommitContext,
    triple: &Triple,
) -> Result<bool, CommitError>
where
    G: GraphStore + ?Sized,
    EM: EmbeddingModel,
    ER: EntityResolver,
    EdgeR: EdgeResolver,
{
    let subject = entities.resolve(&ctx.scope, &triple.subject).await?;
    let object = entities.resolve(&ctx.scope, &triple.object).await?;

    let subject_key = resolution_key(&subject);
    let object_key = resolution_key(&object);
    if subject_key == object_key {
        return Ok(false);
    }

    upsert_node(store, embedder, ctx, &subject).await?;
    upsert_node(store, embedder, ctx, &object).await?;

    let edge = Edge {
        subject_key: subject_key.clone(),
        relation: triple.relation.clone(),
        object_key: object_key.clone(),
        confidence: triple.confidence,
        valid_from: ctx.valid_from,
    };
    let resolution = edges.resolve(&ctx.scope, edge).await?;

    for closed in &resolution.close {
        close_edge(store, ctx, closed).await?;
    }
    upsert_edge(store, ctx, &resolution.open).await?;

    Ok(true)
}

/// Returns the canonical node key a [`Resolution`] identifies.
///
/// The key is the canonical name: a node is `MERGE`d on its scope properties
/// plus this name, so the name is unique within a scope and serves as the stable
/// identity without a separate hash.
fn resolution_key(resolution: &Resolution) -> String {
    match resolution {
        Resolution::Existing { name, .. } | Resolution::New { name } => name.clone(),
    }
}

/// `MERGE`s an entity node, creating it or adding this memory's pid.
///
/// The canonical name's embedding is computed and stored as a JSON-encoded
/// string only `ON CREATE`: an existing node keeps the embedding it was created
/// with, and the JSON encoding round-trips deterministically back through
/// [`GraphStore::query`]'s string-scalar results (the entity catalog parses it).
async fn upsert_node<G: GraphStore + ?Sized, EM: EmbeddingModel>(
    store: &G,
    embedder: &EM,
    ctx: &CommitContext,
    resolution: &Resolution,
) -> Result<(), CommitError> {
    let name = resolution_key(resolution);
    let embedding = embedder.embed(&name).await?;
    let embedding_json = serde_json::to_string(&embedding).expect("serializing Vec<f32> to JSON cannot fail");

    let cypher = format!(
        "MERGE (e:{ENTITY_LABEL} {{agent_id: $agent_id, org_id: $org_id, user_id: $user_id, name: $name}}) \
         ON CREATE SET e.first_seen_at = $now, e.embedding = $embedding, e.memory_pids = [$pid] \
         ON MATCH SET e.memory_pids = \
           CASE WHEN $pid IN e.memory_pids THEN e.memory_pids ELSE e.memory_pids + $pid END"
    );

    let mut params = scope_params(&ctx.scope);
    params.insert("name".to_string(), name);
    params.insert("pid".to_string(), ctx.memory_pid.clone());
    params.insert("now".to_string(), ctx.valid_from.to_rfc3339());
    params.insert("embedding".to_string(), embedding_json);

    store.query(&cypher, &params).await?;
    Ok(())
}

/// `MERGE`s a current relationship edge, creating it or adding this pid.
async fn upsert_edge<G: GraphStore + ?Sized>(store: &G, ctx: &CommitContext, edge: &Edge) -> Result<(), CommitError> {
    let label = sanitize_relation_label(&edge.relation);
    let cypher = format!(
        "MATCH (s:{ENTITY_LABEL} {{agent_id: $agent_id, org_id: $org_id, user_id: $user_id, name: $subject}}) \
         MATCH (o:{ENTITY_LABEL} {{agent_id: $agent_id, org_id: $org_id, user_id: $user_id, name: $object}}) \
         MERGE (s)-[r:{label} {{valid_to: null}}]->(o) \
         ON CREATE SET r.valid_from = $valid_from, r.confidence = $confidence, \
           r.relation = $relation, r.memory_pids = [$pid] \
         ON MATCH SET r.memory_pids = \
           CASE WHEN $pid IN r.memory_pids THEN r.memory_pids ELSE r.memory_pids + $pid END"
    );

    let mut params = scope_params(&ctx.scope);
    params.insert("subject".to_string(), edge.subject_key.clone());
    params.insert("object".to_string(), edge.object_key.clone());
    params.insert("relation".to_string(), edge.relation.clone());
    params.insert("valid_from".to_string(), edge.valid_from.to_rfc3339());
    params.insert("confidence".to_string(), edge.confidence.to_string());
    params.insert("pid".to_string(), ctx.memory_pid.clone());

    store.query(&cypher, &params).await?;
    Ok(())
}

/// Closes a superseded edge by stamping its `valid_to`, keeping it as history.
async fn close_edge<G: GraphStore + ?Sized>(store: &G, ctx: &CommitContext, edge_key: &str) -> Result<(), CommitError> {
    let cypher = "MATCH ()-[r {relation: $relation_key, valid_to: null}]->() \
         WHERE r.agent_id = $agent_id AND r.org_id = $org_id AND r.user_id = $user_id \
         SET r.valid_to = $valid_to"
        .to_string();

    let mut params = scope_params(&ctx.scope);
    params.insert("relation_key".to_string(), edge_key.to_string());
    params.insert("valid_to".to_string(), ctx.valid_from.to_rfc3339());

    store.query(&cypher, &params).await?;
    Ok(())
}

/// Builds the scope parameter map shared by every node/edge write.
fn scope_params(scope: &Scope) -> HashMap<String, String> {
    HashMap::from([
        ("agent_id".to_string(), scope.agent_id.clone()),
        ("org_id".to_string(), scope.org_id.clone()),
        ("user_id".to_string(), scope.user_id.clone()),
    ])
}

/// Sanitizes an open-vocabulary relation into a safe Cypher relationship label.
///
/// Cypher cannot bind a relationship type as a parameter, so the relation string
/// — which originates from LLM extraction over user content — is reduced to an
/// uppercase, underscore-joined identifier of ASCII alphanumerics. This both
/// gives a conventional `:WORKS_AT`-style label and eliminates the injection
/// surface of interpolating raw text into the query structure. A relation that
/// reduces to nothing falls back to [`FALLBACK_RELATION_LABEL`].
fn sanitize_relation_label(relation: &str) -> String {
    let mut label = String::with_capacity(relation.len());
    let mut prev_underscore = false;
    for ch in relation.chars() {
        if ch.is_ascii_alphanumeric() {
            label.extend(ch.to_uppercase());
            prev_underscore = false;
        } else if !prev_underscore && !label.is_empty() {
            label.push('_');
            prev_underscore = true;
        }
    }
    let trimmed = label.trim_end_matches('_');
    if trimmed.is_empty() {
        FALLBACK_RELATION_LABEL.to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::graph::{
        EntityVector, ExactStringResolver, GraphRows, InMemoryEntityCatalog, NaiveAppendResolver,
    };

    /// Embeds every name to the same fixed vector — node writes need only that
    /// an embedding is produced and stored, not its content, in these tests.
    struct StubEmbedding;

    impl EmbeddingModel for StubEmbedding {
        async fn embed(&self, _text: &str) -> Result<Vec<f32>, EmbeddingError> {
            Ok(vec![0.1, 0.2, 0.3])
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    fn scope() -> Scope {
        Scope {
            agent_id: "agent".to_string(),
            org_id: "org".to_string(),
            user_id: "user".to_string(),
        }
    }

    fn now() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2026-06-06T00:00:00Z").expect("valid date")
    }

    fn ctx() -> CommitContext {
        CommitContext {
            scope: scope(),
            memory_pid: "mem1".to_string(),
            valid_from: now(),
        }
    }

    /// A [`GraphStore`] recording every (cypher, params) call for assertions.
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

    fn one_triple(subject: &str, relation: &str, object: &str) -> TripleSet {
        serde_json::from_value(serde_json::json!({
            "triples": [{ "subject": subject, "relation": relation, "object": object, "confidence": 0.9 }]
        }))
        .expect("valid triple json")
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_commit_two_nodes_and_one_edge() {
        let store = RecordingStore::default();
        let entities = ExactStringResolver::new(InMemoryEntityCatalog::new());
        let edges = NaiveAppendResolver::new();

        let committed = commit_triples(&store, &StubEmbedding, &entities, &edges, &ctx(), &one_triple("Alice", "works at", "Acme"))
            .await
            .unwrap();

        assert_eq!(committed, 1);
        let calls = store.calls();
        // two node MERGEs + one edge MERGE
        assert_eq!(calls.len(), 3);
        assert!(calls[2].0.contains(":WORKS_AT"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_bind_user_values_as_params_not_interpolate() {
        let store = RecordingStore::default();
        let entities = ExactStringResolver::new(InMemoryEntityCatalog::new());
        let edges = NaiveAppendResolver::new();

        let injection = r#"Acme"}) DETACH DELETE n //"#;
        commit_triples(&store, &StubEmbedding, &entities, &edges, &ctx(), &one_triple("Alice", "works at", injection))
            .await
            .unwrap();

        let calls = store.calls();
        for (cypher, _) in &calls {
            assert!(!cypher.contains("DETACH DELETE"), "user value leaked into query string");
        }
        assert!(
            calls.iter().any(|(_, params)| params.values().any(|v| v == injection)),
            "the injection value must ride as a bound param somewhere",
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_tag_every_write_with_scope_and_pid() {
        let store = RecordingStore::default();
        let entities = ExactStringResolver::new(InMemoryEntityCatalog::new());
        let edges = NaiveAppendResolver::new();

        commit_triples(&store, &StubEmbedding, &entities, &edges, &ctx(), &one_triple("Alice", "knows", "Bob"))
            .await
            .unwrap();

        for (_, params) in store.calls() {
            assert_eq!(params.get("agent_id").map(String::as_str), Some("agent"));
            assert_eq!(params.get("pid").map(String::as_str), Some("mem1"));
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_skip_self_loop_triple() {
        let store = RecordingStore::default();
        let entities = ExactStringResolver::new(InMemoryEntityCatalog::new());
        let edges = NaiveAppendResolver::new();

        let committed = commit_triples(&store, &StubEmbedding, &entities, &edges, &ctx(), &one_triple("Alice", "is", "Alice"))
            .await
            .unwrap();

        assert_eq!(committed, 0);
        assert!(store.calls().is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_merge_to_existing_node_when_entity_resolves() {
        // An entity already in the catalog resolves to Existing -> the node
        // MERGE still runs (adding this pid), not a duplicate-named node.
        let catalog = InMemoryEntityCatalog::new();
        catalog.insert(
            &scope(),
            EntityVector {
                key: "Alice".to_string(),
                name: "Alice".to_string(),
                embedding: vec![1.0, 0.0],
            },
        );
        let store = RecordingStore::default();
        let entities = ExactStringResolver::new(catalog);
        let edges = NaiveAppendResolver::new();

        commit_triples(&store, &StubEmbedding, &entities, &edges, &ctx(), &one_triple("Alice", "likes", "Tea"))
            .await
            .unwrap();

        let subject_merge = store
            .calls()
            .into_iter()
            .find(|(c, p)| c.contains("MERGE (e:Entity") && p.get("name").map(String::as_str) == Some("Alice"))
            .expect("subject node merged");
        assert_eq!(subject_merge.1.get("name").map(String::as_str), Some("Alice"));
    }

    #[test]
    fn should_sanitize_relation_into_safe_label() {
        assert_eq!(sanitize_relation_label("works at"), "WORKS_AT");
        assert_eq!(sanitize_relation_label("lives-in"), "LIVES_IN");
        assert_eq!(sanitize_relation_label("  prefers  "), "PREFERS");
    }

    #[test]
    fn should_collapse_punctuation_runs_in_label() {
        assert_eq!(sanitize_relation_label("blocked//by"), "BLOCKED_BY");
        assert_eq!(sanitize_relation_label("a & b"), "A_B");
    }

    #[test]
    fn should_fall_back_when_relation_has_no_alphanumerics() {
        assert_eq!(sanitize_relation_label("!!!"), FALLBACK_RELATION_LABEL);
        assert_eq!(sanitize_relation_label(""), FALLBACK_RELATION_LABEL);
    }

    #[test]
    fn should_not_let_injection_survive_label_sanitization() {
        let label = sanitize_relation_label(r#"FOO]->() DETACH DELETE n //"#);
        assert!(!label.contains(']'));
        assert!(!label.contains(' '));
        assert!(!label.contains('-'));
    }
}
