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
            $$ LANGUAGE plpgsql;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(r#"DROP FUNCTION IF EXISTS update_updated_at_column"#)
            .await?;

        Ok(())
    }
}

#[allow(dead_code)]
pub async fn set_update_on_update<T: Iden>(db: &SchemaManagerConnection<'_>, table: T) -> Result<(), DbErr> {
    db.execute_unprepared(&format!(
        r#"
        CREATE TRIGGER set_updated_at
        BEFORE UPDATE ON {table}
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_column()
        "#,
        table = table.to_string()
    ))
    .await?;

    Ok(())
}

// Keeping this in case we need to actually drop a trigger, but more likely,
// just dropping the table is fine.
#[allow(dead_code)]
pub async fn unset_update_on_update<T: Iden>(db: &SchemaManagerConnection<'_>, table: T) -> Result<(), DbErr> {
    db.execute_unprepared(&format!(
        r#"
        DROP TRIGGER IF EXISTS set_updated_at ON {table}
        "#,
        table = table.to_string(),
    ))
    .await?;

    Ok(())
}
