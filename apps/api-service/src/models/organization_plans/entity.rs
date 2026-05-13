use super::*;
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, DatabaseConnection, DbErr, EntityTrait as _,
    IntoActiveModel as _, QueryFilter as _,
};

impl Entity {
    pub(crate) async fn upsert(
        db: &DatabaseConnection,
        organization_id: i32,
        tier: Option<PlanTier>,
        stripe_customer_id: Option<&str>,
        stripe_subscription_id: Option<&str>,
        billing_cycle_start: Option<i64>,
        current_period_end: Option<i64>,
    ) -> Result<Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let billing_cycle_dt = billing_cycle_start
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
            .map(|dt| dt.naive_utc());
        let expires_at_dt = current_period_end
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
            .map(|dt| dt.naive_utc());

        let existing = Entity::find()
            .filter(Column::OrganizationId.eq(organization_id))
            .one(db)
            .await?;

        match existing {
            Some(record) => {
                let mut active: ActiveModel = record.into();

                if let Some(t) = tier {
                    active.tier = Set(t.into());
                }
                if let Some(cid) = stripe_customer_id {
                    active.stripe_customer_id = Set(Some(cid.to_string()));
                }
                if let Some(sid) = stripe_subscription_id {
                    active.stripe_subscription_id = Set(Some(sid.to_string()));
                }
                if let Some(dt) = billing_cycle_dt {
                    active.billing_cycle_start = Set(Some(dt));
                }
                if let Some(dt) = expires_at_dt {
                    active.expires_at = Set(Some(dt));
                }
                active.updated_at = Set(now);

                active.update(db).await
            }
            None => {
                let active = ActiveModel {
                    organization_id: Set(organization_id),
                    tier: Set(tier.unwrap_or_default().into()),
                    expires_at: Set(expires_at_dt),
                    stripe_customer_id: Set(stripe_customer_id.map(String::from)),
                    stripe_subscription_id: Set(stripe_subscription_id.map(String::from)),
                    billing_cycle_start: Set(billing_cycle_dt),
                    created_at: Set(now),
                    updated_at: Set(now),
                };

                active.insert(db).await
            }
        }
    }

    pub(crate) async fn revert_to_free(db: &DatabaseConnection, organization_id: i32) -> Result<Model, DbErr> {
        let mut active = Entity::find()
            .filter(Column::OrganizationId.eq(organization_id))
            .one(db)
            .await?
            .ok_or(sea_orm::DbErr::RecordNotFound("organization_plan not found".into()))?
            .into_active_model();

        active.tier = Set(PlanTier::Free.into());
        active.stripe_subscription_id = Set(None);
        active.expires_at = Set(None);
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        let result = active.update(db).await?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use serial_test::serial;
    use test_context::test_context;

    // --- Entity::upsert tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_create_new_plan_when_org_has_none(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("upsert-create").await;

        let result = Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Pro),
            Some("cus_123"),
            Some("sub_456"),
            Some(1704067200), // 2024-01-01 00:00:00 UTC
            None,
        )
        .await;

        assert!(result.is_ok(), "upsert should succeed");
        let plan = result.unwrap();
        assert_eq!(plan.organization_id, org.id);
        assert_eq!(plan.tier, "pro");
        assert_eq!(plan.stripe_customer_id, Some("cus_123".to_string()));
        assert_eq!(plan.stripe_subscription_id, Some("sub_456".to_string()));
        assert!(plan.billing_cycle_start.is_some());

        // Track for cleanup
        ctx.created_organization_plans.push(org.id);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_update_existing_plan_tier(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("upsert-update").await;
        let _initial_plan = ctx.create_organization_plan(org.id, "free").await;

        let result = Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Enterprise),
            None,
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_ok(), "upsert should succeed");
        let plan = result.unwrap();
        assert_eq!(plan.tier, "enterprise");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_only_update_provided_fields(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("upsert-partial").await;

        // Create initial plan with stripe IDs
        Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Pro),
            Some("cus_original"),
            Some("sub_original"),
            None,
            None,
        )
        .await
        .expect("initial upsert should succeed");
        ctx.created_organization_plans.push(org.id);

        // Update only tier, leave other fields as None
        let result = Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Enterprise),
            None, // Should preserve existing
            None, // Should preserve existing
            None,
            None,
        )
        .await;

        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.tier, "enterprise");
        // Stripe IDs should be preserved
        assert_eq!(plan.stripe_customer_id, Some("cus_original".to_string()));
        assert_eq!(plan.stripe_subscription_id, Some("sub_original".to_string()));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_set_default_free_tier_when_none_provided_on_insert(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("upsert-default-tier").await;

        let result = Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            None, // No tier specified
            Some("cus_123"),
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.tier, "free"); // Should default to free

        ctx.created_organization_plans.push(org.id);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_convert_billing_cycle_timestamp_to_datetime(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("upsert-billing-cycle").await;
        let timestamp: i64 = 1704067200; // 2024-01-01 00:00:00 UTC

        let result = Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Pro),
            None,
            None,
            Some(timestamp),
            None,
        )
        .await;

        assert!(result.is_ok());
        let plan = result.unwrap();
        assert!(plan.billing_cycle_start.is_some());

        let stored_dt = plan.billing_cycle_start.unwrap();
        let expected_dt = chrono::DateTime::from_timestamp(timestamp, 0).unwrap().naive_utc();
        assert_eq!(stored_dt, expected_dt);

        ctx.created_organization_plans.push(org.id);
    }

    // --- Entity::revert_to_free tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_revert_paid_plan_to_free(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("revert-paid").await;

        // Create enterprise plan with subscription
        Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Enterprise),
            Some("cus_123"),
            Some("sub_456"),
            Some(1704067200),
            None,
        )
        .await
        .expect("initial upsert should succeed");
        ctx.created_organization_plans.push(org.id);

        let result = Entity::revert_to_free(ctx.context.db.as_ref(), org.id).await;

        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.tier, "free");
        assert_eq!(plan.stripe_subscription_id, None);
        assert_eq!(plan.expires_at, None);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_return_error_when_org_has_no_plan(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("revert-no-plan").await;

        let result = Entity::revert_to_free(ctx.context.db.as_ref(), org.id).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, DbErr::RecordNotFound(_)));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(db)]
    async fn should_preserve_stripe_customer_id_on_revert(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("revert-preserve-customer").await;

        // Create plan with customer ID
        Entity::upsert(
            ctx.context.db.as_ref(),
            org.id,
            Some(PlanTier::Pro),
            Some("cus_keep_this"),
            Some("sub_remove_this"),
            None,
            None,
        )
        .await
        .expect("initial upsert should succeed");
        ctx.created_organization_plans.push(org.id);

        let result = Entity::revert_to_free(ctx.context.db.as_ref(), org.id).await;

        assert!(result.is_ok());
        let plan = result.unwrap();
        // Customer ID preserved for potential re-subscription
        assert_eq!(plan.stripe_customer_id, Some("cus_keep_this".to_string()));
        // Subscription ID cleared
        assert_eq!(plan.stripe_subscription_id, None);
    }
}
