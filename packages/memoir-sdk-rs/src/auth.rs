//! Authentication helpers for memoir-service clients.
//!
//! memoir-service's authenticated modes expect an `authorization: Bearer
//! <token>` header on every call. [`BearerAuth`] is a [`tonic`] interceptor
//! that attaches it, so callers compose it with any generated client via
//! `with_interceptor` instead of hand-rolling header plumbing.
//!
//! # Examples
//!
//! ```no_run
//! use memoir_sdk::BearerAuth;
//! use memoir_sdk::memoir::v1::memory_service_client::MemoryServiceClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let channel = tonic::transport::Channel::from_static("http://localhost:5153")
//!     .connect()
//!     .await?;
//! let auth = BearerAuth::new("my-api-token")?;
//! let mut client = MemoryServiceClient::with_interceptor(channel, auth);
//! # Ok(())
//! # }
//! ```

use std::fmt;

use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

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
        write!(f, "BearerAuth(...)")
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
}
