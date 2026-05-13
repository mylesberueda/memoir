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
                    .table(MessageReactions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MessageReactions::MessagePid).string().not_null())
                    .col(ColumnDef::new(MessageReactions::UserId).string().not_null())
                    .col(ColumnDef::new(MessageReactions::Emoji).string().not_null())
                    .col(
                        ColumnDef::new(MessageReactions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(MessageReactions::MessagePid)
                            .col(MessageReactions::UserId)
                            .col(MessageReactions::Emoji),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_reactions_message_pid")
                            .from(MessageReactions::Table, MessageReactions::MessagePid)
                            .to(ChannelEvents::Table, ChannelEvents::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for counting reactions per message
        manager
            .create_index(
                Index::create()
                    .name("idx_message_reactions_message_emoji")
                    .table(MessageReactions::Table)
                    .col(MessageReactions::MessagePid)
                    .col(MessageReactions::Emoji)
                    .to_owned(),
            )
            .await?;

        // Index for finding user's reactions
        manager
            .create_index(
                Index::create()
                    .name("idx_message_reactions_user_id")
                    .table(MessageReactions::Table)
                    .col(MessageReactions::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageReactions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MessageReactions {
    Table,
    MessagePid,
    UserId,
    Emoji,
    CreatedAt,
}
