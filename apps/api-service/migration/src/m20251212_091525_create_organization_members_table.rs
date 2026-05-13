use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
use super::m20251212_091452_create_users_table::Users;
use super::m20251212_091515_create_organizations_table::Organizations;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationMembers::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::Pid)
                            .string()
                            .not_null()
                            .unique_key()
                            .default(Expr::cust("nanoid()")),
                    )
                    .col(ColumnDef::new(OrganizationMembers::OrganizationId).integer().not_null())
                    .col(
                        ColumnDef::new(OrganizationMembers::UserId)
                            .string()
                            .not_null()
                            .comment("Zitadel user_id"),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::Role)
                            .string()
                            .not_null()
                            .default("Member")
                            .comment("Organization role: Owner, Admin, Member, Guest"),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::Permissions)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb"))
                            .comment("Per-member permission overrides (additive to role defaults)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_members_organization")
                            .from(OrganizationMembers::Table, OrganizationMembers::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_members_user")
                            .from(OrganizationMembers::Table, OrganizationMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_org_members_unique")
                    .table(OrganizationMembers::Table)
                    .col(OrganizationMembers::OrganizationId)
                    .col(OrganizationMembers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_org_members_user")
                    .table(OrganizationMembers::Table)
                    .col(OrganizationMembers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_org_members_organization")
                    .table(OrganizationMembers::Table)
                    .col(OrganizationMembers::OrganizationId)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, OrganizationMembers::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    Id,
    Pid,
    OrganizationId,
    UserId,
    Role,
    Permissions,
    CreatedAt,
    UpdatedAt,
}
