//! Integration tests for `Client::remember` and the async embed substrate.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::MemoryKind;
use memoir_core::store::MemoryStore;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remember_write_pending_then_substrate_indexes() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // First write: scope is empty, so the retrieve side returns no related memories.
    let initial = client
        .remember("the user said hello world", scope.clone())
        .await?;
    assert!(
        initial.list().is_empty(),
        "first remember in an empty scope returns no related memories"
    );

    // Confirm the row exists via the store directly. The row is the one we
    // just wrote; its pid is the only entry in this fresh scope.
    let listed_pids = client.store().indexed_pids_in_scope(&scope).await?;
    // `indexed_pids_in_scope` only returns indexed rows. Immediately after
    // write the row is still `pending`, so this is expected to be empty for
    // the first call.
    let _ = listed_pids;

    // Drive a search for the same content. Once the substrate has flipped
    // status to `indexed`, the just-written row appears in the result list.
    let pid = wait_for_first_pid(&client, &scope, "hello world", Duration::from_secs(15)).await?;

    // After indexing the row is observable via Recall at any state.
    let recalled = client.recall(&pid).await?;
    assert_eq!(recalled.content, "the user said hello world");
    assert_eq!(recalled.kind, MemoryKind::Episodic);
    assert_eq!(recalled.scope, scope);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remember_always_writes_episodic_kind() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    // Toggling `.semantic()` on the retrieve side must not change the kind of
    // the written row — the write is always episodic.
    let _ = client
        .remember("episodic content", scope.clone())
        .semantic()
        .await?;

    let pid = wait_for_first_pid(&client, &scope, "episodic content", Duration::from_secs(15)).await?;
    let row = client.recall(&pid).await?;
    assert_eq!(
        row.kind,
        MemoryKind::Episodic,
        "remember always writes episodic regardless of retrieve-side kind toggles"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_isolate_scopes_on_retrieve() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    let _ = client
        .remember("only visible to A", scope_a.clone())
        .await?;
    let _ = client
        .remember("only visible to B", scope_b.clone())
        .await?;

    // Wait until both rows are observable in their own scopes.
    let _ = wait_for_first_pid(&client, &scope_a, "visible to A", Duration::from_secs(15)).await?;
    let _ = wait_for_first_pid(&client, &scope_b, "visible to B", Duration::from_secs(15)).await?;

    let from_a = client.remember("visible", scope_a.clone()).limit(50).await?;
    let from_b = client.remember("visible", scope_b.clone()).limit(50).await?;

    let a_contents: Vec<&str> = from_a.list().iter().map(|m| m.content.as_str()).collect();
    let b_contents: Vec<&str> = from_b.list().iter().map(|m| m.content.as_str()).collect();
    assert!(
        a_contents.iter().all(|c| !c.contains("visible to B")),
        "scope A must not see scope B's memories; got {a_contents:?}"
    );
    assert!(
        b_contents.iter().all(|c| !c.contains("visible to A")),
        "scope B must not see scope A's memories; got {b_contents:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_filter_kind_on_retrieve_semantic_returns_empty_in_mvp() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client
        .remember("seed content for kind filter test", scope.clone())
        .await?;
    let _ = wait_for_first_pid(
        &client,
        &scope,
        "seed content for kind filter test",
        Duration::from_secs(15),
    )
    .await?;

    // In the vector-only MVP, no semantic rows exist (LLM extraction is epic
    // 0006). Filtering to `.semantic()` therefore returns an empty list.
    let semantic_only = client
        .remember("seed content", scope.clone())
        .semantic()
        .limit(50)
        .await?;
    assert!(
        semantic_only.list().is_empty(),
        "semantic-only retrieve returns empty in vector-only MVP (no semantic rows yet); got {} memories",
        semantic_only.list().len()
    );

    // Without filters, the episodic row we just wrote IS returned.
    let unfiltered = client.remember("seed content", scope.clone()).limit(50).await?;
    assert!(
        !unfiltered.list().is_empty(),
        "default retrieve (both kinds) should return the episodic row"
    );

    Ok(())
}

/// Polls the scope until at least one indexed row is observable, returning its pid.
async fn wait_for_first_pid(
    client: &memoir_core::client::Client,
    scope: &memoir_core::memory::Scope,
    query: &str,
    timeout: Duration,
) -> anyhow::Result<String> {
    let deadline = std::time::Instant::now() + timeout;
    let mut delay = Duration::from_millis(50);

    while std::time::Instant::now() < deadline {
        let hits = client.remember(query, scope.clone()).limit(50).await?;
        if let Some(first) = hits.list().first() {
            return Ok(first.pid.clone());
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    anyhow::bail!("no indexed row appeared in scope within {timeout:?}")
}
