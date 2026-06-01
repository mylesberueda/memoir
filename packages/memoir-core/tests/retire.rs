//! Integration tests for the correction-model retirement primitives
//! (`Client::reject` / `Client::mark_stale`, epic 0011 ticket 0009).
//!
//! Verifies the load-bearing behaviors against live Postgres + Qdrant: a
//! retired row disappears from search but stays recall-reachable (carrying
//! its reason), and its vector is evicted.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::memory::RetirementReason;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_hide_from_search_but_keep_recall_reachable() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("the user dislikes cilantro", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "cilantro", Duration::from_secs(15)).await?;

    // Pre-rejection: the row is searchable.
    let before = client.search("cilantro", scope.clone()).limit(10).await?;
    assert!(
        before.list().iter().any(|m| m.pid == written.pid),
        "row must be searchable before rejection"
    );

    client.reject(&written.pid).await?;

    // Post-rejection: gone from search (vector evicted + read filter).
    let after = client.search("cilantro", scope.clone()).limit(10).await?;
    assert!(
        !after.list().iter().any(|m| m.pid == written.pid),
        "rejected row must not appear in search results"
    );

    // But still recall-reachable by pid, carrying its reason — the row is
    // kept for the reprocess guard + accuracy metric.
    let recalled = client.recall(&written.pid).await?;
    assert_eq!(
        recalled.retirement,
        Some(RetirementReason::Rejected),
        "recalled row must report Rejected; got {:?}",
        recalled.retirement
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_mark_stale_hide_from_search_with_stale_reason() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("deployment is on Friday", scope.clone()).await?;
    common::wait_until_indexed(&client, &written.pid, &scope, "deployment", Duration::from_secs(15)).await?;

    client.mark_stale(&written.pid).await?;

    let after = client.search("deployment", scope.clone()).limit(10).await?;
    assert!(
        !after.list().iter().any(|m| m.pid == written.pid),
        "stale row must not appear in search results"
    );

    let recalled = client.recall(&written.pid).await?;
    assert_eq!(
        recalled.retirement,
        Some(RetirementReason::Stale),
        "recalled row must report Stale; got {:?}",
        recalled.retirement
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_not_found_when_retiring_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let result = client.reject("does-not-exist").await;
    assert!(result.is_err(), "retiring an unknown pid must error; got {result:?}");

    Ok(())
}
