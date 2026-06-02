//! Integration tests for `Client::timeline`.

#![cfg(feature = "integration")]

use std::time::Duration;

use sea_orm::{ConnectionTrait, Statement, Value};

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_memories_in_newest_first_order_by_default() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let first = client.remember("first", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let second = client.remember("second", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let third = client.remember("third", scope.clone()).await?;

    let memories = client.timeline(scope).await?;

    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(
        pids,
        vec![third.pid.as_str(), second.pid.as_str(), first.pid.as_str()],
        "default order must be newest-first by created_at"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_memories_in_oldest_first_order_when_ascending() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let first = client.remember("first", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let second = client.remember("second", scope.clone()).await?;

    let memories = client.timeline(scope).ascending().await?;

    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(pids, vec![first.pid.as_str(), second.pid.as_str()]);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_include_superseded_memories_by_default() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let loser = client.remember("v1", scope.clone()).await?;
    let winner = client.remember("v2", scope.clone()).await?;

    let db = client.raw_db().await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from(loser.pid.as_str()), Value::from(winner.pid.as_str())],
    ))
    .await?;

    let memories = client.timeline(scope).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&loser.pid.as_str()));
    assert!(pids.contains(&winner.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_exclude_superseded_memories_when_excluded() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let loser = client.remember("v1", scope.clone()).await?;
    let winner = client.remember("v2", scope.clone()).await?;

    let db = client.raw_db().await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from(loser.pid.as_str()), Value::from(winner.pid.as_str())],
    ))
    .await?;

    let memories = client.timeline(scope).exclude_superseded().await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(!pids.contains(&loser.pid.as_str()), "superseded row must be excluded");
    assert!(pids.contains(&winner.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_filter_by_kind_when_episodic() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let episodic = client.remember("episodic row", scope.clone()).await?;

    let db = client.raw_db().await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO memories (pid, agent_id, org_id, user_id, content, kind, source_pid)
        VALUES ($1, $2, $3, $4, $5, 'semantic', $6)
        "#,
        [
            Value::from("test_timeline_semantic"),
            Value::from(scope.agent_id.as_str()),
            Value::from(scope.org_id.as_str()),
            Value::from(scope.user_id.as_str()),
            Value::from("semantic row"),
            Value::from(episodic.pid.as_str()),
        ],
    ))
    .await?;

    let memories = client.timeline(scope).episodic().await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&episodic.pid.as_str()));
    assert!(!pids.contains(&"test_timeline_semantic"));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_apply_created_at_window() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let before = client.remember("before window", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let window_start: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let inside = client.remember("inside window", scope.clone()).await?;
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let window_end: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let after = client.remember("after window", scope.clone()).await?;

    let memories = client
        .timeline(scope)
        .created_after(window_start)
        .created_before(window_end)
        .await?;

    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(pids, vec![inside.pid.as_str()]);
    assert!(!pids.contains(&before.pid.as_str()));
    assert!(!pids.contains(&after.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_respect_limit() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    for i in 0..5 {
        client.remember(format!("row {i}"), scope.clone()).await?;
    }

    let memories = client.timeline(scope).limit(3).await?;
    assert_eq!(memories.len(), 3);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_isolate_by_scope() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    let a = client.remember("scope a content", scope_a.clone()).await?;
    let _b = client.remember("scope b content", scope_b.clone()).await?;

    let memories = client.timeline(scope_a).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(pids, vec![a.pid.as_str()]);
    Ok(())
}
