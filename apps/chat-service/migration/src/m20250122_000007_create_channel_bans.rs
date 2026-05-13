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
                    .table(ChannelBans::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChannelBans::ChannelId).string().not_null())
                    .col(ColumnDef::new(ChannelBans::UserId).string().not_null())
                    .col(ColumnDef::new(ChannelBans::BannedByUserId).string().not_null())
                    .col(ColumnDef::new(ChannelBans::Reason).string())
                    .col(
                        ColumnDef::new(ChannelBans::BannedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(ChannelBans::ChannelId)
                            .col(ChannelBans::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_bans_channel_id")
                            .from(ChannelBans::Table, ChannelBans::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for checking if user is banned
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_bans_user_id")
                    .table(ChannelBans::Table)
                    .col(ChannelBans::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChannelBans::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ChannelBans {
    Table,
    ChannelId,
    UserId,
    BannedByUserId,
    Reason,
    BannedAt,
}
