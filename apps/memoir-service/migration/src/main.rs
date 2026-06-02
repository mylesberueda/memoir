//! CLI entrypoint for memoir-service's migrator.
//!
//! Invoked via `sea-orm-cli` (typically `pnpm nx run memoir-service:migrate:up`).
//! Reads `DATABASE_URL` from environment and writes memoir-service's auth +
//! tenant tables into the configured schema (default `memoir_service`). The
//! schema name comes from the `SERVICE_SCHEMA` env var, defaulting to
//! [`migration::DEFAULT_SCHEMA`]. The schema is created if absent before
//! delegating to sea-orm's CLI machinery, mirroring the runtime path at
//! [`migration::bootstrap_and_migrate`].

use sea_orm_migration::sea_orm::{ConnectionTrait, Database};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let schema =
        std::env::var("SERVICE_SCHEMA").unwrap_or_else(|_| migration::DEFAULT_SCHEMA.to_owned());

    bootstrap_schema(&schema).await;

    // Forward SERVICE_SCHEMA into the DATABASE_SCHEMA env var that
    // sea-orm-cli reads to set the search_path on its connection. Operators
    // configure exactly one variable.
    if std::env::var_os("DATABASE_SCHEMA").is_none() {
        // SAFETY: set_var runs synchronously before any task is spawned;
        // sea-orm-cli reads the env synchronously inside `run_cli`.
        unsafe { std::env::set_var("DATABASE_SCHEMA", &schema) };
    }

    sea_orm_migration::cli::run_cli(migration::Migrator).await;
}

async fn bootstrap_schema(schema: &str) {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set for memoir-service migrations"));

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
