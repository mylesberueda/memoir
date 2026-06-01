//! [`MemoryStore`] implementation backed by Postgres.

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaOrmValue};

use super::{AsOfParams, EditPatch, IndexStatus, MemoryStore, StoreError, TimelineDirection, TimelineParams};
use crate::memory::{ForgetTarget, Memory, MemoryKind, Scope, SupersessionEvent};

const PID_LENGTH: usize = 21;

/// Column list shared by every `Memory::try_from`-bound SELECT.
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
    m.qdrant_status,
    m.source_pid,
    m.superseded_by,
    m.created_at,
    m.updated_at,
    m.event_at,
    m.confidence,
    m.category,
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
        event_at: Option<DateTime<FixedOffset>>,
    ) -> Result<Memory, StoreError> {
        scope.validate()?;

        let pid = nanoid::nanoid!(PID_LENGTH);

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO memories (pid, agent_id, org_id, user_id, content, metadata, kind, source_pid, event_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                pid, agent_id, org_id, user_id, content, metadata, kind,
                qdrant_status, source_pid, superseded_by, created_at, updated_at, event_at,
                confidence, category,
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
                SeaOrmValue::ChronoDateTimeWithTimeZone(event_at),
            ],
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await?
            .ok_or_else(|| StoreError::CacheInvariant("insert returned no row".to_string()))?;

        Memory::try_from(&row).map(|mut m| {
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
            .await?
            .ok_or_else(|| StoreError::NotFound(pid.to_string()))?;

        Memory::try_from(&row)
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

        let rows = self.db.query_all_raw(stmt).await?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(Memory::try_from(row)?);
        }
        Ok(memories)
    }

    async fn timeline(&self, scope: Scope, params: TimelineParams) -> Result<Vec<Memory>, StoreError> {
        scope.validate()?;

        let mut where_clauses: Vec<String> = vec![
            "m.agent_id = $1".into(),
            "m.org_id = $2".into(),
            "m.user_id = $3".into(),
        ];
        let mut values: Vec<SeaOrmValue> = vec![
            SeaOrmValue::String(Some(scope.agent_id)),
            SeaOrmValue::String(Some(scope.org_id)),
            SeaOrmValue::String(Some(scope.user_id)),
        ];

        let included = params.kinds.included_kinds();
        if included.is_empty() {
            return Ok(Vec::new());
        }
        if !params.kinds.includes_all() {
            let placeholders: Vec<String> = included
                .iter()
                .map(|kind| {
                    values.push(SeaOrmValue::String(Some(kind.to_string())));
                    format!("${}", values.len())
                })
                .collect();
            where_clauses.push(format!("m.kind IN ({})", placeholders.join(", ")));
        }

        if let Some(t) = params.created_after {
            values.push(SeaOrmValue::ChronoDateTimeWithTimeZone(Some(t)));
            where_clauses.push(format!("m.created_at >= ${}", values.len()));
        }
        if let Some(t) = params.created_before {
            values.push(SeaOrmValue::ChronoDateTimeWithTimeZone(Some(t)));
            where_clauses.push(format!("m.created_at < ${}", values.len()));
        }
        if let Some(t) = params.event_at_after {
            values.push(SeaOrmValue::ChronoDateTimeWithTimeZone(Some(t)));
            where_clauses.push(format!("m.event_at >= ${}", values.len()));
        }
        if let Some(t) = params.event_at_before {
            values.push(SeaOrmValue::ChronoDateTimeWithTimeZone(Some(t)));
            where_clauses.push(format!("m.event_at < ${}", values.len()));
        }
        if !params.include_superseded {
            where_clauses.push("m.superseded_by IS NULL".into());
        }

        let order = match params.direction {
            TimelineDirection::Descending => "DESC",
            TimelineDirection::Ascending => "ASC",
        };

        values.push(SeaOrmValue::BigInt(Some(params.limit as i64)));
        let limit_placeholder = values.len();

        let sql = format!(
            "SELECT {MEMORY_SELECT_COLUMNS} FROM memories m \
             WHERE {where_sql} \
             ORDER BY m.created_at {order} \
             LIMIT ${limit_placeholder}",
            where_sql = where_clauses.join(" AND "),
        );
        let stmt = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

        let rows = self.db.query_all_raw(stmt).await?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(Memory::try_from(row)?);
        }
        Ok(memories)
    }

    async fn memories_as_of(&self, scope: Scope, params: AsOfParams) -> Result<Vec<Memory>, StoreError> {
        scope.validate()?;

        let included = params.kinds.included_kinds();
        if included.is_empty() {
            return Ok(Vec::new());
        }

        let mut where_clauses: Vec<String> = vec![
            "m.agent_id = $1".into(),
            "m.org_id = $2".into(),
            "m.user_id = $3".into(),
            "m.created_at <= $4".into(),
            "latest_event.winner_pid IS NULL".into(),
        ];
        let mut values: Vec<SeaOrmValue> = vec![
            SeaOrmValue::String(Some(scope.agent_id)),
            SeaOrmValue::String(Some(scope.org_id)),
            SeaOrmValue::String(Some(scope.user_id)),
            SeaOrmValue::ChronoDateTimeWithTimeZone(Some(params.as_of)),
        ];

        if !params.kinds.includes_all() {
            let placeholders: Vec<String> = included
                .iter()
                .map(|kind| {
                    values.push(SeaOrmValue::String(Some(kind.to_string())));
                    format!("${}", values.len())
                })
                .collect();
            where_clauses.push(format!("m.kind IN ({})", placeholders.join(", ")));
        }

        values.push(SeaOrmValue::BigInt(Some(params.limit as i64)));
        let limit_placeholder = values.len();

        let sql = format!(
            "SELECT {MEMORY_SELECT_COLUMNS} \
             FROM memories m \
             LEFT JOIN LATERAL ( \
                 SELECT loser_pid, winner_pid, decided_at \
                 FROM supersession_events \
                 WHERE loser_pid = m.pid AND decided_at <= $4 \
                 ORDER BY decided_at DESC \
                 LIMIT 1 \
             ) AS latest_event ON TRUE \
             WHERE {where_sql} \
             ORDER BY m.created_at DESC \
             LIMIT ${limit_placeholder}",
            where_sql = where_clauses.join(" AND "),
        );
        let stmt = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

        let rows = self.db.query_all_raw(stmt).await?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(Memory::try_from(row)?);
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

        let result = self.db.execute_raw(stmt).await?;

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

        let rows = self.db.query_all_raw(stmt).await?;
        let mut memories = Vec::with_capacity(rows.len());
        for row in &rows {
            memories.push(Memory::try_from(row)?);
        }
        Ok(memories)
    }

    async fn list_scopes(&self) -> Result<Vec<Scope>, StoreError> {
        let stmt = Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT DISTINCT agent_id, org_id, user_id FROM memories".to_string(),
        );
        let rows = self.db.query_all_raw(stmt).await?;

        let mut scopes = Vec::with_capacity(rows.len());
        for row in &rows {
            scopes.push(Scope {
                agent_id: row.try_get::<String>("", "agent_id")?,
                org_id: row.try_get::<String>("", "org_id")?,
                user_id: row.try_get::<String>("", "user_id")?,
            });
        }
        Ok(scopes)
    }

    async fn list_agent_ids(&self, org_id: &str, user_id: &str) -> Result<Vec<String>, StoreError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT DISTINCT agent_id FROM memories
            WHERE org_id = $1 AND user_id = $2
            ORDER BY agent_id ASC
            "#,
            [
                SeaOrmValue::String(Some(org_id.to_owned())),
                SeaOrmValue::String(Some(user_id.to_owned())),
            ],
        );

        let rows = self.db.query_all_raw(stmt).await?;
        let mut agent_ids = Vec::with_capacity(rows.len());
        for row in &rows {
            agent_ids.push(row.try_get::<String>("", "agent_id")?);
        }
        Ok(agent_ids)
    }

    async fn indexed_pids_in_scope(&self, scope: &Scope) -> Result<Vec<String>, StoreError> {
        scope.validate()?;

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

        let rows = self.db.query_all_raw(stmt).await?;
        let mut pids = Vec::with_capacity(rows.len());
        for row in &rows {
            pids.push(row.try_get::<String>("", "pid")?);
        }
        Ok(pids)
    }

    async fn edit(&self, pid: &str, patch: EditPatch) -> Result<Memory, StoreError> {
        if patch.is_empty() {
            return self.recall(pid).await;
        }

        let current = self.recall(pid).await?;
        if current.kind != MemoryKind::Episodic {
            return Err(StoreError::UnsupportedEdit {
                pid: pid.to_string(),
                kind: current.kind,
            });
        }

        let mut set_fragments: Vec<String> = Vec::with_capacity(3);
        let mut values: Vec<SeaOrmValue> = Vec::with_capacity(4);

        if let Some(content) = patch.content {
            set_fragments.push(format!("content = ${}", values.len() + 1));
            values.push(SeaOrmValue::String(Some(content)));
        }
        if let Some(metadata) = patch.metadata {
            set_fragments.push(format!("metadata = ${}", values.len() + 1));
            values.push(SeaOrmValue::Json(Some(Box::new(metadata))));
        }
        if let Some(event_at) = patch.event_at {
            set_fragments.push(format!("event_at = ${}", values.len() + 1));
            values.push(SeaOrmValue::ChronoDateTimeWithTimeZone(event_at));
        }

        let pid_placeholder = values.len() + 1;
        values.push(SeaOrmValue::String(Some(pid.to_string())));

        let sql = format!(
            "UPDATE memories SET {set} WHERE pid = ${pid_placeholder}",
            set = set_fragments.join(", "),
        );
        let stmt = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, sql, values);

        let result = self.db.execute_raw(stmt).await?;
        if result.rows_affected() == 0 {
            return Err(StoreError::NotFound(pid.to_string()));
        }

        self.recall(pid).await
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

        let result = self.db.execute_raw(stmt).await?;

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

        let result = self.db.execute_raw(stmt).await?;

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

        let row = self.db.query_one_raw(stmt).await?;
        match row {
            None => Ok(None),
            Some(row) => row.try_get("", "winner_pid").map_err(StoreError::from),
        }
    }

    async fn supersession_history(&self, pid: &str) -> Result<Vec<SupersessionEvent>, StoreError> {
        // The compound index `supersession_events_loser_decided_idx` makes
        // this an indexed forward scan. Per-pid trails are tiny (a handful
        // of events) so no LIMIT.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT winner_pid, decided_at
            FROM supersession_events
            WHERE loser_pid = $1
            ORDER BY decided_at ASC
            "#,
            [SeaOrmValue::String(Some(pid.to_string()))],
        );

        let rows = self.db.query_all_raw(stmt).await?;
        let mut trail = Vec::with_capacity(rows.len());
        for row in &rows {
            trail.push(SupersessionEvent {
                winner_pid: row.try_get("", "winner_pid")?,
                decided_at: row.try_get("", "decided_at")?,
            });
        }
        Ok(trail)
    }
}

impl PostgresStore {
    async fn forget_pid(&self, pid: &str) -> Result<Vec<String>, StoreError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memories WHERE pid = $1 RETURNING pid",
            [SeaOrmValue::String(Some(pid.to_string()))],
        );
        let rows = self.db.query_all_raw(stmt).await?;
        let mut deleted = Vec::with_capacity(rows.len());
        for row in &rows {
            deleted.push(row.try_get::<String>("", "pid")?);
        }
        Ok(deleted)
    }

    async fn forget_scope(&self, scope: Scope) -> Result<Vec<String>, StoreError> {
        scope.validate()?;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memories WHERE agent_id = $1 AND org_id = $2 AND user_id = $3 RETURNING pid",
            [
                SeaOrmValue::String(Some(scope.agent_id)),
                SeaOrmValue::String(Some(scope.org_id)),
                SeaOrmValue::String(Some(scope.user_id)),
            ],
        );
        let rows = self.db.query_all_raw(stmt).await?;
        let mut deleted = Vec::with_capacity(rows.len());
        for row in &rows {
            deleted.push(row.try_get::<String>("", "pid")?);
        }
        Ok(deleted)
    }
}

impl TryFrom<&sea_orm::QueryResult> for Memory {
    type Error = StoreError;

    fn try_from(row: &sea_orm::QueryResult) -> Result<Self, Self::Error> {
        let pid: String = row.try_get("", "pid")?;
        let agent_id: String = row.try_get("", "agent_id")?;
        let org_id: String = row.try_get("", "org_id")?;
        let user_id: String = row.try_get("", "user_id")?;
        let content: String = row.try_get("", "content")?;
        let metadata: serde_json::Value = row.try_get("", "metadata")?;
        let kind_str: String = row.try_get("", "kind")?;
        let status_str: String = row.try_get("", "qdrant_status")?;
        let source_pid: Option<String> = row.try_get("", "source_pid")?;
        let superseded_by: Option<String> = row.try_get("", "superseded_by")?;
        let created_at: DateTime<FixedOffset> = row.try_get("", "created_at")?;
        let updated_at: DateTime<FixedOffset> = row.try_get("", "updated_at")?;
        let event_at: Option<DateTime<FixedOffset>> = row.try_get("", "event_at")?;
        let confidence_raw: i16 = row.try_get("", "confidence")?;
        let category: Option<String> = row.try_get("", "category")?;
        let supersession_at: Option<DateTime<FixedOffset>> = row.try_get("", "supersession_at")?;

        let kind: MemoryKind = kind_str
            .parse()
            .map_err(|_| StoreError::CacheInvariant(format!("unknown memory kind: {kind_str}")))?;

        let status: IndexStatus = status_str
            .parse()
            .map_err(|_| StoreError::CacheInvariant(format!("unknown qdrant status: {status_str}")))?;

        // The `memories.confidence` CHECK constrains the column to 0-100, so an
        // `i16` from the DB always fits `i8`. `Confidence::new` clamps as
        // defense-in-depth against a corrupted row rather than erroring.
        let confidence = crate::memory::Confidence::new(confidence_raw.clamp(0, 100) as i8);

        let supersession = match (superseded_by, supersession_at) {
            (Some(winner_pid), Some(at)) => Some(crate::memory::SupersessionInfo { winner_pid, at }),
            (None, None) => None,
            (Some(winner_pid), None) => {
                return Err(StoreError::CacheInvariant(format!(
                    "row {pid}: superseded_by={winner_pid} but no supersession_events row found"
                )));
            }
            (None, Some(_)) => {
                return Err(StoreError::CacheInvariant(format!(
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
            status,
            confidence,
            category,
        })
    }
}
