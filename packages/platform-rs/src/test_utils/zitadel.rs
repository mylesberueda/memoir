use base64::Engine;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum ZitadelTestError {
    #[error("Failed to load service account: {0}")]
    ServiceAccountLoad(String),
    #[error("Failed to get access token: {0}")]
    TokenError(String),
    #[error("Failed to create user: {0}")]
    CreateUser(String),
    #[error("Failed to delete user: {0}")]
    DeleteUser(String),
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
    #[serde(default)]
    id_token: Option<String>,
    #[serde(default)]
    expires_in: u64,
}

/// Access token + optional ID token pair.
#[derive(Clone, Debug)]
pub struct TokenPair {
    pub access_token: String,
    pub id_token: Option<String>,
}

struct CachedToken {
    access_token: String,
    id_token: Option<String>,
    expires_at: u64,
}

/// Shared Zitadel client for integration tests.
///
/// Provides token exchange (with per-scope caching) and user management
/// (create/delete) for test setup and teardown.
///
/// # Usage
///
/// ```ignore
/// use common_rs::test_utils::ZitadelTestClient;
///
/// let client = ZitadelTestClient::from_env().await.unwrap();
///
/// // Admin token — for Zitadel management API (create/delete users)
/// let admin_token = client.get_admin_access_token().await.unwrap();
///
/// // Project token — for calling your own gRPC services
/// let project_token = client.get_project_access_token().await.unwrap();
///
/// // Create and clean up test users
/// let user_id = client.create_user("test@example.com", "Test", "User", "Password123!").await.unwrap();
/// client.delete_user(&user_id).await.unwrap();
/// ```
#[derive(Clone)]
pub struct ZitadelTestClient {
    http: Client,
    zitadel_url: String,
    service_account: Arc<ServiceAccountKey>,
    encoding_key: Arc<EncodingKey>,
    /// Per-scope token cache (scope string → cached token)
    cached_tokens: Arc<RwLock<HashMap<String, CachedToken>>>,
}

impl ZitadelTestClient {
    /// Create a client from environment variables.
    ///
    /// Reads `ZITADEL_URL` and `ZITADEL_PRIVATE_KEY` (base64-encoded JSON).
    pub async fn from_env() -> Result<Self, ZitadelTestError> {
        let zitadel_url = std::env::var("ZITADEL_URL").expect("ZITADEL_URL must be set");
        let service_key_base64 = std::env::var("ZITADEL_PRIVATE_KEY")
            .map_err(|_| ZitadelTestError::ServiceAccountLoad("ZITADEL_PRIVATE_KEY not set".to_string()))?;

        let service_key_bytes =
            base64::engine::general_purpose::STANDARD
                .decode(&service_key_base64)
                .map_err(|e| ZitadelTestError::ServiceAccountLoad(format!("Invalid base64: {e}")))?;

        let service_key_json = String::from_utf8(service_key_bytes)
            .map_err(|e| ZitadelTestError::ServiceAccountLoad(format!("Invalid UTF-8: {e}")))?;

        let service_account: ServiceAccountKey = serde_json::from_str(&service_key_json)
            .map_err(|e| ZitadelTestError::ServiceAccountLoad(format!("Invalid JSON: {e}")))?;

        let encoding_key = EncodingKey::from_rsa_pem(service_account.key.as_bytes())
            .map_err(|e| ZitadelTestError::ServiceAccountLoad(format!("Invalid RSA key: {e}")))?;

        Ok(Self {
            http: Client::new(),
            zitadel_url,
            service_account: Arc::new(service_account),
            encoding_key: Arc::new(encoding_key),
            cached_tokens: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Exchange a self-signed JWT assertion for a token pair (access + ID token) with the given scope.
    ///
    /// Tokens are cached per-scope and refreshed 60 seconds before expiry.
    pub async fn get_token_pair(&self, scope: &str) -> Result<TokenPair, ZitadelTestError> {
        // Check cache
        {
            let cache = self.cached_tokens.read().await;
            if let Some(token) = cache.get(scope) {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if token.expires_at > now + 60 {
                    return Ok(TokenPair {
                        access_token: token.access_token.clone(),
                        id_token: token.id_token.clone(),
                    });
                }
            }
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let claims = JwtClaims {
            iss: self.service_account.user_id.clone(),
            sub: self.service_account.user_id.clone(),
            aud: self.zitadel_url.clone(),
            iat: now,
            exp: now + 3600,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.service_account.key_id.clone());

        let jwt = jsonwebtoken::encode(&header, &claims, &self.encoding_key)
            .map_err(|e| ZitadelTestError::TokenError(format!("Failed to encode JWT: {e}")))?;

        let response = self
            .http
            .post(format!("{}/oauth/v2/token", self.zitadel_url))
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("scope", scope),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| ZitadelTestError::TokenError(format!("Token request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ZitadelTestError::TokenError(format!(
                "Token request failed with {status}: {body}"
            )));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| ZitadelTestError::TokenError(format!("Failed to parse token response: {e}")))?;

        let expires_at = if token_response.expires_in > 0 {
            now + token_response.expires_in
        } else {
            now + 3600 // default 1 hour if not provided
        };

        {
            let mut cache = self.cached_tokens.write().await;
            cache.insert(
                scope.to_string(),
                CachedToken {
                    access_token: token_response.access_token.clone(),
                    id_token: token_response.id_token.clone(),
                    expires_at,
                },
            );
        }

        Ok(TokenPair {
            access_token: token_response.access_token,
            id_token: token_response.id_token,
        })
    }

    /// Exchange a self-signed JWT assertion for an access token with the given scope.
    ///
    /// Convenience wrapper over `get_token_pair` that discards the ID token.
    pub async fn get_access_token(&self, scope: &str) -> Result<String, ZitadelTestError> {
        self.get_token_pair(scope).await.map(|p| p.access_token)
    }

    /// Get an access token scoped to the Zitadel project (for calling your own services).
    ///
    /// Reads `ZITADEL_PROJECT_ID` from the environment.
    /// Scope: `openid urn:zitadel:iam:org:project:id:{project_id}:aud`
    pub async fn get_project_access_token(&self) -> Result<String, ZitadelTestError> {
        self.get_project_token_pair().await.map(|p| p.access_token)
    }

    /// Get access + ID token pair scoped to the Zitadel project.
    ///
    /// Use when you need to forward both tokens for service-to-service calls
    /// (api-service requires the ID token for email/name extraction).
    pub async fn get_project_token_pair(&self) -> Result<TokenPair, ZitadelTestError> {
        let project_id = std::env::var("ZITADEL_PROJECT_ID").expect("ZITADEL_PROJECT_ID must be set");
        let scope = format!("openid urn:zitadel:iam:org:project:id:{}:aud", project_id);
        self.get_token_pair(&scope).await
    }

    /// Get an access token with admin scope (for Zitadel management API).
    ///
    /// Scope: `openid urn:zitadel:iam:org:project:id:zitadel:aud`
    pub async fn get_admin_access_token(&self) -> Result<String, ZitadelTestError> {
        self.get_access_token("openid urn:zitadel:iam:org:project:id:zitadel:aud")
            .await
    }

    /// Create a human user in Zitadel for testing purposes.
    ///
    /// Returns the Zitadel user ID.
    pub async fn create_user(
        &self,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> Result<String, ZitadelTestError> {
        let token = self.get_admin_access_token().await?;

        let body = serde_json::json!({
            "userName": email,
            "profile": {
                "firstName": first_name,
                "lastName": last_name,
                "displayName": format!("{} {}", first_name, last_name),
            },
            "email": {
                "email": email,
                "isEmailVerified": true,
            },
            "initialPassword": password,
        });

        let response = self
            .http
            .post(format!("{}/management/v1/users/human", self.zitadel_url))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| ZitadelTestError::CreateUser(format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ZitadelTestError::CreateUser(format!(
                "Create user failed with {status}: {body}"
            )));
        }

        #[derive(Deserialize)]
        struct CreateUserResponse {
            #[serde(rename = "userId")]
            user_id: String,
        }

        let result: CreateUserResponse = response
            .json()
            .await
            .map_err(|e| ZitadelTestError::CreateUser(format!("Failed to parse response: {e}")))?;

        tracing::info!(user_id = %result.user_id, email = %email, "Created test user in Zitadel");

        Ok(result.user_id)
    }

    /// Delete a user from Zitadel.
    ///
    /// Used for test cleanup.
    pub async fn delete_user(&self, user_id: &str) -> Result<(), ZitadelTestError> {
        let token = self.get_admin_access_token().await?;

        let response = self
            .http
            .delete(format!("{}/v2/users/{}", self.zitadel_url, user_id))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| ZitadelTestError::DeleteUser(format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ZitadelTestError::DeleteUser(format!(
                "Delete user failed with {status}: {body}"
            )));
        }

        tracing::info!(user_id = %user_id, "Deleted test user from Zitadel");

        Ok(())
    }
}
