use crate::models::{
    organization_members,
    organization_plans::{self, PlanTier},
    organizations,
};
use platform_rs::cache::UserCache;
use sea_orm::{ColumnTrait as _, DatabaseConnection, EntityTrait as _, QueryFilter as _};

/// Input data for subscription change operations.
///
/// This struct decouples business logic from Stripe SDK types, making the
/// core tier-update logic testable without constructing complex Stripe objects.
#[derive(Debug, Clone)]
pub(crate) struct SubscriptionChangeInput {
    pub organization_id: i32,
    pub customer_id: String,
    pub subscription_id: String,
    pub price_id: Option<String>,
    pub is_active: bool,
    pub billing_cycle_anchor: i64,
    pub current_period_end: Option<i64>,
}

/// Input data for subscription deletion operations.
#[derive(Debug, Clone)]
pub(crate) struct SubscriptionDeletedInput {
    pub organization_id: i32,
}

/// Result of a subscription operation for observability.
#[derive(Debug)]
pub(crate) struct SubscriptionChangeResult {
    pub tier: PlanTier,
}

/// Namespace for Stripe webhook business logic operations.
///
/// These methods contain the core tier-update logic, decoupled from Stripe SDK types.
pub(crate) struct StripeOps;

impl StripeOps {
    /// Processes a subscription change (created or updated).
    ///
    /// This is the core business logic for tier upgrades/downgrades:
    /// 1. Maps price_id to tier using the provided mapper
    /// 2. Updates local database
    /// 3. Updates all org members' caches with the new tier
    ///
    /// The `price_to_tier` closure allows injecting the price mapping logic,
    /// making this testable without environment variables.
    pub async fn process_subscription_change<F>(
        db: &DatabaseConnection,
        user_cache: &UserCache,
        input: SubscriptionChangeInput,
        price_to_tier: F,
    ) -> Result<SubscriptionChangeResult, SubscriptionChangeError>
    where
        F: Fn(&str) -> PlanTier,
    {
        let tier = if input.is_active {
            input.price_id.as_deref().map(&price_to_tier).unwrap_or(PlanTier::Free)
        } else {
            PlanTier::Free
        };

        let _db_result = organization_plans::Entity::upsert(
            db,
            input.organization_id,
            Some(tier),
            Some(&input.customer_id),
            Some(&input.subscription_id),
            Some(input.billing_cycle_anchor),
            input.current_period_end,
        )
        .await
        .map_err(SubscriptionChangeError::Database)?;

        // Update all org members' caches with the new tier
        update_org_tier_in_member_caches(db, user_cache, input.organization_id, tier).await;

        Ok(SubscriptionChangeResult { tier })
    }

    /// Processes a subscription deletion (cancellation). Reverts the org to free tier.
    pub async fn process_subscription_deleted(
        db: &DatabaseConnection,
        user_cache: &UserCache,
        input: SubscriptionDeletedInput,
    ) -> Result<(), SubscriptionChangeError> {
        organization_plans::Entity::revert_to_free(db, input.organization_id)
            .await
            .map_err(SubscriptionChangeError::Database)?;

        // Update all org members' caches to free tier
        update_org_tier_in_member_caches(db, user_cache, input.organization_id, PlanTier::Free).await;

        Ok(())
    }
}

/// Fan-out cache update: find all members of an org and update the org's tier in each user's cache.
async fn update_org_tier_in_member_caches(
    db: &DatabaseConnection,
    user_cache: &UserCache,
    organization_id: i32,
    new_tier: PlanTier,
) {
    // Resolve org PID from the integer ID
    let org_pid = match organizations::Entity::find_by_id(organization_id).one(db).await {
        Ok(Some(org)) => org.pid,
        Ok(None) => {
            tracing::error!(organization_id, "Organization not found for cache update");
            return;
        }
        Err(e) => {
            tracing::error!(error = %e, organization_id, "Failed to look up org for cache update");
            return;
        }
    };

    let members = match organization_members::Entity::find()
        .filter(organization_members::Column::OrganizationId.eq(organization_id))
        .all(db)
        .await
    {
        Ok(m) => m,
        Err(e) => {
            tracing::error!(error = %e, organization_id, "Failed to query org members for cache update");
            return;
        }
    };

    for member in &members {
        if let Some(mut data) = user_cache.get(&member.user_id).await {
            if let Some(org) = data.organizations.iter_mut().find(|o| o.pid == org_pid) {
                org.tier = new_tier;
            }
            user_cache.set(&member.user_id, &data).await;
        }
    }

    tracing::debug!(
        org_pid = %org_pid,
        member_count = members.len(),
        new_tier = %new_tier,
        "Updated org tier in member caches"
    );
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum SubscriptionChangeError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use serial_test::serial;
    use test_context::test_context;

    fn test_price_mapper(price_id: &str) -> PlanTier {
        match price_id {
            "price_pro" => PlanTier::Pro,
            "price_plus" => PlanTier::Plus,
            "price_enterprise" => PlanTier::Enterprise,
            _ => PlanTier::Free,
        }
    }

    fn test_user_cache(ctx: &TestContext) -> UserCache {
        UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY)
    }

    // --- process_subscription_change tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_upgrade_org_to_pro_tier(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("pro").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_123".to_string(),
            subscription_id: "sub_test_456".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok(), "Should succeed: {:?}", result.err());
        let result = result.unwrap();
        assert_eq!(result.tier, PlanTier::Pro);

        // Verify DB state
        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed");

        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.tier, "pro");
        assert_eq!(plan.stripe_customer_id, Some("cus_test_123".to_string()));
        assert_eq!(plan.stripe_subscription_id, Some("sub_test_456".to_string()));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_upgrade_org_to_plus_tier(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("plus").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_plus".to_string(),
            subscription_id: "sub_test_plus".to_string(),
            price_id: Some("price_plus".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Plus);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "plus");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_upgrade_org_to_enterprise_tier(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("enterprise").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_ent".to_string(),
            subscription_id: "sub_test_ent".to_string(),
            price_id: Some("price_enterprise".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Enterprise);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "enterprise");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_set_free_tier_when_subscription_inactive(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("inactive").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_inactive".to_string(),
            subscription_id: "sub_test_inactive".to_string(),
            price_id: Some("price_pro".to_string()), // Pro price but...
            is_active: false,                        // ...subscription is not active
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Free);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "free");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_set_free_tier_when_price_id_unknown(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("unknown-price").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_unknown".to_string(),
            subscription_id: "sub_test_unknown".to_string(),
            price_id: Some("price_unknown_xyz".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Free);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_set_free_tier_when_no_price_id(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("no-price").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_test_no_price".to_string(),
            subscription_id: "sub_test_no_price".to_string(),
            price_id: None,
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Free);
    }

    // --- process_subscription_deleted tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_downgrade_to_free_on_subscription_deleted(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("delete").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        // First create a Pro subscription
        let create_input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_delete_test".to_string(),
            subscription_id: "sub_delete_test".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, create_input, test_price_mapper)
            .await
            .expect("Create should succeed");

        // Verify Pro tier
        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");
        assert_eq!(plan.tier, "pro");

        // Now delete
        let delete_input = SubscriptionDeletedInput {
            organization_id: org.id,
        };

        let result = StripeOps::process_subscription_deleted(ctx.context.db.as_ref(), &cache, delete_input).await;

        assert!(result.is_ok());

        // Verify Free tier and cleared subscription_id
        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "free");
        assert!(plan.stripe_subscription_id.is_none());
        // Customer ID should be preserved
        assert_eq!(plan.stripe_customer_id, Some("cus_delete_test".to_string()));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_fail_deletion_for_nonexistent_org_plan(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("no-plan-delete").await;
        let cache = test_user_cache(ctx);

        let input = SubscriptionDeletedInput {
            organization_id: org.id,
        };

        let result = StripeOps::process_subscription_deleted(ctx.context.db.as_ref(), &cache, input).await;

        // Should fail because organization_plan record doesn't exist (RecordNotFound)
        assert!(result.is_err());
    }

    // --- Billing cycle anchor tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_store_billing_cycle_anchor(ctx: &mut TestContext) {
        let (_user, org) = ctx.create_user_with_personal_org("billing-anchor").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let billing_anchor: i64 = 1704067200; // 2024-01-01 00:00:00 UTC

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_billing_test".to_string(),
            subscription_id: "sub_billing_test".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: billing_anchor,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper)
            .await
            .expect("Should succeed");

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert!(
            plan.billing_cycle_start.is_some(),
            "Billing cycle start should be stored"
        );

        // Verify the timestamp was converted correctly
        let stored_dt = plan.billing_cycle_start.unwrap();
        let expected_dt = chrono::DateTime::from_timestamp(billing_anchor, 0).unwrap().naive_utc();
        assert_eq!(stored_dt, expected_dt, "Billing cycle timestamp should match");
    }

    // --- Subscription update (tier change) tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_update_tier_on_subscription_change(ctx: &mut TestContext) {
        // Simulates an org upgrading from Pro to Enterprise mid-subscription
        let (_user, org) = ctx.create_user_with_personal_org("tier-change").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        // First: create Pro subscription
        let initial_input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_upgrade_test".to_string(),
            subscription_id: "sub_upgrade_test".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, initial_input, test_price_mapper)
            .await
            .expect("Initial subscription should succeed");

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");
        assert_eq!(plan.tier, "pro", "Should start as Pro");

        // Second: upgrade to Enterprise (subscription.updated event)
        let upgrade_input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_upgrade_test".to_string(),
            subscription_id: "sub_upgrade_test".to_string(), // Same subscription
            price_id: Some("price_enterprise".to_string()),  // New price
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, upgrade_input, test_price_mapper)
                .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Enterprise);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "enterprise", "Should be upgraded to Enterprise");
        // Subscription ID should remain the same
        assert_eq!(plan.stripe_subscription_id, Some("sub_upgrade_test".to_string()));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_downgrade_tier_on_subscription_change(ctx: &mut TestContext) {
        // Simulates an org downgrading from Enterprise to Pro
        let (_user, org) = ctx.create_user_with_personal_org("tier-downgrade").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        // First: create Enterprise subscription
        let initial_input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_downgrade_test".to_string(),
            subscription_id: "sub_downgrade_test".to_string(),
            price_id: Some("price_enterprise".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, initial_input, test_price_mapper)
            .await
            .expect("Initial subscription should succeed");

        // Second: downgrade to Pro
        let downgrade_input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_downgrade_test".to_string(),
            subscription_id: "sub_downgrade_test".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        let result =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, downgrade_input, test_price_mapper)
                .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().tier, PlanTier::Pro);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "pro", "Should be downgraded to Pro");
    }

    // --- Retry scenario tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_be_idempotent_on_duplicate_subscription_change(ctx: &mut TestContext) {
        // Simulates Stripe retrying a webhook: same event processed twice should yield same result
        let (_user, org) = ctx.create_user_with_personal_org("idempotent").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        let input = SubscriptionChangeInput {
            organization_id: org.id,

            customer_id: "cus_idempotent".to_string(),
            subscription_id: "sub_idempotent".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        // First call - creates the subscription
        let result1 =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input.clone(), test_price_mapper)
                .await;
        assert!(result1.is_ok(), "First call should succeed");
        assert_eq!(result1.unwrap().tier, PlanTier::Pro);

        // Second call - simulates Stripe retry with same event
        let result2 =
            StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input.clone(), test_price_mapper)
                .await;
        assert!(result2.is_ok(), "Retry should also succeed (idempotent)");
        assert_eq!(result2.unwrap().tier, PlanTier::Pro);

        // Verify final state is correct (not duplicated or corrupted)
        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Plan should exist");

        assert_eq!(plan.tier, "pro");
        assert_eq!(plan.stripe_customer_id, Some("cus_idempotent".to_string()));
        assert_eq!(plan.stripe_subscription_id, Some("sub_idempotent".to_string()));
    }

    // --- Multi-org independence tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_maintain_independent_plans_for_different_orgs(ctx: &mut TestContext) {
        // Two orgs owned by the same user should have completely independent plans
        let (user, org_a) = ctx.create_user_with_personal_org("multi-org").await;

        let org_b = ctx.create_organization("company-org").await;
        ctx.create_organization_member(org_b.id, &user.id, "owner").await;

        ctx.created_organization_plans.push(org_a.id);
        ctx.created_organization_plans.push(org_b.id);
        let cache = test_user_cache(ctx);

        // Upgrade org_a to Pro
        let input_a = SubscriptionChangeInput {
            organization_id: org_a.id,

            customer_id: "cus_org_a".to_string(),
            subscription_id: "sub_org_a".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input_a, test_price_mapper)
            .await
            .expect("Org A upgrade should succeed");

        // Upgrade org_b to Enterprise
        let input_b = SubscriptionChangeInput {
            organization_id: org_b.id,

            customer_id: "cus_org_b".to_string(),
            subscription_id: "sub_org_b".to_string(),
            price_id: Some("price_enterprise".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input_b, test_price_mapper)
            .await
            .expect("Org B upgrade should succeed");

        // Verify each org has its own independent plan
        let plan_a = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org_a.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Org A plan should exist");

        let plan_b = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org_b.id))
            .one(ctx.context.db.as_ref())
            .await
            .expect("DB query failed")
            .expect("Org B plan should exist");

        assert_eq!(plan_a.tier, "pro", "Org A should be Pro");
        assert_eq!(plan_b.tier, "enterprise", "Org B should be Enterprise");
        assert_ne!(
            plan_a.stripe_customer_id, plan_b.stripe_customer_id,
            "Each org should have its own Stripe customer"
        );
    }

    // --- Cache fan-out tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_update_all_members_caches_on_tier_change(ctx: &mut TestContext) {
        use platform_rs::cache::{CachedOrg, CachedUserData};

        let (owner, org) = ctx.create_user_with_personal_org("cache-fanout-owner").await;
        let member = ctx.create_user("cache-fanout-member").await;
        ctx.create_organization_member(org.id, &member.id, "member").await;
        ctx.created_organization_plans.push(org.id);
        let cache = test_user_cache(ctx);

        // Seed both users' caches with the org at free tier
        cache
            .set(
                &owner.id,
                &CachedUserData {
                    email: owner.email.clone(),
                    organizations: vec![CachedOrg::new(
                        org.pid.clone(),
                        PlanTier::Free,
                        platform_rs::cache::OrgRole::Owner,
                    )],
                },
            )
            .await;

        cache
            .set(
                &member.id,
                &CachedUserData {
                    email: member.email.clone(),
                    organizations: vec![CachedOrg::new(
                        org.pid.clone(),
                        PlanTier::Free,
                        platform_rs::cache::OrgRole::Member,
                    )],
                },
            )
            .await;

        // Stripe webhook upgrades org to Pro
        let input = SubscriptionChangeInput {
            organization_id: org.id,
            customer_id: "cus_fanout".to_string(),
            subscription_id: "sub_fanout".to_string(),
            price_id: Some("price_pro".to_string()),
            is_active: true,
            billing_cycle_anchor: 1704067200,
            current_period_end: None,
        };

        StripeOps::process_subscription_change(ctx.context.db.as_ref(), &cache, input, test_price_mapper)
            .await
            .expect("Tier change should succeed");

        // Verify BOTH users' caches were updated to pro
        let owner_cached = cache.get(&owner.id).await.expect("Owner cache should exist");
        assert_eq!(
            owner_cached.org(&org.pid).unwrap().tier,
            PlanTier::Pro,
            "Owner's cache should show pro"
        );

        let member_cached = cache.get(&member.id).await.expect("Member cache should exist");
        assert_eq!(
            member_cached.org(&org.pid).unwrap().tier,
            PlanTier::Pro,
            "Member's cache should show pro"
        );
    }
}
