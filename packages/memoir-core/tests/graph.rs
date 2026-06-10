//! Integration tests for the knowledge-graph write/read/forget/reconcile path.
//!
//! Wires a `TestClient` against live Postgres + Qdrant + FalkorDB + Ollama (via
//! [`common::fresh_graph_client`]). The worker drains an episodic write through
//! `RelationalExtract` → synthesis → graph commit; tests poll the live graph
//! ([`common::wait_until_graph_committed`]) and assert on the committed nodes
//! and edges via `Client::inspect_graph`.
//!
//! Assertions test *structural shape*, not exact model output: extraction runs a
//! real LLM, so the contract under test is "a relationship between Alice and
//! Acme was committed", never "the relation label is exactly `WORKS_AT`". Each
//! test mints its scope via [`common::TestClient::fresh_scope`] so its graph
//! data is wiped on drop — the suite shares one FalkorDB graph name.
//!
//! Requires `--features integration,knowledge-graph` and the env vars
//! `DATABASE_URL`, `QDRANT_URL`, `FALKOR_URL`, `OLLAMA_URL`, `OLLAMA_MODEL`.

#![cfg(all(feature = "integration", feature = "knowledge-graph"))]

mod common;

use std::time::Duration;

use memoir_core::graph::GraphSnapshot;

/// Generous ceiling for a real-LLM relational-extraction + synthesis round.
///
/// The graph commit waits on two LLM passes (semantic extract + relational
/// extract) plus synthesis; a cold large model can take tens of seconds per
/// call. Mirrors the lease/timeout discipline the harness documents.
const GRAPH_COMMIT_TIMEOUT: Duration = Duration::from_secs(120);

/// Returns whether the snapshot holds an edge between two entities, either way.
///
/// Direction-agnostic and label-agnostic on purpose: the LLM decides relation
/// phrasing and triple direction, so a test asserts only that the two entities
/// are related at all. Names match case-insensitively on a substring so
/// "Alice" matches a committed "Alice Smith".
fn has_edge_between(snapshot: &GraphSnapshot, a: &str, b: &str) -> bool {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    snapshot.edges.iter().any(|e| {
        let s = e.subject.to_lowercase();
        let o = e.object.to_lowercase();
        (s.contains(&a) && o.contains(&b)) || (s.contains(&b) && o.contains(&a))
    })
}

/// Returns whether the snapshot holds an entity whose name contains `name`.
fn has_entity(snapshot: &GraphSnapshot, name: &str) -> bool {
    let name = name.to_lowercase();
    snapshot.nodes.iter().any(|n| n.name.to_lowercase().contains(&name))
}

/// Counts entities whose name contains `name` (case-insensitive substring).
fn count_entities(snapshot: &GraphSnapshot, name: &str) -> usize {
    let name = name.to_lowercase();
    snapshot
        .nodes
        .iter()
        .filter(|n| n.name.to_lowercase().contains(&name))
        .count()
}

/// Returns the edges whose subject and object span the two entities, either way.
fn edges_between<'a>(snapshot: &'a GraphSnapshot, a: &str, b: &str) -> Vec<&'a memoir_core::graph::GraphEdge> {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    snapshot
        .edges
        .iter()
        .filter(|e| {
            let s = e.subject.to_lowercase();
            let o = e.object.to_lowercase();
            (s.contains(&a) && o.contains(&b)) || (s.contains(&b) && o.contains(&a))
        })
        .collect()
}

// ─── c3: write → commit ──────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_commit_entities_and_edge_when_episodic_memory_is_written() -> anyhow::Result<()> {
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    client.remember("Alice works at Acme", scope.clone()).await?;

    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    assert!(
        has_entity(&snapshot, "Alice"),
        "expected an Alice entity, got {:?}",
        snapshot.nodes
    );
    assert!(
        has_entity(&snapshot, "Acme"),
        "expected an Acme entity, got {:?}",
        snapshot.nodes
    );
    assert!(
        has_edge_between(&snapshot, "Alice", "Acme"),
        "expected an Alice<->Acme edge, got {:?}",
        snapshot.edges,
    );
    Ok(())
}

// ─── c4: synthesis is idempotent — one source commits one set of triples ─────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_commit_a_single_node_per_entity_when_one_source_is_synthesized() -> anyhow::Result<()> {
    // Synthesis is the two-parent fan-in (extract ∥ relational → synthesize). Its
    // "fires exactly once" guarantee is observable as: one episodic source leaves
    // exactly one node per entity and no duplicate edge — a second synthesis pass
    // would double-write. (The complementary "zero synthesis on a *failed* parent"
    // case needs job-failure injection and is left to a focused follow-up; here we
    // assert the committed-state idempotency that a live single write proves.)
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    client.remember("Alice works at Acme", scope.clone()).await?;
    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    assert_eq!(
        count_entities(&snapshot, "Alice"),
        1,
        "Alice must be one node, got {:?}",
        snapshot.nodes
    );
    assert_eq!(
        count_entities(&snapshot, "Acme"),
        1,
        "Acme must be one node, got {:?}",
        snapshot.nodes
    );
    assert_eq!(
        edges_between(&snapshot, "Alice", "Acme").len(),
        1,
        "exactly one Alice<->Acme edge, got {:?}",
        snapshot.edges,
    );
    Ok(())
}

// ─── c5: entity resolution dedup ─────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_resolve_to_one_entity_when_a_later_memory_names_it_more_fully() -> anyhow::Result<()> {
    // "Alice" vs "Alice Smith" must clear MIN_ENTITY_SIMILARITY and resolve to one
    // node — the embedding resolver's job, unprovable against a staged store.
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    client.remember("Alice works at Acme", scope.clone()).await?;
    common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    client
        .remember("Alice Smith leads the platform team", scope.clone())
        .await?;
    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        s.edges.iter().any(|e| {
            let pair = format!("{} {}", e.subject, e.object).to_lowercase();
            pair.contains("team") || pair.contains("platform")
        })
    })
    .await?;

    assert_eq!(
        count_entities(&snapshot, "Alice"),
        1,
        "Alice and Alice Smith must dedup to one node, got {:?}",
        snapshot.nodes,
    );
    Ok(())
}

// ─── c6: contradiction = temporal invalidation, not delete ───────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_invalidate_old_edge_and_preserve_history_when_a_fact_is_superseded() -> anyhow::Result<()> {
    // The inspect snapshot deliberately includes superseded edges, so a closed
    // (valid_to set) Acme edge and a current Globex edge must coexist.
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    client.remember("Alice works at Acme", scope.clone()).await?;
    common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    client.remember("Alice now works at Globex", scope.clone()).await?;
    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Globex")
    })
    .await?;

    assert!(
        has_edge_between(&snapshot, "Alice", "Globex"),
        "new Globex edge must exist, got {:?}",
        snapshot.edges,
    );
    let acme_edges = edges_between(&snapshot, "Alice", "Acme");
    assert!(
        !acme_edges.is_empty(),
        "old Acme edge must be preserved as history, not deleted, got {:?}",
        snapshot.edges,
    );
    assert!(
        acme_edges.iter().any(|e| e.valid_to.is_some()),
        "old Acme edge must be closed (valid_to set), got {:?}",
        acme_edges,
    );
    Ok(())
}

// ─── c7: search enrichment ───────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_enrich_search_results_with_graph_context_when_requested() -> anyhow::Result<()> {
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    client.remember("Alice works at Acme", scope.clone()).await?;
    common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    let enriched = client.search("Alice", scope.clone()).with_graph().limit(10).await?;

    assert!(!enriched.list().is_empty(), "search must return the hit");
    assert!(
        !enriched.graph().relationships.is_empty(),
        "with_graph() must surface graph relationships, got {:?}",
        enriched.graph(),
    );
    Ok(())
}

// ─── c8: forget ref-counts ───────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remove_graph_nodes_when_their_only_source_memory_is_forgotten() -> anyhow::Result<()> {
    use memoir_core::memory::ForgetTarget;

    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    let written = client.remember("Alice works at Acme", scope.clone()).await?;
    common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    client.forget(ForgetTarget::Pid(written.pid.clone())).await?;

    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        !has_edge_between(s, "Alice", "Acme")
    })
    .await?;

    assert!(
        !has_edge_between(&snapshot, "Alice", "Acme"),
        "forgetting the only source must remove the edge, got {:?}",
        snapshot.edges,
    );
    Ok(())
}

// ─── inherited 0012: live cross-scope inspect ────────────────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_whole_org_when_inspect_scope_omits_agent_and_user() -> anyhow::Result<()> {
    // Two scopes in one org: an org-only inspect must span both — the cross-scope
    // read no staged-store unit test could prove.
    let mut client = common::fresh_graph_client().await?;
    let org = format!("org_{}", nanoid::nanoid!(8));
    let scope_a = client.fresh_scope_in_org(&org);
    let scope_b = client.fresh_scope_in_org(&org);

    client.remember("Alice works at Acme", scope_a.clone()).await?;
    common::wait_until_graph_committed(&client, &scope_a, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Alice", "Acme")
    })
    .await?;
    client.remember("Bob works at Globex", scope_b.clone()).await?;
    common::wait_until_graph_committed(&client, &scope_b, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Bob", "Globex")
    })
    .await?;

    let snapshot = client.inspect_graph().org(org.clone()).await?;

    assert!(
        has_entity(&snapshot, "Alice"),
        "org-wide inspect must see scope_a's Alice, got {:?}",
        snapshot.nodes
    );
    assert!(
        has_entity(&snapshot, "Bob"),
        "org-wide inspect must see scope_b's Bob, got {:?}",
        snapshot.nodes
    );
    Ok(())
}

// ─── c9 + inherited 0013: reconcile rebuild + same-created_at page boundary ──

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_rebuild_every_memory_exactly_once_across_a_same_timestamp_page_boundary() -> anyhow::Result<()> {
    use sea_orm::{ConnectionTrait, Statement, Value};

    // Five distinct episodic facts → five distinct entities. We then clamp two of
    // them to one created_at so a page boundary cuts through the shared timestamp,
    // and rebuild with page_size = 2. Correct pagination re-derives ALL five
    // exactly once: a skip at the seam drops an entity; a double counts one twice.
    let mut client = common::fresh_graph_client().await?;
    let scope = client.fresh_scope();

    let people = ["Alice", "Bob", "Carol", "Dave", "Erin"];
    let mut pids = Vec::new();
    for person in people {
        let written = client
            .remember(format!("{person} works at Acme"), scope.clone())
            .await?;
        pids.push(written.pid);
    }
    common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        has_edge_between(s, "Erin", "Acme")
    })
    .await?;

    // Clamp the middle two pids to one shared created_at (test-crate raw SQL — no
    // production path can set created_at; the reconcile cursor pages by it).
    let shared_pids = vec![pids[1].clone(), pids[2].clone()];
    let raw = client.raw_db().await?;
    let clamp = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "UPDATE memories SET created_at = '2026-01-01T00:00:00+00:00' WHERE pid = ANY($1)",
        [Value::Array(
            sea_orm::sea_query::ArrayType::String,
            Some(Box::new(
                shared_pids.into_iter().map(|p| Value::String(Some(p))).collect(),
            )),
        )],
    );
    raw.execute_raw(clamp).await?;

    // page_size 2 forces the shared-timestamp pair to straddle a page seam.
    let summary = client
        .reconcile()
        .rebuild_graph(scope.clone())
        .rebuild_page_size(2)
        .await?;
    assert_eq!(
        summary.graph_rebuild_enqueued, 5,
        "all five episodic memories must re-enqueue exactly once"
    );

    let snapshot = common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
        people.iter().all(|p| has_entity(s, p))
    })
    .await?;

    for person in people {
        assert_eq!(
            count_entities(&snapshot, person),
            1,
            "{person} must rebuild exactly once (no skip, no dup), got {:?}",
            snapshot.nodes,
        );
    }
    Ok(())
}

// ─── cleanup: the teardown guard actually wipes a scope ──────────────────────

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_leave_no_graph_residue_after_a_tracked_scope_is_dropped() -> anyhow::Result<()> {
    let scope = {
        let mut client = common::fresh_graph_client().await?;
        let scope = client.fresh_scope();
        client.remember("Alice works at Acme", scope.clone()).await?;
        common::wait_until_graph_committed(&client, &scope, GRAPH_COMMIT_TIMEOUT, |s| {
            has_edge_between(s, "Alice", "Acme")
        })
        .await?;
        scope
        // client drops here → Drop forgets `scope` from the shared graph
    };

    let client = common::fresh_graph_client().await?;
    let snapshot = client
        .inspect_graph()
        .agent(scope.agent_id.clone())
        .org(scope.org_id.clone())
        .user(scope.user_id.clone())
        .await?;

    assert!(
        snapshot.is_empty(),
        "dropped TestClient must wipe its scope's graph, found {:?} / {:?}",
        snapshot.nodes,
        snapshot.edges,
    );
    Ok(())
}
