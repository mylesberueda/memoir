use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Providers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Providers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Providers::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Providers::OrganizationPid)
                            .string()
                            .comment("Organization PID from api-service. NULL for system providers."),
                    )
                    .col(
                        ColumnDef::new(Providers::CreatedBy)
                            .string()
                            .comment("Zitadel user ID. NULL = system provider"),
                    )
                    .col(ColumnDef::new(Providers::Name).string().not_null())
                    .col(
                        ColumnDef::new(Providers::ProviderType)
                            .string()
                            .not_null()
                            .comment("ollama, openai, anthropic, etc."),
                    )
                    .col(ColumnDef::new(Providers::BaseUrl).string())
                    .col(ColumnDef::new(Providers::IsActive).boolean().not_null().default(true))
                    .col(
                        ColumnDef::new(Providers::IsDeprecated)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Providers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Providers::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Index on pid for external lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_providers_pid")
                    .table(Providers::Table)
                    .col(Providers::Pid)
                    .to_owned(),
            )
            .await?;

        // Composite index for org-scoped queries
        manager
            .create_index(
                Index::create()
                    .name("idx_providers_org_active")
                    .table(Providers::Table)
                    .col(Providers::OrganizationPid)
                    .col(Providers::IsActive)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Providers::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Providers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Providers {
    Table,
    Id,
    Pid,
    OrganizationPid,
    CreatedBy,
    Name,
    ProviderType,
    BaseUrl,
    IsActive,
    IsDeprecated,
    CreatedAt,
    UpdatedAt,
}
