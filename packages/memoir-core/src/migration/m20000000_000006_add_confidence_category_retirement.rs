use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // First-class confidence (epic 0011). Previously the extraction LLM's
        // per-fact score lived only inside the `metadata` JSON blob; this
        // promotes it to a queryable, rankable column. SMALLINT holds the
        // 0-100 percentage (i8 on the Rust side). DEFAULT 100: episodic rows
        // ("the user said it") are certain by construction; the extract worker
        // (ticket 0006) overwrites this for semantic rows from the scaled LLM
        // score. The CHECK enforces the range at the DB, matching the
        // constrain-at-the-DB habit of the `kind`/`qdrant_status` CHECKs at
        // `m20000000_000001_create_memories.rs:34-37`.
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN confidence SMALLINT NOT NULL DEFAULT 100
                    CHECK (confidence BETWEEN 0 AND 100)
            "#,
        )
        .await?;

        // First-class category (epic 0011), populated asynchronously by the
        // categorize worker (ticket 0005). NULL until categorized — a row
        // without a category is unfiltered, not rejected. Left an open TEXT
        // with NO CHECK on purpose: the taxonomy (which category values are
        // valid) is decided in ticket 0005, and locking a CHECK here would
        // force a follow-up migration once that taxonomy lands.
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN category TEXT NULL
            "#,
        )
        .await?;

        // Retirement reason for the correction model (epic 0011 Track B).
        // Distinguishes WHY a derived memory was retired, because only one
        // reason is an extraction error and thus counts against the accuracy
        // metric (ticket 0013):
        //   - 'rejected' — the extraction was wrong; the user said so via
        //     feedback. THIS is the accuracy-metric numerator.
        //   - 'stale'    — the episodic source was edited/deleted, so derived
        //     semantics no longer reflect it. NOT a model error.
        // NULL = not retired by this mechanism.
        //
        // Deliberately does NOT include a 'superseded' value and does NOT
        // touch the supersession trigger (migration 0005). Supersession stays
        // expressed by its own `superseded_by` column, which is reversible
        // (an unsupersede event clears it). Folding 'superseded' into this
        // column would force the trigger to maintain it too — and a row that
        // is 'rejected' then later superseded would have its reason clobbered,
        // corrupting the accuracy metric. So "active" is the conjunction
        // `superseded_by IS NULL AND retirement_reason IS NULL`, and this
        // column carries only the two new reasons.
        db.execute_unprepared(
            r#"
            ALTER TABLE memories
                ADD COLUMN retirement_reason TEXT NULL
                    CHECK (retirement_reason IN ('rejected', 'stale'))
            "#,
        )
        .await?;

        // Partial index over retired rows. Most rows are active, so the index
        // stays small (mirrors `memories_superseded_by_idx` at
        // `m20000000_000003_add_superseded_by.rs:35-39`). Supports the
        // active-row read filter (0009) and the rejected-count metric (0013).
        db.execute_unprepared(
            "CREATE INDEX memories_retirement_reason_idx
             ON memories (retirement_reason)
             WHERE retirement_reason IS NOT NULL",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS memories_retirement_reason_idx").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN retirement_reason").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN category").await?;
        db.execute_unprepared("ALTER TABLE memories DROP COLUMN confidence").await?;
        Ok(())
    }
}

// Rust guideline compliant 2026-02-21
