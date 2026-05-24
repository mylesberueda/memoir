use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Soft-deletion marker for the semantic-memory contradiction-detection
        // path (epic 0006 ticket 0009). When a future detection engine
        // (NLP-math first, LLM fallback) decides two semantic memories
        // contradict, it sets `superseded_by` on the loser to point at the
        // winner. Search and retrieval paths filter `WHERE superseded_by IS
        // NULL` so superseded rows are invisible to normal callers but
        // recoverable via admin.
        //
        // ON DELETE SET NULL: if the superseder itself is later forgotten,
        // the previously-superseded row becomes active again — the
        // supersession had a basis (the winner existed); take the basis
        // away and the soft-deletion no longer applies.
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN superseded_by TEXT NULL
                REFERENCES memories(pid) ON DELETE SET NULL
            "#,
        )
        .await?;

        // Partial index supports the admin-list path
        // (`SELECT ... WHERE superseded_by IS NOT NULL`). Most rows are
        // active so a partial index stays small.
        db.execute_unprepared(
            "CREATE INDEX memories_superseded_by_idx
             ON memories (superseded_by)
             WHERE superseded_by IS NOT NULL",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS memories_superseded_by_idx").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN superseded_by").await?;
        Ok(())
    }
}
