use super::m20000000_000002_add_updated_at_trigger::set_update_on_update;
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
                    .table(OrganizationPlans::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationPlans::OrganizationId)
                            .integer()
                            .not_null()
                            .primary_key()
                            .comment("FK to organizations.id"),
                    )
                    .col(
                        ColumnDef::new(OrganizationPlans::Tier)
                            .string()
                            .not_null()
                            .default("Free")
                            .comment("Plan tier: Free, Pro, Plus, Enterprise"),
                    )
                    .col(ColumnDef::new(OrganizationPlans::ExpiresAt).timestamp())
                    .col(
                        ColumnDef::new(OrganizationPlans::StripeCustomerId)
                            .string_len(255)
                            .comment("Stripe customer ID (cus_...)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationPlans::StripeSubscriptionId)
                            .string_len(255)
                            .comment("Stripe subscription ID (sub_...)"),
                    )
                    .col(
                        ColumnDef::new(OrganizationPlans::BillingCycleStart)
                            .timestamp()
                            .comment("Start of current billing period"),
                    )
                    .col(
                        ColumnDef::new(OrganizationPlans::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrganizationPlans::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organization_plans_org")
                            .from(OrganizationPlans::Table, OrganizationPlans::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_organization_plans_org")
                    .table(OrganizationPlans::Table)
                    .col(OrganizationPlans::OrganizationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_organization_plans_expires_at")
                    .table(OrganizationPlans::Table)
                    .col(OrganizationPlans::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_organization_plans_stripe_customer")
                    .table(OrganizationPlans::Table)
                    .col(OrganizationPlans::StripeCustomerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_organization_plans_stripe_subscription")
                    .table(OrganizationPlans::Table)
                    .col(OrganizationPlans::StripeSubscriptionId)
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        set_update_on_update(db, OrganizationPlans::Table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationPlans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrganizationPlans {
    Table,
    OrganizationId,
    Tier,
    ExpiresAt,
    StripeCustomerId,
    StripeSubscriptionId,
    BillingCycleStart,
    CreatedAt,
    UpdatedAt,
}
