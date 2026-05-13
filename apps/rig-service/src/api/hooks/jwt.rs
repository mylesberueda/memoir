//! Auth layer lifecycle hook.

use super::{Hooks, HooksError};

use platform_rs::middleware::auth::{AuthConfig, AuthLayer, ZitadelUserExtractor};

const DEV_ZITADEL_URL: &str = "http://localhost:5150";
const JWKS_REFRESH_TIMEOUT: u64 = 86400;

pub(crate) struct Auth {}

impl Auth {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn init() -> Result<AuthLayer<ZitadelUserExtractor>, HooksError> {
        Self::new().on_startup().await
    }
}

impl Hooks<AuthLayer<ZitadelUserExtractor>> for Auth {
    async fn on_startup(&self) -> Result<AuthLayer<ZitadelUserExtractor>, HooksError> {
        let jwks_url = std::env::var("ZITADEL_JWKS_URL").unwrap_or_else(|_| {
            format!(
                "{}/oauth/v2/keys",
                std::env::var("ZITADEL_URL").unwrap_or_else(|_| DEV_ZITADEL_URL.to_string())
            )
        });

        let zitadel_url = std::env::var("ZITADEL_ISSUER")
            .or_else(|_| std::env::var("ZITADEL_URL"))
            .unwrap_or_else(|_| DEV_ZITADEL_URL.to_string());

        let audience = std::env::var("ZITADEL_AUDIENCE")
            .map_err(|_| HooksError::Config("ZITADEL_AUDIENCE must be set for JWT validation".to_string()))?;

        tracing::info!("Initializing auth layer with JWKS URL: {jwks_url}");

        let auth_config = AuthConfig {
            jwks_url,
            issuer: zitadel_url,
            audience,
        };

        let auth_layer = AuthLayer::<ZitadelUserExtractor>::new(&auth_config)
            .await
            .map_err(|e| HooksError::Connection(format!("Failed to initialize auth layer: {e}")))?;

        auth_layer.start_key_refresh(JWKS_REFRESH_TIMEOUT);
        tracing::info!("Started JWT key refresh task");

        Ok(auth_layer)
    }
}
