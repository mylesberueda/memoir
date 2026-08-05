//! Authentication helpers for memoir-service clients.
//!
//! memoir-service accepts two credential shapes, each on its own header, and
//! this module provides one [`tonic`] interceptor for each. Callers compose
//! either with a generated client via `with_interceptor` instead of
//! hand-rolling header plumbing.
//!
//! | Credential | Header | Interceptor |
//! |---|---|---|
//! | User JWT from `Login` | `authorization: Bearer <jwt>` | [`BearerAuth`] |
//! | API key (`mk.<id>.<secret>`) | `x-api-key: <key>` | [`ApiKeyAuth`] |
//!
//! The headers are distinct so the server never has to infer which
//! verification path a token wants from the token's own shape. Pick the
//! interceptor that matches the credential you hold: a JWT sent as an API key
//! (or the reverse) is rejected as `unauthenticated`.
//!
//! # Examples
//!
//! Service-to-service callers hold a long-lived API key:
//!
//! ```no_run
//! use memoir_sdk::ApiKeyAuth;
//! use memoir_sdk::memoir::v1::memory_service_client::MemoryServiceClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = tonic::transport::Channel::from_static("http://localhost:5153")
//!     .connect()
//!     .await?;
//! let auth = ApiKeyAuth::new("mk.abc123.xyz789")?;
//! let mut client = MemoryServiceClient::with_interceptor(channel, auth);
//! # Ok(())
//! # }
//! ```
//!
//! Interactive callers hold a JWT minted by `Login`:
//!
//! ```no_run
//! use memoir_sdk::BearerAuth;
//! use memoir_sdk::memoir::v1::memory_service_client::MemoryServiceClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = tonic::transport::Channel::from_static("http://localhost:5153")
//!     .connect()
//!     .await?;
//! let auth = BearerAuth::new("my-access-token")?;
//! let mut client = MemoryServiceClient::with_interceptor(channel, auth);
//! # Ok(())
//! # }
//! ```

use std::fmt;

use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

/// Metadata header carrying an API key, matching memoir-service's reader.
const API_KEY_HEADER: &str = "x-api-key";

/// Prefix every API key issued by `CreateApiKey` starts with.
const API_KEY_PREFIX: &str = "mk.";

/// Interceptor that attaches `authorization: Bearer <token>` to every request.
///
/// Build once with [`BearerAuth::new`] and pass to a generated client's
/// `with_interceptor` constructor. The header value is validated at
/// construction, so interception itself never fails.
///
/// The token is sensitive: the [`Debug`] impl deliberately omits it.
#[derive(Clone)]
pub struct BearerAuth {
    header: MetadataValue<Ascii>,
}

impl BearerAuth {
    /// Creates an interceptor for the given bearer token.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidMetadataValue`] when the token contains characters
    /// that are not valid in an ASCII header value (e.g. control characters
    /// or non-ASCII bytes).
    pub fn new(token: &str) -> Result<Self, InvalidMetadataValue> {
        let header = MetadataValue::try_from(format!("Bearer {token}"))?;
        Ok(Self { header })
    }
}

impl Interceptor for BearerAuth {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert("authorization", self.header.clone());
        Ok(request)
    }
}

impl fmt::Debug for BearerAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BearerAuth([redacted])")
    }
}

/// Reason an [`ApiKeyAuth`] could not be built from a key string.
///
/// The key itself is never included in the message: these errors are
/// routinely logged, and an API key is a secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiKeyAuthError {
    /// The key does not start with `mk.`, so it is not a memoir API key.
    ///
    /// Most often this is a JWT passed by mistake — those belong in
    /// [`BearerAuth`] instead.
    MalformedKey,

    /// The key contains bytes that are not valid in an ASCII header value.
    InvalidHeaderValue,
}

impl fmt::Display for ApiKeyAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedKey => {
                write!(
                    f,
                    "api key must start with `{API_KEY_PREFIX}`; use BearerAuth for a JWT"
                )
            }
            Self::InvalidHeaderValue => write!(f, "api key contains characters invalid in a header value"),
        }
    }
}

impl std::error::Error for ApiKeyAuthError {}

impl From<InvalidMetadataValue> for ApiKeyAuthError {
    fn from(_: InvalidMetadataValue) -> Self {
        Self::InvalidHeaderValue
    }
}

/// Interceptor that attaches `x-api-key: <key>` to every request.
///
/// Build once with [`ApiKeyAuth::new`] and pass to a generated client's
/// `with_interceptor` constructor. The header value is validated at
/// construction, so interception itself never fails.
///
/// Use this for long-lived `mk.<key_id>.<secret>` keys issued by
/// `CreateApiKey`. JWTs from `Login` go through [`BearerAuth`] instead —
/// memoir-service verifies the two headers by different paths and will not
/// accept a credential on the wrong one.
///
/// The key is sensitive: the [`Debug`] impl deliberately omits it.
///
/// # Examples
///
/// ```
/// use memoir_sdk::ApiKeyAuth;
///
/// let auth = ApiKeyAuth::new("mk.abc123.xyz789")?;
/// assert_eq!(format!("{auth:?}"), "ApiKeyAuth([redacted])");
/// # Ok::<(), memoir_sdk::auth::ApiKeyAuthError>(())
/// ```
#[derive(Clone)]
pub struct ApiKeyAuth {
    header: MetadataValue<Ascii>,
}

impl ApiKeyAuth {
    /// Creates an interceptor for the given API key.
    ///
    /// # Errors
    ///
    /// Returns [`ApiKeyAuthError::MalformedKey`] when the key does not begin
    /// with `mk.`, and [`ApiKeyAuthError::InvalidHeaderValue`] when it
    /// contains characters that are not valid in an ASCII header value.
    pub fn new(key: &str) -> Result<Self, ApiKeyAuthError> {
        if !key.starts_with(API_KEY_PREFIX) {
            return Err(ApiKeyAuthError::MalformedKey);
        }
        let header = MetadataValue::try_from(key)?;
        Ok(Self { header })
    }
}

impl Interceptor for ApiKeyAuth {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert(API_KEY_HEADER, self.header.clone());
        Ok(request)
    }
}

impl fmt::Debug for ApiKeyAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKeyAuth([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_attach_bearer_header_to_request() {
        let mut auth = BearerAuth::new("token-123").expect("plain ASCII token must be valid");
        let request = auth.call(Request::new(())).expect("interception must succeed");
        let header = request
            .metadata()
            .get("authorization")
            .expect("authorization header must be present");
        assert_eq!(header.to_str().unwrap(), "Bearer token-123");
    }

    #[test]
    fn should_reject_token_with_invalid_header_characters() {
        assert!(BearerAuth::new("bad\ntoken").is_err());
    }

    #[test]
    fn should_not_leak_token_in_debug_output() {
        let token = "552d3454-d0d5-445d-ab9f-ef2ae3a8896a";
        let auth = BearerAuth::new(token).expect("plain ASCII token must be valid");
        let rendered = format!("{auth:?}");
        assert!(rendered.contains("BearerAuth"));
        assert!(!rendered.contains(token));
    }

    #[test]
    fn should_attach_api_key_on_x_api_key_header() {
        let mut auth = ApiKeyAuth::new("mk.abc123.xyz789").expect("well-formed key must be valid");
        let request = auth.call(Request::new(())).expect("interception must succeed");
        let header = request
            .metadata()
            .get("x-api-key")
            .expect("x-api-key header must be present");
        assert_eq!(header.to_str().unwrap(), "mk.abc123.xyz789");
    }

    #[test]
    fn should_not_attach_authorization_header_for_api_key() {
        // memoir-service routes `authorization` to JWT verification, which an
        // api key can never satisfy. Sending it there is the bug this type fixes.
        let mut auth = ApiKeyAuth::new("mk.abc123.xyz789").expect("well-formed key must be valid");
        let request = auth.call(Request::new(())).expect("interception must succeed");
        assert!(request.metadata().get("authorization").is_none());
    }

    #[test]
    fn should_reject_key_without_mk_prefix() {
        let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIn0.signature";
        let error = ApiKeyAuth::new(jwt).err().expect("a jwt is not an api key");
        assert_eq!(error, ApiKeyAuthError::MalformedKey);
    }

    #[test]
    fn should_reject_key_with_invalid_header_characters() {
        let error = ApiKeyAuth::new("mk.abc.bad\nsecret")
            .err()
            .expect("a newline is invalid in a header value");
        assert_eq!(error, ApiKeyAuthError::InvalidHeaderValue);
    }

    #[test]
    fn should_not_leak_api_key_in_debug_output() {
        let key = "mk.abc123.552d3454d0d5445dab9fef2ae3a8896a";
        let auth = ApiKeyAuth::new(key).expect("well-formed key must be valid");
        let rendered = format!("{auth:?}");
        assert!(rendered.contains("ApiKeyAuth"));
        assert!(!rendered.contains("552d3454d0d5445dab9fef2ae3a8896a"));
    }
}
