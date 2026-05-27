//! Integration tests for `Client::edit`.

#![cfg(feature = "integration")]

use std::time::Duration;

use memoir_core::client::ClientError;
use memoir_core::store::StoreError;
use sea_orm::{ConnectionTrait, Statement, Value};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_overwrite_content_on_edit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("original text", scope.clone()).await?;
    let edited = client.edit(&written.pid).content("corrected text").await?;

    assert_eq!(edited.pid, written.pid, "edit must return the same pid");
    assert_eq!(edited.content, "corrected text", "content must be overwritten");
    assert_eq!(edited.scope, scope, "scope must be preserved");

    let reloaded = client.recall(&written.pid).await?;
    assert_eq!(reloaded.content, "corrected text");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_preserve_created_at_on_edit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("first", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;

    let edited = client.edit(&written.pid).content("second").await?;

    assert_eq!(edited.created_at, written.created_at, "created_at must not change");
    assert!(
        edited.updated_at > written.updated_at,
        "updated_at must bump on edit (was {}, now {})",
        written.updated_at,
        edited.updated_at,
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_notfound_when_editing_unknown_pid() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;

    let result = client.edit("not-a-real-pid").content("anything").await;

    match result {
        Err(ClientError::Store(StoreError::NotFound(pid))) => {
            assert_eq!(pid, "not-a-real-pid");
        }
        other => panic!("expected NotFound for missing pid; got {other:?}"),
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_edit_on_semantic_memory() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let source = client.remember("source episodic", scope.clone()).await?;
    let db = client.raw_db().await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO memories (pid, agent_id, org_id, user_id, content, kind, source_pid)
        VALUES ($1, $2, $3, $4, $5, 'semantic', $6)
        "#,
        [
            Value::from("test_edit_semantic"),
            Value::from(scope.agent_id.as_str()),
            Value::from(scope.org_id.as_str()),
            Value::from(scope.user_id.as_str()),
            Value::from("derived fact"),
            Value::from(source.pid.as_str()),
        ],
    ))
    .await?;

    let result = client.edit("test_edit_semantic").content("hand-edit attempt").await;

    match result {
        Err(ClientError::Store(StoreError::UnsupportedEdit { pid, kind })) => {
            assert_eq!(pid, "test_edit_semantic");
            assert_eq!(kind, memoir_core::memory::MemoryKind::Semantic);
        }
        other => panic!("expected UnsupportedEdit for semantic kind; got {other:?}"),
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_allow_edit_on_superseded_memory() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let original = client.remember("v1 text", scope.clone()).await?;
    let winner = client.remember("v2 text", scope.clone()).await?;

    let db = client.raw_db().await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from(original.pid.as_str()), Value::from(winner.pid.as_str())],
    ))
    .await?;

    let edited = client.edit(&original.pid).content("v1 text corrected").await?;
    assert_eq!(edited.content, "v1 text corrected");
    assert!(
        edited.supersession.is_some(),
        "supersession state must be preserved across edits to soft-deleted rows",
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_set_event_at_via_edit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("deployment happened", scope.clone()).await?;
    assert!(
        written.event_at.is_none(),
        "fresh remember without .event_at must default to None"
    );

    let deploy_time = chrono::DateTime::parse_from_rfc3339("2026-04-01T15:30:00Z")?;
    let edited = client.edit(&written.pid).event_at(deploy_time).await?;

    assert_eq!(edited.event_at, Some(deploy_time));
    assert_eq!(
        edited.content, "deployment happened",
        "content must be untouched when only event_at is set"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_replace_metadata_on_edit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client
        .remember("with metadata", scope.clone())
        .metadata(serde_json::json!({ "version": 1, "tag": "old" }))
        .await?;

    let edited = client
        .edit(&written.pid)
        .metadata(serde_json::json!({ "version": 2 }))
        .await?;

    assert_eq!(edited.metadata, serde_json::json!({ "version": 2 }));
    assert_eq!(edited.content, "with metadata", "content untouched");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_edit_with_reserved_metadata_key() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("reserved key test", scope.clone()).await?;
    let result = client
        .edit(&written.pid)
        .metadata(serde_json::json!({ "pid": "smuggled" }))
        .await;

    match result {
        Err(ClientError::ReservedMetadataKey { key }) => {
            assert_eq!(key, "pid");
        }
        other => panic!("expected ReservedMetadataKey rejection; got {other:?}"),
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_enqueue_embed_job_after_content_edit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("edit-enqueue-check", scope.clone()).await?;
    common::wait_until_indexed(
        &client,
        &written.pid,
        &scope,
        "edit-enqueue-check",
        Duration::from_secs(15),
    )
    .await?;

    let _ = client.edit(&written.pid).content("edited content").await?;

    let db = client.raw_db().await?;
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT COUNT(*) AS n
            FROM memory_jobs
            WHERE source_pid = $1
              AND kind = 'embed'
              AND payload ->> 'origin' = 'edit'
            "#,
            [Value::from(written.pid.as_str())],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("count returned no row"))?;
    let n: i64 = row.try_get("", "n")?;
    assert_eq!(
        n, 1,
        "content edits must enqueue exactly one Embed job with origin = 'edit' for the pid"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_not_enqueue_embed_job_when_only_metadata_changes() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("metadata-only-edit", scope.clone()).await?;
    common::wait_until_indexed(
        &client,
        &written.pid,
        &scope,
        "metadata-only-edit",
        Duration::from_secs(15),
    )
    .await?;

    let _ = client
        .edit(&written.pid)
        .metadata(serde_json::json!({ "note": "changed" }))
        .await?;

    let db = client.raw_db().await?;
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT COUNT(*) AS n
            FROM memory_jobs
            WHERE source_pid = $1
              AND kind = 'embed'
              AND payload ->> 'origin' = 'edit'
            "#,
            [Value::from(written.pid.as_str())],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("count returned no row"))?;
    let n: i64 = row.try_get("", "n")?;
    assert_eq!(
        n, 0,
        "metadata-only edits must not enqueue re-embed jobs; embedding vector is still representative of content",
    );

    Ok(())
}
