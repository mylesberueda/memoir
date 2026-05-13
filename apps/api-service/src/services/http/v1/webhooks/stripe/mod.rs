use crate::{REDIS_SERVICE_KEY, models::organization_plans, models::organizations};
use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use fred::{
    clients::Client as RedisClient,
    interfaces::KeysInterface,
    types::{Expiration, SetOptions},
};
use platform_rs::cache::UserCache;
use sea_orm::{ColumnTrait as _, DatabaseConnection, EntityTrait as _, QueryFilter as _};
use std::sync::Arc;
use stripe_webhook::{Event, EventObject, Webhook, WebhookError};

mod ops;

use ops::{StripeOps, SubscriptionChangeError, SubscriptionChangeInput, SubscriptionDeletedInput};

const IDEMPOTENCY_TTL_SECS: i64 = 259200; // 3 days (Stripe's retry window)
const STRIPE_EVENT_KEY_PREFIX: &str = "stripe:evt";

#[derive(Debug, thiserror::Error)]
pub(crate) enum WebhookHandlerError {
    #[error("Missing Stripe-Signature header")]
    MissingSignature,

    #[error("Invalid payload encoding")]
    InvalidPayload,

    #[error("Signature verification failed: {0}")]
    SignatureVerification(#[from] WebhookError),

    #[error("Redis error: {0}")]
    Redis(#[from] fred::error::Error),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Missing org_pid in event metadata")]
    MissingOrgPid,

    #[error("Organization not found for pid in metadata")]
    OrganizationNotFound,
}

impl WebhookHandlerError {
    fn status_code(&self) -> StatusCode {
        match self {
            // Client errors - don't retry
            Self::MissingSignature | Self::InvalidPayload | Self::SignatureVerification(_) => StatusCode::BAD_REQUEST,
            // Server errors - Stripe will retry
            Self::Redis(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Missing metadata or unknown org is our problem, not Stripe's - acknowledge it
            Self::MissingOrgPid | Self::OrganizationNotFound => StatusCode::OK,
        }
    }
}

impl From<SubscriptionChangeError> for WebhookHandlerError {
    fn from(err: SubscriptionChangeError) -> Self {
        match err {
            SubscriptionChangeError::Database(e) => Self::Database(e),
        }
    }
}

#[derive(Clone)]
pub(crate) struct StripeConfig {
    webhook_secret: String,
    price_id_plus: String,
    price_id_plus_annual: String,
    price_id_pro: String,
    price_id_pro_annual: String,
    price_id_enterprise: String,
    price_id_enterprise_annual: String,
}

impl StripeConfig {
    pub(crate) fn from_env() -> Self {
        Self {
            webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").expect("STRIPE_WEBHOOK_SECRET must be set"),
            price_id_plus: std::env::var("STRIPE_PRICE_ID_PLUS").expect("STRIPE_PRICE_ID_PLUS must be set"),
            price_id_plus_annual: std::env::var("STRIPE_PRICE_ID_PLUS_ANNUAL")
                .expect("STRIPE_PRICE_ID_PLUS_ANNUAL must be set"),
            price_id_pro: std::env::var("STRIPE_PRICE_ID_PRO").expect("STRIPE_PRICE_ID_PRO must be set"),
            price_id_pro_annual: std::env::var("STRIPE_PRICE_ID_PRO_ANNUAL")
                .expect("STRIPE_PRICE_ID_PRO_ANNUAL must be set"),
            price_id_enterprise: std::env::var("STRIPE_PRICE_ID_ENTERPRISE")
                .expect("STRIPE_PRICE_ID_ENTERPRISE must be set"),
            price_id_enterprise_annual: std::env::var("STRIPE_PRICE_ID_ENTERPRISE_ANNUAL")
                .expect("STRIPE_PRICE_ID_ENTERPRISE_ANNUAL must be set"),
        }
    }

    fn price_to_tier(&self, price_id: &str) -> organization_plans::PlanTier {
        let tiers = [
            (&self.price_id_enterprise, organization_plans::PlanTier::Enterprise),
            (
                &self.price_id_enterprise_annual,
                organization_plans::PlanTier::Enterprise,
            ),
            (&self.price_id_pro, organization_plans::PlanTier::Pro),
            (&self.price_id_pro_annual, organization_plans::PlanTier::Pro),
            (&self.price_id_plus, organization_plans::PlanTier::Plus),
            (&self.price_id_plus_annual, organization_plans::PlanTier::Plus),
        ];

        tiers
            .into_iter()
            .find(|(id, _)| id.as_str() == price_id)
            .map(|(_, tier)| tier)
            .unwrap_or(organization_plans::PlanTier::Free)
    }
}

#[derive(Clone)]
pub(crate) struct WebhookState {
    pub(crate) db: Arc<DatabaseConnection>,
    pub(crate) redis: Arc<RedisClient>,
    pub(crate) user_cache: UserCache,
    pub(crate) config: StripeConfig,
}

pub(crate) async fn router(db: Arc<DatabaseConnection>, redis: Arc<RedisClient>) -> Router {
    let user_cache = UserCache::new(redis.clone(), REDIS_SERVICE_KEY);
    let state = WebhookState {
        db,
        redis,
        user_cache,
        config: StripeConfig::from_env(),
    };

    Router::new().route("/", post(handle_webhook)).with_state(state)
}

async fn handle_webhook(State(state): State<WebhookState>, headers: HeaderMap, body: Bytes) -> StatusCode {
    match process_webhook(&state, &headers, &body).await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            let status = e.status_code();
            if status == StatusCode::INTERNAL_SERVER_ERROR {
                tracing::error!(error = %e, "Webhook processing failed");
            } else if status == StatusCode::BAD_REQUEST {
                tracing::warn!(error = %e, "Invalid webhook request");
            } else {
                tracing::debug!(error = %e, "Webhook acknowledged with issue");
            }
            status
        }
    }
}

async fn process_webhook(state: &WebhookState, headers: &HeaderMap, body: &Bytes) -> Result<(), WebhookHandlerError> {
    // 1. Extract and verify signature
    let sig_header = headers
        .get("Stripe-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(WebhookHandlerError::MissingSignature)?;

    let payload = std::str::from_utf8(body).map_err(|_| WebhookHandlerError::InvalidPayload)?;

    let event = Webhook::construct_event(payload, sig_header, &state.config.webhook_secret)?;

    tracing::debug!(event_id = %event.id, event_type = ?event.type_, "Received Stripe webhook");

    // 2. Idempotency check - SET NX returns Some("OK") if set, None if key existed
    let key = format!("{REDIS_SERVICE_KEY}:{STRIPE_EVENT_KEY_PREFIX}:{}", event.id);
    let result: Option<String> = state
        .redis
        .set(
            &key,
            "1",
            Some(Expiration::EX(IDEMPOTENCY_TTL_SECS)),
            Some(SetOptions::NX),
            false,
        )
        .await?;

    if result.is_none() {
        tracing::debug!(event_id = %event.id, "Event already processed, skipping");
        return Ok(());
    }

    // 3. Process based on event type
    match &event.data.object {
        EventObject::CheckoutSessionCompleted(session) => handle_checkout_completed(state, &event, session).await,
        EventObject::CustomerSubscriptionCreated(subscription)
        | EventObject::CustomerSubscriptionUpdated(subscription) => {
            handle_subscription_change(state, &event, subscription).await
        }
        EventObject::CustomerSubscriptionDeleted(subscription) => {
            handle_subscription_deleted(state, &event, subscription).await
        }
        EventObject::InvoicePaymentFailed(invoice) => {
            handle_invoice_payment_failed(&event, invoice);
            Ok(())
        }
        _ => {
            tracing::debug!(event_type = ?event.type_, "Ignoring unhandled event type");
            Ok(())
        }
    }
}

async fn handle_checkout_completed(
    _state: &WebhookState,
    event: &Event,
    session: &stripe_shared::CheckoutSession,
) -> Result<(), WebhookHandlerError> {
    // We just log the checkout completion for observability.
    let customer_id = session.customer.as_ref().map(|c| c.id().to_string());
    let subscription_id = session.subscription.as_ref().map(|s| s.id().to_string());

    tracing::info!(
        event_id = %event.id,
        customer_id = ?customer_id,
        subscription_id = ?subscription_id,
        "Checkout completed - awaiting subscription event for tier update"
    );

    Ok(())
}

/// Resolves an org_pid from Stripe metadata to the internal organization ID.
async fn resolve_org_from_metadata(
    db: &DatabaseConnection,
    org_pid: &str,
) -> Result<(i32, String), WebhookHandlerError> {
    let org = organizations::Entity::find()
        .filter(organizations::Column::Pid.eq(org_pid))
        .one(db)
        .await?
        .ok_or_else(|| {
            tracing::warn!(org_pid = %org_pid, "Organization not found for pid in Stripe metadata");
            WebhookHandlerError::OrganizationNotFound
        })?;
    Ok((org.id, org.pid.clone()))
}

async fn handle_subscription_change(
    state: &WebhookState,
    event: &Event,
    subscription: &stripe_billing::Subscription,
) -> Result<(), WebhookHandlerError> {
    let org_pid = extract_org_pid_from_metadata(Some(&subscription.metadata), &event.id, "subscription")?;
    let (org_id, org_pid) = resolve_org_from_metadata(&state.db, &org_pid).await?;

    tracing::info!(
        price_id = ?subscription.items.data.first().map(|item| item.price.id.to_string()),
        items_count = subscription.items.data.len(),
        "Subscription items debug"
    );

    let is_active = matches!(
        subscription.status,
        stripe_billing::SubscriptionStatus::Active | stripe_billing::SubscriptionStatus::Trialing
    );

    let first_item = subscription.items.data.first();

    let input = SubscriptionChangeInput {
        organization_id: org_id,
        customer_id: subscription.customer.id().to_string(),
        subscription_id: subscription.id.to_string(),
        price_id: first_item.map(|item| item.price.id.to_string()),
        is_active,
        billing_cycle_anchor: subscription.billing_cycle_anchor,
        current_period_end: first_item.map(|item| item.current_period_end),
    };

    tracing::info!(
        org_pid = %org_pid,
        event_id = %event.id,
        status = ?subscription.status,
        "Processing subscription change"
    );

    let result = StripeOps::process_subscription_change(&state.db, &state.user_cache, input, |price_id| {
        state.config.price_to_tier(price_id)
    })
    .await?;

    tracing::info!(
        org_pid = %org_pid,
        tier = ?result.tier,
        "Subscription change processed"
    );

    Ok(())
}

async fn handle_subscription_deleted(
    state: &WebhookState,
    event: &Event,
    subscription: &stripe_billing::Subscription,
) -> Result<(), WebhookHandlerError> {
    let org_pid = extract_org_pid_from_metadata(Some(&subscription.metadata), &event.id, "subscription")?;
    let (org_id, org_pid) = resolve_org_from_metadata(&state.db, &org_pid).await?;

    tracing::info!(
        org_pid = %org_pid,
        event_id = %event.id,
        "Processing subscription deletion"
    );

    let input = SubscriptionDeletedInput {
        organization_id: org_id,
    };

    StripeOps::process_subscription_deleted(&state.db, &state.user_cache, input).await?;

    Ok(())
}

fn handle_invoice_payment_failed(event: &Event, invoice: &stripe_shared::Invoice) {
    let customer_id = invoice.customer.as_ref().map(|c| c.id().to_string());
    let subscription_id = invoice.subscription.as_ref().map(|s| s.id().to_string());

    tracing::warn!(
        event_id = %event.id,
        customer_id = ?customer_id,
        subscription_id = ?subscription_id,
        "Invoice payment failed - Stripe will retry or send subscription.deleted"
    );

    // Don't change org plan - Stripe handles retries and will send subscription.deleted if payment ultimately fails
}

fn extract_org_pid_from_metadata(
    metadata: Option<&std::collections::HashMap<String, String>>,
    event_id: &stripe_shared::EventId,
    context: &str,
) -> Result<String, WebhookHandlerError> {
    metadata.and_then(|m| m.get("org_pid").cloned()).ok_or_else(|| {
        tracing::warn!(
            event_id = %event_id,
            context = %context,
            "Missing org_pid in metadata"
        );
        WebhookHandlerError::MissingOrgPid
    })
}

#[cfg(all(test, feature = "unit"))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_config() -> StripeConfig {
        StripeConfig {
            webhook_secret: "whsec_test".to_string(),
            price_id_pro: "price_pro_123".to_string(),
            price_id_pro_annual: "price_pro_annual_123".to_string(),
            price_id_plus: "price_plus_456".to_string(),
            price_id_plus_annual: "price_plus_pro_456".to_string(),
            price_id_enterprise: "price_ent_789".to_string(),
            price_id_enterprise_annual: "price_ent_pro_789".to_string(),
        }
    }

    #[test]
    fn should_map_price_to_pro_tier() {
        let config = test_config();
        assert_eq!(config.price_to_tier("price_pro_123"), organization_plans::PlanTier::Pro);
    }

    #[test]
    fn should_map_price_to_plus_tier() {
        let config = test_config();
        assert_eq!(
            config.price_to_tier("price_plus_456"),
            organization_plans::PlanTier::Plus
        );
    }

    #[test]
    fn should_map_price_to_enterprise_tier() {
        let config = test_config();
        assert_eq!(
            config.price_to_tier("price_ent_789"),
            organization_plans::PlanTier::Enterprise
        );
    }

    #[test]
    fn should_map_unknown_price_to_free_tier() {
        let config = test_config();
        assert_eq!(
            config.price_to_tier("unknown_price"),
            organization_plans::PlanTier::Free
        );
    }

    #[test]
    fn should_return_bad_request_for_missing_signature() {
        assert_eq!(
            WebhookHandlerError::MissingSignature.status_code(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn should_return_bad_request_for_invalid_payload() {
        assert_eq!(
            WebhookHandlerError::InvalidPayload.status_code(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn should_return_internal_error_for_database_error() {
        let err = WebhookHandlerError::Database(sea_orm::DbErr::RecordNotFound("test".into()));
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn should_return_ok_for_missing_org_pid() {
        // MissingOrgPid returns OK so Stripe doesn't retry our config problem
        assert_eq!(WebhookHandlerError::MissingOrgPid.status_code(), StatusCode::OK);
    }

    fn mock_event_id() -> stripe_shared::EventId {
        "evt_test_123".parse().unwrap()
    }

    #[test]
    fn should_extract_org_pid_when_present() {
        let mut metadata = HashMap::new();
        metadata.insert("org_pid".to_string(), "org_abc123".to_string());
        let event_id = mock_event_id();

        let result = extract_org_pid_from_metadata(Some(&metadata), &event_id, "test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "org_abc123");
    }

    #[test]
    fn should_return_error_when_metadata_is_none() {
        let event_id = mock_event_id();

        let result = extract_org_pid_from_metadata(None, &event_id, "test");
        assert!(matches!(result, Err(WebhookHandlerError::MissingOrgPid)));
    }

    #[test]
    fn should_return_error_when_org_pid_key_missing() {
        let metadata = HashMap::new();
        let event_id = mock_event_id();

        let result = extract_org_pid_from_metadata(Some(&metadata), &event_id, "test");
        assert!(matches!(result, Err(WebhookHandlerError::MissingOrgPid)));
    }

    #[test]
    fn should_return_error_when_metadata_has_other_keys_but_not_org_pid() {
        let mut metadata = HashMap::new();
        metadata.insert("customer_id".to_string(), "cus_123".to_string());
        metadata.insert("order_id".to_string(), "ord_456".to_string());
        let event_id = mock_event_id();

        let result = extract_org_pid_from_metadata(Some(&metadata), &event_id, "test");
        assert!(matches!(result, Err(WebhookHandlerError::MissingOrgPid)));
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use fred::{clients::Client as RedisClient, prelude::ClientLike, types::config::Config as RedisConfig};
    use serial_test::serial;
    use tower::ServiceExt;

    fn test_config() -> StripeConfig {
        StripeConfig {
            webhook_secret: "whsec_test_secret".to_string(),
            price_id_pro: "price_pro_123".to_string(),
            price_id_pro_annual: "price_pro_annual_123".to_string(),
            price_id_plus: "price_plus_456".to_string(),
            price_id_plus_annual: "price_plus_annual_456".to_string(),
            price_id_enterprise: "price_ent_789".to_string(),
            price_id_enterprise_annual: "price_ent_annual_789".to_string(),
        }
    }

    async fn setup_test_state() -> WebhookState {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = Arc::new(
            sea_orm::Database::connect(&db_url)
                .await
                .expect("Failed to connect to test database"),
        );

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_config = RedisConfig::from_url(&redis_url).expect("Invalid REDIS_URL");
        let redis = RedisClient::new(redis_config, None, None, None);
        redis.init().await.expect("Failed to connect to Redis");
        let redis = Arc::new(redis);

        let user_cache = UserCache::new(redis.clone(), crate::REDIS_SERVICE_KEY);

        WebhookState {
            db,
            redis,
            user_cache,
            config: test_config(),
        }
    }

    fn create_test_router(state: WebhookState) -> Router {
        Router::new().route("/", post(handle_webhook)).with_state(state)
    }

    #[tokio::test]
    #[serial(stripe)]
    async fn should_reject_request_without_signature_header() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"type": "test"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial(stripe)]
    async fn should_reject_invalid_signature() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("Content-Type", "application/json")
            .header("Stripe-Signature", "t=123,v1=invalid_signature")
            .body(Body::from(r#"{"id": "evt_test", "type": "test"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial(stripe)]
    async fn should_reject_invalid_utf8_payload() {
        let state = setup_test_state().await;
        let app = create_test_router(state);

        // Invalid UTF-8 bytes
        let invalid_bytes: Vec<u8> = vec![0xFF, 0xFE, 0x00, 0x01];

        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("Content-Type", "application/json")
            .header("Stripe-Signature", "t=123,v1=test")
            .body(Body::from(invalid_bytes))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

/// Behavioral integration tests for the Stripe webhook HTTP pipeline.
///
/// These tests drive real-shape Stripe payloads through the full handler
/// (`Webhook::construct_event` → idempotency → `EventObject` dispatch →
/// `handle_subscription_*` → `StripeOps::*` → DB write) against the same
/// `ctx.context.db` the test seeds. The router is mounted in-process via
/// `app.oneshot(request)`; no separately-running api-service or `stripe-cli`
/// container is required, so the tests survive the per-suite test-DB
/// isolation introduced in `chore(services): adds db isolation to integration tests`.
///
/// Payload realism: the inner `Subscription` object is fetched as raw JSON
/// from the live Stripe API after `ctx.create_stripe_subscription`, so the
/// shape is exactly what Stripe sends in production webhooks. The `Event`
/// envelope is constructed in test code following the documented Stripe
/// shape and signed with `stripe_webhook::Webhook::generate_test_header`
/// using a test-only secret.
///
/// What these tests do not cover (by design):
/// - That the deployed `STRIPE_WEBHOOK_SECRET` matches what Stripe is signing
///   with — that's a deploy concern surfaced by Stripe Dashboard delivery
///   metrics, not by `cargo test`.
/// - That the dev `stripe-cli` container is running — dev-loop concern.
#[cfg(all(test, feature = "integration"))]
mod webhook_behavioral_tests {
    use super::*;
    use crate::test_utils::TestContext;
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::{Value, json};
    use serial_test::serial;
    use stripe_webhook::Webhook;
    use test_context::test_context;
    use tower::ServiceExt as _;

    const TEST_WEBHOOK_SECRET: &str = "whsec_behavioral_tests";

    /// Build a `WebhookState` wired to the test's isolated DB and Redis,
    /// with a `StripeConfig` whose price IDs match what `ctx.stripe` uses.
    fn webhook_state(ctx: &TestContext) -> WebhookState {
        let user_cache = UserCache::new(ctx.context.redis.clone(), crate::REDIS_SERVICE_KEY);
        WebhookState {
            db: ctx.context.db.clone(),
            redis: ctx.context.redis.clone(),
            user_cache,
            config: StripeConfig {
                webhook_secret: TEST_WEBHOOK_SECRET.to_string(),
                price_id_plus: ctx.stripe.price_id_plus().to_string(),
                price_id_plus_annual: format!("{}_annual_unused", ctx.stripe.price_id_plus()),
                price_id_pro: ctx.stripe.price_id_pro().to_string(),
                price_id_pro_annual: format!("{}_annual_unused", ctx.stripe.price_id_pro()),
                price_id_enterprise: format!("{}_unused", ctx.stripe.price_id_pro()),
                price_id_enterprise_annual: format!("{}_annual_unused", ctx.stripe.price_id_pro()),
            },
        }
    }

    /// Wrap a Stripe inner-object JSON in an `Event` envelope.
    ///
    /// `event_id` must be unique per test run — the handler uses it as the
    /// Redis idempotency key, so colliding IDs across tests would skip
    /// processing.
    fn build_event_envelope(event_id: &str, event_type: &str, inner_object: Value) -> Value {
        json!({
            "id": event_id,
            "object": "event",
            "api_version": "2024-06-20",
            "created": chrono::Utc::now().timestamp(),
            "livemode": false,
            "pending_webhooks": 1,
            "request": null,
            "type": event_type,
            "data": { "object": inner_object },
        })
    }

    /// Sign `body_json` with `TEST_WEBHOOK_SECRET`, POST to a fresh router
    /// built from `state`, and return the response status.
    async fn post_signed_event(state: WebhookState, body_json: &Value) -> StatusCode {
        let body = serde_json::to_string(body_json).expect("envelope serializes");
        let signature = Webhook::generate_test_header(&body, TEST_WEBHOOK_SECRET, None);

        let app = Router::new().route("/", post(handle_webhook)).with_state(state);

        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header("Content-Type", "application/json")
            .header("Stripe-Signature", &signature)
            .body(Body::from(body))
            .unwrap();

        app.oneshot(request).await.unwrap().status()
    }

    /// Generate a unique event ID for use as the Redis idempotency key.
    fn unique_event_id(prefix: &str) -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("evt_test_{prefix}_{nanos}")
    }

    /// Mutate a fetched subscription JSON to test specific edge cases.
    fn override_status(mut sub: Value, status: &str) -> Value {
        sub["status"] = json!(status);
        sub
    }

    fn override_first_item_price(mut sub: Value, price_id: &str) -> Value {
        sub["items"]["data"][0]["price"]["id"] = json!(price_id);
        sub
    }

    fn fetch_plan(ctx: &TestContext, org_id: i32) -> impl std::future::Future<Output = Option<organization_plans::Model>> + '_ {
        async move {
            organization_plans::Entity::find()
                .filter(organization_plans::Column::OrganizationId.eq(org_id))
                .one(ctx.context.db.as_ref())
                .await
                .expect("plan query failed")
        }
    }

    // ---------------------------------------------------------------------
    // Group 1 — subscription.created → tier upgrades
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_create_pro_plan_on_subscription_created_event(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("pro-create").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let envelope = build_event_envelope(&unique_event_id("pro_create"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK);

        let plan = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(plan.tier, "pro");
        assert_eq!(plan.stripe_customer_id.as_deref(), Some(cust.as_str()));
        assert_eq!(plan.stripe_subscription_id.as_deref(), Some(sub_id.as_str()));
        assert!(plan.billing_cycle_start.is_some(), "billing cycle anchor should be persisted");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_create_plus_plan_on_subscription_created_event(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("plus-create").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_plus().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let envelope = build_event_envelope(&unique_event_id("plus_create"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK);

        let plan = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(plan.tier, "plus");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_set_free_tier_when_price_id_unknown(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("unknown-price").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        // Mutate the price ID to something we don't recognize
        let sub_json = override_first_item_price(
            ctx.fetch_stripe_subscription_json(&sub_id).await,
            "price_unknown_to_us",
        );
        let envelope = build_event_envelope(&unique_event_id("unknown"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK);

        let plan = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(plan.tier, "free");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_set_free_tier_when_subscription_inactive(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("inactive").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        // Override status to a non-active value
        let sub_json = override_status(ctx.fetch_stripe_subscription_json(&sub_id).await, "past_due");
        let envelope = build_event_envelope(&unique_event_id("inactive"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK);

        let plan = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(plan.tier, "free");
    }

    // ---------------------------------------------------------------------
    // Group 2 — subscription.deleted lifecycle
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_clear_subscription_id_and_reset_to_free_on_deleted_event(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("delete-flow").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        // Step 1: deliver the created event so the plan exists
        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let created_envelope =
            build_event_envelope(&unique_event_id("dflow_create"), "customer.subscription.created", sub_json.clone());
        assert_eq!(post_signed_event(webhook_state(ctx), &created_envelope).await, StatusCode::OK);

        let plan_before = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(plan_before.tier, "pro");
        let stored_customer = plan_before.stripe_customer_id.clone();

        // Step 2: deliver the deleted event
        let deleted_envelope =
            build_event_envelope(&unique_event_id("dflow_delete"), "customer.subscription.deleted", sub_json);
        assert_eq!(post_signed_event(webhook_state(ctx), &deleted_envelope).await, StatusCode::OK);

        let plan_after = fetch_plan(ctx, org.id).await.expect("plan should still exist");
        assert_eq!(plan_after.tier, "free");
        assert_eq!(
            plan_after.stripe_customer_id, stored_customer,
            "customer id should be preserved for re-subscription"
        );
        assert!(
            plan_after.stripe_subscription_id.is_none(),
            "subscription id should be cleared"
        );
    }

    // ---------------------------------------------------------------------
    // Group 3 — idempotency through the HTTP path
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_be_idempotent_when_same_event_id_posted_twice(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("idempotent").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org.pid)
            .await;

        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let event_id = unique_event_id("idempotent");
        let envelope = build_event_envelope(&event_id, "customer.subscription.created", sub_json);

        // First delivery: processes and writes
        assert_eq!(post_signed_event(webhook_state(ctx), &envelope).await, StatusCode::OK);
        let plan_after_first = fetch_plan(ctx, org.id).await.expect("plan should exist");
        let updated_at_first = plan_after_first.updated_at;

        // Second delivery with identical event ID: should short-circuit on Redis SET-NX
        assert_eq!(post_signed_event(webhook_state(ctx), &envelope).await, StatusCode::OK);
        let plan_after_second = fetch_plan(ctx, org.id).await.expect("plan should exist");
        assert_eq!(
            plan_after_second.updated_at, updated_at_first,
            "second delivery of same event id should not write to DB"
        );
    }

    // ---------------------------------------------------------------------
    // Group 4 — org isolation
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_only_update_target_org_when_other_orgs_have_plans(ctx: &mut TestContext) {
        let (user_a, org_a) = ctx.create_user_with_personal_org("iso-a").await;
        let (_user_b, org_b) = ctx.create_user_with_personal_org("iso-b").await;
        ctx.created_organization_plans.push(org_a.id);
        ctx.created_organization_plans.push(org_b.id);

        // Pre-seed org B with a Plus plan; the event is for org A and must not touch org B
        ctx.create_organization_plan(org_b.id, "plus").await;

        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user_a.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, &org_a.pid)
            .await;

        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let envelope = build_event_envelope(&unique_event_id("isolation"), "customer.subscription.created", sub_json);
        assert_eq!(post_signed_event(webhook_state(ctx), &envelope).await, StatusCode::OK);

        let plan_a = fetch_plan(ctx, org_a.id).await.expect("org A plan");
        let plan_b = fetch_plan(ctx, org_b.id).await.expect("org B plan");
        assert_eq!(plan_a.tier, "pro", "org A should be upgraded to Pro");
        assert_eq!(plan_b.tier, "plus", "org B must not be touched");
    }

    // ---------------------------------------------------------------------
    // Group 5 — acknowledged-with-issue paths (200 OK, no DB write)
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_acknowledge_with_200_when_org_pid_does_not_match_any_org(ctx: &mut TestContext) {
        let user = ctx.create_user("bad-org-pid").await;
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, "nonexistent_org_pid_xyz")
            .await;

        let sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        let envelope = build_event_envelope(&unique_event_id("bad_pid"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK, "must not retry — Stripe doesn't know about our orgs");

        let plans = organization_plans::Entity::find()
            .filter(organization_plans::Column::StripeCustomerId.eq(cust.to_string()))
            .all(ctx.context.db.as_ref())
            .await
            .expect("query");
        assert!(plans.is_empty(), "no plan should be created for unknown org_pid");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_acknowledge_with_200_when_metadata_missing_org_pid(ctx: &mut TestContext) {
        let (user, _org) = ctx.create_user_with_personal_org("missing-meta").await;
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let price_id = ctx.stripe.price_id_pro().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &price_id, "placeholder")
            .await;

        // Strip org_pid from the fetched payload's metadata
        let mut sub_json = ctx.fetch_stripe_subscription_json(&sub_id).await;
        sub_json["metadata"] = json!({});
        let envelope = build_event_envelope(&unique_event_id("no_meta"), "customer.subscription.created", sub_json);

        let status = post_signed_event(webhook_state(ctx), &envelope).await;
        assert_eq!(status, StatusCode::OK);

        let plans = organization_plans::Entity::find()
            .filter(organization_plans::Column::StripeCustomerId.eq(cust.to_string()))
            .all(ctx.context.db.as_ref())
            .await
            .expect("query");
        assert!(plans.is_empty(), "no plan when metadata is empty");
    }

    // ---------------------------------------------------------------------
    // Group 6 — multi-event sequence
    // ---------------------------------------------------------------------

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial(stripe)]
    async fn should_transition_pro_then_plus_then_free_through_event_sequence(ctx: &mut TestContext) {
        let (user, org) = ctx.create_user_with_personal_org("sequence").await;
        ctx.created_organization_plans.push(org.id);
        let (cust, pm) = ctx.create_stripe_customer_with_payment_method(&user.email).await;
        let pro_price_id = ctx.stripe.price_id_pro().to_string();
        let plus_price_id = ctx.stripe.price_id_plus().to_string();
        let sub_id = ctx
            .create_stripe_subscription(&cust, &pm, &pro_price_id, &org.pid)
            .await;

        let pro_payload = ctx.fetch_stripe_subscription_json(&sub_id).await;

        // 1. Pro create
        let created = build_event_envelope(&unique_event_id("seq_create"), "customer.subscription.created", pro_payload.clone());
        assert_eq!(post_signed_event(webhook_state(ctx), &created).await, StatusCode::OK);
        assert_eq!(fetch_plan(ctx, org.id).await.unwrap().tier, "pro");

        // 2. Update to Plus (mutate the fetched payload's price)
        let plus_payload = override_first_item_price(pro_payload.clone(), &plus_price_id);
        let updated = build_event_envelope(&unique_event_id("seq_update"), "customer.subscription.updated", plus_payload);
        assert_eq!(post_signed_event(webhook_state(ctx), &updated).await, StatusCode::OK);
        assert_eq!(fetch_plan(ctx, org.id).await.unwrap().tier, "plus");

        // 3. Delete
        let deleted = build_event_envelope(&unique_event_id("seq_delete"), "customer.subscription.deleted", pro_payload);
        assert_eq!(post_signed_event(webhook_state(ctx), &deleted).await, StatusCode::OK);
        let final_plan = fetch_plan(ctx, org.id).await.unwrap();
        assert_eq!(final_plan.tier, "free");
        assert!(final_plan.stripe_customer_id.is_some(), "customer id preserved");
        assert!(final_plan.stripe_subscription_id.is_none(), "sub id cleared");
    }
}
