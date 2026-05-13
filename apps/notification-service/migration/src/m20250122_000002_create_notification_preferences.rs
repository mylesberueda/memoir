use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NotificationPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NotificationPreferences::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // One preferences row per user
                    .col(
                        ColumnDef::new(NotificationPreferences::UserId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    // Global toggles
                    .col(
                        ColumnDef::new(NotificationPreferences::PushEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(NotificationPreferences::EmailEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(NotificationPreferences::SoundEnabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    // Per-category preferences as JSONB
                    // Structure: [{ "category": "chat", "enabled": true, "push": true, "email": false, "min_priority": "normal" }]
                    .col(ColumnDef::new(NotificationPreferences::CategoryPreferences).json_binary())
                    // Timestamps
                    .col(
                        ColumnDef::new(NotificationPreferences::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(NotificationPreferences::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Set up updated_at trigger
        let db = manager.get_connection();
        set_update_on_update(db, NotificationPreferences::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NotificationPreferences::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum NotificationPreferences {
    Table,
    Id,
    UserId,
    PushEnabled,
    EmailEnabled,
    SoundEnabled,
    CategoryPreferences,
    CreatedAt,
    UpdatedAt,
}
