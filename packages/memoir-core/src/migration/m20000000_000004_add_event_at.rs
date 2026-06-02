use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Event-time for the thing the memory records, distinct from the
        // wall-clock `created_at` of when memoir was told. Nullable: many
        // memories ("I love coffee") have no meaningful event-time, while
        // others ("the deployment happened Friday") do. Populated either by
        // the consumer via the remember builder's `.event_at(...)` (ticket
        // 0005) or by LLM-extraction (ticket 0011).
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN event_at TIMESTAMPTZ NULL
            "#,
        )
        .await?;

        // Partial index — most rows will likely have NULL event_at, so a
        // full index would waste storage. Pattern mirrors the partial
        // indexes at `m20000000_000001_create_memories.rs:50-55`
        // (`memories_qdrant_status_idx`) and `m20000000_000003_add_superseded_by.rs:35-39`
        // (`memories_superseded_by_idx`).
        db.execute_unprepared(
            "CREATE INDEX memories_event_at_idx
             ON memories (event_at)
             WHERE event_at IS NOT NULL",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS memories_event_at_idx").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN event_at").await?;
        Ok(())
    }
}

// Rust guideline compliant 2026-02-21
