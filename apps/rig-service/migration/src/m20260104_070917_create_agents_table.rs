use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20260104_070833_create_secrets_table::Secrets;
use super::m20260104_070900_create_language_models_table::LanguageModels;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Agents::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Agents::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Agents::OrganizationPid)
                            .string()
                            .not_null()
                            .comment("Organization PID from api-service"),
                    )
                    .col(
                        ColumnDef::new(Agents::CreatedBy)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID"),
                    )
                    .col(ColumnDef::new(Agents::Name).string().not_null())
                    .col(ColumnDef::new(Agents::Slug).string().not_null())
                    .col(
                        ColumnDef::new(Agents::Kind)
                            .string()
                            .not_null()
                            .default("startup")
                            .comment("startup, custom, etc"),
                    )
                    .col(ColumnDef::new(Agents::ModelId).big_integer().not_null())
                    .col(ColumnDef::new(Agents::Description).text())
                    .col(ColumnDef::new(Agents::Temperature).double().not_null().default(0.7))
                    .col(ColumnDef::new(Agents::SystemPrompt).text())
                    .col(ColumnDef::new(Agents::Config).json_binary().not_null().default("{}"))
                    .col(ColumnDef::new(Agents::IsActive).boolean().not_null().default(true))
                    .col(
                        ColumnDef::new(Agents::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Agents::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Agents::Table, Agents::ModelId)
                            .to(LanguageModels::Table, LanguageModels::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_agents_pid")
                    .table(Agents::Table)
                    .col(Agents::Pid)
                    .to_owned(),
            )
            .await?;

        // Slug unique within organization
        manager
            .create_index(
                Index::create()
                    .name("idx_agents_org_slug")
                    .table(Agents::Table)
                    .col(Agents::OrganizationPid)
                    .col(Agents::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_agents_org_active")
                    .table(Agents::Table)
                    .col(Agents::OrganizationPid)
                    .col(Agents::IsActive)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Agents::Table).await?;

        // Create agent_secrets junction table
        manager
            .create_table(
                Table::create()
                    .table(AgentSecrets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AgentSecrets::AgentId).big_integer().not_null())
                    .col(ColumnDef::new(AgentSecrets::SecretId).big_integer().not_null())
                    .primary_key(Index::create().col(AgentSecrets::AgentId).col(AgentSecrets::SecretId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AgentSecrets::Table, AgentSecrets::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AgentSecrets::Table, AgentSecrets::SecretId)
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
            .drop_table(Table::drop().table(AgentSecrets::Table).to_owned())
            .await?;
        manager.drop_table(Table::drop().table(Agents::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Agents {
    Table,
    Id,
    Pid,
    OrganizationPid,
    CreatedBy,
    ModelId,
    Name,
    Slug,
    Kind,
    Description,
    Temperature,
    SystemPrompt,
    Config,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub(crate) enum AgentSecrets {
    Table,
    AgentId,
    SecretId,
}
