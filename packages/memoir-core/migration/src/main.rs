//! CLI entrypoint for memoir-core's migrator.
//!
//! Invoked via `sea-orm-cli` (typically `pnpm nx run memoir-core:migrate:up`).
//! Reads `DATABASE_URL` from environment and writes memoir-core's tables into
//! the configured schema (default `memoir`). The schema name comes from the
//! `MEMOIR_SCHEMA` env var, defaulting to
//! [`memoir_core_migration::DEFAULT_SCHEMA`]. The schema is created if absent
//! before delegating to sea-orm's CLI machinery, mirroring the runtime path
//! at [`memoir_core_migration::bootstrap_and_migrate`].

use sea_orm_migration::sea_orm::{ConnectionTrait, Database};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let schema =
        std::env::var("MEMOIR_SCHEMA").unwrap_or_else(|_| memoir_core_migration::DEFAULT_SCHEMA.to_owned());

    // Bootstrap the schema before sea-orm-cli takes over. sea-orm-cli sets
    // the search_path on its own connection per `--database-schema` (or
    // `DATABASE_SCHEMA` env), but it does NOT create the schema first; if
    // the schema is absent every CREATE TABLE silently lands in `public`.
    bootstrap_schema(&schema).await;

    // sea-orm-cli reads `--database-schema` or the `DATABASE_SCHEMA` env var
    // for the search_path. Forward MEMOIR_SCHEMA into it so the operator
    // only configures one env var.
    if std::env::var_os("DATABASE_SCHEMA").is_none() {
        // SAFETY: setting env in a single-threaded prelude (before any other
        // task is spawned). sea-orm-cli reads this synchronously inside
        // `run_cli`.
        unsafe { std::env::set_var("DATABASE_SCHEMA", &schema) };
    }

    sea_orm_migration::cli::run_cli(memoir_core_migration::Migrator).await;
}

async fn bootstrap_schema(schema: &str) {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL must be set for memoir-core migrations");
    });

    let db = Database::connect(&url)
        .await
        .unwrap_or_else(|err| panic!("failed to connect for schema bootstrap: {err}"));

    db.execute_unprepared(&format!("CREATE SCHEMA IF NOT EXISTS {schema}"))
        .await
        .unwrap_or_else(|err| panic!("failed to create schema {schema}: {err}"));

    db.close()
        .await
        .unwrap_or_else(|err| panic!("failed to close bootstrap connection: {err}"));
}
