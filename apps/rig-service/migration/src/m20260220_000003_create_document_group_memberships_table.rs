use sea_orm_migration::prelude::*;

use crate::m20260220_000001_create_documents_table::Documents;
use crate::m20260220_000002_create_document_groups_table::DocumentGroups;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DocumentGroupMemberships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentGroupMemberships::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DocumentGroupMemberships::DocumentId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DocumentGroupMemberships::GroupId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DocumentGroupMemberships::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dgm_document_id")
                            .from(
                                DocumentGroupMemberships::Table,
                                DocumentGroupMemberships::DocumentId,
                            )
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dgm_group_id")
                            .from(
                                DocumentGroupMemberships::Table,
                                DocumentGroupMemberships::GroupId,
                            )
                            .to(DocumentGroups::Table, DocumentGroups::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dgm_document_group_unique")
                    .table(DocumentGroupMemberships::Table)
                    .col(DocumentGroupMemberships::DocumentId)
                    .col(DocumentGroupMemberships::GroupId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dgm_document_id")
                    .table(DocumentGroupMemberships::Table)
                    .col(DocumentGroupMemberships::DocumentId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_dgm_group_id")
                    .table(DocumentGroupMemberships::Table)
                    .col(DocumentGroupMemberships::GroupId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(DocumentGroupMemberships::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum DocumentGroupMemberships {
    Table,
    Id,
    DocumentId,
    GroupId,
    CreatedAt,
}
