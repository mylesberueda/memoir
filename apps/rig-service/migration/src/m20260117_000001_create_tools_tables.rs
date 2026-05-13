use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20260104_070833_create_secrets_table::Secrets;
use super::m20260104_070917_create_agents_table::Agents;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tools table
        manager
            .create_table(
                Table::create()
                    .table(Tools::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tools::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Tools::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Tools::Name)
                            .string()
                            .not_null()
                            .unique_key()
                            .comment("Tool identifier (e.g., 'current_time')"),
                    )
                    .col(
                        ColumnDef::new(Tools::DisplayName)
                            .string()
                            .not_null()
                            .comment("Human-readable name"),
                    )
                    .col(
                        ColumnDef::new(Tools::Description)
                            .text()
                            .not_null()
                            .comment("Shown to LLM for tool selection"),
                    )
                    .col(
                        ColumnDef::new(Tools::ToolType)
                            .string()
                            .not_null()
                            .default("system")
                            .comment("'system' or 'user_defined'"),
                    )
                    .col(
                        ColumnDef::new(Tools::ParametersSchema)
                            .json_binary()
                            .not_null()
                            .default("{}")
                            .comment("JSON schema for tool arguments"),
                    )
                    .col(
                        ColumnDef::new(Tools::IsActive)
                            .boolean()
                            .not_null()
                            .default(true)
                            .comment("Soft disable"),
                    )
                    .col(
                        ColumnDef::new(Tools::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tools::UpdatedAt)
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
                    .name("idx_tools_pid")
                    .table(Tools::Table)
                    .col(Tools::Pid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tools_name")
                    .table(Tools::Table)
                    .col(Tools::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tools_type_active")
                    .table(Tools::Table)
                    .col(Tools::ToolType)
                    .col(Tools::IsActive)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Tools::Table).await?;

        // Create agent_tools junction table
        manager
            .create_table(
                Table::create()
                    .table(AgentTools::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AgentTools::AgentId).big_integer().not_null())
                    .col(ColumnDef::new(AgentTools::ToolId).big_integer().not_null())
                    .col(
                        ColumnDef::new(AgentTools::Config)
                            .json_binary()
                            .not_null()
                            .default("{}")
                            .comment("Agent-specific tool config (optional overrides)"),
                    )
                    .primary_key(Index::create().col(AgentTools::AgentId).col(AgentTools::ToolId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AgentTools::Table, AgentTools::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AgentTools::Table, AgentTools::ToolId)
                            .to(Tools::Table, Tools::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create tool_secrets junction table
        manager
            .create_table(
                Table::create()
                    .table(ToolSecrets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ToolSecrets::ToolId).big_integer().not_null())
                    .col(ColumnDef::new(ToolSecrets::SecretId).big_integer().not_null())
                    .primary_key(Index::create().col(ToolSecrets::ToolId).col(ToolSecrets::SecretId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ToolSecrets::Table, ToolSecrets::ToolId)
                            .to(Tools::Table, Tools::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ToolSecrets::Table, ToolSecrets::SecretId)
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
            .drop_table(Table::drop().table(ToolSecrets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AgentTools::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tools::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Tools {
    Table,
    Id,
    Pid,
    Name,
    DisplayName,
    Description,
    ToolType,
    ParametersSchema,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub(crate) enum AgentTools {
    Table,
    AgentId,
    ToolId,
    Config,
}

#[derive(DeriveIden)]
pub(crate) enum ToolSecrets {
    Table,
    ToolId,
    SecretId,
}
