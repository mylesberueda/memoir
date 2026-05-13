use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "moderation_action_type")]
pub enum ModerationActionType {
    #[sea_orm(string_value = "mute")]
    Mute,
    #[sea_orm(string_value = "unmute")]
    Unmute,
    #[sea_orm(string_value = "ban")]
    Ban,
    #[sea_orm(string_value = "unban")]
    Unban,
    #[sea_orm(string_value = "kick")]
    Kick,
    #[sea_orm(string_value = "warn")]
    Warn,
    #[sea_orm(string_value = "delete_message")]
    DeleteMessage,
    #[sea_orm(string_value = "role_change")]
    RoleChange,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<ModerationActionType>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(ModerationLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModerationLog::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ModerationLog::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    .col(ColumnDef::new(ModerationLog::ChannelId).string().not_null())
                    .col(ColumnDef::new(ModerationLog::ModeratorId).string().not_null())
                    .col(ColumnDef::new(ModerationLog::TargetUserId).string().not_null())
                    .col(
                        ColumnDef::new(ModerationLog::ActionType)
                            .custom(ModerationActionType::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(ModerationLog::Reason).text())
                    .col(ColumnDef::new(ModerationLog::Metadata).json_binary()) // Additional context
                    .col(
                        ColumnDef::new(ModerationLog::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_moderation_log_channel_id")
                            .from(ModerationLog::Table, ModerationLog::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for finding actions by moderator
        manager
            .create_index(
                Index::create()
                    .name("idx_moderation_log_moderator_id")
                    .table(ModerationLog::Table)
                    .col(ModerationLog::ModeratorId)
                    .to_owned(),
            )
            .await?;

        // Index for finding actions against a user
        manager
            .create_index(
                Index::create()
                    .name("idx_moderation_log_target_user_id")
                    .table(ModerationLog::Table)
                    .col(ModerationLog::TargetUserId)
                    .to_owned(),
            )
            .await?;

        // Index for finding actions by channel + time
        manager
            .create_index(
                Index::create()
                    .name("idx_moderation_log_channel_created")
                    .table(ModerationLog::Table)
                    .col(ModerationLog::ChannelId)
                    .col(ModerationLog::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ModerationLog::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ModerationActionType::name()).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ModerationLog {
    Table,
    Id,
    Pid,
    ChannelId,
    ModeratorId,
    TargetUserId,
    ActionType,
    Reason,
    Metadata,
    CreatedAt,
}
