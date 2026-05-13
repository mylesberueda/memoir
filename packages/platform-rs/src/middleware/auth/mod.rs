use super::BoxFuture;
use std::sync::Arc;
use tonic::Status;
use tower::{Layer, Service};

mod extractors;
mod jwt;
mod user;

pub use extractors::*;
pub use jwt::*;
pub use user::*;

/// Header name for the ID token (lowercase for HTTP/2 compatibility).
const IDENTITY_TOKEN_HEADER: &str = "x-id-token";

/// gRPC health check path prefix - excluded from authentication.
const GRPC_HEALTH_PATH: &str = "/grpc.health.v1.Health/";

/// Configuration for authentication middleware
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// URL to fetch JWKS keys from
    pub jwks_url: String,
    /// Expected token issuer
    pub issuer: String,
    /// Expected token audience
    pub audience: String,
}

#[derive(Debug, Clone)]
pub struct AuthLayer<E> {
    validator: Arc<JwtValidator>,
    extractor: Arc<E>,
}

impl<E: UserExtractor> AuthLayer<E> {
    pub async fn new(config: &AuthConfig) -> crate::Result<Self> {
        let validator =
            Arc::new(JwtValidator::new(config.jwks_url.clone(), config.issuer.clone(), config.audience.clone()).await?);

        Ok(Self {
            validator,
            extractor: Arc::new(E::default()),
        })
    }

    /// Create a new AuthLayer with a custom extractor instance
    pub async fn with_extractor(config: &AuthConfig, extractor: E) -> crate::Result<Self> {
        let validator =
            Arc::new(JwtValidator::new(config.jwks_url.clone(), config.issuer.clone(), config.audience.clone()).await?);

        Ok(Self {
            validator,
            extractor: Arc::new(extractor),
        })
    }

    /// Start background JWKS key refresh.
    pub fn start_key_refresh(&self, interval_seconds: u64) {
        self.validator.clone().start_key_refresh_task(interval_seconds);
    }
}

impl<S, E> Layer<S> for AuthLayer<E>
where
    E: UserExtractor + Clone,
{
    type Service = AuthMiddleware<S, E>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            validator: self.validator.clone(),
            extractor: self.extractor.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthMiddleware<S, E> {
    inner: S,
    validator: Arc<JwtValidator>,
    extractor: Arc<E>,
}

impl<S, E, ReqBody, ResBody> Service<http::Request<ReqBody>> for AuthMiddleware<S, E>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    E: UserExtractor,
    ReqBody: Default + Send + 'static,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let validator = self.validator.clone();
        let extractor = self.extractor.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Skip authentication for gRPC health checks
            if req.uri().path().starts_with(GRPC_HEALTH_PATH) {
                return inner.call(req).await;
            }

            let access_token = match req
                .headers()
                .get(http::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
            {
                Some(t) => t,
                None => {
                    return Ok(Status::unauthenticated("Missing authorization header").into_http());
                }
            };

            let access_claims = match validator.validate(access_token).await {
                Ok(claims) => claims,
                Err(e) => {
                    tracing::error!(error = ?e, "JWT validation failed");
                    return Ok(Status::unauthenticated(format!("Invalid token: {e}")).into_http());
                }
            };

            let id_claims =
                if let Some(id_token) = req.headers().get(IDENTITY_TOKEN_HEADER).and_then(|v| v.to_str().ok()) {
                    match validator.validate_id_token(id_token).await {
                        Ok(claims) => {
                            tracing::debug!("Identity token validated");
                            Some(claims)
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "Failed to validate identity token");
                            None
                        }
                    }
                } else {
                    tracing::trace!("No x-id-token header present");
                    None
                };

            match extractor.extract_user(&access_claims, id_claims.as_ref()) {
                Ok(user) => {
                    tracing::debug!(
                        user_id = %user.id,
                        email = ?user.email,
                        org_count = user.org_roles.len(),
                        "User extracted from token claims"
                    );
                    req.extensions_mut().insert(user);
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to extract user from claims");
                    return Ok(tonic::Status::unauthenticated(format!("Invalid token claims: {e}")).into_http());
                }
            }

            inner.call(req).await
        })
    }
}
