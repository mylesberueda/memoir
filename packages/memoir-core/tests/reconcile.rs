//! Integration tests for `Client::reconcile`.

#![cfg(feature = "integration")]

use std::time::Duration;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reconcile_be_idempotent_on_clean_db() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let first = client.reconcile().await?;
    let second = client.reconcile().await?;

    assert_eq!(first.failed_retried, 0);
    assert_eq!(first.failed_recovered, 0);
    assert_eq!(first.orphans_deleted, 0);
    assert_eq!(second.failed_retried, 0);
    assert_eq!(second.failed_recovered, 0);
    assert_eq!(second.orphans_deleted, 0);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reconcile_only_retry_failed_skip_orphan_pass() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("reconcile-target", scope.clone()).await?;
    // Let the substrate run before reconciling — these rows reach `indexed`,
    // not `failed`, in the happy path. retry-failed therefore touches nothing.
    tokio::time::sleep(Duration::from_secs(2)).await;

    let summary = client.reconcile().only_retry_failed().await?;
    assert_eq!(summary.failed_retried, 0, "happy-path rows reach indexed, not failed");
    assert_eq!(summary.orphans_deleted, 0, "orphan pass should not have run");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reconcile_only_clean_orphans_skip_retry_pass() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let summary = client.reconcile().only_clean_orphans().await?;
    assert_eq!(summary.failed_retried, 0, "retry pass should not have run");
    assert_eq!(
        summary.failed_recovered, 0,
        "retry pass should not have run, so nothing recovered"
    );
    // orphans_deleted may be 0 on a fresh empty DB, or non-zero if a previous
    // test leaked. Both are valid; we just assert the call succeeded.

    Ok(())
}
