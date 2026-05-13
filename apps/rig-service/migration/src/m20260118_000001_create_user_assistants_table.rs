use super::m20260104_070917_create_agents_table::Agents;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user_assistants junction table
        // Links users to their single assistant agent (user-scoped, not org-scoped)
        // The assistant has visibility across all user contexts (personal + orgs)
        manager
            .create_table(
                Table::create()
                    .table(UserAssistants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAssistants::UserId)
                            .string()
                            .not_null()
                            .primary_key()
                            .comment("Zitadel user ID - one assistant per user"),
                    )
                    .col(
                        ColumnDef::new(UserAssistants::AgentId)
                            .big_integer()
                            .not_null()
                            .unique_key()
                            .comment("Each agent can only be one user's assistant"),
                    )
                    .col(
                        ColumnDef::new(UserAssistants::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserAssistants::Table, UserAssistants::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserAssistants::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum UserAssistants {
    Table,
    UserId,
    AgentId,
    CreatedAt,
}
