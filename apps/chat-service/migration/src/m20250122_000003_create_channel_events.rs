use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "message_kind")]
pub enum MessageKind {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "agent")]
    Agent,
    #[sea_orm(string_value = "system")]
    System,
    #[sea_orm(string_value = "unknown")]
    Unknown,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create message_kind enum from ActiveEnum
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<MessageKind>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(ChannelEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChannelEvents::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ChannelEvents::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    .col(ColumnDef::new(ChannelEvents::ChannelId).string().not_null())
                    .col(
                        ColumnDef::new(ChannelEvents::MessageKind)
                            .custom(MessageKind::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(ChannelEvents::UserId).string())
                    .col(ColumnDef::new(ChannelEvents::SenderName).string().not_null())
                    .col(ColumnDef::new(ChannelEvents::Content).json_binary().not_null())
                    .col(ColumnDef::new(ChannelEvents::ParentPid).string()) // Thread parent
                    .col(
                        ColumnDef::new(ChannelEvents::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ChannelEvents::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ChannelEvents::IsEdited)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ChannelEvents::EditedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ChannelEvents::IsPinned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ChannelEvents::PinnedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ChannelEvents::PinnedByUserId).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_events_channel_id")
                            .from(ChannelEvents::Table, ChannelEvents::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Composite index for message history pagination
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_events_channel_timestamp")
                    .table(ChannelEvents::Table)
                    .col(ChannelEvents::ChannelId)
                    .col(ChannelEvents::Timestamp)
                    .to_owned(),
            )
            .await?;

        // Index for thread lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_events_parent_pid")
                    .table(ChannelEvents::Table)
                    .col(ChannelEvents::ParentPid)
                    .to_owned(),
            )
            .await?;

        // Index for user's messages
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_events_user_id")
                    .table(ChannelEvents::Table)
                    .col(ChannelEvents::UserId)
                    .to_owned(),
            )
            .await?;

        // Index for pinned messages
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_events_pinned")
                    .table(ChannelEvents::Table)
                    .col(ChannelEvents::ChannelId)
                    .col(ChannelEvents::IsPinned)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChannelEvents::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(MessageKind::name()).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum ChannelEvents {
    Table,
    Id,
    Pid,
    ChannelId,
    MessageKind,
    UserId,
    SenderName,
    Content,
    ParentPid,
    Timestamp,
    IsDeleted,
    IsEdited,
    EditedAt,
    IsPinned,
    PinnedAt,
    PinnedByUserId,
}
