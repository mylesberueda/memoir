use sea_orm_migration::prelude::*;

use crate::m20260104_070929_create_conversations_table::Conversations;
use crate::m20260220_000001_create_documents_table::Documents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ConversationDocuments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConversationDocuments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ConversationDocuments::ConversationId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConversationDocuments::DocumentId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConversationDocuments::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cd_conversation_id")
                            .from(
                                ConversationDocuments::Table,
                                ConversationDocuments::ConversationId,
                            )
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cd_document_id")
                            .from(
                                ConversationDocuments::Table,
                                ConversationDocuments::DocumentId,
                            )
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cd_conversation_document_unique")
                    .table(ConversationDocuments::Table)
                    .col(ConversationDocuments::ConversationId)
                    .col(ConversationDocuments::DocumentId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cd_conversation_id")
                    .table(ConversationDocuments::Table)
                    .col(ConversationDocuments::ConversationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cd_document_id")
                    .table(ConversationDocuments::Table)
                    .col(ConversationDocuments::DocumentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ConversationDocuments::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum ConversationDocuments {
    Table,
    Id,
    ConversationId,
    DocumentId,
    CreatedAt,
}
