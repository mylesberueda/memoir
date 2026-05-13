#![cfg(all(test, feature = "integration"))]

use crate::{
    AppContext,
    clients::zitadel::ZitadelClient,
    middleware::OrganizationContext,
    models::{OrganizationRole, organization_members, organization_plans, organizations, users},
    test_utils::stripe::{StripeTestClient, StripeTestResources},
};
use migration::MigratorTrait as _;
use platform_rs::{middleware::auth::User, test_utils::ZitadelTestClient};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, TransactionTrait};
use std::{str::FromStr, sync::Arc};
use test_context::AsyncTestContext;
use tonic::{Request, metadata::MetadataValue};

/// Test context for integration tests
///
/// Automatically sets up database connection and tracks created test data
/// for cleanup. Use with #[test_context(TestContext)] on test functions.
pub struct TestContext {
    pub context: Arc<AppContext>,
    pub zitadel: Arc<ZitadelClient>,
    pub zitadel_test: Arc<ZitadelTestClient>,
    pub stripe: StripeTestClient,
    pub stripe_resources: StripeTestResources,
    pub created_users: Vec<String>,
    pub created_zitadel_users: Vec<String>,
    pub created_organization_plans: Vec<i32>,
    pub created_organizations: Vec<i32>,
    pub created_organization_members: Vec<i32>,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let context = AppContext::new().await.expect("Failed to load context");

        // Run migrations (idempotent - only applies pending migrations)
        migration::Migrator::up(context.db.as_ref(), None)
            .await
            .expect("Failed to run migrations on test database");

        let zitadel = Arc::new(
            ZitadelClient::from_env()
                .await
                .expect("Failed to initialize Zitadel client for tests"),
        );

        let zitadel_test = Arc::new(
            ZitadelTestClient::from_env()
                .await
                .expect("Failed to initialize Zitadel test client"),
        );

        let stripe = StripeTestClient::from_env();

        Self {
            context,
            zitadel,
            zitadel_test,
            stripe,
            stripe_resources: StripeTestResources::new(),
            created_users: Vec::new(),
            created_zitadel_users: Vec::new(),
            created_organization_plans: Vec::new(),
            created_organizations: Vec::new(),
            created_organization_members: Vec::new(),
        }
    }

    async fn teardown(self) {
        // Clean up in reverse dependency order

        // 0. Stripe resources (external, clean up first)
        self.stripe.cleanup(&self.stripe_resources).await;

        // 0b. Zitadel users (external, clean up before local users)
        for user_id in &self.created_zitadel_users {
            let _ = self.zitadel_test.delete_user(user_id).await;
        }

        // 1. Organization plans (depends on organizations)
        for org_id in &self.created_organization_plans {
            let _ = organization_plans::Entity::delete_by_id(*org_id)
                .exec(self.context.db.as_ref())
                .await;
        }

        // 2. Organization members (depends on organizations and users)
        for member_id in &self.created_organization_members {
            let _ = organization_members::Entity::delete_by_id(*member_id)
                .exec(self.context.db.as_ref())
                .await;
        }

        // 3. Organizations
        for org_id in &self.created_organizations {
            let _ = organizations::Entity::delete_by_id(*org_id)
                .exec(self.context.db.as_ref())
                .await;
        }

        // 4. Users (local database)
        for user_id in &self.created_users {
            let _ = users::Entity::delete_by_id(user_id)
                .exec(self.context.db.as_ref())
                .await;
        }
    }
}

impl TestContext {
    /// Creates a test user in Zitadel and tracks it for cleanup on teardown.
    ///
    /// Returns the Zitadel user ID.
    pub async fn create_zitadel_user(&mut self, suffix: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let email = format!("test-{}-{}@example.com", suffix, timestamp);

        let user_id = self
            .zitadel_test
            .create_user(&email, "Test", suffix, "TestPassword123!")
            .await
            .expect("Failed to create Zitadel test user");

        self.created_zitadel_users.push(user_id.clone());
        user_id
    }

    /// Creates a test user in the local database only.
    ///
    /// This is fast and suitable for most integration tests that don't require
    /// Zitadel integration. The user will be automatically cleaned up on teardown.
    pub async fn create_user(&mut self, suffix: &str) -> users::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let user_id = format!("test-user-{}-{}", suffix, timestamp);

        let settings = proto_rs::api::v1::UserSettings {
            theme: None,
            language: None,
            timezone: None,
            email_notifications: false,
        };
        let settings_json = serde_json::to_value(&settings).expect("Failed to serialize settings");

        let user = users::ActiveModel {
            id: Set(user_id.clone()),
            email: Set(format!("test-{}@example.com", timestamp)),
            display_name: Set(Some(format!("Test User {}", suffix))),
            avatar_url: Set(None),
            bio: Set(None),
            settings: Set(settings_json),
            ..Default::default()
        };

        let user = user
            .insert(self.context.db.as_ref())
            .await
            .expect("Failed to create test user");

        // Track for cleanup
        self.created_users.push(user.id.clone());
        user
    }

    /// Creates a test user with a personal org, mirroring the middleware provisioning path.
    ///
    /// This replicates the behavior of `UserContextMiddleware`: user + org + membership
    /// are created atomically in a transaction. The user is the Owner of their personal org.
    pub async fn create_user_with_personal_org(&mut self, suffix: &str) -> (users::Model, organizations::Model) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let user_id = format!("test-user-{}-{}", suffix, timestamp);
        let email = format!("test-{}@example.com", timestamp);
        let display_name = format!("Test User {}", suffix);

        let settings = proto_rs::api::v1::UserSettings {
            theme: None,
            language: None,
            timezone: None,
            email_notifications: false,
        };
        let settings_json = serde_json::to_value(&settings).expect("Failed to serialize settings");

        let txn = self.context.db.begin().await.expect("Failed to begin transaction");

        let user = users::ActiveModel {
            id: Set(user_id.clone()),
            email: Set(email),
            display_name: Set(Some(display_name.clone())),
            avatar_url: Set(None),
            bio: Set(None),
            settings: Set(settings_json),
            ..Default::default()
        };
        let user = user.insert(&txn).await.expect("Failed to create test user");

        let org = organizations::ActiveModel {
            name: Set(display_name),
            settings: Set(serde_json::json!({})),
            ..Default::default()
        };
        let org = org.insert(&txn).await.expect("Failed to create personal org");

        let member = organization_members::ActiveModel {
            organization_id: Set(org.id),
            user_id: Set(user_id.clone()),
            role: Set(OrganizationRole::Owner.into()),
            ..Default::default()
        };
        member.insert(&txn).await.expect("Failed to create org membership");

        txn.commit().await.expect("Failed to commit transaction");

        self.created_users.push(user.id.clone());
        self.created_organizations.push(org.id);

        (user, org)
    }

    /// Creates a test organization plan for the given organization
    ///
    /// The plan will be automatically cleaned up on teardown.
    pub async fn create_organization_plan(&mut self, organization_id: i32, tier: &str) -> organization_plans::Model {
        let plan = organization_plans::ActiveModel {
            organization_id: Set(organization_id),
            tier: Set(tier.to_string()),
            expires_at: Set(None),
            ..Default::default()
        };

        let plan = plan
            .insert(self.context.db.as_ref())
            .await
            .expect("Failed to create test organization plan");

        // Track for cleanup
        self.created_organization_plans.push(organization_id);
        plan
    }

    /// Creates a test organization with a unique slug
    ///
    /// The organization will be automatically cleaned up on teardown.
    pub async fn create_organization(&mut self, suffix: &str) -> organizations::Model {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let slug = format!("test-org-{}-{}", suffix, timestamp);

        let settings = proto_rs::api::v1::OrganizationSettings {
            branding: None,
            security: None,
            notifications: None,
            locale: None,
            timezone: None,
        };
        let settings_json = serde_json::to_value(&settings).expect("Failed to serialize settings");

        let org = organizations::ActiveModel {
            name: Set(format!("Test Organization {}", suffix)),
            slug: Set(slug),
            settings: Set(settings_json),
            ..Default::default()
        };

        let org = org
            .insert(self.context.db.as_ref())
            .await
            .expect("Failed to create test organization");

        // Track for cleanup
        self.created_organizations.push(org.id);
        org
    }

    /// Creates an organization member relationship
    ///
    /// The membership will be automatically cleaned up on teardown.
    pub async fn create_organization_member(
        &mut self,
        org_id: i32,
        user_id: &str,
        role: &str,
    ) -> organization_members::Model {
        let role_enum = OrganizationRole::from_str(role).expect("Invalid role in test");

        let member = organization_members::ActiveModel {
            organization_id: Set(org_id),
            user_id: Set(user_id.to_string()),
            role: Set(role_enum.into()),
            ..Default::default()
        };

        let member = member
            .insert(self.context.db.as_ref())
            .await
            .expect("Failed to create organization member");

        // Track for cleanup
        self.created_organization_members.push(member.id);
        member
    }

    /// Creates an authenticated tonic::Request with mock User
    ///
    /// # Example
    /// ```
    /// let request = ctx.authenticated_request(MeRequest {}, &user.id);
    /// let response = service.me(request).await?;
    /// ```
    pub fn authenticated_request<T>(&self, inner: T, user_id: &str) -> Request<T> {
        let mut request = Request::new(inner);

        let user = User {
            id: user_id.to_string(),
            email: Some(format!("{}@example.com", user_id)),
            name: Some(format!("Test User {}", user_id)),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        };

        request.extensions_mut().insert(user);
        request
    }

    /// Creates an authenticated request with organization context
    ///
    /// This simulates a request that has passed through the full middleware stack
    /// (Auth → OrganizationId → OrganizationContext). Use this for testing
    /// organization-scoped endpoints.
    ///
    /// # Example
    /// ```
    /// let request = ctx.organization_request(
    ///     GetOrganizationRequest { organization_id: org.pid.clone() },
    ///     &user.id,
    ///     &org.pid,
    ///     org.id,
    ///     "owner"
    /// );
    /// let response = service.get_organization(request).await?;
    /// ```
    pub fn organization_request<T>(
        &self,
        inner: T,
        user_id: &str,
        organization_pid: &str,
        organization_id: i32,
        user_role: &str,
    ) -> Request<T> {
        let mut request = self.authenticated_request(inner, user_id);

        // Add organization context (from OrganizationContextLayer)
        let org_context = OrganizationContext {
            organization_id,
            organization_pid: organization_pid.to_string(),
            user_role: user_role.to_string(),
        };
        request.extensions_mut().insert(org_context);

        // Add x-organization-id header (from OrganizationLayer)
        request.metadata_mut().insert(
            "x-organization-id",
            MetadataValue::try_from(organization_pid).expect("Invalid organization PID"),
        );

        request
    }

    /// Creates a Stripe customer with a payment method attached.
    ///
    /// Uses Stripe's `pm_card_visa` test token during customer creation.
    /// Returns the customer ID and payment method ID. Customer is tracked for cleanup.
    pub async fn create_stripe_customer_with_payment_method(
        &mut self,
        email: &str,
    ) -> (stripe_shared::CustomerId, stripe_shared::PaymentMethodId) {
        let (customer_id, payment_method_id) = self
            .stripe
            .create_customer_with_payment_method(email)
            .await
            .expect("Failed to create Stripe customer with payment method");

        self.stripe_resources.track_customer(customer_id.clone());

        (customer_id, payment_method_id)
    }

    /// Creates a Stripe subscription for an organization.
    ///
    /// This triggers a `customer.subscription.created` webhook event that flows
    /// through the Stripe CLI to your webhook endpoint.
    ///
    /// The subscription is tracked for cleanup.
    pub async fn create_stripe_subscription(
        &mut self,
        customer_id: &stripe_shared::CustomerId,
        payment_method_id: &stripe_shared::PaymentMethodId,
        price_id: &str,
        org_pid: &str,
    ) -> stripe_shared::SubscriptionId {
        let subscription_id = self
            .stripe
            .create_subscription(customer_id, payment_method_id, price_id, org_pid)
            .await
            .expect("Failed to create Stripe subscription");

        self.stripe_resources.track_subscription(subscription_id.clone());

        subscription_id
    }

    /// Cancels a Stripe subscription.
    ///
    /// This triggers a `customer.subscription.deleted` webhook event.
    pub async fn cancel_stripe_subscription(&self, subscription_id: &stripe_shared::SubscriptionId) {
        self.stripe
            .cancel_subscription(subscription_id)
            .await
            .expect("Failed to cancel Stripe subscription");
    }

    /// Fetch a Stripe subscription as raw JSON.
    ///
    /// Returns the on-the-wire shape Stripe sends in a webhook payload's
    /// `data.object` field. Tests use this to wrap real Stripe data in a
    /// webhook envelope and POST it through our handler in-process.
    pub async fn fetch_stripe_subscription_json(
        &self,
        subscription_id: &stripe_shared::SubscriptionId,
    ) -> serde_json::Value {
        self.stripe
            .fetch_subscription_json(subscription_id)
            .await
            .expect("Failed to fetch Stripe subscription as JSON")
    }
}
