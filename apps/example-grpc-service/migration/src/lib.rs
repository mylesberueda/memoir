pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
mod m20000000_000002_add_updated_at_trigger;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20000000_000001_nanoid::Migration),
        ]
    }
}
