use super::m20260104_070929_create_conversations_table::Conversations;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Messages::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()"),
                    )
                    .col(ColumnDef::new(Messages::ConversationId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Messages::Role)
                            .string()
                            .not_null()
                            .comment("user, assistant, system"),
                    )
                    .col(ColumnDef::new(Messages::Content).text())
                    .col(ColumnDef::new(Messages::Parts).json_binary().not_null().default("[]"))
                    .col(
                        ColumnDef::new(Messages::Status)
                            .string()
                            .not_null()
                            .default("complete")
                            .comment("complete, cancelled, error"),
                    )
                    .col(ColumnDef::new(Messages::IsDeleted).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(Messages::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Messages::Table, Messages::ConversationId)
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // History queries: get messages in order
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_conversation_created")
                    .table(Messages::Table)
                    .col(Messages::ConversationId)
                    .col(Messages::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Cursor pagination
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_conversation_id")
                    .table(Messages::Table)
                    .col(Messages::ConversationId)
                    .col(Messages::Id)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();

        // Enable pgvector extension for embedding storage
        db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector;").await?;

        // Add embedding column for semantic search (384 dims = BGE-small-en-v1.5)
        db.execute_unprepared("ALTER TABLE messages ADD COLUMN embedding vector(384);")
            .await?;

        // HNSW index for fast cosine similarity search
        db.execute_unprepared(
            r#"
            CREATE INDEX idx_messages_embedding
            ON messages
            USING hnsw (embedding vector_cosine_ops);
            "#,
        )
        .await?;

        // Attach trigger: update conversation on message insert
        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_conversation_on_insert
            AFTER INSERT ON messages
            FOR EACH ROW
            EXECUTE FUNCTION update_conversation_on_message_insert();
            "#,
        )
        .await?;

        // Attach trigger: recalculate conversation on soft delete
        db.execute_unprepared(
            r#"
            CREATE TRIGGER update_conversation_on_soft_delete
            AFTER UPDATE ON messages
            FOR EACH ROW
            EXECUTE FUNCTION update_conversation_on_message_soft_delete();
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,
    Pid,
    ConversationId,
    Role,
    Content,
    Parts,
    Status,
    IsDeleted,
    CreatedAt,
}
