//! Bearer-token authentication helper for AuthService RPCs.
//!
//! Per ticket 0011: tonic 0.14's `Interceptor` trait is synchronous and
//! cannot await DB lookups. Rather than introduce a `tower::Service`
//! middleware layer, this module exposes [`authenticate`] as an async
//! helper that each RPC calls at the top of its body. The handler-explicit
//! placement also makes the exemption list trivial — handlers that should
//! run pre-auth (`ConsumeBootstrapToken`) simply do not call this.
//!
//! The trade-off vs. a true middleware: per-RPC role/scope checks live in
//! handler code (not in a centralized policy table). For v0.1's surface,
//! handler-explicit is the smallest correct shape.

use common_rs::crypto::hashing::{parse_api_key, verify_password};
use memoir_sdk::memoir::v1::ApiKeyRole;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tonic::{Request, Status};

use crate::models::ApiKeys;
use crate::models::_entity::api_keys;

/// HTTP/gRPC metadata header that carries the bearer token.
///
/// Standard `authorization: Bearer <token>` form. Lowercased per HTTP/2 metadata
/// convention; tonic stores metadata keys lowercased.
const AUTH_HEADER: &str = "authorization";

/// Prefix that precedes the token in the `authorization` header value.
///
/// Standard OAuth2 / RFC 6750 Bearer scheme. Anything else (e.g. `Basic`)
/// is rejected.
const BEARER_PREFIX: &str = "Bearer ";

/// String value persisted in `api_keys.status` for active keys.
///
/// Must match the value in the migration's CHECK constraint and the
/// constant in `services/auth.rs`. Kept private here to avoid coupling
/// the middleware to the services module's internals.
const STATUS_ACTIVE: &str = "active";

/// String value persisted in `api_keys.role` for admin keys.
const ROLE_ADMIN: &str = "admin";

/// String value persisted in `api_keys.role` for integration keys.
const ROLE_INTEGRATION: &str = "integration";

/// Identity of the API key that authenticated the current request.
///
/// Constructed by [`authenticate`] and inspected by handlers for role
/// enforcement. Additional fields (`key_id`, `org_id` for scope binding,
/// `key_row_id` for audit chains) will be added when handlers need them
/// — not eagerly.
#[derive(Debug, Clone)]
pub(crate) struct CallerIdentity {
    /// Role assigned to the key. Determines what RPCs the caller may invoke.
    pub(crate) role: ApiKeyRole,
}

impl CallerIdentity {
    /// Returns an error if the caller is not an admin.
    ///
    /// Used by handlers that mutate auth state (CreateUser, DeleteUser,
    /// CreateApiKey, RotateApiKey, RevokeApiKey).
    pub(crate) fn require_admin(&self) -> Result<(), Status> {
        match self.role {
            ApiKeyRole::Admin => Ok(()),
            _ => Err(Status::permission_denied("admin role required")),
        }
    }
}

/// Validates a request's bearer token and returns the authenticated caller.
///
/// The flow: read `authorization: Bearer mk.<key_id>.<secret>` from request
/// metadata, parse, look up the `api_keys` row by `key_id`, Argon2-verify
/// the secret half, check the row is `status='active'`, and return a
/// [`CallerIdentity`] for the handler to inspect.
///
/// # Errors
///
/// - [`Status::unauthenticated`] when the header is missing, malformed,
///   carries an unknown key_id, fails secret verification, or matches a
///   revoked key. The same status is returned for all of these so that
///   probing callers cannot distinguish "no such key" from "wrong secret"
///   via the error code alone.
/// - [`Status::internal`] when a DB error prevents lookup. The underlying
///   error is logged at error level; the client sees only a generic message.
pub(crate) async fn authenticate<T>(
    db: &DatabaseConnection,
    request: &Request<T>,
) -> Result<CallerIdentity, Status> {
    let token = extract_bearer(request)?;
    let (key_id, secret) =
        parse_api_key(token).map_err(|_| Status::unauthenticated("invalid credentials"))?;

    let row = ApiKeys::find()
        .filter(api_keys::Column::KeyId.eq(key_id))
        .one(db)
        .await
        .map_err(|err| {
            tracing::error!(error.message = %err, "db error during authenticate");
            Status::internal("internal error")
        })?;

    let row = row.ok_or_else(|| Status::unauthenticated("invalid credentials"))?;

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

    tracing::debug!(api_key.key_id = %row.key_id, api_key.pid = %row.pid, "request authenticated");

    Ok(CallerIdentity { role })
}

fn extract_bearer<T>(request: &Request<T>) -> Result<&str, Status> {
    let value = request
        .metadata()
        .get(AUTH_HEADER)
        .ok_or_else(|| Status::unauthenticated("missing authorization metadata"))?;
    let raw = value
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid authorization metadata"))?;
    raw.strip_prefix(BEARER_PREFIX)
        .ok_or_else(|| Status::unauthenticated("authorization must use Bearer scheme"))
}
