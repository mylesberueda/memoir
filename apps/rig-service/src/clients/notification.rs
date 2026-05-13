use proto_rs::notification::v1::{
    ActionType, NotificationAction, NotificationCategory, NotificationPriority, OriginService, PushRequest,
    PushResponse, notification_service_client::NotificationServiceClient,
};
use std::ops::Deref;
use tonic::transport::Channel;

type InnerClient = NotificationServiceClient<Channel>;

pub(crate) struct NotificationClient {
    client: InnerClient,
}

impl Deref for NotificationClient {
    type Target = InnerClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl NotificationClient {
    /// Create a new notification client with lazy connection.
    pub(crate) fn new(url: &str) -> Result<Self, tonic::transport::Error> {
        let channel = Channel::from_shared(url.to_string())
            .map_err(|e| {
                // InvalidUri doesn't impl Into<tonic::transport::Error>, so we log and panic
                // This is a configuration error that should fail fast at startup
                panic!("Invalid NOTIFICATION_SERVICE_URL: {}", e)
            })?
            .connect_lazy();

        Ok(Self {
            client: NotificationServiceClient::new(channel),
        })
    }

    /// Build a tonic::Request with the user's Bearer token attached.
    fn authenticated_request<T>(&self, inner: T, token: &str) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        let bearer = format!("Bearer {}", token);
        request
            .metadata_mut()
            .insert("authorization", bearer.parse().expect("bearer token is valid ASCII"));
        request
    }

    /// Send a success notification for model sync completion.
    pub(crate) async fn send_model_sync_success(
        &self,
        token: &str,
        user_id: &str,
        org_pid: &str,
        provider_pid: &str,
        provider_name: &str,
        model_count: usize,
    ) -> Result<PushResponse, tonic::Status> {
        let payload = PushRequest {
            user_id: user_id.to_string(),
            org_pid: org_pid.to_string(),
            title: "Models synced".to_string(),
            description: format!("{} models synced for {}", model_count, provider_name),
            icon_url: None,
            category: NotificationCategory::System.into(),
            priority: NotificationPriority::Low.into(),
            actions: vec![NotificationAction {
                kind: ActionType::Navigate.into(),
                target: format!("/providers/{}/models", provider_pid),
                params: Default::default(),
            }],
            origin_service: OriginService::Rig.into(),
            origin_entity_type: None,
            origin_entity_pid: Some(provider_pid.to_string()),
            expires_at: None,
            idempotency_key: None,
        };

        tracing::debug!(user_id = %user_id, provider_pid = %provider_pid, model_count = model_count, "sending model sync success notification");

        let mut client = self.client.clone();
        client
            .push(self.authenticated_request(payload, token))
            .await
            .map(|r| r.into_inner())
    }

    /// Send a failure notification for model sync failure.
    pub(crate) async fn send_model_sync_failure(
        &self,
        token: &str,
        user_id: &str,
        org_pid: &str,
        provider_pid: &str,
        provider_name: &str,
        attempts: u32,
    ) -> Result<PushResponse, tonic::Status> {
        let payload = PushRequest {
            user_id: user_id.to_string(),
            org_pid: org_pid.to_string(),
            title: "Model sync failed".to_string(),
            description: format!(
                "Failed to sync models for {} after {} attempts",
                provider_name, attempts
            ),
            icon_url: None,
            category: NotificationCategory::System.into(),
            priority: NotificationPriority::High.into(),
            actions: vec![NotificationAction {
                kind: ActionType::Navigate.into(),
                target: format!("/providers/{}", provider_pid),
                params: Default::default(),
            }],
            origin_service: OriginService::Rig.into(),
            origin_entity_type: None,
            origin_entity_pid: Some(provider_pid.to_string()),
            expires_at: None,
            idempotency_key: None,
        };

        tracing::debug!(user_id = %user_id, provider_pid = %provider_pid, attempts = attempts, "sending model sync failure notification");

        let mut client = self.client.clone();
        client
            .push(self.authenticated_request(payload, token))
            .await
            .map(|r| r.into_inner())
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use platform_rs::test_utils::ZitadelTestClient;

    fn notification_service_url() -> String {
        std::env::var("NOTIFICATION_SERVICE_URL").expect("NOTIFICATION_SERVICE_URL must be set")
    }

    async fn get_zitadel_access_token() -> String {
        let zitadel = ZitadelTestClient::from_env()
            .await
            .expect("Failed to create ZitadelTestClient");
        zitadel
            .get_project_access_token()
            .await
            .expect("Failed to get project access token")
    }

    #[tokio::test]
    async fn should_create_client_with_valid_url() {
        let url = notification_service_url();
        let client = NotificationClient::new(&url);
        assert!(client.is_ok(), "should create client with valid URL");
    }

    #[tokio::test]
    #[should_panic(expected = "Invalid NOTIFICATION_SERVICE_URL")]
    async fn should_panic_on_invalid_url() {
        // Empty string is invalid
        let _ = NotificationClient::new("");
    }

    #[tokio::test]
    async fn should_send_success_notification() {
        let url = notification_service_url();
        let token = get_zitadel_access_token().await;
        let client = NotificationClient::new(&url).unwrap();

        let response = client
            .send_model_sync_success(&token, "test_user", "org_456", "provider_123", "Test Provider", 5)
            .await
            .expect("push should succeed");

        assert!(
            !response.notification_pid.is_empty(),
            "should return a notification pid"
        );
    }

    #[tokio::test]
    async fn should_send_failure_notification() {
        let url = notification_service_url();
        let token = get_zitadel_access_token().await;
        let client = NotificationClient::new(&url).unwrap();

        let response = client
            .send_model_sync_failure(&token, "test_user", "org_456", "provider_123", "Test Provider", 10)
            .await
            .expect("push should succeed");

        assert!(
            !response.notification_pid.is_empty(),
            "should return a notification pid"
        );
    }

    #[tokio::test]
    async fn should_allow_access_to_inner_client_via_deref() {
        let url = notification_service_url();
        let client = NotificationClient::new(&url).unwrap();

        // Deref allows us to call inner client methods
        let _inner: &NotificationServiceClient<Channel> = &*client;
    }
}
