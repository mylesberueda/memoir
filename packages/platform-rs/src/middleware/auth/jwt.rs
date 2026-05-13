use dashmap::DashMap;
use std::sync::Arc;

use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Jwk {
    kid: String,
    kty: String,
    #[allow(unused)]
    alg: String,
    n: String,
    e: String,
}

#[derive(Debug, Clone, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone)]
pub struct JwtValidator {
    jwks_url: String,
    issuer: String,
    audience: String,
    keys: Arc<DashMap<String, DecodingKey>>,
}

impl JwtValidator {
    pub async fn new(jwks_url: String, issuer: String, audience: String) -> crate::Result<Self> {
        let validator = Self {
            jwks_url,
            issuer,
            audience,
            keys: Arc::new(DashMap::new()),
        };

        validator.refresh_keys().await?;

        Ok(validator)
    }

    async fn refresh_keys(&self) -> crate::Result<()> {
        tracing::debug!(url = %self.jwks_url, "Fetching JWKS keys");
        let res = reqwest::get(&self.jwks_url).await?;
        let jwk_set: JwkSet = res.json().await?;

        self.keys.clear();

        for jwk in jwk_set.keys {
            if jwk.kty == "RSA" {
                tracing::debug!(kid = %jwk.kid, "Loaded RSA key");
                let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;
                self.keys.insert(jwk.kid, decoding_key);
            }
        }

        tracing::debug!(count = self.keys.len(), "JWKS keys loaded");
        Ok(())
    }

    /// Validate an access token and extract authentication claims.
    pub async fn validate(&self, token: &str) -> crate::Result<serde_json::Value> {
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or("Missing kid in JWT header")
            .map_err(|e| color_eyre::eyre::eyre!(e))?;

        tracing::trace!(kid = %kid, available_kids = ?self.keys.iter().map(|r| r.key().clone()).collect::<Vec<_>>(), "Validating JWT");

        let key = self
            .keys
            .get(&kid)
            .ok_or("Unknown kid")
            .map_err(|e| color_eyre::eyre::eyre!(e))?;

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<serde_json::Value>(token, key.value(), &validation)?;

        Ok(token_data.claims)
    }

    /// Validate an ID token and extract identity claims.
    /// Uses the same JWKS keys as access token validation.
    pub async fn validate_id_token(&self, token: &str) -> crate::Result<serde_json::Value> {
        let header = decode_header(token)?;
        let kid = header
            .kid
            .ok_or("Missing kid in JWT header")
            .map_err(|e| color_eyre::eyre::eyre!(e))?;

        let key = self
            .keys
            .get(&kid)
            .ok_or("Unknown kid")
            .map_err(|e| color_eyre::eyre::eyre!(e))?;

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<serde_json::Value>(token, key.value(), &validation)?;
        Ok(token_data.claims)
    }

    pub fn start_key_refresh_task(self: Arc<Self>, interval_seconds: u64) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_seconds));
            loop {
                interval.tick().await;
                if let Err(e) = self.refresh_keys().await {
                    tracing::error!(error = %e, "Failed to refresh JWKS keys");
                }
            }
        });
    }
}
