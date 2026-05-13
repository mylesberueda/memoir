use super::m20260104_070556_create_providers_table::Providers;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProviderRates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProviderRates::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProviderRates::ProviderId).big_integer().not_null())
                    .col(
                        ColumnDef::new(ProviderRates::LimitType)
                            .string()
                            .not_null()
                            .comment("rpm, tpm, rpd"),
                    )
                    .col(ColumnDef::new(ProviderRates::LimitValue).integer().not_null())
                    .col(ColumnDef::new(ProviderRates::WindowSeconds).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProviderRates::Table, ProviderRates::ProviderId)
                            .to(Providers::Table, Providers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint: one limit type per provider
        manager
            .create_index(
                Index::create()
                    .name("idx_provider_rates_unique")
                    .table(ProviderRates::Table)
                    .col(ProviderRates::ProviderId)
                    .col(ProviderRates::LimitType)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderRates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProviderRates {
    Table,
    Id,
    ProviderId,
    LimitType,
    LimitValue,
    WindowSeconds,
}
