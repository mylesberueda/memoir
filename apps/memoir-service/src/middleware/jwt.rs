//! HS256 JWT issuance + verification for user (browser) sessions.
//!
//! Two token shapes flow through this module:
//!   - **access tokens** (audience `"access"`, default TTL 15 minutes): carried
//!     by the browser on every gRPC request, verified by [`Jwt::verify`] on
//!     the auth hot path.
//!   - **refresh tokens** (audience `"refresh"`, default TTL 7 days): held by
//!     the browser, exchanged via the `RefreshToken` RPC for a fresh access
//!     token without re-prompting for the password.
//!
//! The two audiences are validated at decode time so a refresh token
//! presented in the `authorization` header is rejected before it can
//! authenticate a non-refresh RPC, and an access token presented to
//! `RefreshToken` is rejected before it can mint another access token.
//!
//! The secret is read from `JWT_SECRET` once at process start and
//! held as bytes for the lifetime of the [`Jwt`] instance. Rotation is an
//! operator concern — restart the service with the new value.

use std::sync::Arc;

use base64::Engine as _;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// Environment variable name that holds the JWT signing secret.
///
/// Value is a base64-encoded byte string of at least 32 bytes (256 bits) of
/// entropy. Refusing shorter values is intentional — HS256 with < 256 bits
/// of secret is below the algorithm's security target.
pub(crate) const ENV_JWT_SECRET: &str = "JWT_SECRET";

/// Minimum acceptable secret length, in bytes, after base64 decode.
///
/// Matches HS256's 256-bit security target. Anything shorter is rejected at
/// [`Jwt::from_env`] so misconfiguration fails loudly at startup rather
/// than silently weakening every token signed thereafter.
const MIN_SECRET_BYTES: usize = 32;

/// Default access-token TTL in seconds. 15 minutes.
///
/// Browser uses this for normal RPCs. Short so a stolen access token has a
/// bounded blast radius; the refresh token lets the browser keep the
/// session alive without re-prompting.
pub(crate) const DEFAULT_ACCESS_TTL_SECS: i64 = 15 * 60;

/// Default refresh-token TTL in seconds. 7 days.
///
/// Browser exchanges this for fresh access tokens via the `RefreshToken`
/// RPC. Long enough that a daily-active user is never logged out; short
/// enough that a stolen refresh token expires within a week.
pub(crate) const DEFAULT_REFRESH_TTL_SECS: i64 = 7 * 24 * 60 * 60;

/// Audience claim for access tokens.
const AUD_ACCESS: &str = "access";

/// Audience claim for refresh tokens.
const AUD_REFRESH: &str = "refresh";

/// Failure modes for JWT issuance + verification.
#[derive(Debug, thiserror::Error)]
pub(crate) enum JwtError {
    #[error("JWT_SECRET environment variable is not set")]
    SecretMissing,

    #[error("JWT_SECRET is not valid base64: {0}")]
    SecretBase64(base64::DecodeError),

    #[error("JWT_SECRET decodes to {actual} bytes; minimum is {min} for HS256")]
    SecretTooShort { actual: usize, min: usize },

    #[error("token signing failed: {0}")]
    Encode(jsonwebtoken::errors::Error),

    #[error("token rejected: {0}")]
    Decode(jsonwebtoken::errors::Error),
}

/// Audience a token was minted for.
///
/// Returned by [`Jwt::issue`] to make the call site explicit about which
/// kind of token it just produced, and accepted by [`Jwt::verify`] as the
/// expected audience for an incoming token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Access,
    Refresh,
}

impl TokenKind {
    fn as_audience(self) -> &'static str {
        match self {
            Self::Access => AUD_ACCESS,
            Self::Refresh => AUD_REFRESH,
        }
    }

    fn default_ttl_secs(self) -> i64 {
        match self {
            Self::Access => DEFAULT_ACCESS_TTL_SECS,
            Self::Refresh => DEFAULT_REFRESH_TTL_SECS,
        }
    }
}

/// Claims set baked into every memoir JWT.
///
/// `sub` is the user's pid (stable across renames). `exp` and `iat` are
/// unix-seconds. `aud` is `"access"` or `"refresh"` per [`TokenKind`].
/// Intentionally narrow: no email, no role, no org — those are looked up
/// from the user row on each request so revoked / demoted users lose
/// privileges within one access-token cycle rather than at token expiry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) aud: String,
    pub(crate) exp: i64,
    pub(crate) iat: i64,
}

/// JWT signer + verifier backed by a single HS256 secret.
///
/// Cheap to clone — the secret lives behind an [`Arc`]. Instantiated once
/// at process start by [`crate::context::AppContext`] and shared across
/// every handler that needs to mint or verify a token.
#[derive(Clone)]
pub(crate) struct Jwt {
    encoding: EncodingKey,
    decoding: DecodingKey,
    // Kept for completeness so future changes can read the raw secret length
    // for diagnostics without re-deriving the keys.
    _secret: Arc<[u8]>,
}

impl std::fmt::Debug for Jwt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Deliberately omit the secret bytes; logging them would defeat the
        // whole point of having a signing secret.
        f.debug_struct("Jwt").finish_non_exhaustive()
    }
}

impl Jwt {
    /// Builds the signer + verifier from the `JWT_SECRET` env var.
    ///
    /// # Errors
    ///
    /// - [`JwtError::SecretMissing`] when the env var is unset.
    /// - [`JwtError::SecretBase64`] when the value isn't valid base64.
    /// - [`JwtError::SecretTooShort`] when the decoded value is shorter than
    ///   32 bytes (256 bits).
    pub(crate) fn from_env() -> Result<Self, JwtError> {
        let raw = std::env::var(ENV_JWT_SECRET).map_err(|_| JwtError::SecretMissing)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(raw.trim())
            .map_err(JwtError::SecretBase64)?;
        if bytes.len() < MIN_SECRET_BYTES {
            return Err(JwtError::SecretTooShort {
                actual: bytes.len(),
                min: MIN_SECRET_BYTES,
            });
        }
        let encoding = EncodingKey::from_secret(&bytes);
        let decoding = DecodingKey::from_secret(&bytes);
        Ok(Self {
            encoding,
            decoding,
            _secret: Arc::from(bytes),
        })
    }

    /// Builds the signer + verifier from raw bytes. Test-only constructor.
    #[cfg(test)]
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        let encoding = EncodingKey::from_secret(&bytes);
        let decoding = DecodingKey::from_secret(&bytes);
        Self {
            encoding,
            decoding,
            _secret: Arc::from(bytes),
        }
    }

    /// Signs a token of the given kind for the given user pid.
    ///
    /// Uses the kind's default TTL. The returned string is the wire-format
    /// JWT (`<header>.<payload>.<signature>`) ready to be set as a cookie
    /// or attached to a header.
    ///
    /// # Errors
    ///
    /// Returns [`JwtError::Encode`] for any signing-time failure (this is
    /// essentially never expected under HS256 with a valid secret).
    pub(crate) fn issue(&self, sub: &str, kind: TokenKind) -> Result<String, JwtError> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: sub.to_owned(),
            aud: kind.as_audience().to_owned(),
            exp: now + kind.default_ttl_secs(),
            iat: now,
        };
        encode(&Header::default(), &claims, &self.encoding).map_err(JwtError::Encode)
    }

    /// Verifies a token and returns its claims, refusing audience mismatches.
    ///
    /// Validation includes signature, expiry, and audience. A refresh
    /// token presented where an access token was expected (or vice versa)
    /// is rejected — the audience binding is what keeps the two token
    /// types from being interchangeable.
    ///
    /// # Errors
    ///
    /// Returns [`JwtError::Decode`] for any verification-time failure:
    /// invalid signature, expired token, audience mismatch, or malformed
    /// claims.
    pub(crate) fn verify(&self, token: &str, expected: TokenKind) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&[expected.as_audience()]);
        let data = decode::<Claims>(token, &self.decoding, &validation).map_err(JwtError::Decode)?;
        Ok(data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_jwt() -> Jwt {
        Jwt::from_bytes(vec![0u8; MIN_SECRET_BYTES])
    }

    #[test]
    fn should_round_trip_access_token() {
        let jwt = fixture_jwt();
        let token = jwt.issue("user-pid-1", TokenKind::Access).unwrap();
        let claims = jwt.verify(&token, TokenKind::Access).unwrap();
        assert_eq!(claims.sub, "user-pid-1");
        assert_eq!(claims.aud, AUD_ACCESS);
    }

    #[test]
    fn should_round_trip_refresh_token() {
        let jwt = fixture_jwt();
        let token = jwt.issue("user-pid-2", TokenKind::Refresh).unwrap();
        let claims = jwt.verify(&token, TokenKind::Refresh).unwrap();
        assert_eq!(claims.sub, "user-pid-2");
        assert_eq!(claims.aud, AUD_REFRESH);
    }

    #[test]
    fn should_reject_refresh_token_when_access_expected() {
        let jwt = fixture_jwt();
        let token = jwt.issue("user-pid-3", TokenKind::Refresh).unwrap();
        let result = jwt.verify(&token, TokenKind::Access);
        assert!(matches!(result, Err(JwtError::Decode(_))));
    }

    #[test]
    fn should_reject_access_token_when_refresh_expected() {
        let jwt = fixture_jwt();
        let token = jwt.issue("user-pid-4", TokenKind::Access).unwrap();
        let result = jwt.verify(&token, TokenKind::Refresh);
        assert!(matches!(result, Err(JwtError::Decode(_))));
    }

    #[test]
    fn should_reject_token_signed_with_different_secret() {
        let jwt_a = Jwt::from_bytes(vec![1u8; MIN_SECRET_BYTES]);
        let jwt_b = Jwt::from_bytes(vec![2u8; MIN_SECRET_BYTES]);
        let token = jwt_a.issue("user-pid-5", TokenKind::Access).unwrap();
        let result = jwt_b.verify(&token, TokenKind::Access);
        assert!(matches!(result, Err(JwtError::Decode(_))));
    }

    #[test]
    fn should_redact_secret_in_debug_output() {
        let jwt = fixture_jwt();
        let debug = format!("{jwt:?}");
        // No way to leak 32 zero bytes through Debug; the assertion that
        // matters is that the field name "_secret" is absent.
        assert!(
            !debug.contains("secret"),
            "Debug must not surface secret field; got {debug}"
        );
    }
}
