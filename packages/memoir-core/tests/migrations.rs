//! Integration tests for migrations 0004 (add_event_at) and 0005
//! (create_supersession_events + trigger).
//!
//! These tests issue raw SQL via the harness's `raw_db()` helper because the
//! features under test — the new column, the new audit table, and the
//! trigger that syncs the `superseded_by` cache — are not yet reachable
//! through the `Client` or `MemoryStore` APIs. The storage-layer rewrite
//! (ticket 0010/0003) is what makes them callable; these migration tests
//! exist to lock the schema's behavior independently of that work.
//!
//! Each test runs against a fresh per-schema partition (see
//! `common::fresh_client`), so they do not contaminate each other and do
//! not require a teardown step in the test body.

#![cfg(feature = "integration")]

use sea_orm::{ConnectionTrait, Statement, Value};

mod common;

/// Inserts a minimum-viable memory row via raw SQL.
///
/// Bypasses `Client::remember` because these migration tests precede the
/// store-layer rewrite that would make `event_at` and the supersession
/// audit table reachable through the trait. Returns the inserted pid.
async fn insert_memory(
    db: &sea_orm::DatabaseConnection,
    pid: &str,
    content: &str,
) -> anyhow::Result<()> {
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO memories (pid, agent_id, org_id, user_id, content, kind)
        VALUES ($1, 'a', 'o', 'u', $2, 'episodic')
        "#,
        [Value::from(pid), Value::from(content)],
    ))
    .await?;
    Ok(())
}

/// Reads `memories.superseded_by` for the given pid.
async fn read_superseded_by(
    db: &sea_orm::DatabaseConnection,
    pid: &str,
) -> anyhow::Result<Option<String>> {
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT superseded_by FROM memories WHERE pid = $1",
            [Value::from(pid)],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("no row for pid {pid}"))?;
    Ok(row.try_get("", "superseded_by")?)
}

// ---------- Migration 0004 (add_event_at) ----------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_default_event_at_to_null_when_unspecified_on_insert() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "test_pid_0004_default", "hello").await?;

    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT event_at FROM memories WHERE pid = $1",
            [Value::from("test_pid_0004_default")],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("row not found"))?;

    let event_at: Option<chrono::DateTime<chrono::FixedOffset>> = row.try_get("", "event_at")?;
    assert!(event_at.is_none(), "event_at must default to NULL when not specified");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_round_trip_event_at_when_set_via_raw_sql() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO memories (pid, agent_id, org_id, user_id, content, kind, event_at)
        VALUES ($1, 'a', 'o', 'u', 'event content', 'episodic', '2026-01-15T12:00:00Z')
        "#,
        [Value::from("test_pid_0004_set")],
    ))
    .await?;

    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT event_at FROM memories WHERE pid = $1",
            [Value::from("test_pid_0004_set")],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("row not found"))?;

    let event_at: Option<chrono::DateTime<chrono::FixedOffset>> = row.try_get("", "event_at")?;
    let event_at = event_at.expect("event_at should be Some after explicit insert");
    assert_eq!(event_at.to_rfc3339(), "2026-01-15T12:00:00+00:00");

    Ok(())
}

// ---------- Migration 0005 (supersession_events + trigger) ----------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_sync_superseded_by_cache_when_supersession_event_inserted() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;
    insert_memory(&db, "winner_b", "fact B").await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from("loser_a"), Value::from("winner_b")],
    ))
    .await?;

    let cached = read_superseded_by(&db, "loser_a").await?;
    assert_eq!(
        cached.as_deref(),
        Some("winner_b"),
        "trigger must set memories.superseded_by to the winner pid"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_clear_superseded_by_cache_when_unsupersede_event_inserted() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;
    insert_memory(&db, "winner_b", "fact B").await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from("loser_a"), Value::from("winner_b")],
    ))
    .await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, NULL)",
        [Value::from("loser_a")],
    ))
    .await?;

    let cached = read_superseded_by(&db, "loser_a").await?;
    assert!(cached.is_none(), "an unsupersede event must clear the cache to NULL");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_resolve_to_latest_event_when_multiple_events_for_same_loser() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;
    insert_memory(&db, "winner_b", "fact B").await?;
    insert_memory(&db, "winner_c", "fact C").await?;

    // Two supersession events for loser_a with explicit decided_at values
    // ordered in time. The cache must reflect the later one.
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO supersession_events (loser_pid, winner_pid, decided_at)
        VALUES ($1, $2, '2026-01-01T00:00:00Z')
        "#,
        [Value::from("loser_a"), Value::from("winner_b")],
    ))
    .await?;
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        r#"
        INSERT INTO supersession_events (loser_pid, winner_pid, decided_at)
        VALUES ($1, $2, '2026-02-01T00:00:00Z')
        "#,
        [Value::from("loser_a"), Value::from("winner_c")],
    ))
    .await?;

    let cached = read_superseded_by(&db, "loser_a").await?;
    assert_eq!(
        cached.as_deref(),
        Some("winner_c"),
        "the latest supersession event by decided_at must win the cache"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_supersession_event_when_loser_pid_does_not_exist() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "winner_b", "fact B").await?;

    let result = db
        .execute_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
            [Value::from("nonexistent_loser"), Value::from("winner_b")],
        ))
        .await;

    assert!(result.is_err(), "FK violation must reject the insert");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_supersession_event_when_winner_pid_does_not_exist() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;

    let result = db
        .execute_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
            [Value::from("loser_a"), Value::from("nonexistent_winner")],
        ))
        .await;

    assert!(result.is_err(), "FK violation must reject the insert");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_clear_winner_pid_and_cache_when_winner_is_forgotten() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;
    insert_memory(&db, "winner_b", "fact B").await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from("loser_a"), Value::from("winner_b")],
    ))
    .await?;

    // Delete the winner. ON DELETE SET NULL must null out winner_pid in
    // supersession_events. The cache update on memories.superseded_by is
    // separately driven by migration 0003's existing FK (superseded_by
    // REFERENCES memories(pid) ON DELETE SET NULL).
    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "DELETE FROM memories WHERE pid = $1",
        [Value::from("winner_b")],
    ))
    .await?;

    // The supersession_events row still exists but its winner_pid is NULL.
    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT winner_pid FROM supersession_events WHERE loser_pid = $1",
            [Value::from("loser_a")],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("event row missing after winner deletion"))?;
    let winner_pid: Option<String> = row.try_get("", "winner_pid")?;
    assert!(winner_pid.is_none(), "winner_pid must be NULL after winner forget (ON DELETE SET NULL)");

    // The memories.superseded_by cache for the loser is cleared by the FK
    // from migration 0003. The trigger does not re-fire on the winner
    // deletion (it only fires on supersession_events INSERT), but the
    // memories.superseded_by FK with ON DELETE SET NULL handles this case
    // independently.
    let cached = read_superseded_by(&db, "loser_a").await?;
    assert!(cached.is_none(), "loser's cached superseded_by must be NULL after winner forget");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_cascade_delete_events_when_loser_is_forgotten() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;
    insert_memory(&db, "winner_b", "fact B").await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $2)",
        [Value::from("loser_a"), Value::from("winner_b")],
    ))
    .await?;

    db.execute_raw(Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        "DELETE FROM memories WHERE pid = $1",
        [Value::from("loser_a")],
    ))
    .await?;

    let row = db
        .query_one_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*) AS c FROM supersession_events WHERE loser_pid = $1",
            [Value::from("loser_a")],
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("count query returned no row"))?;
    let count: i64 = row.try_get("", "c")?;
    assert_eq!(count, 0, "supersession_events must cascade-delete when loser is forgotten");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn should_reject_same_row_supersession_via_check_constraint() -> anyhow::Result<()> {
    let client = common::fresh_client().await?;
    let db = client.raw_db().await?;

    insert_memory(&db, "loser_a", "fact A").await?;

    let result = db
        .execute_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "INSERT INTO supersession_events (loser_pid, winner_pid) VALUES ($1, $1)",
            [Value::from("loser_a")],
        ))
        .await;

    assert!(result.is_err(), "CHECK (loser_pid != winner_pid) must reject same-row supersession");

    Ok(())
}
