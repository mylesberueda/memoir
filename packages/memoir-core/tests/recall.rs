//! Integration tests for `Client::recall`.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::client::ClientError;
use memoir_core::store::StoreError;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_recall_return_not_found_for_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let result = client.recall("definitely-not-a-real-pid").await;
    match result {
        Err(ClientError::Store(StoreError::NotFound(pid))) => {
            assert_eq!(pid, "definitely-not-a-real-pid");
        }
        other => panic!("expected ClientError::Store(NotFound), got {other:?}"),
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_recall_a_freshly_written_pending_row() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("recall-me-pending", scope.clone()).await?;

    // Race: the substrate may have flipped the row to `indexed` already, but
    // recall is supposed to work at any lifecycle state, including `pending`.
    // We retrieve the pid via search (which only sees indexed rows) once
    // available, then verify recall returns it.
    let pid = wait_for_first_pid(&client, &scope, "recall-me-pending", Duration::from_secs(15)).await?;
    let row = client.recall(&pid).await?;
    assert_eq!(row.content, "recall-me-pending");
    assert_eq!(row.scope, scope);

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
