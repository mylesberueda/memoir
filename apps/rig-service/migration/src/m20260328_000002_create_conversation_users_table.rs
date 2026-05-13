use sea_orm_migration::prelude::*;

use crate::m20260104_070929_create_conversations_table::Conversations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConversationUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConversationUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConversationUsers::ConversationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConversationUsers::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID of the recipient"),
                    )
                    .col(
                        ColumnDef::new(ConversationUsers::Permissions)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("Bitfield: 1=read, 2=write, 4=execute"),
                    )
                    .col(
                        ColumnDef::new(ConversationUsers::SharedBy)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID of the sharer"),
                    )
                    .col(
                        ColumnDef::new(ConversationUsers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conversation_users_conversation_id")
                            .from(ConversationUsers::Table, ConversationUsers::ConversationId)
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_users_conv_user_unique")
                    .table(ConversationUsers::Table)
                    .col(ConversationUsers::ConversationId)
                    .col(ConversationUsers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversation_users_user_id")
                    .table(ConversationUsers::Table)
                    .col(ConversationUsers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConversationUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum ConversationUsers {
    Table,
    Id,
    ConversationId,
    UserId,
    Permissions,
    SharedBy,
    CreatedAt,
}
