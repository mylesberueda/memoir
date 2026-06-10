//! [`MemoryJobsStore`] implementation backed by Postgres.

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaOrmValue};

use super::{FailedJob, Job, JobKind, JobState, JobsError, MemoryJobsStore};

/// Default [`MemoryJobsStore`] backed by Postgres.
///
/// Constructed via [`Self::new`] from an existing
/// [`sea_orm::DatabaseConnection`]. The caller owns the connection's
/// lifecycle; this store does not pool or reconnect.
#[derive(Debug, Clone)]
pub struct PostgresJobsStore {
    db: DatabaseConnection,
}

impl PostgresJobsStore {
    /// Builds a jobs store from an existing Postgres connection.
    #[must_use]
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Returns the underlying Postgres connection.
    #[must_use]
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl MemoryJobsStore for PostgresJobsStore {
    async fn enqueue(
        &self,
        kind: JobKind,
        source_pid: String,
        payload: serde_json::Value,
    ) -> Result<i64, JobsError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO memory_jobs (source_pid, kind, payload)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            [
                SeaOrmValue::String(Some(source_pid)),
                SeaOrmValue::String(Some(kind.to_string())),
                SeaOrmValue::Json(Some(Box::new(payload))),
            ],
        );

        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(database)?
            .ok_or_else(|| JobsError::Database("insert returned no row".to_string()))?;

        row.try_get::<i64>("", "id").map_err(database)
    }

    async fn enqueue_synthesis_if_ready(&self, source_pid: &str, caller_job_id: i64) -> Result<bool, JobsError> {
        // Single atomic statement: insert the synthesize row only if no *other*
        // LLM-derived sibling (extract / relational_extract) still has a row for
        // this source, AND no synthesize row exists yet. `id <> $2` excludes the
        // calling sibling's own still-claimed row — the handler fires this before
        // the worker completes (deletes) it, so without the exclusion a job would
        // always see itself and synthesis could never fire. The `SELECT ... WHERE
        // NOT EXISTS` and the `INSERT` share one snapshot, so two concurrent
        // sibling completions cannot both insert. `RETURNING` lets us report
        // whether a row landed.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO memory_jobs (source_pid, kind, payload)
            SELECT $1, 'synthesize', '{}'::jsonb
            WHERE NOT EXISTS (
                SELECT 1 FROM memory_jobs
                WHERE source_pid = $1
                  AND id <> $2
                  AND kind IN ('extract', 'relational_extract', 'synthesize')
            )
            RETURNING id
            "#,
            [
                SeaOrmValue::String(Some(source_pid.to_owned())),
                SeaOrmValue::BigInt(Some(caller_job_id)),
            ],
        );

        let inserted = self.db.query_one_raw(stmt).await.map_err(database)?.is_some();
        Ok(inserted)
    }

    async fn claim(&self, claimed_by: Option<&str>) -> Result<Option<Job>, JobsError> {
        // Single-statement claim: UPDATE the oldest pending row, where the
        // inner SELECT uses FOR UPDATE SKIP LOCKED so concurrent workers
        // never collide.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            UPDATE memory_jobs
            SET state = 'claimed',
                claimed_at = CURRENT_TIMESTAMP,
                claimed_by = $1
            WHERE id = (
                SELECT id FROM memory_jobs
                WHERE state = 'pending'
                ORDER BY created_at
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, source_pid, kind, state, payload, attempts,
                      failure_reason, claimed_at, claimed_by,
                      created_at, updated_at
            "#,
            [SeaOrmValue::String(claimed_by.map(str::to_owned))],
        );

        match self.db.query_one_raw(stmt).await.map_err(database)? {
            Some(row) => Ok(Some(job_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn complete(&self, id: i64) -> Result<(), JobsError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memory_jobs WHERE id = $1 AND state = 'claimed'",
            [SeaOrmValue::BigInt(Some(id))],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;

        if result.rows_affected() == 0 {
            return Err(JobsError::NotFound(id.to_string()));
        }
        Ok(())
    }

    async fn fail(&self, id: i64, reason: String, max_attempts: i32) -> Result<(), JobsError> {
        // attempts + 1 is computed at SQL time; the new state depends on
        // whether the post-increment value reaches max_attempts.
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            UPDATE memory_jobs
            SET attempts = attempts + 1,
                failure_reason = $2,
                claimed_at = NULL,
                claimed_by = NULL,
                state = CASE
                    WHEN attempts + 1 >= $3 THEN 'failed'
                    ELSE 'pending'
                END
            WHERE id = $1 AND state = 'claimed'
            "#,
            [
                SeaOrmValue::BigInt(Some(id)),
                SeaOrmValue::String(Some(reason)),
                SeaOrmValue::Int(Some(max_attempts)),
            ],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;

        if result.rows_affected() == 0 {
            return Err(JobsError::NotFound(id.to_string()));
        }
        Ok(())
    }

    async fn reset_expired_leases(
        &self,
        lease: std::time::Duration,
    ) -> Result<u64, JobsError> {
        // Postgres `make_interval(secs => ...)` needs a numeric bind. `as_secs`
        // returns u64; Postgres accepts up to i64::MAX seconds in an interval.
        // Saturate on overflow rather than wrapping silently.
        let seconds = i64::try_from(lease.as_secs()).unwrap_or(i64::MAX);
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            UPDATE memory_jobs
            SET state = 'pending',
                claimed_at = NULL,
                claimed_by = NULL
            WHERE state = 'claimed'
              AND claimed_at < CURRENT_TIMESTAMP - make_interval(secs => $1::float)
            "#,
            [SeaOrmValue::BigInt(Some(seconds))],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;
        Ok(result.rows_affected())
    }

    async fn list_failed(&self, limit: usize) -> Result<Vec<FailedJob>, JobsError> {
        // Saturate the limit: usize is 64-bit on our targets but Postgres
        // takes i64, so an absurd usize value gets capped rather than wrapped.
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT id, source_pid, kind, attempts, failure_reason, updated_at
            FROM memory_jobs
            WHERE state = 'failed'
            ORDER BY updated_at DESC
            LIMIT $1
            "#,
            [SeaOrmValue::BigInt(Some(limit))],
        );

        let rows = self.db.query_all_raw(stmt).await.map_err(database)?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            out.push(failed_job_from_row(row)?);
        }
        Ok(out)
    }

    async fn retry_job(&self, id: i64) -> Result<(), JobsError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            UPDATE memory_jobs
            SET state = 'pending',
                attempts = 0,
                failure_reason = NULL,
                claimed_at = NULL,
                claimed_by = NULL
            WHERE id = $1 AND state = 'failed'
            "#,
            [SeaOrmValue::BigInt(Some(id))],
        );

        let result = self.db.execute_raw(stmt).await.map_err(database)?;
        if result.rows_affected() == 0 {
            return Err(JobsError::NotFound(id.to_string()));
        }
        Ok(())
    }

    async fn bulk_retry(
        &self,
        kind: Option<JobKind>,
        dry_run: bool,
    ) -> Result<u64, JobsError> {
        if dry_run {
            // Just count.
            let stmt = if let Some(k) = kind {
                Statement::from_sql_and_values(
                    sea_orm::DatabaseBackend::Postgres,
                    "SELECT COUNT(*)::BIGINT AS n FROM memory_jobs WHERE state = 'failed' AND kind = $1",
                    [SeaOrmValue::String(Some(k.to_string()))],
                )
            } else {
                Statement::from_string(
                    sea_orm::DatabaseBackend::Postgres,
                    "SELECT COUNT(*)::BIGINT AS n FROM memory_jobs WHERE state = 'failed'".to_string(),
                )
            };
            let row = self
                .db
                .query_one_raw(stmt)
                .await
                .map_err(database)?
                .ok_or_else(|| JobsError::Database("count returned no row".to_string()))?;
            let n: i64 = row.try_get("", "n").map_err(database)?;
            return Ok(u64::try_from(n).unwrap_or(0));
        }

        let stmt = if let Some(k) = kind {
            Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                UPDATE memory_jobs
                SET state = 'pending',
                    attempts = 0,
                    failure_reason = NULL,
                    claimed_at = NULL,
                    claimed_by = NULL
                WHERE state = 'failed' AND kind = $1
                "#,
                [SeaOrmValue::String(Some(k.to_string()))],
            )
        } else {
            Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                r#"
                UPDATE memory_jobs
                SET state = 'pending',
                    attempts = 0,
                    failure_reason = NULL,
                    claimed_at = NULL,
                    claimed_by = NULL
                WHERE state = 'failed'
                "#
                .to_string(),
            )
        };

        let result = self.db.execute_raw(stmt).await.map_err(database)?;
        Ok(result.rows_affected())
    }

    async fn delete_failed(&self, id: i64) -> Result<(), JobsError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM memory_jobs WHERE id = $1 AND state = 'failed'",
            [SeaOrmValue::BigInt(Some(id))],
        );
        let result = self.db.execute_raw(stmt).await.map_err(database)?;
        if result.rows_affected() == 0 {
            return Err(JobsError::NotFound(id.to_string()));
        }
        Ok(())
    }

    async fn pending_count(&self) -> Result<u64, JobsError> {
        let stmt = Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*)::BIGINT AS n FROM memory_jobs WHERE state = 'pending'".to_string(),
        );
        let row = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(database)?
            .ok_or_else(|| JobsError::Database("count returned no row".to_string()))?;
        let n: i64 = row.try_get("", "n").map_err(database)?;
        Ok(u64::try_from(n).unwrap_or(0))
    }
}

fn database<E: std::fmt::Display>(err: E) -> JobsError {
    JobsError::Database(err.to_string())
}

fn job_from_row(row: &sea_orm::QueryResult) -> Result<Job, JobsError> {
    let id: i64 = row.try_get("", "id").map_err(database)?;
    let source_pid: String = row.try_get("", "source_pid").map_err(database)?;
    let kind_str: String = row.try_get("", "kind").map_err(database)?;
    let state_str: String = row.try_get("", "state").map_err(database)?;
    let payload: serde_json::Value = row.try_get("", "payload").map_err(database)?;
    let attempts: i32 = row.try_get("", "attempts").map_err(database)?;
    let failure_reason: Option<String> = row.try_get("", "failure_reason").map_err(database)?;
    let claimed_at: Option<DateTime<FixedOffset>> = row.try_get("", "claimed_at").map_err(database)?;
    let claimed_by: Option<String> = row.try_get("", "claimed_by").map_err(database)?;
    let created_at: DateTime<FixedOffset> = row.try_get("", "created_at").map_err(database)?;
    let updated_at: DateTime<FixedOffset> = row.try_get("", "updated_at").map_err(database)?;

    let kind: JobKind = kind_str
        .parse()
        .map_err(|_| JobsError::Database(format!("unknown job kind: {kind_str}")))?;

    let state: JobState = state_str
        .parse()
        .map_err(|_| JobsError::Database(format!("unknown job state: {state_str}")))?;

    Ok(Job {
        id,
        source_pid,
        kind,
        state,
        payload,
        attempts,
        failure_reason,
        claimed_at,
        claimed_by,
        created_at,
        updated_at,
    })
}

fn failed_job_from_row(row: &sea_orm::QueryResult) -> Result<FailedJob, JobsError> {
    let id: i64 = row.try_get("", "id").map_err(database)?;
    let source_pid: String = row.try_get("", "source_pid").map_err(database)?;
    let kind_str: String = row.try_get("", "kind").map_err(database)?;
    let attempts: i32 = row.try_get("", "attempts").map_err(database)?;
    let failure_reason: Option<String> = row.try_get("", "failure_reason").map_err(database)?;
    let updated_at: DateTime<FixedOffset> = row.try_get("", "updated_at").map_err(database)?;

    let kind: JobKind = kind_str
        .parse()
        .map_err(|_| JobsError::Database(format!("unknown job kind: {kind_str}")))?;

    Ok(FailedJob {
        id,
        source_pid,
        kind,
        attempts,
        failure_reason,
        updated_at,
    })
}
