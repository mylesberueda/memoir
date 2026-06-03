//! Integration tests for the reprocess correction engine (epic 0011 ticket 0010).
//!
//! Two tiers. The first exercises the new `active_semantics_for_source` store
//! query against live Postgres without an LLM — it is the primitive the engine
//! retires through, so its filtering (active rows only) is load-bearing. The
//! second drives the full worker-backed engine with a real Ollama extraction
//! LLM: feedback retires the wrong derivation, re-derives a corrected one, and
//! persists the feedback as an `origin: feedback` episodic row.

#![cfg(feature = "integration")]

use memoir_core::memory::{Confidence, MemoryKind, RetirementReason};
use memoir_core::store::{MemoryStore, NewMemory};

mod common;

/// Writes a semantic row derived from `source_pid` directly via the store.
///
/// Bypasses the extraction worker so the query-filtering tests need no LLM.
async fn write_semantic(
    store: &impl MemoryStore,
    scope: &memoir_core::memory::Scope,
    source_pid: &str,
    content: &str,
) -> anyhow::Result<String> {
    let written = store
        .remember(NewMemory {
            scope: scope.clone(),
            content: content.to_string(),
            metadata: serde_json::json!({ "origin": "test" }),
            kind: MemoryKind::Semantic,
            source_pid: Some(source_pid.to_string()),
            event_at: None,
            confidence: Confidence::new(90),
        })
        .await?;
    Ok(written.pid)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_only_active_semantics_for_source() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let store = client.store();

    let episodic = client.remember("the user said three things", scope.clone()).await?;

    let active = write_semantic(store, &scope, &episodic.pid, "fact one").await?;
    let to_retire = write_semantic(store, &scope, &episodic.pid, "fact two").await?;
    let to_supersede = write_semantic(store, &scope, &episodic.pid, "fact three").await?;
    let winner = write_semantic(store, &scope, &episodic.pid, "fact three corrected").await?;

    // Retire one and supersede another — both must drop out of the active set.
    store.retire(&to_retire, RetirementReason::Rejected).await?;
    store.supersede(&to_supersede, &winner).await?;

    let derived = store.active_semantics_for_source(&episodic.pid).await?;
    let pids: Vec<&str> = derived.iter().map(|m| m.pid.as_str()).collect();

    assert!(pids.contains(&active.as_str()), "active row must be returned");
    assert!(pids.contains(&winner.as_str()), "the superseding winner must be returned");
    assert!(
        !pids.contains(&to_retire.as_str()),
        "retired row must be excluded from the active set"
    );
    assert!(
        !pids.contains(&to_supersede.as_str()),
        "superseded row must be excluded from the active set"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_empty_active_semantics_for_episodic_only_source() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let store = client.store();

    // An episodic source with no derived semantics yields an empty set, not an
    // error — the reprocess engine treats "nothing to retire" as a clean pass.
    let episodic = client.remember("a bare utterance with no extraction", scope.clone()).await?;

    let derived = store.active_semantics_for_source(&episodic.pid).await?;
    assert!(derived.is_empty(), "episodic-only source must have no active semantics");

    let unknown = store.active_semantics_for_source("does-not-exist").await?;
    assert!(unknown.is_empty(), "unknown source must yield an empty set, not an error");

    Ok(())
}

// The full worker-driven engine E2E (feedback → retire derivation → re-derive
// corrected → persist origin:feedback episodic row) is deferred to ticket 0011,
// which adds the `Client::reprocess`/`feedback` trigger that enqueues the job.
// 0010 has no public trigger by design — the engine is driven by enqueuing a
// `JobKind::Reprocess` job. The handler's payload parsing is covered by the
// `ReprocessRequest` unit tests; the retire-set primitive is covered above.
