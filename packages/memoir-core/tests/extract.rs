//! Integration tests for the extraction worker stage.
//!
//! Wires a `TestClient` against live Postgres + Qdrant + Ollama. The worker
//! drains episodic writes into embed jobs and (because `extraction_llm` is
//! configured) extract jobs. Tests poll until semantic memories appear under
//! the source pid. Assertions are tolerant of real-model variance — the
//! contract under test is "extraction produces semantic rows linked to
//! their episodic source", not "the model emits exactly these facts".

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::ForgetTarget;

mod common;

const EXTRACTION_TIMEOUT: Duration = Duration::from_secs(120);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_extract_semantic_memories_from_episodic_content() -> anyhow::Result<()> {
    // Given: a client with a real Ollama extraction LLM wired up
    let client = common::fresh_client_with_extraction().await?;
    let scope = common::fresh_scope();

    // When: an episodic memory is written with content that has clear facts
    let content = "Alice works at Acme Corp as a senior engineer. She lives in Berlin.";
    let _ = client.remember(content, scope.clone()).await?;

    // Resolve the just-written episodic pid via the embed substrate.
    // We could read `Client::remember`'s return value directly, but the
    // extraction worker stage only fires after the embed job lands — so
    // polling until the row becomes searchable also serves as a "worker
    // has caught up" gate.
    let episodic_pid =
        common::wait_for_first_pid(&client, &scope, content, Duration::from_secs(15)).await?;

    // Then: the worker eventually writes semantic rows linked to the source.
    let semantics = common::wait_until_extracted(
        &client,
        &scope,
        &episodic_pid,
        EXTRACTION_TIMEOUT,
    )
    .await?;

    // And: every observed semantic row references the episodic source via
    // source_pid. Tolerant assertion — we don't check exact fact content.
    assert!(
        !semantics.is_empty(),
        "expected at least one semantic memory derived from {episodic_pid}",
    );
    for m in &semantics {
        assert_eq!(
            m.source_pid.as_deref(),
            Some(episodic_pid.as_str()),
            "semantic memory {} should reference source {}",
            m.pid,
            episodic_pid,
        );
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_cascade_delete_semantic_memories_when_episodic_is_forgotten() -> anyhow::Result<()> {
    // Given: an episodic memory has been extracted into semantic rows
    let client = common::fresh_client_with_extraction().await?;
    let scope = common::fresh_scope();
    let content = "Bob is a researcher at MIT studying quantum cryptography.";
    let _ = client.remember(content, scope.clone()).await?;
    let episodic_pid =
        common::wait_for_first_pid(&client, &scope, content, Duration::from_secs(15)).await?;
    let semantics_before = common::wait_until_extracted(
        &client,
        &scope,
        &episodic_pid,
        EXTRACTION_TIMEOUT,
    )
    .await?;
    assert!(
        !semantics_before.is_empty(),
        "expected at least one semantic memory before forget",
    );

    // When: the episodic memory is forgotten
    let deleted = client.forget(ForgetTarget::Pid(episodic_pid.clone())).await?;
    assert_eq!(
        deleted,
        vec![episodic_pid.clone()],
        "forget returns the deleted episodic pid",
    );

    // Then: the semantic derivatives are gone too (FK ON DELETE CASCADE).
    // Inspect the store directly so we observe DB state, not search behavior.
    use memoir_core::store::MemoryStore;
    let surviving_pids = client.store().indexed_pids_in_scope(&scope).await?;
    let surviving_refs: Vec<&str> = surviving_pids.iter().map(String::as_str).collect();
    let surviving = client.store().find_by_pids(&surviving_refs).await?;
    let still_linked: Vec<_> = surviving
        .iter()
        .filter(|m| m.source_pid.as_deref() == Some(episodic_pid.as_str()))
        .collect();
    assert!(
        still_linked.is_empty(),
        "semantic derivatives should cascade-delete with episodic source; survived: {:?}",
        still_linked.iter().map(|m| &m.pid).collect::<Vec<_>>(),
    );

    Ok(())
}
