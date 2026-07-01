//! Bearer-token authentication for AuthService RPCs.
//!
//! Two credential shapes flow through the same authenticate call:
//!   - **User JWTs** in the `authorization: Bearer <jwt>` header. Minted by
//!     [`AuthService::Login`] and carrying the user's pid as `sub`.
//!     Verified locally via the [`Jwt`] signer — no DB lookup on the
//!     fast path.
//!   - **API keys** in the `x-api-key` header. Long-lived `mk.<id>.<secret>`
//!     bearer tokens issued by `CreateApiKey`. Verified by Argon2-hashing
//!     the secret half against the row pointed to by the key_id half.
//!
//! Per ticket 0011: tonic 0.14's `Interceptor` trait is synchronous and
//! cannot await DB lookups. Rather than introduce a `tower::Service`
//! middleware layer, this module exposes [`Authenticator::authenticate`]
//! as an async helper that each RPC calls at the top of its body. The
//! handler-explicit placement also makes the exemption list trivial —
//! handlers that should run pre-auth (`ConsumeBootstrapToken`, `Login`,
//! `RefreshToken`) simply do not call this.

use common_rs::crypto::hashing::{parse_api_key, verify_password};
use http::HeaderMap;
use memoir_sdk::memoir::v1::ApiKeyRole;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tonic::{Request, Status};

use super::jwt::{Jwt, TokenKind};
use crate::models::_entity::{api_keys, users};
use crate::models::{ApiKeys, Users};

/// HTTP/gRPC metadata header that carries the user JWT.
///
/// Standard `authorization: Bearer <token>` form. Lowercased per HTTP/2
/// metadata convention; tonic stores metadata keys lowercased.
const AUTH_HEADER: &str = "authorization";

/// Prefix that precedes the JWT in the `authorization` header value.
///
/// Standard OAuth2 / RFC 6750 Bearer scheme. Anything else is rejected.
const BEARER_PREFIX: &str = "Bearer ";

/// HTTP/gRPC metadata header that carries an integration API key.
///
/// Distinct from `authorization` so the dispatcher never has to sniff the
/// token shape to decide which verification path to use. CLI / SDK
/// consumers set this header; browser-driven consumers set `authorization`.
const API_KEY_HEADER: &str = "x-api-key";

/// String value persisted in `api_keys.status` for active keys.
const STATUS_ACTIVE: &str = "active";

/// String value persisted in `api_keys.role` for admin keys.
const ROLE_ADMIN: &str = "admin";

/// String value persisted in `api_keys.role` for integration keys.
const ROLE_INTEGRATION: &str = "integration";

/// A transport that carries authentication headers.
///
/// tonic's `Request<T>` (gRPC metadata) and axum's request (raw headers)
/// both surface an [`HeaderMap`]; this trait lets [`Authenticator`] read
/// credentials from either without knowing which transport it holds.
pub(crate) trait CredentialSource {
    /// Returns the request's headers.
    fn headers(&self) -> &HeaderMap;
}

impl<T> CredentialSource for Request<T> {
    fn headers(&self) -> &HeaderMap {
        self.metadata().as_ref()
    }
}

impl CredentialSource for HeaderMap {
    fn headers(&self) -> &HeaderMap {
        self
    }
}

/// The single credential a request authenticates with.
///
/// A request may carry both an `x-api-key` and an `authorization: Bearer`
/// header. Precedence — API key wins — is resolved in [`Self::extract`], so
/// verification sees exactly one credential and never re-decides.
#[derive(Debug)]
enum Credential<'a> {
    ApiKey(&'a str),
    Bearer(&'a str),
}

impl<'a> Credential<'a> {
    /// Reads the winning credential from a request's headers.
    ///
    /// `x-api-key` takes precedence over `authorization`: explicit
    /// service-to-service credentials beat fallthrough user sessions.
    ///
    /// # Errors
    ///
    /// [`Status::unauthenticated`] when a header is present but its value is
    /// non-ASCII, or when `authorization` does not use the `Bearer` scheme.
    fn extract(source: &'a impl CredentialSource) -> Result<Option<Self>, Status> {
        let headers = source.headers();

        if let Some(value) = headers.get(API_KEY_HEADER) {
            let raw = value
                .to_str()
                .map_err(|_| Status::unauthenticated("invalid x-api-key metadata"))?;
            return Ok(Some(Self::ApiKey(raw)));
        }

        if let Some(value) = headers.get(AUTH_HEADER) {
            let raw = value
                .to_str()
                .map_err(|_| Status::unauthenticated("invalid authorization metadata"))?;
            let token = raw
                .strip_prefix(BEARER_PREFIX)
                .ok_or_else(|| Status::unauthenticated("authorization must use Bearer scheme"))?;
            return Ok(Some(Self::Bearer(token)));
        }

        Ok(None)
    }
}

/// Identifies which credential type authenticated the request.
///
/// Carries the pid of the principal so handlers can audit "user X did this"
/// vs "API key Y did this" without re-querying the DB.
#[derive(Debug, Clone)]
pub(crate) enum Principal {
    /// A user-driven session, authenticated via a JWT access token.
    User { pid: String },

    /// A service-to-service call, authenticated via an API key.
    ApiKey { pid: String },
}

impl Principal {
    pub(crate) fn pid(&self) -> &str {
        match self {
            Self::User { pid } => pid,
            Self::ApiKey { pid } => pid,
        }
    }
}

/// Authenticated caller of the current RPC.
///
/// Constructed by [`Authenticator::authenticate`]; inspected by handlers
/// that enforce role-based access. The single `is_admin` predicate
/// collapses the two credential shapes' role models (user
/// `is_admin: bool`, api-key `role: Admin | Integration`) into one
/// consumer-facing question.
#[derive(Debug, Clone)]
pub(crate) struct CallerIdentity {
    pub(crate) principal: Principal,
    pub(crate) is_admin: bool,
}

impl CallerIdentity {
    /// Returns an error if the caller is not an admin.
    ///
    /// Used by handlers that mutate auth state (CreateUser, DeleteUser,
    /// CreateApiKey, RotateApiKey, RevokeApiKey).
    pub(crate) fn require_admin(&self) -> Result<(), Status> {
        if self.is_admin {
            Ok(())
        } else {
            Err(Status::permission_denied("admin role required"))
        }
    }
}

/// Verifies JWTs and API keys against the live database.
///
/// Owns the DB handle and JWT signer. Constructed once at process start by
/// [`crate::context::AppContext`] and held behind an [`std::sync::Arc`]
/// so handlers can share one instance without per-request allocation.
#[derive(Debug, Clone)]
pub(crate) struct Authenticator {
    db: DatabaseConnection,
    jwt: Jwt,
}

impl Authenticator {
    /// Builds an authenticator from a DB pool + JWT signer.
    pub(crate) fn new(db: DatabaseConnection, jwt: Jwt) -> Self {
        Self { db, jwt }
    }

    /// Returns the JWT signer for use by Login / RefreshToken handlers.
    pub(crate) fn jwt(&self) -> &Jwt {
        &self.jwt
    }

    /// Validates a request's credentials and returns the authenticated caller.
    ///
    /// Looks first at `x-api-key` (integration callers), then at
    /// `authorization: Bearer <jwt>` (user sessions). If neither header is
    /// present, the request is unauthenticated. If both are present, the
    /// API key wins — explicit service-to-service credentials take
    /// precedence over fallthrough user sessions.
    ///
    /// # Errors
    ///
    /// - [`Status::unauthenticated`] when no credential header is present,
    ///   when the header value is malformed, when the API key is unknown
    ///   or revoked, when the API-key secret half fails verification, or
    ///   when the JWT signature or audience is invalid. The same status is
    ///   returned for all of these so probing callers cannot distinguish
    ///   "no such key" from "wrong secret" via the error code alone.
    /// - [`Status::internal`] when a DB error prevents an API-key lookup.
    ///   The underlying error is logged at error level; the client sees
    ///   only a generic message.
    pub(crate) async fn authenticate(&self, source: &impl CredentialSource) -> Result<CallerIdentity, Status> {
        match Credential::extract(source)? {
            Some(Credential::ApiKey(key)) => self.verify_api_key(key).await,
            Some(Credential::Bearer(jwt)) => self.verify_jwt(jwt).await,
            None => Err(Status::unauthenticated("missing credentials")),
        }
    }

    async fn verify_api_key(&self, token: &str) -> Result<CallerIdentity, Status> {
        let (key_id, secret) = parse_api_key(token).map_err(|_| Status::unauthenticated("invalid credentials"))?;

        let row = ApiKeys::find()
            .filter(api_keys::Column::KeyId.eq(key_id))
            .one(&self.db)
            .await
            .map_err(|err| {
                tracing::error!(error.message = %err, "db error during authenticate");
                Status::internal("internal error")
            })?
            .ok_or_else(|| Status::unauthenticated("invalid credentials"))?;

        if row.status != STATUS_ACTIVE {
            return Err(Status::unauthenticated("invalid credentials"));
        }

        let verified = verify_password(secret, &row.key_hash).map_err(|err| {
            tracing::error!(error.message = %err, "hash verify error during authenticate");
            Status::internal("internal error")
        })?;
        if !verified {
            return Err(Status::unauthenticated("invalid credentials"));
        }

        let role = match row.role.as_str() {
            ROLE_ADMIN => ApiKeyRole::Admin,
            ROLE_INTEGRATION => ApiKeyRole::Integration,
            _ => ApiKeyRole::Unspecified,
        };

        tracing::debug!(
            api_key.key_id = %row.key_id,
            api_key.pid = %row.pid,
            "request authenticated via api key",
        );

        Ok(CallerIdentity {
            principal: Principal::ApiKey { pid: row.pid },
            is_admin: matches!(role, ApiKeyRole::Admin),
        })
    }

    async fn verify_jwt(&self, token: &str) -> Result<CallerIdentity, Status> {
        let claims = self
            .jwt
            .verify(token, TokenKind::Access)
            .map_err(|_| Status::unauthenticated("invalid credentials"))?;

        // is_admin is NOT carried in the JWT to keep role changes effective
        // within one access-token cycle rather than at refresh time. We
        // look it up here against the live `users` row so a freshly
        // demoted user loses admin access on their next request.
        let row = Users::find()
            .filter(users::Column::Pid.eq(&claims.sub))
            .one(&self.db)
            .await
            .map_err(|err| {
                tracing::error!(error.message = %err, "db error resolving is_admin during authenticate");
                Status::internal("internal error")
            })?
            .ok_or_else(|| Status::unauthenticated("invalid credentials"))?;

        tracing::debug!(user.pid = %row.pid, user.is_admin = row.is_admin, "request authenticated via jwt");

        Ok(CallerIdentity {
            principal: Principal::User { pid: row.pid },
            is_admin: row.is_admin,
        })
    }
}

#[cfg(test)]
mod tests {
    use http::{HeaderMap, HeaderName, HeaderValue};

    use super::*;

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (key, value) in pairs {
            map.insert(
                HeaderName::from_bytes(key.as_bytes()).expect("valid header name"),
                HeaderValue::from_str(value).expect("valid header value"),
            );
        }
        map
    }

    #[test]
    fn should_prefer_api_key_when_both_headers_present() {
        let headers = headers(&[(API_KEY_HEADER, "mk.abc.secret"), (AUTH_HEADER, "Bearer jwt.token")]);
        assert!(matches!(
            Credential::extract(&headers),
            Ok(Some(Credential::ApiKey("mk.abc.secret")))
        ));
    }

    #[test]
    fn should_prefer_api_key_when_authorization_is_malformed() {
        // A valid api-key short-circuits before the authorization header is
        // parsed, so a malformed bearer does not poison a request that also
        // carries a valid api-key. Webapps vary here (accept either, or reject
        // if either is malformed); this pins our choice: api-key wins outright.
        let mut headers = headers(&[(API_KEY_HEADER, "mk.abc.secret")]);
        headers.insert(AUTH_HEADER, HeaderValue::from_bytes(b"\xff\xfe").expect("bytes"));
        assert!(matches!(
            Credential::extract(&headers),
            Ok(Some(Credential::ApiKey("mk.abc.secret")))
        ));
    }

    #[test]
    fn should_return_none_when_no_credential_header() {
        assert!(matches!(Credential::extract(&HeaderMap::new()), Ok(None)));
    }

    #[test]
    fn should_reject_when_authorization_is_not_bearer_scheme() {
        let headers = headers(&[(AUTH_HEADER, "Basic dXNlcjpwYXNz")]);
        let error = Credential::extract(&headers).expect_err("non-Bearer scheme is rejected");
        assert_eq!(error.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn should_extract_bearer_when_only_authorization_present() {
        let headers = headers(&[(AUTH_HEADER, "Bearer jwt.token")]);
        assert!(matches!(
            Credential::extract(&headers),
            Ok(Some(Credential::Bearer("jwt.token")))
        ));
    }
}
