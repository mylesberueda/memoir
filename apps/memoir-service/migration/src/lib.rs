//! Memoir-service's auth + tenant migrations.
//!
//! All migrations run inside a configurable Postgres schema (default
//! `memoir`), matching the discipline established by memoir-core. The
//! `bootstrap_and_migrate` helper validates the schema name, creates the
//! schema if absent, sets the connection's search_path, then applies all
//! migrations.

pub use sea_orm_migration::prelude::*;

use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm_migration::sea_orm::DatabaseConnection;

mod m20000000_000001_nanoid;
pub mod m20000000_000002_add_updated_at_trigger;
mod m20000000_000003_create_users;
mod m20000000_000004_create_api_keys;
mod m20000000_000005_create_bootstrap_tokens;

/// Default Postgres schema for memoir-service's auth + tenant tables.
///
/// Matches memoir-core's [`memoir_core_migration::DEFAULT_SCHEMA`] so that
/// the entire memoir installation (auth + memory) lives under a single
/// operator-visible namespace.
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

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // Chronological order. Later migrations may depend on earlier ones
        // (e.g., 000003+ depend on the nanoid() function from 000001 and on
        // the trigger helper from 000002).
        vec![
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20000000_000003_create_users::Migration),
            Box::new(m20000000_000004_create_api_keys::Migration),
            Box::new(m20000000_000005_create_bootstrap_tokens::Migration),
        ]
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
        assert!(matches!(
            validate_schema_name("memoir; DROP TABLE users; --"),
            Err(MigrationError::InvalidSchema(_))
        ));
    }

    #[test]
    fn should_reject_schema_starting_with_digit() {
        assert!(matches!(validate_schema_name("1memoir"), Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_empty_schema() {
        assert!(matches!(validate_schema_name(""), Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_uppercase_schema() {
        assert!(matches!(validate_schema_name("Memoir"), Err(MigrationError::InvalidSchema(_))));
    }

    #[test]
    fn should_reject_schema_with_special_chars() {
        assert!(matches!(
            validate_schema_name("memoir-project"),
            Err(MigrationError::InvalidSchema(_))
        ));
    }
}
