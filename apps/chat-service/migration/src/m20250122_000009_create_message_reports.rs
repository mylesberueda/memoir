use sea_orm::{ActiveEnum, DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

use crate::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20250122_000001_create_channels::Channels;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_reason")]
pub enum ReportReason {
    #[sea_orm(string_value = "spam")]
    Spam,
    #[sea_orm(string_value = "harassment")]
    Harassment,
    #[sea_orm(string_value = "hate_speech")]
    HateSpeech,
    #[sea_orm(string_value = "violence")]
    Violence,
    #[sea_orm(string_value = "nsfw")]
    Nsfw,
    #[sea_orm(string_value = "misinformation")]
    Misinformation,
    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_status")]
pub enum ReportStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "reviewing")]
    Reviewing,
    #[sea_orm(string_value = "resolved")]
    Resolved,
    #[sea_orm(string_value = "dismissed")]
    Dismissed,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_type")]
pub enum ReportType {
    #[sea_orm(string_value = "message")]
    Message,
    #[sea_orm(string_value = "user")]
    User,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_resolution")]
pub enum ReportResolution {
    #[sea_orm(string_value = "no_action")]
    NoAction,
    #[sea_orm(string_value = "warning_issued")]
    WarningIssued,
    #[sea_orm(string_value = "content_removed")]
    ContentRemoved,
    #[sea_orm(string_value = "user_muted")]
    UserMuted,
    #[sea_orm(string_value = "user_banned")]
    UserBanned,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create enums
        let schema = sea_orm::Schema::new(sea_orm::DbBackend::Postgres);
        if let Some(stmt) = schema.create_enum_from_active_enum::<ReportReason>() {
            manager.create_type(stmt).await?;
        }
        if let Some(stmt) = schema.create_enum_from_active_enum::<ReportStatus>() {
            manager.create_type(stmt).await?;
        }
        if let Some(stmt) = schema.create_enum_from_active_enum::<ReportType>() {
            manager.create_type(stmt).await?;
        }
        if let Some(stmt) = schema.create_enum_from_active_enum::<ReportResolution>() {
            manager.create_type(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(MessageReports::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MessageReports::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MessageReports::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .extra("DEFAULT nanoid()".to_string()),
                    )
                    .col(
                        ColumnDef::new(MessageReports::ReportType)
                            .custom(ReportType::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(MessageReports::ReporterId).string().not_null())
                    .col(ColumnDef::new(MessageReports::TargetMessagePid).string())
                    .col(ColumnDef::new(MessageReports::TargetUserId).string())
                    .col(ColumnDef::new(MessageReports::ChannelId).string().not_null())
                    .col(
                        ColumnDef::new(MessageReports::Reason)
                            .custom(ReportReason::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(MessageReports::Details).text())
                    .col(
                        ColumnDef::new(MessageReports::Status)
                            .custom(ReportStatus::name())
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(MessageReports::Resolution).custom(ReportResolution::name()))
                    .col(ColumnDef::new(MessageReports::ResolvedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(MessageReports::ResolvedByUserId).string())
                    .col(ColumnDef::new(MessageReports::ResolutionNotes).text())
                    .col(
                        ColumnDef::new(MessageReports::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MessageReports::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_reports_channel_id")
                            .from(MessageReports::Table, MessageReports::ChannelId)
                            .to(Channels::Table, Channels::Pid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, MessageReports::Table).await?;

        // Prevent duplicate reports from same user on same message
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("idx_message_reports_unique_user_message")
                    .table(MessageReports::Table)
                    .col(MessageReports::ReporterId)
                    .col(MessageReports::TargetMessagePid)
                    .to_owned(),
            )
            .await?;

        // Index for finding reports by status
        manager
            .create_index(
                Index::create()
                    .name("idx_message_reports_status")
                    .table(MessageReports::Table)
                    .col(MessageReports::Status)
                    .to_owned(),
            )
            .await?;

        // Index for finding reports by channel
        manager
            .create_index(
                Index::create()
                    .name("idx_message_reports_channel_id")
                    .table(MessageReports::Table)
                    .col(MessageReports::ChannelId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageReports::Table).to_owned())
            .await?;

        manager.drop_type(Type::drop().name(ReportResolution::name()).to_owned()).await?;
        manager.drop_type(Type::drop().name(ReportType::name()).to_owned()).await?;
        manager.drop_type(Type::drop().name(ReportStatus::name()).to_owned()).await?;
        manager.drop_type(Type::drop().name(ReportReason::name()).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum MessageReports {
    Table,
    Id,
    Pid,
    ReportType,
    ReporterId,
    TargetMessagePid,
    TargetUserId,
    ChannelId,
    Reason,
    Details,
    Status,
    Resolution,
    ResolvedAt,
    ResolvedByUserId,
    ResolutionNotes,
    CreatedAt,
    UpdatedAt,
}
