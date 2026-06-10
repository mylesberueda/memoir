//! Holds relational triples between extraction and the synthesis fan-in.
//!
//! `relational_extract` produces triples but does not commit them; the
//! `synthesize` job (which reconciles them against semantic facts before commit)
//! may run after either sibling finishes. [`TripleStaging`] is the order-
//! independent handoff: relational stages a source's triples here, synthesis
//! reads and then clears them. Worker-internal persistence, not consumer API.
//!
//! Only compiled with the `knowledge-graph` feature.

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement, Value as SeaOrmValue};

use super::{GraphError, TripleSet};

/// Stages and retrieves a source's relational triples for synthesis.
///
/// Backed by the `graph_triple_staging` table (one row per `source_pid`).
/// Construct from the same Postgres connection the rest of memoir-core uses.
#[derive(Debug, Clone)]
pub struct TripleStaging {
    db: DatabaseConnection,
}

impl TripleStaging {
    /// Builds a staging store from an existing Postgres connection.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Stages `triples` for `source_pid`, replacing any prior staged set.
    ///
    /// Idempotent per source: re-running relational extraction overwrites the
    /// staged triples rather than accumulating them.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Query`] on database failure, or
    /// [`GraphError::BadRequest`] if the triples cannot be serialized.
    pub async fn stage(&self, source_pid: &str, triples: &TripleSet) -> Result<(), GraphError> {
        let json = serde_json::to_value(triples)
            .map_err(|err| GraphError::BadRequest(format!("serialize triples: {err}")))?;
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            INSERT INTO graph_triple_staging (source_pid, triples)
            VALUES ($1, $2)
            ON CONFLICT (source_pid) DO UPDATE SET triples = EXCLUDED.triples
            "#,
            [SeaOrmValue::String(Some(source_pid.to_owned())), SeaOrmValue::Json(Some(Box::new(json)))],
        );
        self.db.execute_raw(stmt).await.map_err(|err| GraphError::Query(err.to_string()))?;
        Ok(())
    }

    /// Returns the triples staged for `source_pid`, or `None` if none staged.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Query`] on database failure, or
    /// [`GraphError::BadRequest`] if a staged row fails to deserialize.
    pub async fn take_pending(&self, source_pid: &str) -> Result<Option<TripleSet>, GraphError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT triples FROM graph_triple_staging WHERE source_pid = $1",
            [SeaOrmValue::String(Some(source_pid.to_owned()))],
        );
        let Some(row) = self
            .db
            .query_one_raw(stmt)
            .await
            .map_err(|err| GraphError::Query(err.to_string()))?
        else {
            return Ok(None);
        };
        let json: serde_json::Value = row
            .try_get("", "triples")
            .map_err(|err| GraphError::Query(err.to_string()))?;
        let triples =
            serde_json::from_value(json).map_err(|err| GraphError::BadRequest(format!("deserialize triples: {err}")))?;
        Ok(Some(triples))
    }

    /// Deletes the staged triples for `source_pid`.
    ///
    /// Called by the synthesize handler after it commits the reconciled graph.
    /// Deleting an absent row is not an error (idempotent cleanup).
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::Query`] on database failure.
    pub async fn clear(&self, source_pid: &str) -> Result<(), GraphError> {
        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "DELETE FROM graph_triple_staging WHERE source_pid = $1",
            [SeaOrmValue::String(Some(source_pid.to_owned()))],
        );
        self.db.execute_raw(stmt).await.map_err(|err| GraphError::Query(err.to_string()))?;
        Ok(())
    }
}
