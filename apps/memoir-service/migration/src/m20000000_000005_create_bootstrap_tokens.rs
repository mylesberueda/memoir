use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // DECISION: token TTL is 24 hours. Long enough that first-run setups
        // don't fail under operational friction; short enough to limit the
        // window in which a leaked log entry could be replayed. The server
        // checks expires_at > now() when consuming.
        //
        // DECISION: only one pending token may exist at a time, enforced via
        // a partial UNIQUE index. This serializes the race between two
        // memoir-server processes starting concurrently against a fresh DB:
        // the second process's INSERT fails with a unique constraint violation,
        // and the bootstrap path treats that as "another process already
        // initialized; skip" rather than an error.
        //
        // DECISION: token_hash stores an Argon2 PHC string of the plaintext
        // token. The plaintext is logged to stdout exactly once on first start
        // and never persisted.
        db.execute_unprepared(
            r#"
            CREATE TABLE bootstrap_tokens (
                id              BIGSERIAL PRIMARY KEY,
                token_hash      TEXT NOT NULL,
                status          TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'consumed', 'expired')),
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                expires_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP + INTERVAL '24 hours',
                consumed_at     TIMESTAMPTZ
            );
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE UNIQUE INDEX bootstrap_tokens_one_pending_idx
             ON bootstrap_tokens (status)
             WHERE status = 'pending'",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE bootstrap_tokens").await?;
        Ok(())
    }
}
