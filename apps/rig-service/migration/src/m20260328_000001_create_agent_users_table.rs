use sea_orm_migration::prelude::*;

use crate::m20260104_070917_create_agents_table::Agents;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AgentUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AgentUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AgentUsers::AgentId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AgentUsers::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID of the recipient"),
                    )
                    .col(
                        ColumnDef::new(AgentUsers::Permissions)
                            .small_integer()
                            .not_null()
                            .default(0)
                            .comment("Bitfield: 1=read, 2=write, 4=execute"),
                    )
                    .col(
                        ColumnDef::new(AgentUsers::SharedBy)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID of the sharer"),
                    )
                    .col(
                        ColumnDef::new(AgentUsers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_agent_users_agent_id")
                            .from(AgentUsers::Table, AgentUsers::AgentId)
                            .to(Agents::Table, Agents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_agent_users_agent_user_unique")
                    .table(AgentUsers::Table)
                    .col(AgentUsers::AgentId)
                    .col(AgentUsers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_agent_users_user_id")
                    .table(AgentUsers::Table)
                    .col(AgentUsers::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AgentUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum AgentUsers {
    Table,
    Id,
    AgentId,
    UserId,
    Permissions,
    SharedBy,
    CreatedAt,
}
