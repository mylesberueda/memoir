//! Memoir core's database migrations.
//!
//! Migrations run in a configurable Postgres schema (default `memoir`),
//! isolating memoir-core's tables from the caller's `public` schema.
//! Call [`bootstrap_and_migrate`] from `memoir_core::Client::migrate`.

use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, DatabaseConnection};

mod m20000000_000001_create_memories;

/// Default Postgres schema for memoir-core's tables.
pub const DEFAULT_SCHEMA: &str = "memoir";

static SCHEMA_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z_][a-z0-9_]*$").unwrap());

/// Failure modes for [`bootstrap_and_migrate`].
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("invalid schema name '{0}': must match [a-z_][a-z0-9_]*")]
    InvalidSchema(String),

    #[error("database error: {0}")]
    Database(#[from] DbErr),
}

/// SeaORM migrator for memoir-core's schema.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20000000_000001_create_memories::Migration)]
    }
}

/// Creates the configured schema if absent, then applies all migrations.
///
/// The schema name is validated against `[a-z_][a-z0-9_]*` before any SQL
/// executes; this is the only safe path because the schema name is
/// interpolated into raw SQL (Postgres does not accept bound parameters for
/// identifiers).
///
/// # Errors
///
/// Returns [`MigrationError::InvalidSchema`] if `schema` fails validation,
/// [`MigrationError::Database`] for any Postgres failure during schema
/// creation, search-path configuration, or migration application.
pub async fn bootstrap_and_migrate(
    db: &DatabaseConnection,
    schema: &str,
) -> Result<(), MigrationError> {
    validate_schema_name(schema)?;

    db.execute_unprepared(&format!("CREATE SCHEMA IF NOT EXISTS {schema}"))
        .await?;
    db.execute_unprepared(&format!("SET search_path TO {schema}, public"))
        .await?;

    Migrator::up(db, None).await?;

    Ok(())
}

fn validate_schema_name(schema: &str) -> Result<(), MigrationError> {
    if SCHEMA_NAME_RE.is_match(schema) {
        Ok(())
    } else {
        Err(MigrationError::InvalidSchema(schema.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_accept_simple_lowercase_schema_name() {
        assert!(validate_schema_name("memoir").is_ok());
        assert!(validate_schema_name("project_a").is_ok());
        assert!(validate_schema_name("_underscore_start").is_ok());
        assert!(validate_schema_name("with123digits").is_ok());
    }

    #[test]
    fn should_reject_schema_with_sql_injection_attempt() {
        let result = validate_schema_name("memoir; DROP TABLE users; --");
        assert!(matches!(result, Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_schema_starting_with_digit() {
        let result = validate_schema_name("1memoir");
        assert!(matches!(result, Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_empty_schema() {
        let result = validate_schema_name("");
        assert!(matches!(result, Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_uppercase_schema() {
        let result = validate_schema_name("Memoir");
        assert!(matches!(result, Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_schema_with_special_chars() {
        assert!(matches!(
            validate_schema_name("memoir-project"),
            Err(MigrationError::InvalidSchema(_))
        ));
        assert!(matches!(
            validate_schema_name("memoir.project"),
            Err(MigrationError::InvalidSchema(_))
        ));
        assert!(matches!(
            validate_schema_name("memoir project"),
            Err(MigrationError::InvalidSchema(_))
        ));
    }
}
