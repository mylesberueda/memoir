use crate::{AppContext, middleware::RequestExt, models::organization_plans};
use proto_rs::api::v1::{
    BillingInterval, CreateCheckoutSessionRequest, CreateCheckoutSessionResponse, CreatePortalSessionRequest,
    CreatePortalSessionResponse, GetCurrentPlanRequest, GetCurrentPlanResponse, GetPricingRequest, GetPricingResponse,
    PricingTier, RedirectType, Tier, billing_service_server,
};
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use stripe::Client as StripeClient;
use stripe_billing::{
    billing_portal_session::{
        CreateBillingPortalSession, CreateBillingPortalSessionFlowData,
        CreateBillingPortalSessionFlowDataSubscriptionUpdate,
        CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm,
        CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems, CreateBillingPortalSessionFlowDataType,
    },
    subscription::RetrieveSubscription,
};
use stripe_checkout::checkout_session::{
    CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionSubscriptionData,
};
use stripe_product::price::RetrievePrice;
use stripe_shared::{CheckoutSessionMode, PriceId};
use tokio::sync::RwLock;
use tonic::{Response, Status};
use tracing::instrument;

/// Cached pricing data with expiration timestamp
struct CachedPricing {
    tiers: Vec<PricingTier>,
    expires_at: u64,
}

/// Cache duration for pricing data (24 hour)
const PRICING_CACHE_DURATION_SECS: u64 = 3600 * 24;

#[derive(Clone)]
pub(crate) struct BillingService {
    context: Arc<AppContext>,
    stripe: StripeClient,
    web_url: String,
    price_ids: PriceIds,
    portal_configuration_id: Option<String>,
    cached_pricing: Arc<RwLock<Option<CachedPricing>>>,
}

impl std::fmt::Debug for BillingService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BillingService")
            .field("db", &"Arc<DatabaseConnection>")
            .field("stripe", &"StripeClient")
            .field("web_url", &self.web_url)
            .field("price_ids", &self.price_ids)
            .field("portal_configuration_id", &self.portal_configuration_id)
            .field("cached_pricing", &"Arc<RwLock<...>>")
            .finish()
    }
}

#[derive(Debug, Clone)]
struct TierPriceIds {
    monthly: String,
    annual: String,
}

#[derive(Debug, Clone)]
struct PriceIds {
    plus: TierPriceIds,
    pro: TierPriceIds,
    enterprise: TierPriceIds,
}

impl BillingService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        let stripe_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let web_url = std::env::var("UI_URL").expect("UI_URL must be set");

        Self {
            context,
            stripe: StripeClient::new(stripe_key),
            web_url,
            price_ids: PriceIds {
                plus: TierPriceIds {
                    monthly: std::env::var("STRIPE_PRICE_ID_PLUS").expect("STRIPE_PRICE_ID_PLUS must be set"),
                    annual: std::env::var("STRIPE_PRICE_ID_PLUS_ANNUAL")
                        .expect("STRIPE_PRICE_ID_PLUS_ANNUAL must be set"),
                },
                pro: TierPriceIds {
                    monthly: std::env::var("STRIPE_PRICE_ID_PRO").expect("STRIPE_PRICE_ID_PRO must be set"),
                    annual: std::env::var("STRIPE_PRICE_ID_PRO_ANNUAL")
                        .expect("STRIPE_PRICE_ID_PRO_ANNUAL must be set"),
                },
                enterprise: TierPriceIds {
                    monthly: std::env::var("STRIPE_PRICE_ID_ENTERPRISE")
                        .expect("STRIPE_PRICE_ID_ENTERPRISE must be set"),
                    annual: std::env::var("STRIPE_PRICE_ID_ENTERPRISE_ANNUAL")
                        .expect("STRIPE_PRICE_ID_ENTERPRISE_ANNUAL must be set"),
                },
            },
            portal_configuration_id: std::env::var("STRIPE_PORTAL_CONFIGURATION_ID").ok(),
            cached_pricing: Arc::new(RwLock::new(None)),
        }
    }

    /// Fetch pricing tiers from Stripe, using cache when available.
    async fn get_pricing_tiers(&self) -> Result<Vec<PricingTier>, Status> {
        // Check cache first
        {
            let cache = self.cached_pricing.read().await;
            if let Some(ref cached) = *cache {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if cached.expires_at > now {
                    tracing::debug!("Returning cached pricing data");
                    return Ok(cached.tiers.clone());
                }
            }
        }

        tracing::info!("Fetching pricing from Stripe");

        // Fetch all prices from Stripe in parallel (monthly and annual for each tier)
        let (plus_monthly, plus_annual, pro_monthly, pro_annual, enterprise_monthly, enterprise_annual) = tokio::try_join!(
            self.fetch_stripe_price(&self.price_ids.plus.monthly),
            self.fetch_stripe_price(&self.price_ids.plus.annual),
            self.fetch_stripe_price(&self.price_ids.pro.monthly),
            self.fetch_stripe_price(&self.price_ids.pro.annual),
            self.fetch_stripe_price(&self.price_ids.enterprise.monthly),
            self.fetch_stripe_price(&self.price_ids.enterprise.annual),
        )?;

        // Build pricing tiers (Free tier doesn't have a Stripe price)
        let tiers = vec![
            PricingTier {
                tier: Tier::Free.into(),
                name: "Free".to_string(),
                price_cents: 0,
                annual_price_cents: 0,
                currency: "usd".to_string(),
                features: vec![
                    "1 Agent".to_string(),
                    "100 messages/month".to_string(),
                    "Community support".to_string(),
                    "Basic analytics".to_string(),
                ],
            },
            self.stripe_prices_to_tier(
                Tier::Plus,
                "Plus",
                plus_monthly,
                plus_annual,
                vec![
                    "5 Agents".to_string(),
                    "1,000 messages/month".to_string(),
                    "Email support".to_string(),
                    "Advanced analytics".to_string(),
                    "Custom tools".to_string(),
                ],
            ),
            self.stripe_prices_to_tier(
                Tier::Pro,
                "Pro",
                pro_monthly,
                pro_annual,
                vec![
                    "Unlimited Agents".to_string(),
                    "10,000 messages/month".to_string(),
                    "Priority support".to_string(),
                    "Advanced analytics".to_string(),
                    "Custom tools".to_string(),
                    "API access".to_string(),
                ],
            ),
            self.stripe_prices_to_tier(
                Tier::Enterprise,
                "Enterprise",
                enterprise_monthly,
                enterprise_annual,
                vec![
                    "Everything in Pro".to_string(),
                    "Unlimited messages".to_string(),
                    "Dedicated support".to_string(),
                    "Custom integrations".to_string(),
                    "SLA guarantee".to_string(),
                    "SSO/SAML".to_string(),
                ],
            ),
        ];

        tracing::debug!(tiers = ?tiers, "prices");

        // Update cache
        {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let mut cache = self.cached_pricing.write().await;
            *cache = Some(CachedPricing {
                tiers: tiers.clone(),
                expires_at: now + PRICING_CACHE_DURATION_SECS,
            });
        }

        Ok(tiers)
    }

    /// Fetch a single price from Stripe by ID.
    async fn fetch_stripe_price(&self, price_id: &str) -> Result<stripe_shared::Price, Status> {
        let price = RetrievePrice::new(PriceId::from(price_id))
            .expand(vec!["tiers".to_string()])
            .send(&self.stripe)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, price_id = %price_id, "Failed to fetch price from Stripe");
                Status::internal("Failed to fetch pricing data")
            })?;

        tracing::trace!(
            price_id = %price_id,
            unit_amount = ?price.unit_amount,
            unit_amount_decimal = ?price.unit_amount_decimal,
            billing_scheme = ?price.billing_scheme,
            tiers = ?price.tiers,
            custom_unit_amount = ?price.custom_unit_amount,
            currency = %price.currency,
            active = price.active,
            "Fetched price from Stripe"
        );

        Ok(price)
    }

    /// Convert Stripe monthly and annual prices to our PricingTier proto.
    fn stripe_prices_to_tier(
        &self,
        tier: Tier,
        name: &str,
        monthly_price: stripe_shared::Price,
        annual_price: stripe_shared::Price,
        features: Vec<String>,
    ) -> PricingTier {
        // Enterprise tier uses -1 to indicate custom pricing
        let (price_cents, annual_price_cents) = if tier == Tier::Enterprise {
            (-1, -1)
        } else {
            (
                Self::extract_price_cents(&monthly_price),
                Self::extract_price_cents(&annual_price),
            )
        };

        PricingTier {
            tier: tier.into(),
            name: name.to_string(),
            price_cents,
            annual_price_cents,
            currency: monthly_price.currency.to_string(),
            features,
        }
    }

    /// Extract price in cents from a Stripe Price object.
    /// Handles both simple pricing and tiered pricing schemes.
    fn extract_price_cents(price: &stripe_shared::Price) -> i32 {
        price
            .unit_amount
            .or_else(|| {
                price
                    .tiers
                    .as_ref()
                    .and_then(|tiers| tiers.first().and_then(|t| t.unit_amount))
            })
            .unwrap_or(0) as i32
    }

    /// Create a portal session for an existing Stripe customer.
    ///
    /// If `subscription_id` is provided, the portal will deep-link to the subscription update page.
    /// If `price_id` is also provided, the portal skips the plan picker and goes straight to the
    /// confirmation page with that price pre-selected.
    async fn create_portal_url(
        &self,
        customer_id: &str,
        subscription_id: Option<&str>,
        price_id: Option<&str>,
    ) -> Result<String, Status> {
        let mut builder = CreateBillingPortalSession::new()
            .customer_account(customer_id)
            .return_url(format!("{}/settings/billing", self.web_url));

        if let Some(config_id) = &self.portal_configuration_id {
            builder = builder.configuration(config_id);
        }

        if let Some(sub_id) = subscription_id {
            if let Some(target_price) = price_id {
                // Fetch subscription to get the item ID for the confirm flow
                let item_id = self.get_subscription_item_id(sub_id).await?;

                let item = CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems {
                    id: item_id,
                    price: Some(target_price.to_string()),
                    quantity: None,
                };
                let confirm = CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm::new(vec![item], sub_id);
                let mut flow_data = CreateBillingPortalSessionFlowData::new(
                    CreateBillingPortalSessionFlowDataType::SubscriptionUpdateConfirm,
                );
                flow_data.subscription_update_confirm = Some(confirm);
                builder = builder.flow_data(flow_data);
            } else {
                // No target price — open generic subscription update page
                let mut flow_data =
                    CreateBillingPortalSessionFlowData::new(CreateBillingPortalSessionFlowDataType::SubscriptionUpdate);
                flow_data.subscription_update = Some(CreateBillingPortalSessionFlowDataSubscriptionUpdate::new(sub_id));
                builder = builder.flow_data(flow_data);
            }
        }

        let portal = builder.send(&self.stripe).await.map_err(|e| {
            tracing::error!(error = %e, "Stripe portal session creation failed");
            Status::internal("Failed to create portal session")
        })?;

        Ok(portal.url)
    }

    async fn get_subscription_item_id(&self, subscription_id: &str) -> Result<String, Status> {
        let sub = RetrieveSubscription::new(subscription_id)
            .send(&self.stripe)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to fetch subscription from Stripe");
                Status::internal("Failed to fetch subscription")
            })?;

        sub.items.data.first().map(|item| item.id.to_string()).ok_or_else(|| {
            tracing::error!(subscription_id = %subscription_id, "Subscription has no items");
            Status::internal("Subscription has no items")
        })
    }
}

#[tonic::async_trait]
impl billing_service_server::BillingService for BillingService {
    #[instrument(skip(self, request), fields(organization_pid))]
    async fn get_current_plan(
        &self,
        request: tonic::Request<GetCurrentPlanRequest>,
    ) -> Result<Response<GetCurrentPlanResponse>, Status> {
        let org_ctx = request.organization_context()?.clone();
        tracing::Span::current().record("organization_pid", &org_ctx.organization_pid);

        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org_ctx.organization_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Database error looking up organization plan");
                Status::internal("Database error")
            })?;

        let (tier, expires_at) = match plan {
            Some(p) => {
                let tier = match p.tier.as_str() {
                    "pro" => Tier::Pro,
                    "plus" => Tier::Plus,
                    "enterprise" => Tier::Enterprise,
                    _ => Tier::Free,
                };
                (tier, p.expires_at.map(|t| t.and_utc().to_rfc3339()))
            }
            None => (Tier::Free, None),
        };

        Ok(Response::new(GetCurrentPlanResponse {
            tier: tier.into(),
            expires_at,
        }))
    }

    #[instrument(skip(self, _request))]
    async fn get_pricing(
        &self,
        _request: tonic::Request<GetPricingRequest>,
    ) -> Result<Response<GetPricingResponse>, Status> {
        let tiers = self.get_pricing_tiers().await?;
        Ok(Response::new(GetPricingResponse { tiers }))
    }

    #[instrument(skip(self, request), fields(organization_pid, tier))]
    async fn create_checkout_session(
        &self,
        request: tonic::Request<CreateCheckoutSessionRequest>,
    ) -> Result<Response<CreateCheckoutSessionResponse>, Status> {
        let user = request.user_context()?.clone();
        let org_ctx = request.organization_context()?.clone();
        tracing::Span::current().record("organization_pid", &org_ctx.organization_pid);
        let req = request.into_inner();
        tracing::Span::current().record("tier", req.tier);

        let tier = Tier::try_from(req.tier).map_err(|_| {
            tracing::error!("invalid tier");
            Status::invalid_argument("Invalid tier.")
        })?;

        let interval = BillingInterval::try_from(req.interval).unwrap_or(BillingInterval::Monthly);

        let price_id = match (tier, interval) {
            (Tier::Pro, BillingInterval::Annual) => &self.price_ids.pro.annual,
            (Tier::Pro, _) => &self.price_ids.pro.monthly,
            (Tier::Plus, BillingInterval::Annual) => &self.price_ids.plus.annual,
            (Tier::Plus, _) => &self.price_ids.plus.monthly,
            (Tier::Enterprise, BillingInterval::Annual) => &self.price_ids.enterprise.annual,
            (Tier::Enterprise, _) => &self.price_ids.enterprise.monthly,
            (Tier::Free | Tier::Unspecified, _) => {
                return Err(tonic::Status::invalid_argument("Cannot checkout to free tier."));
            }
        };

        // Check for existing plan/subscription for this org
        let existing_plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org_ctx.organization_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Database error checking existing plan");
                Status::internal("Database error")
            })?;

        // If org has a Stripe customer ID, send them to portal
        // This handles: active subscription (manage/upgrade) AND cancelled (resubscribe)
        if let Some(ref plan) = existing_plan
            && let Some(ref customer_id) = plan.stripe_customer_id
        {
            tracing::info!(
                customer_id = %customer_id,
                has_subscription = plan.stripe_subscription_id.is_some(),
                "Existing Stripe customer, redirecting to portal"
            );

            let portal_url = self
                .create_portal_url(customer_id, plan.stripe_subscription_id.as_deref(), Some(price_id))
                .await?;

            return Ok(Response::new(CreateCheckoutSessionResponse {
                redirect_url: portal_url,
                redirect_type: RedirectType::Portal.into(),
            }));
        }

        // New customer - create checkout session with org_pid in subscription metadata
        let mut subscription_metadata = HashMap::new();
        subscription_metadata.insert("org_pid".to_string(), org_ctx.organization_pid.clone());

        let session = CreateCheckoutSession::new()
            .mode(CheckoutSessionMode::Subscription)
            .customer_email(&user.email)
            .success_url(format!(
                "{}/dashboard?checkout=success&session_id={{CHECKOUT_SESSION_ID}}",
                self.web_url
            ))
            .cancel_url(format!("{}/dashboard?checkout=cancelled", self.web_url))
            .line_items(vec![CreateCheckoutSessionLineItems {
                price: Some(price_id.clone()),
                quantity: Some(1),
                ..Default::default()
            }])
            .subscription_data(CreateCheckoutSessionSubscriptionData {
                metadata: Some(subscription_metadata),
                ..Default::default()
            })
            .send(&self.stripe)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Stripe checkout session creation failed");
                Status::internal("Failed to create checkout session")
            })?;

        tracing::info!(session_id = ?session.id, "Checkout session created");

        Ok(Response::new(CreateCheckoutSessionResponse {
            redirect_url: session.url.unwrap_or_default(),
            redirect_type: RedirectType::Checkout.into(),
        }))
    }

    #[instrument(skip(self, request), fields(organization_pid))]
    async fn create_portal_session(
        &self,
        request: tonic::Request<CreatePortalSessionRequest>,
    ) -> Result<Response<CreatePortalSessionResponse>, Status> {
        let org_ctx = request.organization_context()?.clone();
        tracing::Span::current().record("organization_pid", &org_ctx.organization_pid);

        // Look up the org's stripe_customer_id from organization_plans
        let plan = organization_plans::Entity::find()
            .filter(organization_plans::Column::OrganizationId.eq(org_ctx.organization_id))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Database error looking up organization plan");
                Status::internal("Database error")
            })?;

        let customer_id = plan.and_then(|p| p.stripe_customer_id).ok_or_else(|| {
            tracing::warn!("Organization has no Stripe customer ID");
            Status::failed_precondition("No active subscription found")
        })?;

        // "Manage Billing" goes to generic portal (no flow_data)
        let portal_url = self.create_portal_url(&customer_id, None, None).await?;

        tracing::info!("Portal session created");

        Ok(Response::new(CreatePortalSessionResponse { portal_url }))
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::middleware::{OrganizationContext, UserContext};
    use crate::models::organization_plans;
    use crate::test_utils::TestContext;
    use proto_rs::api::v1::billing_service_server::BillingService as BillingServiceTrait;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set};
    use serial_test::serial;
    use test_context::test_context;
    use tonic::Request;

    /// Helper to create a request with UserContext + OrganizationContext
    fn billing_request<T>(inner: T, user_id: &str, email: &str, org_pid: &str, org_id: i32) -> Request<T> {
        let mut request = Request::new(inner);
        request.extensions_mut().insert(UserContext {
            id: user_id.to_string(),
            email: email.to_string(),
        });
        request.extensions_mut().insert(OrganizationContext {
            organization_id: org_id,
            organization_pid: org_pid.to_string(),
            user_role: "owner".to_string(),
        });
        request
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_checkout_redirect_for_new_customer(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("checkout-new").await;
        let service = BillingService::new(ctx.context.clone());

        let req = CreateCheckoutSessionRequest {
            tier: Tier::Plus.into(),
            interval: BillingInterval::Monthly.into(),
        };
        let request = billing_request(req, &user.id, &user.email, &org.pid, org.id);

        let result = service.create_checkout_session(request).await;

        assert!(
            result.is_ok(),
            "create_checkout_session should succeed: {:?}",
            result.err()
        );
        let response = result.unwrap().into_inner();

        assert_eq!(
            response.redirect_type,
            RedirectType::Checkout as i32,
            "New customer should be redirected to checkout"
        );
        assert!(
            response.redirect_url.contains("checkout.stripe.com"),
            "Redirect URL should be a Stripe checkout URL: {}",
            response.redirect_url
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_portal_redirect_for_existing_customer_with_subscription(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("checkout-existing-sub").await;

        // Create a real Stripe customer and subscription
        let (customer_id, payment_method_id) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_plus().to_string();
        let subscription_id = ctx
            .create_stripe_subscription(&customer_id, &payment_method_id, &price_id, &org.pid)
            .await;

        // Create org plan with real stripe_customer_id and subscription_id
        let plan = organization_plans::ActiveModel {
            organization_id: Set(org.id),
            tier: Set("plus".to_string()),
            stripe_customer_id: Set(Some(customer_id.to_string())),
            stripe_subscription_id: Set(Some(subscription_id.to_string())),
            ..Default::default()
        };
        plan.insert(ctx.context.db.as_ref())
            .await
            .expect("Failed to create organization plan");
        ctx.created_organization_plans.push(org.id);

        let service = BillingService::new(ctx.context.clone());

        let req = CreateCheckoutSessionRequest {
            tier: Tier::Pro.into(),
            interval: BillingInterval::Monthly.into(),
        };
        let request = billing_request(req, &user.id, &user.email, &org.pid, org.id);

        let result = service.create_checkout_session(request).await;

        assert!(
            result.is_ok(),
            "create_checkout_session should succeed: {:?}",
            result.err()
        );
        let response = result.unwrap().into_inner();

        assert_eq!(
            response.redirect_type,
            RedirectType::Portal as i32,
            "Existing customer with subscription should be redirected to portal"
        );
        assert!(
            response.redirect_url.contains("billing.stripe.com"),
            "Redirect URL should be a Stripe portal URL: {}",
            response.redirect_url
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_portal_redirect_for_cancelled_customer(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("checkout-cancelled").await;

        // Create a real Stripe customer
        let (customer_id, _) = ctx.create_stripe_customer_with_payment_method(&user.email).await;

        // Create org plan with stripe_customer_id but NO subscription_id (cancelled)
        let plan = organization_plans::ActiveModel {
            organization_id: Set(org.id),
            tier: Set("free".to_string()),
            stripe_customer_id: Set(Some(customer_id.to_string())),
            stripe_subscription_id: Set(None), // Cancelled - no active subscription
            ..Default::default()
        };
        plan.insert(ctx.context.db.as_ref())
            .await
            .expect("Failed to create organization plan");
        ctx.created_organization_plans.push(org.id);

        let service = BillingService::new(ctx.context.clone());

        let req = CreateCheckoutSessionRequest {
            tier: Tier::Plus.into(),
            interval: BillingInterval::Monthly.into(),
        };
        let request = billing_request(req, &user.id, &user.email, &org.pid, org.id);

        let result = service.create_checkout_session(request).await;

        assert!(
            result.is_ok(),
            "create_checkout_session should succeed: {:?}",
            result.err()
        );
        let response = result.unwrap().into_inner();

        assert_eq!(
            response.redirect_type,
            RedirectType::Portal as i32,
            "Cancelled customer (has customer_id but no subscription) should be redirected to portal"
        );
        assert!(
            response.redirect_url.contains("billing.stripe.com"),
            "Redirect URL should be a Stripe portal URL: {}",
            response.redirect_url
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_reject_checkout_to_free_tier(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("checkout-free").await;
        let service = BillingService::new(ctx.context.clone());

        let req = CreateCheckoutSessionRequest {
            tier: Tier::Free.into(),
            interval: BillingInterval::Monthly.into(),
        };
        let request = billing_request(req, &user.id, &user.email, &org.pid, org.id);

        let result = service.create_checkout_session(request).await;

        assert!(result.is_err(), "create_checkout_session should fail for free tier");
        let err = result.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
        assert!(err.message().contains("Cannot checkout to free tier"));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_free_tier_for_org_without_plan(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("plan-none").await;
        let service = BillingService::new(ctx.context.clone());

        let request = ctx.organization_request(GetCurrentPlanRequest {}, &user.id, &org.pid, org.id, "owner");
        let result = service.get_current_plan(request).await;

        assert!(result.is_ok(), "get_current_plan should succeed");
        let response = result.unwrap().into_inner();
        assert_eq!(response.tier, Tier::Free as i32);
        assert!(response.expires_at.is_none());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_correct_tier_for_org_with_plan(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("plan-pro").await;
        ctx.create_organization_plan(org.id, "pro").await;

        let service = BillingService::new(ctx.context.clone());

        let request = ctx.organization_request(GetCurrentPlanRequest {}, &user.id, &org.pid, org.id, "owner");
        let result = service.get_current_plan(request).await;

        assert!(result.is_ok(), "get_current_plan should succeed");
        let response = result.unwrap().into_inner();
        assert_eq!(response.tier, Tier::Pro as i32);
    }

    // --- get_pricing tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_return_all_pricing_tiers(ctx: &mut TestContext) {
        let service = BillingService::new(ctx.context.clone());

        let request = Request::new(GetPricingRequest {});
        let result = service.get_pricing(request).await;

        assert!(result.is_ok(), "get_pricing should succeed");
        let response = result.unwrap().into_inner();

        assert_eq!(response.tiers.len(), 4, "Should return 4 pricing tiers");

        let tier_ids: Vec<i32> = response.tiers.iter().map(|t| t.tier).collect();
        assert!(tier_ids.contains(&(Tier::Free as i32)));
        assert!(tier_ids.contains(&(Tier::Plus as i32)));
        assert!(tier_ids.contains(&(Tier::Pro as i32)));
        assert!(tier_ids.contains(&(Tier::Enterprise as i32)));

        let plus_tier = response.tiers.iter().find(|t| t.tier == Tier::Plus as i32).unwrap();
        assert_eq!(plus_tier.price_cents, 2000);
        assert!(plus_tier.annual_price_cents > 0, "Annual price should be set");
        assert_eq!(plus_tier.currency, "usd");
    }

    // --- create_portal_session tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_create_portal_session_for_existing_customer(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("portal-existing").await;

        let (customer_id, _) = ctx.create_stripe_customer_with_payment_method(&user.email).await;

        let plan = organization_plans::ActiveModel {
            organization_id: Set(org.id),
            tier: Set("plus".to_string()),
            stripe_customer_id: Set(Some(customer_id.to_string())),
            ..Default::default()
        };
        plan.insert(ctx.context.db.as_ref())
            .await
            .expect("Failed to create organization plan");
        ctx.created_organization_plans.push(org.id);

        let service = BillingService::new(ctx.context.clone());

        let request = ctx.organization_request(CreatePortalSessionRequest {}, &user.id, &org.pid, org.id, "owner");
        let result = service.create_portal_session(request).await;

        assert!(
            result.is_ok(),
            "create_portal_session should succeed: {:?}",
            result.err()
        );
        let response = result.unwrap().into_inner();

        assert!(
            response.portal_url.contains("billing.stripe.com"),
            "Portal URL should be a Stripe portal URL: {}",
            response.portal_url
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_fail_portal_session_for_org_without_customer_id(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("portal-no-customer").await;
        let service = BillingService::new(ctx.context.clone());

        let request = ctx.organization_request(CreatePortalSessionRequest {}, &user.id, &org.pid, org.id, "owner");
        let result = service.create_portal_session(request).await;

        assert!(result.is_err(), "create_portal_session should fail without customer ID");
        let err = result.unwrap_err();
        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert!(err.message().contains("No active subscription"));
    }

    // --- Missing org context tests ---

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_fail_get_current_plan_without_org_context(ctx: &mut TestContext) {
        let user = ctx.create_user("plan-no-org-ctx").await;
        let service = BillingService::new(ctx.context.clone());

        // Authenticated request WITHOUT OrganizationContext
        let request = ctx.authenticated_request(GetCurrentPlanRequest {}, &user.id);
        let result = service.get_current_plan(request).await;

        assert!(result.is_err(), "get_current_plan should fail without org context");
        let err = result.unwrap_err();
        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert!(err.message().contains("Organization required"));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(billing)]
    async fn should_fail_create_portal_session_without_org_context(ctx: &mut TestContext) {
        let user = ctx.create_user("portal-no-org-ctx").await;
        let service = BillingService::new(ctx.context.clone());

        let request = ctx.authenticated_request(CreatePortalSessionRequest {}, &user.id);
        let result = service.create_portal_session(request).await;

        assert!(result.is_err(), "create_portal_session should fail without org context");
        let err = result.unwrap_err();
        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert!(err.message().contains("Organization required"));
    }
}
