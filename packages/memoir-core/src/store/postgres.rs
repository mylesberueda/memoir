//! [`MemoryStore`] implementation backed by Postgres.

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaOrmValue};

use super::{IndexStatus, MemoryStore, StoreError};
use crate::memory::{ForgetTarget, Memory, MemoryKind, Scope};

const PID_LENGTH: usize = 21;

/// Column list shared by every `memory_from_row`-bound SELECT.
///
/// `supersession_at` is sourced from the `supersession_events` audit table
/// via correlated subquery, gated on the cached `superseded_by` column so
/// active rows return `NULL` even when a prior unsupersede event exists.
/// The compound index `supersession_events_loser_decided_idx` makes the
/// subquery an indexed lookup.
const MEMORY_SELECT_COLUMNS: &str = "
    m.pid,
    m.agent_id,
    m.org_id,
    m.user_id,
    m.content,
    m.metadata,
    m.kind,
    m.source_pid,
    m.superseded_by,
    m.created_at,
    m.updated_at,
    m.event_at,
    CASE
        WHEN m.superseded_by IS NULL THEN NULL
        ELSE (
            SELECT MAX(decided_at)
            FROM supersession_events
            WHERE loser_pid = m.pid
        )
    END AS supersession_at
";

/// Default [`MemoryStore`] backed by Postgres.
///
/// Constructed via [`Self::new`] from an existing
/// [`sea_orm::DatabaseConnection`]. The caller owns the connection's
/// lifecycle; this store does not pool or reconnect.
#[derive(Debug, Clone)]
pub struct PostgresStore {
    db: DatabaseConnection,
}

impl PostgresStore {
    /// Builds a store from an existing Postgres connection.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Returns the underlying Postgres connection.
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl MemoryStore for PostgresStore {
    async fn remember(
        &self,
        scope: Scope,
        content: String,
        metadata: serde_json::Value,
        kind: MemoryKind,
        source_pid: Option<String>,
    ) -> Result<Memory, StoreError> {
        validate_scope(&scope)?;

        let pid = nanoid::nanoid!(PID_LENGTH);

        // Newly inserted rows have no supersession history by construction,
        // so `RETURNING` includes the on-row columns only and `supersession_at`
        // is hardcoded NULL — saves the correlated-subquery cost on the hot
        // write path. `memory_from_row` handles the NULL uniformly with rows
        // hydrated by SELECT paths that do run the subquery.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO memories (pid, agent_id, org_id, user_id, content, metadata, kind, source_pid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                pid, agent_id, org_id, user_id, content, metadata, kind,
                source_pid, superseded_by, created_at, updated_at, event_at,
                NULL::TIMESTAMPTZ AS supersession_at
            "#,
            [
                SeaOrmValue::String(Some(pid)),
                SeaOrmValue::String(Some(scope.agent_id.clone())),
                SeaOrmValue::String(Some(scope.org_id.clone())),
                SeaOrmValue::String(Some(scope.user_id.clone())),
                SeaOrmValue::String(Some(content)),
                SeaOrmValue::Json(Some(Box::new(metadata))),
                SeaOrmValue::String(Some(kind.to_string())),
                SeaOrmValue::String(source_pid),
            ],
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(database)?
            .ok_or_else(|| StoreError::Database("insert returned no row".to_string()))?;

        memory_from_row(&row).map(|mut m| {
            m.score = None;
            m
        })
    }

    async fn recall(&self, pid: &str) -> Result<Memory, StoreError> {
        if pid.is_empty() {
            return Err(StoreError::NotFound(pid.to_string()));
        }

        let select_sql = format!("SELECT {MEMORY_SELECT_COLUMNS} FROM memories m WHERE m.pid = $1");
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            select_sql,
            [SeaOrmValue::String(Some(pid.to_string()))],
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(database)?
            .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;

        memory_from_row(&row)
    }

    async fn find_by_pids(&self, pids: &[&str]) -> Result<Vec<Memory>, StoreError> {
        if pids.is_empty() {
            return Ok(Vec::new());
        }

        let owned_pids: Vec<String> = pids.iter().map(|p| (*p).to_string()).collect();
        let select_sql = format!(
            "SELECT {MEMORY_SELECT_COLUMNS} FROM memories m \
             WHERE m.pid = ANY($1) AND m.qdrant_status = 'indexed' AND m.superseded_by IS NULL"
        );
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            select_sql,
            [SeaOrmValue::Array(
                sea_orm::sea_query::ArrayType::String,
                Some(Box::new(
                    owned_pids.into_iter().map(|p| SeaOrmValue::String(Some(p))).collect(),
                )),
            )],
        );

        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(memory_from_row(row)?);
        }
        Ok(memories)
    }

    async fn forget(&self, target: ForgetTarget) -> Result<Vec<String>, StoreError> {
        match target {
            ForgetTarget::Pid(pid) => self.forget_pid(&pid).await,
            ForgetTarget::Scope(scope) => self.forget_scope(scope).await,
        }
    }

    async fn set_index_status(&self, pid: &str, status: IndexStatus) -> Result<(), StoreError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "UPDATE memories SET qdrant_status = $1 WHERE pid = $2",
            [
                SeaOrmValue::String(Some(status.to_string())),
                SeaOrmValue::String(Some(pid.to_string())),
            ],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;

        if result.rows_affected() == 0 {
            return Err(StoreError::NotFound(pid.to_string()));
        }
        Ok(())
    }

    async fn find_failed(&self, limit: usize) -> Result<Vec<Memory>, StoreError> {
        let select_sql =
            format!("SELECT {MEMORY_SELECT_COLUMNS} FROM memories m WHERE m.qdrant_status = 'failed' LIMIT $1");
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            select_sql,
            [SeaOrmValue::BigInt(Some(limit as i64))],
        );

        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(memory_from_row(row)?);
        }
        Ok(memories)
    }

    async fn list_scopes(&self) -> Result<Vec<Scope>, StoreError> {
        let stmt = Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT DISTINCT agent_id, org_id, user_id FROM memories".to_string(),
        );
        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;

        let mut scopes = Vec::with_capacity(rows.len());
        for row in &rows {
            scopes.push(Scope {
                agent_id: row.try_get::<String>("", "agent_id").map_err(database)?,
                org_id: row.try_get::<String>("", "org_id").map_err(database)?,
                user_id: row.try_get::<String>("", "user_id").map_err(database)?,
            });
        }
        Ok(scopes)
    }

    async fn indexed_pids_in_scope(&self, scope: &Scope) -> Result<Vec<String>, StoreError> {
        validate_scope(scope)?;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT pid FROM memories
            WHERE agent_id = $1 AND org_id = $2 AND user_id = $3
              AND qdrant_status = 'indexed'
            "#,
            [
                SeaOrmValue::String(Some(scope.agent_id.clone())),
                SeaOrmValue::String(Some(scope.org_id.clone())),
                SeaOrmValue::String(Some(scope.user_id.clone())),
            ],
        );

        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut pids = Vec::with_capacity(rows.len());
        for row in &rows {
            pids.push(row.try_get::<String>("", "pid").map_err(database)?);
        }
        Ok(pids)
    }

    async fn supersede(&self, pid: &str, by_pid: &str) -> Result<(), StoreError> {
        // `INSERT ... SELECT ... WHERE EXISTS` keeps the contract identical
        // to the old UPDATE-based path: if the loser pid does not exist,
        // zero rows are inserted and we surface `NotFound`. The trigger
        // (migration 0005) maintains `memories.superseded_by` from the
        // inserted event. FK violations on `winner_pid` still bubble up as
        // `Database` errors when `by_pid` doesn't exist.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO supersession_events (loser_pid, winner_pid)
            SELECT $1, $2
            WHERE EXISTS (SELECT 1 FROM memories WHERE pid = $1)
            "#,
            [
                SeaOrmValue::String(Some(pid.to_string())),
                SeaOrmValue::String(Some(by_pid.to_string())),
            ],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;

        if result.rows_affected() == 0 {
            return Err(StoreError::NotFound(pid.to_string()));
        }
        Ok(())
    }

    async fn unsupersede(&self, pid: &str) -> Result<(), StoreError> {
        // Same EXISTS-guarded INSERT shape as `supersede`; `winner_pid` is
        // NULL to encode an unsupersede event. Per DP2, this always inserts
        // (no cache pre-check) — the audit table reflects every operator
        // call, even redundant ones against an already-active row.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO supersession_events (loser_pid, winner_pid)
            SELECT $1, NULL
            WHERE EXISTS (SELECT 1 FROM memories WHERE pid = $1)
            "#,
            [SeaOrmValue::String(Some(pid.to_string()))],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;

        if result.rows_affected() == 0 {
            return Err(StoreError::NotFound(pid.to_string()));
        }
        Ok(())
    }

    async fn supersession_at(&self, pid: &str, as_of: DateTime<FixedOffset>) -> Result<Option<String>, StoreError> {
        // Returns the winner_pid for `pid` as of timestamp `as_of`, or
        // `None` if the row was not superseded at that time (either it
        // had no events, or its latest event before `as_of` was an
        // unsupersede). The compound index
        // `supersession_events_loser_decided_idx` makes this an indexed
        // ORDER BY + LIMIT.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT winner_pid
            FROM supersession_events
            WHERE loser_pid = $1 AND decided_at <= $2
            ORDER BY decided_at DESC
            LIMIT 1
            "#,
            [
                SeaOrmValue::String(Some(pid.to_string())),
                SeaOrmValue::ChronoDateTimeWithTimeZone(Some(as_of)),
            ],
        );

        let row = self.db.query_one_raw(stmt).await.map_err(database)?;
        match row {
            None => Ok(None),
            Some(row) => row.try_get("", "winner_pid").map_err(database),
        }
    }
}

impl PostgresStore {
    async fn forget_pid(&self, pid: &str) -> Result<Vec<String>, StoreError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memories WHERE pid = $1 RETURNING pid",
            [SeaOrmValue::String(Some(pid.to_string()))],
        );
        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut deleted = Vec::with_capacity(rows.len());
        for row in &rows {
            deleted.push(row.try_get::<String>("", "pid").map_err(database)?);
        }
        Ok(deleted)
    }

    async fn forget_scope(&self, scope: Scope) -> Result<Vec<String>, StoreError> {
        validate_scope(&scope)?;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memories WHERE agent_id = $1 AND org_id = $2 AND user_id = $3 RETURNING pid",
            [
                SeaOrmValue::String(Some(scope.agent_id)),
                SeaOrmValue::String(Some(scope.org_id)),
                SeaOrmValue::String(Some(scope.user_id)),
            ],
        );
        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut deleted = Vec::with_capacity(rows.len());
        for row in &rows {
            deleted.push(row.try_get::<String>("", "pid").map_err(database)?);
        }
        Ok(deleted)
    }
}

fn validate_scope(scope: &Scope) -> Result<(), StoreError> {
    if scope.agent_id.is_empty() || scope.org_id.is_empty() || scope.user_id.is_empty() {
        return Err(StoreError::InvalidScope(
            "agent_id, org_id, and user_id must all be non-empty".to_string(),
        ));
    }
    Ok(())
}

fn database<E: std::fmt::Display>(err: E) -> StoreError {
    StoreError::Database(err.to_string())
}

fn memory_from_row(row: &sea_orm::QueryResult) -> Result<Memory, StoreError> {
    let pid: String = row.try_get("", "pid").map_err(database)?;
    let agent_id: String = row.try_get("", "agent_id").map_err(database)?;
    let org_id: String = row.try_get("", "org_id").map_err(database)?;
    let user_id: String = row.try_get("", "user_id").map_err(database)?;
    let content: String = row.try_get("", "content").map_err(database)?;
    let metadata: serde_json::Value = row.try_get("", "metadata").map_err(database)?;
    let kind_str: String = row.try_get("", "kind").map_err(database)?;
    let source_pid: Option<String> = row.try_get("", "source_pid").map_err(database)?;
    let superseded_by: Option<String> = row.try_get("", "superseded_by").map_err(database)?;
    let created_at: DateTime<FixedOffset> = row.try_get("", "created_at").map_err(database)?;
    let updated_at: DateTime<FixedOffset> = row.try_get("", "updated_at").map_err(database)?;
    let event_at: Option<DateTime<FixedOffset>> = row.try_get("", "event_at").map_err(database)?;
    let supersession_at: Option<DateTime<FixedOffset>> = row.try_get("", "supersession_at").map_err(database)?;

    let kind: MemoryKind = kind_str
        .parse()
        .map_err(|_| StoreError::Database(format!("unknown memory kind: {kind_str}")))?;

    // `superseded_by` (the cache) and `supersession_at` (computed via
    // correlated subquery) are populated together by the SQL: the CASE
    // expression in MEMORY_SELECT_COLUMNS gates the subquery on
    // `superseded_by IS NOT NULL`, so an active row has both `None` and
    // a superseded row has both `Some`. Mismatched populations would
    // signal a row that's been mutated between cache and audit table —
    // surfaced as a `Database` error here rather than silently dropping
    // one half.
    let supersession = match (superseded_by, supersession_at) {
        (Some(winner_pid), Some(at)) => Some(crate::memory::SupersessionInfo { winner_pid, at }),
        (None, None) => None,
        (Some(winner_pid), None) => {
            return Err(StoreError::Database(format!(
                "row {pid}: superseded_by={winner_pid} but no supersession_events row found"
            )));
        }
        (None, Some(_)) => {
            return Err(StoreError::Database(format!(
                "row {pid}: supersession_at populated but superseded_by is NULL"
            )));
        }
    };

    Ok(Memory {
        pid,
        scope: Scope {
            agent_id,
            org_id,
            user_id,
        },
        content,
        metadata,
        kind,
        source_pid,
        supersession,
        created_at,
        updated_at,
        event_at,
        score: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_reject_scope_with_empty_agent_id() {
        let scope = Scope {
            agent_id: "".to_string(),
            org_id: "o".to_string(),
            user_id: "u".to_string(),
        };
        assert!(matches!(validate_scope(&scope), Err(StoreError::InvalidScope(_))));
    }

    #[test]
    fn should_reject_scope_with_empty_org_id() {
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "".to_string(),
            user_id: "u".to_string(),
        };
        assert!(matches!(validate_scope(&scope), Err(StoreError::InvalidScope(_))));
    }

    #[test]
    fn should_reject_scope_with_empty_user_id() {
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "o".to_string(),
            user_id: "".to_string(),
        };
        assert!(matches!(validate_scope(&scope), Err(StoreError::InvalidScope(_))));
    }

    #[test]
    fn should_accept_scope_with_all_non_empty_fields() {
        let scope = Scope {
            agent_id: "a".to_string(),
            org_id: "o".to_string(),
            user_id: "u".to_string(),
        };
        assert!(validate_scope(&scope).is_ok());
    }
}
