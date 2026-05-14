use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // DECISION: role is TEXT + CHECK constraint rather than a Postgres enum.
        // Native enums require ALTER TYPE for value changes; with two values
        // this is rarely worth the operational cost. Reconsider if the role
        // set grows large or is queried with type-narrowing semantics.
        //
        // DECISION: status is also TEXT + CHECK rather than enum, same rationale.
        //
        // DECISION: key_id is the lookup half of `mk.<key_id>.<secret>`. The
        // interceptor parses the bearer token, finds the row by key_id, then
        // Argon2-verifies the secret half against key_hash. key_id is non-secret;
        // key_hash is Argon2 PHC. The `.` separator (instead of `_`) avoids
        // ambiguity with the base64url alphabet which includes `_`.
        db.execute_unprepared(
            r#"
            CREATE TABLE api_keys (
                id              BIGSERIAL PRIMARY KEY,
                pid             TEXT NOT NULL UNIQUE DEFAULT nanoid(),
                key_id          TEXT NOT NULL UNIQUE,
                key_hash        TEXT NOT NULL,
                name            TEXT NOT NULL,
                role            TEXT NOT NULL CHECK (role IN ('admin', 'integration')),
                org_id          TEXT,
                status          TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'revoked')),
                created_by      BIGINT REFERENCES users(id) ON DELETE SET NULL,
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                last_used_at    TIMESTAMPTZ
            );
            "#,
        )
        .await?;

        // Index supports filtering by status (List active keys is the common case)
        // and by scope binding.
        db.execute_unprepared(
            "CREATE INDEX api_keys_status_idx ON api_keys (status) WHERE status = 'active'",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX api_keys_org_id_idx ON api_keys (org_id) WHERE org_id IS NOT NULL",
        )
        .await?;

        set_update_on_update(&db, ApiKeys::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE api_keys").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum ApiKeys {
    #[iden = "api_keys"]
    Table,
}
