use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use super::m20250122_000001_create_channels::Channels;
use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "profanity_filter_level")]
pub enum ProfanityFilterLevel {
    #[sea_orm(string_value = "low")]
    Low,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "high")]
    High,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "auto_mod_action")]
pub enum AutoModAction {
    #[sea_orm(string_value = "flag")]
    Flag,
    #[sea_orm(string_value = "delete")]
    Delete,
    #[sea_orm(string_value = "mute")]
    Mute,
    #[sea_orm(string_value = "warn")]
    Warn,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<ProfanityFilterLevel>() {
            manager.create_type(stmt).await?;
        }
        if let Some(stmt) = schema.create_enum_from_active_enum::<AutoModAction>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(AutoModSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AutoModSettings::ChannelId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::Enabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Spam detection
                    .col(
                        ColumnDef::new(AutoModSettings::SpamFilterEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::MaxMessagesPerMinute)
                            .integer()
                            .not_null()
                            .default(10),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::MaxDuplicateMessages)
                            .integer()
                            .not_null()
                            .default(3),
                    )
                    // Content filtering
                    .col(
                        ColumnDef::new(AutoModSettings::ProfanityFilterEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::ProfanityFilterLevel)
                            .custom(ProfanityFilterLevel::name())
                            .not_null()
                            .default("medium"),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::CustomBlockedWords)
                            .array(ColumnType::String(StringLen::None))
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::CustomAllowedWords)
                            .array(ColumnType::String(StringLen::None))
                            .not_null()
                            .default("{}"),
                    )
                    // Link filtering
                    .col(
                        ColumnDef::new(AutoModSettings::LinkFilterEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::AllowedDomains)
                            .array(ColumnType::String(StringLen::None))
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::BlockAllLinks)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Mention limits
                    .col(
                        ColumnDef::new(AutoModSettings::MaxMentionsPerMessage)
                            .integer()
                            .not_null()
                            .default(5),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::BlockEveryoneMentions)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::BlockRoleMentions)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Default action
                    .col(
                        ColumnDef::new(AutoModSettings::DefaultAction)
                            .custom(AutoModAction::name())
                            .not_null()
                            .default("flag"),
                    )
                    .col(
                        ColumnDef::new(AutoModSettings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_auto_mod_settings_channel_id")
                            .from(AutoModSettings::Table, AutoModSettings::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, AutoModSettings::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AutoModSettings::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AutoModAction::name()).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(ProfanityFilterLevel::name()).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
#[allow(clippy::enum_variant_names)]
pub enum AutoModSettings {
    Table,
    ChannelId,
    Enabled,
    SpamFilterEnabled,
    MaxMessagesPerMinute,
    MaxDuplicateMessages,
    ProfanityFilterEnabled,
    ProfanityFilterLevel,
    CustomBlockedWords,
    CustomAllowedWords,
    LinkFilterEnabled,
    AllowedDomains,
    BlockAllLinks,
    MaxMentionsPerMessage,
    BlockEveryoneMentions,
    BlockRoleMentions,
    DefaultAction,
    UpdatedAt,
}
