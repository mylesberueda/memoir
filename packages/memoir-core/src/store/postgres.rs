//! [`MemoryStore`] implementation backed by Postgres.

use chrono::{DateTime, FixedOffset};
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaOrmValue};

use super::{IndexStatus, MemoryStore, StoreError};
use crate::memory::{ForgetTarget, Memory, MemoryKind, Scope};

const PID_LENGTH: usize = 21;

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

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO memories (pid, agent_id, org_id, user_id, content, metadata, kind, source_pid)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING pid, agent_id, org_id, user_id, content, metadata, kind, source_pid, created_at
            "#,
            [
                SeaOrmValue::String(Some(pid)),
                SeaOrmValue::String(Some(scope.agent_id.clone())),
                SeaOrmValue::String(Some(scope.org_id.clone())),
                SeaOrmValue::String(Some(scope.user_id.clone())),
                SeaOrmValue::String(Some(content)),
                SeaOrmValue::Json(Some(Box::new(metadata))),
                SeaOrmValue::String(Some(kind.as_str().to_string())),
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

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT pid, agent_id, org_id, user_id, content, metadata, kind, source_pid, created_at
            FROM memories
            WHERE pid = $1
            "#,
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
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT pid, agent_id, org_id, user_id, content, metadata, kind, source_pid, created_at
            FROM memories
            WHERE pid = ANY($1) AND qdrant_status = 'indexed'
            "#,
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
                SeaOrmValue::String(Some(status.as_str().to_string())),
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
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT pid, agent_id, org_id, user_id, content, metadata, kind, source_pid, created_at
            FROM memories
            WHERE qdrant_status = 'failed'
            LIMIT $1
            "#,
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
    let created_at: DateTime<FixedOffset> = row.try_get("", "created_at").map_err(database)?;

    let kind = match kind_str.as_str() {
        "episodic" => MemoryKind::Episodic,
        "semantic" => MemoryKind::Semantic,
        other => return Err(StoreError::Database(format!("unknown memory kind: {other}"))),
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
        created_at,
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
