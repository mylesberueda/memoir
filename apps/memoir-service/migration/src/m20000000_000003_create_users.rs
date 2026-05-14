use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            CREATE TABLE users (
                id              BIGSERIAL PRIMARY KEY,
                pid             TEXT NOT NULL UNIQUE DEFAULT nanoid(),
                username        TEXT NOT NULL UNIQUE,
                password_hash   TEXT NOT NULL,
                is_admin        BOOLEAN NOT NULL DEFAULT FALSE,
                created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .await?;

        set_update_on_update(&db, Users::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE users").await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Users {
    #[iden = "users"]
    Table,
}
