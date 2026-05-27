//! Integration tests for `Client::recall_as_of`.

#![cfg(feature = "integration")]

use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ConnectionTrait, Statement, Value};

mod common;

async fn insert_event(
    db: &sea_orm::DatabaseConnection,
    loser_pid: &str,
    winner_pid: Option<&str>,
    decided_at: DateTime<FixedOffset>,
) -> anyhow::Result<()> {
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid, decided_at) VALUES ($1, $2, $3)",
        [
            Value::from(loser_pid),
            match winner_pid {
                Some(pid) => Value::from(pid),
                None => Value::String(None),
            },
            Value::from(decided_at),
        ],
    ))
    .await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_include_memory_created_before_timestamp() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let written = client.remember("written before t", scope.clone()).await?;
    let as_of: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(60)).into();

    let memories = client.recall_as_of(scope, as_of).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&written.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_exclude_memory_created_after_timestamp() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let as_of: DateTime<FixedOffset> = (Utc::now() - chrono::Duration::seconds(60)).into();
    let _ = client.remember("written after t", scope.clone()).await?;

    let memories = client.recall_as_of(scope, as_of).await?;
    assert!(memories.is_empty());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_include_superseded_memory_as_of_before_supersession() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let a = client.remember("a", scope.clone()).await?;
    let b = client.remember("b", scope.clone()).await?;

    let between: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(5)).into();
    let after_supersede: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(20)).into();

    let db = client.raw_db().await?;
    insert_event(&db, &a.pid, Some(&b.pid), (Utc::now() + chrono::Duration::seconds(10)).into()).await?;

    let memories = client.recall_as_of(scope.clone(), between).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&a.pid.as_str()), "a should be active at t < supersede");
    assert!(pids.contains(&b.pid.as_str()));

    let memories_after = client.recall_as_of(scope, after_supersede).await?;
    let pids_after: Vec<&str> = memories_after.iter().map(|m| m.pid.as_str()).collect();
    assert!(!pids_after.contains(&a.pid.as_str()), "a should be excluded after supersede");
    assert!(pids_after.contains(&b.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_handle_unsupersede_in_history() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let a = client.remember("a", scope.clone()).await?;
    let b = client.remember("b", scope.clone()).await?;

    let t_super: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(5)).into();
    let between_super_and_unsuper: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(10)).into();
    let t_unsuper: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(15)).into();
    let after_unsuper: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(20)).into();

    let db = client.raw_db().await?;
    insert_event(&db, &a.pid, Some(&b.pid), t_super).await?;
    insert_event(&db, &a.pid, None, t_unsuper).await?;

    let mid = client.recall_as_of(scope.clone(), between_super_and_unsuper).await?;
    let mid_pids: Vec<&str> = mid.iter().map(|m| m.pid.as_str()).collect();
    assert!(!mid_pids.contains(&a.pid.as_str()), "a excluded during supersede window");

    let late = client.recall_as_of(scope, after_unsuper).await?;
    let late_pids: Vec<&str> = late.iter().map(|m| m.pid.as_str()).collect();
    assert!(late_pids.contains(&a.pid.as_str()), "a included after unsupersede");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_handle_multiple_supersessions() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let a = client.remember("a", scope.clone()).await?;
    let b = client.remember("b", scope.clone()).await?;
    let c = client.remember("c", scope.clone()).await?;

    let t_super_b: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(5)).into();
    let between: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(10)).into();
    let t_super_c: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(15)).into();
    let after_both: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(20)).into();

    let db = client.raw_db().await?;
    insert_event(&db, &a.pid, Some(&b.pid), t_super_b).await?;
    insert_event(&db, &a.pid, Some(&c.pid), t_super_c).await?;

    let mid = client.recall_as_of(scope.clone(), between).await?;
    assert!(!mid.iter().any(|m| m.pid == a.pid), "a excluded after first supersede");

    let after = client.recall_as_of(scope, after_both).await?;
    assert!(!after.iter().any(|m| m.pid == a.pid), "a still excluded after re-supersede");
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_empty_when_as_of_predates_all_memories() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let _ = client.remember("written today", scope.clone()).await?;
    let ancient: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2000-01-01T00:00:00Z")?;

    let memories = client.recall_as_of(scope, ancient).await?;
    assert!(memories.is_empty());
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_return_current_state_when_as_of_is_future() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope = common::fresh_scope();

    let a = client.remember("a", scope.clone()).await?;
    let b = client.remember("b", scope.clone()).await?;

    let future: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::days(365)).into();
    let memories = client.recall_as_of(scope, future).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert!(pids.contains(&a.pid.as_str()));
    assert!(pids.contains(&b.pid.as_str()));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_isolate_by_scope() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let scope_a = common::fresh_scope();
    let scope_b = common::fresh_scope();

    let a = client.remember("scope a", scope_a.clone()).await?;
    let _b = client.remember("scope b", scope_b.clone()).await?;

    let future: DateTime<FixedOffset> = (Utc::now() + chrono::Duration::seconds(60)).into();
    let memories = client.recall_as_of(scope_a, future).await?;
    let pids: Vec<&str> = memories.iter().map(|m| m.pid.as_str()).collect();
    assert_eq!(pids, vec![a.pid.as_str()]);
    Ok(())
}
