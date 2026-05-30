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
    pub(crate) async fn authenticate<T>(&self, request: &Request<T>) -> Result<CallerIdentity, Status> {
        self.authenticate_credentials(extract_api_key(request)?, extract_jwt(request)?)
            .await
    }

    /// Verifies pre-extracted credential strings.
    ///
    /// Transport-independent shared core of [`Self::authenticate`]. The
    /// tonic path extracts from `Request<T>` metadata; an axum middleware
    /// extracts from `http::HeaderMap`. Both arrive here with the same
    /// precedence: API key wins when both are present.
    ///
    /// # Errors
    ///
    /// Same as [`Self::authenticate`].
    pub(crate) async fn authenticate_credentials(
        &self,
        api_key: Option<&str>,
        bearer: Option<&str>,
    ) -> Result<CallerIdentity, Status> {
        if let Some(api_key) = api_key {
            return self.verify_api_key(api_key).await;
        }
        if let Some(jwt) = bearer {
            return self.verify_jwt(jwt).await;
        }
        Err(Status::unauthenticated("missing credentials"))
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

fn extract_api_key<T>(request: &Request<T>) -> Result<Option<&str>, Status> {
    let Some(value) = request.metadata().get(API_KEY_HEADER) else {
        return Ok(None);
    };
    let raw = value
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid x-api-key metadata"))?;
    Ok(Some(raw))
}

fn extract_jwt<T>(request: &Request<T>) -> Result<Option<&str>, Status> {
    let Some(value) = request.metadata().get(AUTH_HEADER) else {
        return Ok(None);
    };
    let raw = value
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid authorization metadata"))?;
    let token = raw
        .strip_prefix(BEARER_PREFIX)
        .ok_or_else(|| Status::unauthenticated("authorization must use Bearer scheme"))?;
    Ok(Some(token))
}
