//! Integration tests for `Client::remember` and the async embed substrate.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::MemoryKind;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remember_return_written_episodic_memory() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client
        .remember("the user said hello world", scope.clone())
        .await?;

    assert_eq!(written.content, "the user said hello world");
    assert_eq!(written.kind, MemoryKind::Episodic);
    assert_eq!(written.scope, scope);
    assert!(written.source_pid.is_none(), "direct remember writes have no source_pid");

    // The same row is searchable once the embed substrate has flipped it
    // from `pending` to `indexed`. Recall works at any state.
    let pid =
        wait_for_first_pid(&client, &scope, "hello world", Duration::from_secs(15)).await?;
    assert_eq!(pid, written.pid, "the indexed row is the one we just wrote");

    let recalled = client.recall(&written.pid).await?;
    assert_eq!(recalled.content, "the user said hello world");
    assert_eq!(recalled.kind, MemoryKind::Episodic);
    assert_eq!(recalled.scope, scope);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remember_round_trip_metadata_unchanged() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let metadata = serde_json::json!({
        "source": "test",
        "session_id": "abc-123",
        "tags": ["one", "two"],
        "nested": { "count": 42, "weight": 0.5 },
    });

    let written = client
        .remember("memory with non-trivial metadata", scope.clone())
        .metadata(metadata.clone())
        .await?;

    assert_eq!(
        written.metadata, metadata,
        "the just-written row's metadata reflects what the builder sent"
    );

    let recalled = client.recall(&written.pid).await?;
    assert_eq!(
        recalled.metadata, metadata,
        "metadata round-trips through Postgres JSONB without re-shaping"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_remember_default_metadata_to_empty_object_when_unset() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client
        .remember("memory without explicit metadata", scope.clone())
        .await?;

    assert_eq!(
        written.metadata,
        serde_json::json!({}),
        "unset metadata defaults to empty object, matching the column default"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_search_isolate_scopes() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    let _ = client.remember("only visible to A", scope_a.clone()).await?;
    let _ = client.remember("only visible to B", scope_b.clone()).await?;

    // Wait until both rows are observable in their own scopes.
    let _ = wait_for_first_pid(&client, &scope_a, "visible to A", Duration::from_secs(15)).await?;
    let _ = wait_for_first_pid(&client, &scope_b, "visible to B", Duration::from_secs(15)).await?;

    let from_a = client.search("visible", scope_a.clone()).limit(50).await?;
    let from_b = client.search("visible", scope_b.clone()).limit(50).await?;

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
async fn should_search_filter_kind_semantic_returns_empty_in_mvp() -> anyhow::Result<()> {
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
        .search("seed content", scope.clone())
        .semantic()
        .limit(50)
        .await?;
    assert!(
        semantic_only.list().is_empty(),
        "semantic-only search returns empty in vector-only MVP (no semantic rows yet); got {} memories",
        semantic_only.list().len()
    );

    // Without filters, the episodic row we just wrote IS returned.
    let unfiltered = client.search("seed content", scope.clone()).limit(50).await?;
    assert!(
        !unfiltered.list().is_empty(),
        "default search (both kinds) should return the episodic row"
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
        let hits = client.search(query, scope.clone()).limit(50).await?;
        if let Some(first) = hits.list().first() {
            return Ok(first.pid.clone());
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_millis(500));
    }

    anyhow::bail!("no indexed row appeared in scope within {timeout:?}")
}
