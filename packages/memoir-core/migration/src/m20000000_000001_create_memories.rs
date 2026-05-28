use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
              NEW.updated_at = CURRENT_TIMESTAMP;
              RETURN NEW;
            END;
            $$ LANGUAGE plpgsql
            "#,
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TABLE memories (
                id              BIGSERIAL PRIMARY KEY,
                pid             TEXT NOT NULL UNIQUE,
                agent_id        TEXT NOT NULL,
                org_id          TEXT NOT NULL,
                user_id         TEXT NOT NULL,
                content         TEXT NOT NULL,
                metadata        JSONB NOT NULL DEFAULT '{}'::jsonb,
                kind            TEXT NOT NULL DEFAULT 'episodic'
                                CHECK (kind IN ('episodic', 'semantic')),
                qdrant_status   TEXT NOT NULL DEFAULT 'pending'
                                CHECK (qdrant_status IN ('pending', 'indexed', 'failed')),
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX memories_scope_idx ON memories (agent_id, org_id, user_id)",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX memories_qdrant_status_idx
             ON memories (qdrant_status)
             WHERE qdrant_status != 'indexed'",
        )
        .await?;

        db.execute_unprepared(
            "CREATE INDEX memories_kind_scope_idx ON memories (kind, agent_id, org_id, user_id)",
        )
        .await?;

        db.execute_unprepared(
            r#"
            CREATE TRIGGER set_updated_at
            BEFORE UPDATE ON memories
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE memories").await?;
        db.execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column").await?;
        Ok(())
    }
}
