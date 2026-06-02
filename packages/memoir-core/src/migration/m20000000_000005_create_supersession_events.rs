use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Append-only audit log of supersession decisions. Source of truth
        // for "when was this memory superseded by what." The denormalized
        // `memories.superseded_by` column added in migration 0003 becomes a
        // cache of the latest event for each loser pid, maintained by the
        // trigger created below.
        //
        // - `loser_pid` ON DELETE CASCADE — forgetting a memory drops its
        //   supersession history with it. Mirrors `memory_jobs.source_pid`
        //   at `m20000000_000002_create_memory_jobs.rs:43`.
        // - `winner_pid` ON DELETE SET NULL — forgetting the winner becomes
        //   an implicit unsupersede. Mirrors the existing
        //   `memories.superseded_by` behavior at
        //   `m20000000_000003_add_superseded_by.rs:27`.
        // - `winner_pid IS NULL` is also the explicit unsupersede event:
        //   inserting one signals the loser is active again.
        // - The CHECK constraint prevents a row from superseding itself;
        //   defense in depth above any application-layer guard.
        db.execute_unprepared(
            r#"
            CREATE TABLE supersession_events (
                id              BIGSERIAL PRIMARY KEY,
                loser_pid       TEXT NOT NULL REFERENCES memories(pid) ON DELETE CASCADE,
                winner_pid      TEXT NULL REFERENCES memories(pid) ON DELETE SET NULL,
                decided_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                CHECK (loser_pid != winner_pid OR winner_pid IS NULL)
            )
            "#,
        )
        .await?;

        // Compound index supports the trigger's "latest event for this
        // loser_pid" lookup and the point-in-time read path
        // (`Client::recall_as_of` in ticket 0009). DESC ordering on
        // `decided_at` makes the "latest" lookup an index seek.
        db.execute_unprepared(
            "CREATE INDEX supersession_events_loser_decided_idx
             ON supersession_events (loser_pid, decided_at DESC)",
        )
        .await?;

        // Standalone index for admin time-ordered queries
        // ("show all supersession events in time order"). Pays storage
        // cost upfront to avoid a follow-up migration when admin tooling
        // grows.
        db.execute_unprepared(
            "CREATE INDEX supersession_events_decided_at_idx
             ON supersession_events (decided_at DESC)",
        )
        .await?;

        // Trigger function maintains the `memories.superseded_by` cache
        // from the supersession_events history. On every INSERT, looks up
        // the latest event for the inserted loser_pid and writes its
        // `winner_pid` to the corresponding `memories` row.
        //
        // The query looks up by index rather than using `NEW.winner_pid`
        // directly so the trigger stays correct even if rows were ever
        // inserted out of `decided_at` order (which they shouldn't be in
        // practice, but the cost is just one index seek).
        //
        // No UPDATE trigger: supersession_events is conceptually
        // append-only. If rows ever need to be edited, the cache must be
        // rebuilt manually.
        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION sync_superseded_by_cache()
            RETURNS TRIGGER AS $$
            BEGIN
              UPDATE memories
                 SET superseded_by = (
                   SELECT winner_pid
                     FROM supersession_events
                    WHERE loser_pid = NEW.loser_pid
                 ORDER BY decided_at DESC
                    LIMIT 1
                 )
               WHERE pid = NEW.loser_pid;
              RETURN NEW;
            END;
            $$ LANGUAGE plpgsql
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER sync_superseded_by_cache
            AFTER INSERT ON supersession_events
            FOR EACH ROW
            EXECUTE FUNCTION sync_superseded_by_cache()
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // DROP TABLE cascades to its indexes; explicit DROP INDEX statements
        // would be redundant. The trigger is dropped automatically with the
        // table. The function lives in the schema separately, so it gets an
        // explicit DROP — pattern matches `m20000000_000001:78` which drops
        // its own function in `down`.
        db.execute_unprepared("DROP TABLE supersession_events").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS sync_superseded_by_cache").await?;
        Ok(())
    }
}

// Rust guideline compliant 2026-02-21
