use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Holds a source's relational triples between extraction
        // (`relational_extract`) and the synthesis fan-in (`synthesize`), so
        // synthesis can read them regardless of which sibling finished last
        // (epic 0012 ticket 0008). One row per source; the synthesize handler
        // deletes the row once it commits the reconciled graph. ON DELETE
        // CASCADE so forgetting the episodic source clears any pending staging.
        db.execute_unprepared(
            r#"
            CREATE TABLE graph_triple_staging (
                source_pid  TEXT PRIMARY KEY REFERENCES memories(pid) ON DELETE CASCADE,
                triples     JSONB NOT NULL DEFAULT '[]'::jsonb,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE graph_triple_staging").await?;
        Ok(())
    }
}
