use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
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
                    .table(LanguageModels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LanguageModels::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(ColumnDef::new(LanguageModels::ProviderId).big_integer().not_null())
                    .col(
                        ColumnDef::new(LanguageModels::ModelId)
                            .string()
                            .not_null()
                            .comment("Provider's model identifier (e.g., gpt-4o, llama3.2)"),
                    )
                    .col(ColumnDef::new(LanguageModels::Name).string().not_null())
                    .col(ColumnDef::new(LanguageModels::ContextWindow).integer())
                    .col(
                        ColumnDef::new(LanguageModels::Capabilities)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb"))
                            .comment("Model capabilities: vision, function_calling, json_mode, etc."),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::Metadata)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb"))
                            .comment("Provider-specific metadata: pricing, owned_by, etc."),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::DeprecationMessage)
                            .text()
                            .comment("Message explaining deprecation reason and alternatives. NULL = not deprecated"),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::LastFetchedAt)
                            .timestamp()
                            .comment("When model was last synced from provider API"),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(LanguageModels::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(LanguageModels::Table, LanguageModels::ProviderId)
                            .to(Providers::Table, Providers::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_models_pid")
                    .table(LanguageModels::Table)
                    .col(LanguageModels::Pid)
                    .to_owned(),
            )
            .await?;

        // Unique model_id per provider
        manager
            .create_index(
                Index::create()
                    .name("idx_models_provider_model_id")
                    .table(LanguageModels::Table)
                    .col(LanguageModels::ProviderId)
                    .col(LanguageModels::ModelId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_models_provider_active")
                    .table(LanguageModels::Table)
                    .col(LanguageModels::ProviderId)
                    .col(LanguageModels::IsActive)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, LanguageModels::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LanguageModels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum LanguageModels {
    Table,
    Id,
    Pid,
    ProviderId,
    ModelId,
    Name,
    ContextWindow,
    Capabilities,
    Metadata,
    IsActive,
    DeprecationMessage,
    LastFetchedAt,
    CreatedAt,
    UpdatedAt,
}
