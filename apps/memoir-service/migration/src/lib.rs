pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
pub mod m20000000_000002_add_updated_at_trigger;
mod m20000000_000003_create_users;
mod m20000000_000004_create_api_keys;
mod m20000000_000005_create_bootstrap_tokens;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // Chronological order. Later migrations may depend on earlier ones
        // (e.g., 000003+ depend on the nanoid() function from 000001 and on
        // the trigger helper from 000002).
        vec![
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20000000_000003_create_users::Migration),
            Box::new(m20000000_000004_create_api_keys::Migration),
            Box::new(m20000000_000005_create_bootstrap_tokens::Migration),
        ]
    }
}
