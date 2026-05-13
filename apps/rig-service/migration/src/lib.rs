pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
mod m20000000_000002_add_updated_at_trigger;
mod m20260104_070556_create_providers_table;
mod m20260104_070833_create_secrets_table;
mod m20260104_070900_create_language_models_table;
mod m20260104_070902_create_provider_rates_table;
mod m20260104_070917_create_agents_table;
mod m20260104_070929_create_conversations_table;
mod m20260104_070945_create_messages_table;
mod m20260117_000001_create_tools_tables;
mod m20260118_000001_create_user_assistants_table;
mod m20260220_000001_create_documents_table;
mod m20260220_000002_create_document_groups_table;
mod m20260220_000003_create_document_group_memberships_table;
mod m20260222_000001_create_conversation_documents_table;
mod m20260328_000001_create_agent_users_table;
mod m20260328_000002_create_conversation_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20260104_070556_create_providers_table::Migration),
            Box::new(m20260104_070833_create_secrets_table::Migration),
            Box::new(m20260104_070900_create_language_models_table::Migration),
            Box::new(m20260104_070902_create_provider_rates_table::Migration),
            Box::new(m20260104_070917_create_agents_table::Migration),
            Box::new(m20260104_070929_create_conversations_table::Migration),
            Box::new(m20260104_070945_create_messages_table::Migration),
            Box::new(m20260117_000001_create_tools_tables::Migration),
            Box::new(m20260118_000001_create_user_assistants_table::Migration),
            Box::new(m20260220_000001_create_documents_table::Migration),
            Box::new(m20260220_000002_create_document_groups_table::Migration),
            Box::new(m20260220_000003_create_document_group_memberships_table::Migration),
            Box::new(m20260222_000001_create_conversation_documents_table::Migration),
            Box::new(m20260328_000001_create_agent_users_table::Migration),
            Box::new(m20260328_000002_create_conversation_users_table::Migration),
        ]
    }
}
