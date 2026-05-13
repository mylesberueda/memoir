use sea_orm_migration::prelude::*;

use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChannelMutes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChannelMutes::ChannelId).string().not_null())
                    .col(ColumnDef::new(ChannelMutes::UserId).string().not_null())
                    .col(ColumnDef::new(ChannelMutes::MutedByUserId).string().not_null())
                    .col(ColumnDef::new(ChannelMutes::Reason).string())
                    .col(
                        ColumnDef::new(ChannelMutes::MutedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(ChannelMutes::MutedUntil).timestamp_with_time_zone()) // NULL = permanent
                    .primary_key(
                        Index::create()
                            .col(ChannelMutes::ChannelId)
                            .col(ChannelMutes::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_mutes_channel_id")
                            .from(ChannelMutes::Table, ChannelMutes::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for checking if user is muted
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_mutes_user_id")
                    .table(ChannelMutes::Table)
                    .col(ChannelMutes::UserId)
                    .to_owned(),
            )
            .await?;

        // Index for finding expired mutes
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_mutes_until")
                    .table(ChannelMutes::Table)
                    .col(ChannelMutes::MutedUntil)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChannelMutes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ChannelMutes {
    Table,
    ChannelId,
    UserId,
    MutedByUserId,
    Reason,
    MutedAt,
    MutedUntil,
}
