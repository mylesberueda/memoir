use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20260104_070917_create_agents_table::Agents;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Conversations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conversations::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Conversations::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(
                        ColumnDef::new(Conversations::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user ID"),
                    )
                    .col(
                        ColumnDef::new(Conversations::OrganizationPid)
                            .string()
                            .not_null()
                            .comment("Organization PID from api-service"),
                    )
                    .col(ColumnDef::new(Conversations::AgentId).big_integer().not_null())
                    .col(ColumnDef::new(Conversations::Title).string())
                    .col(
                        ColumnDef::new(Conversations::MessageCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Conversations::LastMessageAt).timestamp())
                    .col(
                        ColumnDef::new(Conversations::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Conversations::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Conversations::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Conversations::Table, Conversations::AgentId)
                            .to(Agents::Table, Agents::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_pid")
                    .table(Conversations::Table)
                    .col(Conversations::Pid)
                    .to_owned(),
            )
            .await?;

        // User's conversations in an org, sorted by activity
        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_user_org_updated")
                    .table(Conversations::Table)
                    .col(Conversations::UserId)
                    .col(Conversations::OrganizationPid)
                    .col((Conversations::UpdatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_conversations_agent")
                    .table(Conversations::Table)
                    .col(Conversations::AgentId)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Conversations::Table).await?;

        // Trigger function: update conversation stats on message INSERT
        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION update_conversation_on_message_insert()
            RETURNS TRIGGER AS $$
            BEGIN
              UPDATE conversations
              SET message_count = message_count + 1,
                  last_message_at = NEW.created_at
              WHERE id = NEW.conversation_id;
              RETURN NEW;
            END;
            $$ LANGUAGE plpgsql;
            "#,
        )
        .await?;

        // Trigger function: recalculate conversation stats when message is_deleted changes
        db.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION update_conversation_on_message_soft_delete()
            RETURNS TRIGGER AS $$
            BEGIN
              IF OLD.is_deleted IS DISTINCT FROM NEW.is_deleted THEN
                UPDATE conversations
                SET message_count = (
                      SELECT COUNT(*) FROM messages
                      WHERE conversation_id = NEW.conversation_id AND NOT is_deleted
                    ),
                    last_message_at = (
                      SELECT MAX(created_at) FROM messages
                      WHERE conversation_id = NEW.conversation_id AND NOT is_deleted
                    )
                WHERE id = NEW.conversation_id;
              END IF;
              RETURN NEW;
            END;
            $$ LANGUAGE plpgsql;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Conversations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Conversations {
    Table,
    Id,
    Pid,
    UserId,
    OrganizationPid,
    AgentId,
    Title,
    MessageCount,
    LastMessageAt,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
