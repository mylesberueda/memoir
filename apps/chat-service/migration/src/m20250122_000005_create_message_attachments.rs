use sea_orm_migration::prelude::*;

use super::m20250122_000003_create_channel_events::ChannelEvents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageAttachments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageAttachments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MessageAttachments::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    .col(ColumnDef::new(MessageAttachments::MessagePid).string().not_null())
                    .col(ColumnDef::new(MessageAttachments::Filename).string().not_null())
                    .col(ColumnDef::new(MessageAttachments::ContentType).string().not_null())
                    .col(ColumnDef::new(MessageAttachments::SizeBytes).big_integer().not_null())
                    .col(ColumnDef::new(MessageAttachments::Url).string().not_null())
                    .col(ColumnDef::new(MessageAttachments::ThumbnailUrl).string())
                    .col(
                        ColumnDef::new(MessageAttachments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_attachments_message_pid")
                            .from(MessageAttachments::Table, MessageAttachments::MessagePid)
                            .to(ChannelEvents::Table, ChannelEvents::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_message_attachments_message_pid")
                    .table(MessageAttachments::Table)
                    .col(MessageAttachments::MessagePid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageAttachments::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MessageAttachments {
    Table,
    Id,
    Pid,
    MessagePid,
    Filename,
    ContentType,
    SizeBytes,
    Url,
    ThumbnailUrl,
    CreatedAt,
}
