use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "channel_role")]
pub enum ChannelRole {
    #[sea_orm(string_value = "member")]
    Member,
    #[sea_orm(string_value = "moderator")]
    Moderator,
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "owner")]
    Owner,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create channel_role enum
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<ChannelRole>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(ChannelMembers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ChannelMembers::ChannelId).string().not_null())
                    .col(ColumnDef::new(ChannelMembers::UserId).string().not_null())
                    .col(
                        ColumnDef::new(ChannelMembers::Role)
                            .custom(ChannelRole::name())
                            .not_null()
                            .default("member"),
                    )
                    .col(
                        ColumnDef::new(ChannelMembers::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(ChannelMembers::ChannelId)
                            .col(ChannelMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_members_channel_id")
                            .from(ChannelMembers::Table, ChannelMembers::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for finding user's channels
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_members_user_id")
                    .table(ChannelMembers::Table)
                    .col(ChannelMembers::UserId)
                    .to_owned(),
            )
            .await?;

        // Index for finding members by role
        manager
            .create_index(
                Index::create()
                    .name("idx_channel_members_role")
                    .table(ChannelMembers::Table)
                    .col(ChannelMembers::ChannelId)
                    .col(ChannelMembers::Role)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChannelMembers::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ChannelRole::name()).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ChannelMembers {
    Table,
    ChannelId,
    UserId,
    Role,
    JoinedAt,
}
