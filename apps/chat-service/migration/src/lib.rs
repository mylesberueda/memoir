#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20000000_000001_nanoid;
mod m20000000_000002_add_updated_at_trigger;
mod m20250122_000001_create_channels;
mod m20250122_000002_create_channel_members;
mod m20250122_000003_create_channel_events;
mod m20250122_000004_create_message_reactions;
mod m20250122_000005_create_message_attachments;
mod m20250122_000006_create_read_receipts;
mod m20250122_000007_create_channel_bans;
mod m20250122_000008_create_channel_mutes;
mod m20250122_000009_create_message_reports;
mod m20250122_000010_create_moderation_log;
mod m20250122_000011_create_auto_mod_settings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20000000_000001_nanoid::Migration),
            Box::new(m20000000_000002_add_updated_at_trigger::Migration),
            Box::new(m20250122_000001_create_channels::Migration),
            Box::new(m20250122_000002_create_channel_members::Migration),
            Box::new(m20250122_000003_create_channel_events::Migration),
            Box::new(m20250122_000004_create_message_reactions::Migration),
            Box::new(m20250122_000005_create_message_attachments::Migration),
            Box::new(m20250122_000006_create_read_receipts::Migration),
            Box::new(m20250122_000007_create_channel_bans::Migration),
            Box::new(m20250122_000008_create_channel_mutes::Migration),
            Box::new(m20250122_000009_create_message_reports::Migration),
            Box::new(m20250122_000010_create_moderation_log::Migration),
            Box::new(m20250122_000011_create_auto_mod_settings::Migration),
        ]
    }
}
