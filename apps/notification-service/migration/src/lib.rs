pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
mod m20000000_000002_add_updated_at_trigger;
mod m20250122_000001_create_notifications;
mod m20250122_000002_create_notification_preferences;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20250122_000001_create_notifications::Migration),
            Box::new(m20250122_000002_create_notification_preferences::Migration),
        ]
    }
}
