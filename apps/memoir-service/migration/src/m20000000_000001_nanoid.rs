use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const NANOID_PG_INSTALL: &str = include_str!("./m20000000_000001_nanoid.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(NANOID_PG_INSTALL).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared("DROP FUNCTION IF EXISTS nanoid(int, text, float)").await?;

        db.execute_unprepared("DROP FUNCTION IF EXISTS nanoid_optimized(int, text, int, int)").await?;

        Ok(())
    }
}
