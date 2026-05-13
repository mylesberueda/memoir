//! Stripe test client for integration tests
//!
//! Provides helpers to create real Stripe resources (customers, subscriptions)
//! in test mode, which trigger webhook events through the Stripe CLI.

use std::collections::HashMap;
use stripe::Client as StripeClient;
use stripe_billing::subscription::{CancelSubscription, CreateSubscription, CreateSubscriptionItems};
use stripe_core::customer::{CreateCustomer, CreateCustomerInvoiceSettings, DeleteCustomer};
use stripe_shared::{CustomerId, PaymentMethodId, SubscriptionId};

/// Stripe's pre-defined test PaymentMethod for Visa cards.
/// See: https://docs.stripe.com/testing#payment-methods
pub const TEST_PAYMENT_METHOD_VISA: &str = "pm_card_visa";

/// A helper client for creating Stripe resources in integration tests.
///
/// Resources created through this client will trigger real webhook events
/// that flow through the Stripe CLI to your local webhook endpoint.
#[derive(Clone)]
pub struct StripeTestClient {
    client: StripeClient,
    secret_key: String,
    price_id_pro: String,
    price_id_plus: String,
    price_id_enterprise: String,
}

/// Tracks Stripe resources created during a test for cleanup
#[derive(Default)]
pub struct StripeTestResources {
    pub customers: Vec<CustomerId>,
    pub subscriptions: Vec<SubscriptionId>,
}

impl StripeTestClient {
    /// Create a new Stripe test client from environment variables
    pub fn from_env() -> Self {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        Self {
            client: StripeClient::new(secret_key.clone()),
            secret_key,
            price_id_pro: std::env::var("STRIPE_PRICE_ID_PRO").expect("STRIPE_PRICE_ID_PRO must be set"),
            price_id_plus: std::env::var("STRIPE_PRICE_ID_PLUS").expect("STRIPE_PRICE_ID_PLUS must be set"),
            price_id_enterprise: std::env::var("STRIPE_PRICE_ID_ENTERPRISE")
                .expect("STRIPE_PRICE_ID_ENTERPRISE must be set"),
        }
    }

    /// Fetch a subscription as raw JSON via Stripe's REST API.
    ///
    /// Returns the on-the-wire JSON shape Stripe would send in a webhook's
    /// `data.object` field. Used by integration tests to wrap real Stripe
    /// payloads in a webhook envelope and POST them through our handler.
    pub async fn fetch_subscription_json(
        &self,
        subscription_id: &SubscriptionId,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{subscription_id}");
        reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.secret_key)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await
    }

    /// Get the Pro tier price ID
    pub fn price_id_pro(&self) -> &str {
        &self.price_id_pro
    }

    /// Get the Plus tier price ID
    pub fn price_id_plus(&self) -> &str {
        &self.price_id_plus
    }

    /// Get the Enterprise tier price ID
    pub fn price_id_enterprise(&self) -> &str {
        &self.price_id_enterprise
    }

    /// Create a test customer with email and the test payment method attached.
    ///
    /// Uses Stripe's `pm_card_visa` test token during customer creation. Stripe
    /// converts this token into a real PaymentMethod and attaches it to the customer.
    /// We extract and return the real PaymentMethod ID from the response.
    ///
    /// See: https://docs.stripe.com/testing#payment-methods
    pub async fn create_customer_with_payment_method(
        &self,
        email: &str,
    ) -> Result<(CustomerId, PaymentMethodId), stripe::StripeError> {
        let customer = CreateCustomer::new()
            .email(email)
            .name("Integration Test Customer")
            .payment_method(TEST_PAYMENT_METHOD_VISA)
            .invoice_settings(CreateCustomerInvoiceSettings {
                default_payment_method: Some(TEST_PAYMENT_METHOD_VISA.to_string()),
                custom_fields: None,
                footer: None,
                rendering_options: None,
            })
            .send(&self.client)
            .await?;

        // Extract the real PaymentMethod ID from the customer's invoice settings.
        // Stripe converts `pm_card_visa` into a real PaymentMethod (e.g., `pm_1Abc...`)
        // and stores it in invoice_settings.default_payment_method.
        let payment_method_id = customer
            .invoice_settings
            .as_ref()
            .and_then(|settings| settings.default_payment_method.as_ref())
            .map(|pm| pm.id().clone())
            .expect("Customer should have default_payment_method set after creation with pm_card_visa");

        Ok((customer.id, payment_method_id))
    }

    /// Create a subscription for a customer with the given price ID and org metadata.
    ///
    /// The `org_pid` is stored in subscription metadata, which the webhook handler
    /// uses to associate the subscription with our internal organization.
    ///
    /// This will trigger `customer.subscription.created` webhook event.
    pub async fn create_subscription(
        &self,
        customer_id: &CustomerId,
        payment_method_id: &PaymentMethodId,
        price_id: &str,
        org_pid: &str,
    ) -> Result<SubscriptionId, stripe::StripeError> {
        let mut metadata = HashMap::new();
        metadata.insert("org_pid".to_string(), org_pid.to_string());

        let subscription = CreateSubscription::new()
            .customer(customer_id.as_str())
            .default_payment_method(payment_method_id.as_str())
            .items(vec![CreateSubscriptionItems {
                price: Some(price_id.to_string()),
                quantity: Some(1),
                ..Default::default()
            }])
            .metadata(metadata)
            .send(&self.client)
            .await?;

        Ok(subscription.id)
    }

    /// Cancel a subscription immediately.
    ///
    /// This will trigger `customer.subscription.deleted` webhook event.
    pub async fn cancel_subscription(&self, subscription_id: &SubscriptionId) -> Result<(), stripe::StripeError> {
        CancelSubscription::new(subscription_id.clone())
            .send(&self.client)
            .await?;

        Ok(())
    }

    /// Delete a customer (for cleanup)
    pub async fn delete_customer(&self, customer_id: &CustomerId) -> Result<(), stripe::StripeError> {
        DeleteCustomer::new(customer_id.clone()).send(&self.client).await?;

        Ok(())
    }

    /// Clean up all tracked resources
    pub async fn cleanup(&self, resources: &StripeTestResources) {
        // Cancel subscriptions first
        for sub_id in &resources.subscriptions {
            if let Err(e) = self.cancel_subscription(sub_id).await {
                tracing::warn!(subscription_id = %sub_id, error = %e, "Failed to cancel test subscription");
            }
        }

        // Then delete customers
        for customer_id in &resources.customers {
            if let Err(e) = self.delete_customer(customer_id).await {
                tracing::warn!(customer_id = %customer_id, error = %e, "Failed to delete test customer");
            }
        }
    }
}

impl StripeTestResources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track_customer(&mut self, customer_id: CustomerId) {
        self.customers.push(customer_id);
    }

    pub fn track_subscription(&mut self, subscription_id: SubscriptionId) {
        self.subscriptions.push(subscription_id);
    }
}
