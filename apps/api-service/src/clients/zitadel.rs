use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ZitadelError {
    #[error("Failed to load service account: {0}")]
    ServiceAccountLoad(String),
    #[error("Failed to get access token: {0}")]
    TokenError(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceAccountKey {
    key_id: String,
    key: String,
    user_id: String,
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

struct CachedToken {
    access_token: String,
    expires_at: u64,
}

#[derive(Clone)]
pub(crate) struct ZitadelClient {
    http: Client,
    zitadel_url: String,
    service_account: Arc<ServiceAccountKey>,
    encoding_key: Arc<EncodingKey>,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl ZitadelClient {
    pub(crate) async fn from_env() -> Result<Self, ZitadelError> {
        let zitadel_url = std::env::var("ZITADEL_URL").expect("ZITADEL_URL must be set");
        let service_key_base64 = std::env::var("ZITADEL_PRIVATE_KEY")
            .map_err(|_| ZitadelError::ServiceAccountLoad("ZITADEL_PRIVATE_KEY not set".to_string()))?;

        // Decode base64 to get the JSON string
        let service_key_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &service_key_base64)
            .map_err(|e| ZitadelError::ServiceAccountLoad(format!("Invalid base64: {e}")))?;

        let service_key_json = String::from_utf8(service_key_bytes)
            .map_err(|e| ZitadelError::ServiceAccountLoad(format!("Invalid UTF-8: {e}")))?;

        let service_account: ServiceAccountKey = serde_json::from_str(&service_key_json)
            .map_err(|e| ZitadelError::ServiceAccountLoad(format!("Invalid JSON: {e}")))?;

        let encoding_key = EncodingKey::from_rsa_pem(service_account.key.as_bytes())
            .map_err(|e| ZitadelError::ServiceAccountLoad(format!("Invalid RSA key: {e}")))?;

        Ok(Self {
            http: Client::new(),
            zitadel_url,
            service_account: Arc::new(service_account),
            encoding_key: Arc::new(encoding_key),
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    async fn get_access_token(&self) -> Result<String, ZitadelError> {
        // Check cache first
        {
            let cache = self.cached_token.read().await;
            if let Some(ref token) = *cache {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                // Refresh 60 seconds before expiry
                if token.expires_at > now + 60 {
                    return Ok(token.access_token.clone());
                }
            }
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let claims = JwtClaims {
            iss: self.service_account.user_id.clone(),
            sub: self.service_account.user_id.clone(),
            aud: self.zitadel_url.clone(),
            iat: now,
            exp: now + 3600, // 1 hour
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.service_account.key_id.clone());

        let jwt = jsonwebtoken::encode(&header, &claims, &self.encoding_key)
            .map_err(|e| ZitadelError::TokenError(format!("Failed to encode JWT: {e}")))?;

        let response = self
            .http
            .post(format!("{}/oauth/v2/token", self.zitadel_url))
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("scope", "openid urn:zitadel:iam:org:project:id:zitadel:aud"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| ZitadelError::TokenError(format!("Token request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ZitadelError::TokenError(format!(
                "Token request failed with {status}: {body}"
            )));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| ZitadelError::TokenError(format!("Failed to parse token response: {e}")))?;

        {
            let mut cache = self.cached_token.write().await;
            *cache = Some(CachedToken {
                access_token: token_response.access_token.clone(),
                expires_at: now + token_response.expires_in,
            });
        }

        Ok(token_response.access_token)
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial(zitadel)]
    async fn should_obtain_access_token_from_zitadel() {
        let client = ZitadelClient::from_env().await.expect("Failed to create ZitadelClient");

        let token = client.get_access_token().await;
        assert!(token.is_ok(), "Failed to get access token: {:?}", token.err());

        let token = token.unwrap();
        assert!(!token.is_empty(), "Access token should not be empty");
        // Zitadel tokens are JWTs - they have 3 parts separated by dots
        assert_eq!(token.split('.').count(), 3, "Token should be a JWT");
    }

    #[tokio::test]
    #[serial(zitadel)]
    async fn should_cache_token_across_calls() {
        let client = ZitadelClient::from_env().await.expect("Failed to create ZitadelClient");

        // First call - fetches from Zitadel
        let token1 = client.get_access_token().await.expect("First token fetch failed");

        // Second call - should return cached token
        let token2 = client.get_access_token().await.expect("Second token fetch failed");

        // Same token should be returned (cached)
        assert_eq!(token1, token2, "Cached token should be reused");
    }
}
