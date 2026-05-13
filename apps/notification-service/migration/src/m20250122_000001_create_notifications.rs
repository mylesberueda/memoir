use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

// =============================================================================
// Enums - these will be generated as Rust enums by SeaORM codegen
// =============================================================================

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "notification_category")]
pub enum NotificationCategory {
    #[sea_orm(string_value = "chat")]
    Chat,
    #[sea_orm(string_value = "agent")]
    Agent,
    #[sea_orm(string_value = "system")]
    System,
    #[sea_orm(string_value = "moderation")]
    Moderation,
    #[sea_orm(string_value = "billing")]
    Billing,
    #[sea_orm(string_value = "social")]
    Social,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "notification_priority")]
pub enum NotificationPriority {
    #[sea_orm(string_value = "low")]
    Low,
    #[sea_orm(string_value = "normal")]
    Normal,
    #[sea_orm(string_value = "high")]
    High,
    #[sea_orm(string_value = "urgent")]
    Urgent,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "origin_service")]
pub enum OriginService {
    #[sea_orm(string_value = "api")]
    Api,
    #[sea_orm(string_value = "chat")]
    Chat,
    #[sea_orm(string_value = "rig")]
    Rig,
    #[sea_orm(string_value = "agent")]
    Agent,
    #[sea_orm(string_value = "notification")]
    Notification,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "origin_entity_type")]
pub enum OriginEntityType {
    #[sea_orm(string_value = "channel")]
    Channel,
    #[sea_orm(string_value = "message")]
    Message,
    #[sea_orm(string_value = "agent")]
    Agent,
    #[sea_orm(string_value = "run")]
    Run,
    #[sea_orm(string_value = "workflow")]
    Workflow,
    #[sea_orm(string_value = "user")]
    User,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create all enum types
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);

        manager
            .create_type(
                schema
                    .create_enum_from_active_enum::<NotificationCategory>()
                    .expect("NotificationCategory should exist"),
            )
            .await?;
        manager
            .create_type(
                schema
                    .create_enum_from_active_enum::<NotificationPriority>()
                    .expect("NotificationPriority should exist"),
            )
            .await?;
        manager
            .create_type(
                schema
                    .create_enum_from_active_enum::<OriginService>()
                    .expect("OriginService should exist"),
            )
            .await?;
        manager
            .create_type(
                schema
                    .create_enum_from_active_enum::<OriginEntityType>()
                    .expect("OriginEntityType should exist"),
            )
            .await?;

        // Create notifications table
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    // Primary key
                    .col(
                        ColumnDef::new(Notifications::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Notifications::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    // User context
                    .col(ColumnDef::new(Notifications::UserId).string().not_null())
                    .col(ColumnDef::new(Notifications::OrgPid).string().not_null())
                    // Content
                    .col(ColumnDef::new(Notifications::Title).string().not_null())
                    .col(ColumnDef::new(Notifications::Description).text().not_null())
                    .col(ColumnDef::new(Notifications::IconUrl).text())
                    // Classification (enums)
                    .col(
                        ColumnDef::new(Notifications::Category)
                            .custom(NotificationCategory::name())
                            .not_null()
                            .default("system"),
                    )
                    .col(
                        ColumnDef::new(Notifications::Priority)
                            .custom(NotificationPriority::name())
                            .not_null()
                            .default("normal"),
                    )
                    // Actions (JSONB array of { kind, target, params })
                    .col(ColumnDef::new(Notifications::Actions).json_binary())
                    // Origin (flattened, service is enum)
                    .col(
                        ColumnDef::new(Notifications::OriginService)
                            .custom(OriginService::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notifications::OriginEntityType).custom(OriginEntityType::name()))
                    .col(ColumnDef::new(Notifications::OriginEntityPid).string())
                    // State
                    .col(
                        ColumnDef::new(Notifications::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Notifications::ReadAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Notifications::IsDismissed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Notifications::ExpiresAt).timestamp_with_time_zone())
                    // Deduplication
                    .col(ColumnDef::new(Notifications::IdempotencyKey).string())
                    // Timestamps
                    .col(
                        ColumnDef::new(Notifications::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Notifications::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Set up updated_at trigger
        let db = manager.get_connection();
        set_update_on_update(db, Notifications::Table).await?;

        // Indexes
        // Primary query pattern: list user's notifications sorted by time
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_user_id_created_at")
                    .table(Notifications::Table)
                    .col(Notifications::UserId)
                    .col((Notifications::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        // Unread count query (partial index for efficiency)
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE INDEX idx_notifications_user_unread
                ON notifications (user_id, is_read)
                WHERE NOT is_dismissed
                "#,
            )
            .await?;

        // Category filter
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE INDEX idx_notifications_user_category
                ON notifications (user_id, category)
                WHERE NOT is_dismissed
                "#,
            )
            .await?;

        // Idempotency (unique per user)
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE UNIQUE INDEX idx_notifications_idempotency
                ON notifications (user_id, idempotency_key)
                WHERE idempotency_key IS NOT NULL
                "#,
            )
            .await?;

        // Expiration cleanup job
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE INDEX idx_notifications_expires_at
                ON notifications (expires_at)
                WHERE expires_at IS NOT NULL AND NOT is_dismissed
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notifications::Table).to_owned())
            .await?;

        // Drop enum types
        manager
            .drop_type(Type::drop().name(OriginEntityType::name()).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OriginService::name()).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(NotificationPriority::name()).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(NotificationCategory::name()).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Notifications {
    Table,
    Id,
    Pid,
    UserId,
    OrgPid,
    Title,
    Description,
    IconUrl,
    Category,
    Priority,
    Actions,
    OriginService,
    OriginEntityType,
    OriginEntityPid,
    IsRead,
    ReadAt,
    IsDismissed,
    ExpiresAt,
    IdempotencyKey,
    CreatedAt,
    UpdatedAt,
}
