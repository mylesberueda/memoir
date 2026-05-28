//! Integration tests for `Client::forget`.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::client::ClientError;
use memoir_core::memory::{ForgetTarget, Scope};
use memoir_core::store::StoreError;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_forget_pid_remove_row_and_make_recall_fail() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("forget me by pid", scope.clone()).await?;
    let pid = wait_for_first_pid(&client, &scope, "forget me by pid", Duration::from_secs(15)).await?;

    let deleted = client.forget(ForgetTarget::Pid(pid.clone())).await?;
    assert_eq!(deleted, vec![pid.clone()], "forget(Pid) returns exactly the deleted pid");

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
        deleted.len() >= 1,
        "forget(Scope) deletes at least the rows we wrote; got {} deleted",
        deleted.len()
    );

    // After bulk delete, the scope should be empty on retrieve.
    let post = client.remember("first", scope.clone()).limit(50).await?;
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
