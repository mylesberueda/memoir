use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Widen the `memory_jobs.kind` CHECK to admit the `relational_extract`
        // job (epic 0012 — graph derivation). Builds on `m20000000_000008`'s
        // set ('embed', 'extract', 'categorize', 'reprocess'). Postgres has no
        // ALTER ... MODIFY CHECK, so drop and recreate.
        db.execute_unprepared("ALTER TABLE memory_jobs DROP CONSTRAINT memory_jobs_kind_check")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE memory_jobs
                ADD CONSTRAINT memory_jobs_kind_check
                CHECK (kind IN ('embed', 'extract', 'categorize', 'reprocess', 'relational_extract'))",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE memory_jobs DROP CONSTRAINT memory_jobs_kind_check")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE memory_jobs
                ADD CONSTRAINT memory_jobs_kind_check
                CHECK (kind IN ('embed', 'extract', 'categorize', 'reprocess'))",
        )
        .await?;
        Ok(())
    }
}
