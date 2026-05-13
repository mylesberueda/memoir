use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DocumentGroups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentGroups::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DocumentGroups::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(DocumentGroups::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID"),
                    )
                    .col(ColumnDef::new(DocumentGroups::OrganizationPid).string().not_null())
                    .col(ColumnDef::new(DocumentGroups::Name).string().not_null())
                    .col(ColumnDef::new(DocumentGroups::Description).text())
                    .col(
                        ColumnDef::new(DocumentGroups::IsOrgShared)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(DocumentGroups::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DocumentGroups::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, DocumentGroups::Table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_document_groups_user_org")
                    .table(DocumentGroups::Table)
                    .col(DocumentGroups::UserId)
                    .col(DocumentGroups::OrganizationPid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DocumentGroups::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum DocumentGroups {
    Table,
    Id,
    Pid,
    UserId,
    OrganizationPid,
    Name,
    Description,
    IsOrgShared,
    CreatedAt,
    UpdatedAt,
}
