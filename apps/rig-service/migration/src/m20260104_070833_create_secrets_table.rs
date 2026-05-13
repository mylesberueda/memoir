use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20260104_070556_create_providers_table::Providers;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create secrets table
        manager
            .create_table(
                Table::create()
                    .table(Secrets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Secrets::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Secrets::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Secrets::SecretType)
                            .string()
                            .not_null()
                            .comment("api_key, webhook_secret, etc."),
                    )
                    .col(
                        ColumnDef::new(Secrets::EncryptedValue)
                            .binary()
                            .not_null()
                            .comment("AES-256-GCM encrypted"),
                    )
                    .col(
                        ColumnDef::new(Secrets::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Secrets::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_secrets_pid")
                    .table(Secrets::Table)
                    .col(Secrets::Pid)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Secrets::Table).await?;

        // Create provider_secrets junction table
        manager
            .create_table(
                Table::create()
                    .table(ProviderSecrets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ProviderSecrets::ProviderId).big_integer().not_null())
                    .col(ColumnDef::new(ProviderSecrets::SecretId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(ProviderSecrets::ProviderId)
                            .col(ProviderSecrets::SecretId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProviderSecrets::Table, ProviderSecrets::ProviderId)
                            .to(Providers::Table, Providers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProviderSecrets::Table, ProviderSecrets::SecretId)
                            .to(Secrets::Table, Secrets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderSecrets::Table).to_owned())
            .await?;
        manager.drop_table(Table::drop().table(Secrets::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Secrets {
    Table,
    Id,
    Pid,
    SecretType,
    EncryptedValue,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub(crate) enum ProviderSecrets {
    Table,
    ProviderId,
    SecretId,
}
