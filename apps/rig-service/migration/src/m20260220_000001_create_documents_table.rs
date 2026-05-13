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
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Documents::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Documents::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Documents::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID"),
                    )
                    .col(ColumnDef::new(Documents::OrganizationPid).string().not_null())
                    .col(ColumnDef::new(Documents::Filename).string().not_null())
                    .col(ColumnDef::new(Documents::ContentType).string().not_null())
                    .col(ColumnDef::new(Documents::SizeBytes).big_integer().not_null())
                    .col(ColumnDef::new(Documents::StoragePath).string().not_null())
                    .col(ColumnDef::new(Documents::Summary).text())
                    .col(ColumnDef::new(Documents::Status).string().not_null().default("pending"))
                    .col(ColumnDef::new(Documents::ErrorMessage).text())
                    .col(
                        ColumnDef::new(Documents::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Documents::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Documents::Table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_documents_user_org")
                    .table(Documents::Table)
                    .col(Documents::UserId)
                    .col(Documents::OrganizationPid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_documents_status")
                    .table(Documents::Table)
                    .col(Documents::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Documents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Documents {
    Table,
    Id,
    Pid,
    UserId,
    OrganizationPid,
    Filename,
    ContentType,
    SizeBytes,
    StoragePath,
    Summary,
    Status,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}
