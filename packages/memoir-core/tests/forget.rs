//! Integration tests for `Client::forget`.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::client::ClientError;
use memoir_core::memory::{Confidence, ForgetTarget, MemoryKind, Scope};
use memoir_core::store::{MemoryStore, NewMemory, StoreError};
use memoir_core::vector::VectorIndex;
use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};

mod common;

/// Returns whether the test collection holds a Qdrant point for `pid`.
async fn qdrant_has_point(qdrant: &qdrant_client::Qdrant, collection: &str, pid: &str) -> anyhow::Result<bool> {
    let response = qdrant
        .scroll(
            ScrollPointsBuilder::new(collection)
                .filter(Filter {
                    must: vec![Condition::matches("pid", pid.to_string())],
                    ..Default::default()
                })
                .limit(1u32),
        )
        .await?;
    Ok(!response.result.is_empty())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_pid_remove_row_and_make_recall_fail() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("forget me by pid", scope.clone()).await?;
    let pid = wait_for_first_pid(&client, &scope, "forget me by pid", Duration::from_secs(15)).await?;

    let deleted = client.forget(ForgetTarget::Pid(pid.clone())).await?;
    assert_eq!(
        deleted,
        vec![pid.clone()],
        "forget(Pid) returns exactly the deleted pid"
    );

    // Recall must now report NotFound — the row is gone from the source of truth.
    let result = client.recall(&pid).await;
    assert!(
        matches!(result, Err(ClientError::Store(StoreError::NotFound(_)))),
        "recall after forget must be NotFound; got {result:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_scope_bulk_delete_all_matching_memories() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("first", scope.clone()).await?;
    let _ = client.remember("second", scope.clone()).await?;
    let _ = client.remember("third", scope.clone()).await?;

    // Wait until at least one is indexed (proves writes settled).
    let _ = wait_for_first_pid(&client, &scope, "first", Duration::from_secs(15)).await?;
    // Give the substrate a moment to flush the other two writes too.
    tokio::time::sleep(Duration::from_millis(500)).await;

    let deleted = client.forget(ForgetTarget::Scope(scope.clone())).await?;
    assert!(
        !deleted.is_empty(),
        "forget(Scope) deletes at least the rows we wrote; got {} deleted",
        deleted.len()
    );

    // After bulk delete, the scope should be empty on retrieve.
    let post = client.search("first", scope.clone()).limit(50).await?;
    assert!(
        post.list().is_empty(),
        "scope must be empty after bulk forget; got {} memories back",
        post.list().len()
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_return_empty_for_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let deleted = client.forget(ForgetTarget::Pid("nonexistent".to_string())).await?;
    assert!(
        deleted.is_empty(),
        "forget of unknown pid is success-with-zero, not an error; got {deleted:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_reject_empty_scope_fields() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let empty = Scope {
        agent_id: "".to_string(),
        org_id: "".to_string(),
        user_id: "".to_string(),
    };
    let result = client.forget(ForgetTarget::Scope(empty)).await;
    assert!(
        matches!(result, Err(ClientError::Store(StoreError::InvalidScope(_)))),
        "empty scope must be rejected as InvalidScope; got {result:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_evict_derived_semantic_vectors_when_episodic_source_is_forgotten() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();
    let qdrant = client.raw_qdrant()?;

    // An indexed episodic source plus a derived semantic row, both with vectors
    // in Qdrant. The semantic row's `source_pid` ties it to the source via the
    // ON DELETE CASCADE foreign key.
    let source = client.remember("my favorite color is green", scope.clone()).await?;
    common::wait_until_indexed(
        &client,
        &source.pid,
        &scope,
        "my favorite color is green",
        Duration::from_secs(15),
    )
    .await?;

    let derived = client
        .store()
        .remember(NewMemory {
            scope: scope.clone(),
            content: "the user's favorite color is green".to_string(),
            metadata: serde_json::json!({ "origin": "test" }),
            kind: MemoryKind::Semantic,
            source_pid: Some(source.pid.clone()),
            event_at: None,
            confidence: Confidence::new(80),
        })
        .await?;
    // Index the derived row directly: a fixed 384-dim vector is enough to prove
    // a point exists then gets evicted — eviction, not search quality, is under
    // test, so the vector's content is irrelevant.
    client.index().upsert(&derived, vec![0.1_f32; 384]).await?;

    assert!(
        qdrant_has_point(&qdrant, &client.collection, &derived.pid).await?,
        "precondition: derived semantic point must be indexed before forget",
    );

    let deleted = client.forget(ForgetTarget::Pid(source.pid.clone())).await?;
    assert!(
        deleted.contains(&source.pid) && deleted.contains(&derived.pid),
        "forget(Pid) must return both the source and its cascade-deleted derived pid; got {deleted:?}",
    );

    assert!(
        !qdrant_has_point(&qdrant, &client.collection, &derived.pid).await?,
        "the derived semantic row's vector must be evicted — the cascade must not leave an orphaned point",
    );
    assert!(
        !qdrant_has_point(&qdrant, &client.collection, &source.pid).await?,
        "the source's own vector must also be evicted",
    );

    Ok(())
}

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
