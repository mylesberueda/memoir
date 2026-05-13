use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReadReceipts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ReadReceipts::ChannelId).string().not_null())
                    .col(ColumnDef::new(ReadReceipts::UserId).string().not_null())
                    .col(ColumnDef::new(ReadReceipts::LastReadMessagePid).string())
                    .col(
                        ColumnDef::new(ReadReceipts::LastReadAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ReadReceipts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(ReadReceipts::ChannelId)
                            .col(ReadReceipts::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_read_receipts_channel_id")
                            .from(ReadReceipts::Table, ReadReceipts::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, ReadReceipts::Table).await?;

        // Index for finding user's read state across channels
        manager
            .create_index(
                Index::create()
                    .name("idx_read_receipts_user_id")
                    .table(ReadReceipts::Table)
                    .col(ReadReceipts::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReadReceipts::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ReadReceipts {
    Table,
    ChannelId,
    UserId,
    LastReadMessagePid,
    LastReadAt,
    UpdatedAt,
}
