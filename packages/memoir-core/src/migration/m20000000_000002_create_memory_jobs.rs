use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Extend `memories` with the back-reference to the source episodic
        // memory. Episodic rows keep `source_pid = NULL`; semantic rows
        // (epic 0006 ticket 0006) carry the originating pid. ON DELETE
        // CASCADE so that forgetting an episodic memory automatically clears
        // its derived semantic memories.
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN source_pid TEXT NULL
                REFERENCES memories(pid) ON DELETE CASCADE
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX memories_source_pid_idx
             ON memories (source_pid)
             WHERE source_pid IS NOT NULL",
        )
        .await?;

        // Persistent write-behind queue. Polled by the in-library worker
        // (ticket 0003) via `SELECT FOR UPDATE SKIP LOCKED`. Survives process
        // crashes by construction.
        //
        // The CHECK constraint values for `kind` and `state` are the conservative
        // starting set; ticket 0002 may extend them via a follow-up migration as
        // new job kinds or states are needed.
        db.execute_unprepared(
            r#"
            CREATE TABLE memory_jobs (
                id              BIGSERIAL PRIMARY KEY,
                source_pid      TEXT NOT NULL REFERENCES memories(pid) ON DELETE CASCADE,
                kind            TEXT NOT NULL
                                CHECK (kind IN ('embed', 'extract')),
                state           TEXT NOT NULL DEFAULT 'pending'
                                CHECK (state IN ('pending', 'claimed', 'failed')),
                payload         JSONB NOT NULL DEFAULT '{}'::jsonb,
                attempts        INTEGER NOT NULL DEFAULT 0,
                failure_reason  TEXT NULL,
                claimed_at      TIMESTAMPTZ NULL,
                claimed_by      TEXT NULL,
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .await?;

        // Partial index supports the queue-claim query
        // (`WHERE state = 'pending' ORDER BY created_at`). Pattern mirrors
        // the `memories_qdrant_status_idx` partial index in the prior
        // migration.
        db.execute_unprepared(
            "CREATE INDEX memory_jobs_pending_idx
             ON memory_jobs (created_at)
             WHERE state = 'pending'",
        )
        .await?;

        // Supports the lease-recovery sweep
        // (`WHERE state = 'claimed' AND claimed_at < <cutoff>`).
        db.execute_unprepared(
            "CREATE INDEX memory_jobs_claimed_idx
             ON memory_jobs (claimed_at)
             WHERE state = 'claimed'",
        )
        .await?;

        // Reuses the trigger function created by the prior migration.
        db.execute_unprepared(
            r#"
            CREATE TRIGGER set_updated_at
            BEFORE UPDATE ON memory_jobs
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE memory_jobs").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN source_pid")
            .await?;
        Ok(())
    }
}
