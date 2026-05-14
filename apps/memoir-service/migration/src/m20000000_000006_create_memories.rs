use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // DECISION: scope tuple (agent_id, org_id, user_id) lives as three TEXT
        // columns rather than a composite type. Matches the proto-side `Scope`
        // shape (one field per identifier) and avoids Postgres composite-type
        // ergonomics overhead. Composite index defined below makes scope-bound
        // queries cheap.
        //
        // DECISION: content is TEXT, unbounded at the DB layer. Per-request
        // size limits are enforced by the Remember handler (ticket 0006),
        // not the schema. Operators wanting hard caps configure them there.
        //
        // DECISION: metadata defaults to '{}'::jsonb rather than NULL. Every
        // memory has a metadata object; absence of caller-supplied metadata
        // means "empty object," not "unknown." Handlers read this field on
        // every row, so the non-null default avoids per-row Option handling.
        //
        // DECISION: qdrant_status is TEXT + CHECK rather than a native enum.
        // Matches the convention established by m20000000_000004 for
        // api_keys.role and api_keys.status. Postgres enum migrations
        // (ALTER TYPE) are operationally painful; the CHECK constraint
        // provides equivalent safety with simpler evolution.
        db.execute_unprepared(
            r#"
            CREATE TABLE memories (
                id              BIGSERIAL PRIMARY KEY,
                pid             TEXT NOT NULL UNIQUE DEFAULT nanoid(),
                agent_id        TEXT NOT NULL,
                org_id          TEXT NOT NULL,
                user_id         TEXT NOT NULL,
                content         TEXT NOT NULL,
                metadata        JSONB NOT NULL DEFAULT '{}'::jsonb,
                qdrant_status   TEXT NOT NULL DEFAULT 'pending'
                                CHECK (qdrant_status IN ('pending', 'indexed', 'failed')),
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .await?;

        // Composite scope index supports scope-bound list queries
        // (e.g. Forget by scope in ticket 0009, recall-by-scope patterns).
        // Order matches the tuple's lexicographic priority: agent first
        // because a single agent typically spans the smallest cardinality.
        db.execute_unprepared(
            "CREATE INDEX memories_scope_idx ON memories (agent_id, org_id, user_id)",
        )
        .await?;

        // Partial index on the failed/pending lifecycle states for the
        // reconciliation sweep in ticket 0010. Indexed rows are the common
        // case and don't need this index; excluding them keeps the index
        // tiny in steady state.
        db.execute_unprepared(
            "CREATE INDEX memories_qdrant_status_idx
             ON memories (qdrant_status)
             WHERE qdrant_status != 'indexed'",
        )
        .await?;

        set_update_on_update(&db, Memories::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE memories").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Memories {
    #[iden = "memories"]
    Table,
}
