pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
mod m20000000_000002_add_updated_at_trigger;
mod m20251212_091452_create_users_table;
mod m20251212_091515_create_organizations_table;
mod m20251212_091525_create_organization_members_table;
mod m20251212_091536_create_user_plans_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20251212_091452_create_users_table::Migration),
            Box::new(m20251212_091515_create_organizations_table::Migration),
            Box::new(m20251212_091525_create_organization_members_table::Migration),
            Box::new(m20251212_091536_create_user_plans_table::Migration),
        ]
    }
}
