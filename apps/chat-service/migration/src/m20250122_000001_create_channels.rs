use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "channel_type")]
pub enum ChannelType {
    #[sea_orm(string_value = "global")]
    Global,
    #[sea_orm(string_value = "organization")]
    Organization,
    #[sea_orm(string_value = "private")]
    Private,
    #[sea_orm(string_value = "system")]
    System,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create channel_type enum from ActiveEnum
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<ChannelType>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channels::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Channels::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    .col(ColumnDef::new(Channels::Name).string().not_null())
                    .col(
                        ColumnDef::new(Channels::ChannelType)
                            .custom(ChannelType::name())
                            .not_null()
                            .default("global"),
                    )
                    .col(ColumnDef::new(Channels::Description).string())
                    .col(ColumnDef::new(Channels::OrganizationPid).string())
                    .col(ColumnDef::new(Channels::IconUrl).string())
                    .col(ColumnDef::new(Channels::Archived).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(Channels::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Channels::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, Channels::Table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_channels_organization_pid")
                    .table(Channels::Table)
                    .col(Channels::OrganizationPid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_channels_type")
                    .table(Channels::Table)
                    .col(Channels::ChannelType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_channels_archived")
                    .table(Channels::Table)
                    .col(Channels::Archived)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ChannelType::name()).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Channels {
    Table,
    Id,
    Pid,
    Name,
    ChannelType,
    Description,
    OrganizationPid,
    IconUrl,
    Archived,
    CreatedAt,
    UpdatedAt,
}
