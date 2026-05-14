//! `AuthService` gRPC handler.
//!
//! Implements the 10 RPCs defined by `memoir.v1.AuthService`. Maps DB entities
//! to proto types at the handler boundary — entities (with their hash columns)
//! never leave this module over gRPC.
//!
//! Security note: this module is unauthenticated until ticket 0011 wires the
//! gRPC interceptor. Until then any caller can invoke any RPC.

use std::sync::Arc;

use base64::Engine as _;
use chrono::{DateTime, FixedOffset};
use common_rs::crypto::hashing::{generate_api_key, hash_password, verify_password};
use memoir_sdk::memoir::v1::auth_service_server::AuthService;
use memoir_sdk::memoir::v1::{
    ApiKey, ApiKeyRole, ApiKeyStatus, ConsumeBootstrapTokenRequest, ConsumeBootstrapTokenResponse,
    CreateApiKeyRequest, CreateApiKeyResponse, CreateUserRequest, CreateUserResponse,
    DeleteUserRequest, DeleteUserResponse, GetApiKeyRequest, GetApiKeyResponse, GetUserRequest,
    GetUserResponse, ListApiKeysRequest, ListApiKeysResponse, ListUsersRequest, ListUsersResponse,
    RevokeApiKeyRequest, RevokeApiKeyResponse, RotateApiKeyRequest, RotateApiKeyResponse, User,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use tonic::{Request, Response, Status};

use crate::AppContext;
use crate::models::{ApiKeys, BootstrapTokens, Users};
use crate::models::_entity::{api_keys, bootstrap_tokens, users};

/// Maximum number of rows a list RPC may return in a single page.
///
/// Capping protects the server from clients requesting unbounded result sets
/// that could exhaust memory. Clients paginate via the returned `next_cursor`.
const MAX_LIST_LIMIT: i32 = 100;

/// Default page size when the request specifies `limit = 0`.
const DEFAULT_LIST_LIMIT: i32 = 50;

/// String value persisted in `api_keys.role` for admin keys.
///
/// Must match the CHECK constraint in migration 000004.
const ROLE_ADMIN: &str = "admin";

/// String value persisted in `api_keys.role` for integration keys.
const ROLE_INTEGRATION: &str = "integration";

/// String value persisted in `api_keys.status` for active keys.
const STATUS_ACTIVE: &str = "active";

/// String value persisted in `api_keys.status` for revoked keys.
const STATUS_REVOKED: &str = "revoked";

/// String value persisted in `bootstrap_tokens.status` for unconsumed tokens.
const BOOTSTRAP_PENDING: &str = "pending";

/// String value persisted in `bootstrap_tokens.status` for consumed tokens.
const BOOTSTRAP_CONSUMED: &str = "consumed";

/// `AuthService` RPC handler backed by Postgres via SeaORM.
pub struct Auth {
    ctx: Arc<AppContext>,
}

impl Auth {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }

    fn db(&self) -> &DatabaseConnection {
        self.ctx.db.as_ref()
    }
}

#[tonic::async_trait]
impl AuthService for Auth {
    async fn consume_bootstrap_token(
        &self,
        request: Request<ConsumeBootstrapTokenRequest>,
    ) -> Result<Response<ConsumeBootstrapTokenResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("token", &req.token)?;
        validate_non_empty("username", &req.username)?;
        validate_non_empty("password", &req.password)?;

        let now: DateTime<FixedOffset> = chrono::Utc::now().into();
        let pending: Vec<bootstrap_tokens::Model> = BootstrapTokens::find()
            .filter(bootstrap_tokens::Column::Status.eq(BOOTSTRAP_PENDING))
            .filter(bootstrap_tokens::Column::ExpiresAt.gt(now))
            .all(self.db())
            .await
            .map_err(internal_error)?;

        // Find the pending row whose token_hash verifies against the supplied
        // plaintext token. Linear scan is acceptable: the partial-unique index
        // on bootstrap_tokens guarantees at most one pending row exists.
        let matched = pending
            .into_iter()
            .find(|row| verify_password(&req.token, &row.token_hash).unwrap_or(false))
            .ok_or_else(|| Status::not_found("invalid or expired bootstrap token"))?;

        // Bootstrap creates an admin user. Same path as CreateUser internally
        // but bypasses the (not-yet-wired) auth interceptor.
        let password_hash = hash_password(&req.password).map_err(internal_error)?;
        let new_user = users::ActiveModel {
            username: Set(req.username),
            password_hash: Set(password_hash),
            is_admin: Set(true),
            ..Default::default()
        };
        let inserted_user = new_user
            .insert(self.db())
            .await
            .map_err(|err| match err {
                DbErr::Query(_) | DbErr::Exec(_) => Status::already_exists("username taken"),
                _ => internal_error(err),
            })?;

        // Mark the token consumed.
        let mut consumed: bootstrap_tokens::ActiveModel = matched.into();
        consumed.status = Set(BOOTSTRAP_CONSUMED.to_string());
        consumed.consumed_at = Set(Some(now));
        consumed.update(self.db()).await.map_err(internal_error)?;

        tracing::info!(
            user.pid = %inserted_user.pid,
            "bootstrap token consumed; admin user created"
        );

        Ok(Response::new(ConsumeBootstrapTokenResponse {
            user: Some(user_to_proto(&inserted_user)),
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("username", &req.username)?;
        validate_non_empty("password", &req.password)?;

        let password_hash = hash_password(&req.password).map_err(internal_error)?;
        let new_user = users::ActiveModel {
            username: Set(req.username),
            password_hash: Set(password_hash),
            is_admin: Set(req.is_admin),
            ..Default::default()
        };
        let inserted = new_user
            .insert(self.db())
            .await
            .map_err(|err| match err {
                DbErr::Query(_) | DbErr::Exec(_) => Status::already_exists("username taken"),
                _ => internal_error(err),
            })?;

        tracing::info!(user.pid = %inserted.pid, "user created");

        Ok(Response::new(CreateUserResponse {
            user: Some(user_to_proto(&inserted)),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("pid", &req.pid)?;

        let row = Users::find()
            .filter(users::Column::Pid.eq(req.pid))
            .one(self.db())
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found("user not found"))?;

        Ok(Response::new(GetUserResponse {
            user: Some(user_to_proto(&row)),
        }))
    }

    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let req = request.into_inner();
        let limit = resolve_limit(req.limit);
        let after = decode_cursor(req.cursor.as_deref())?;

        let mut query = Users::find().order_by_asc(users::Column::Id);
        if let Some(id) = after {
            query = query.filter(users::Column::Id.gt(id));
        }

        // Fetch limit+1 rows to detect whether more pages exist.
        let mut rows = query
            .limit(u64::from(limit as u32) + 1)
            .all(self.db())
            .await
            .map_err(internal_error)?;

        let next_cursor = if rows.len() > limit as usize {
            rows.pop().map(|row| encode_cursor(row.id))
        } else {
            None
        };

        Ok(Response::new(ListUsersResponse {
            users: rows.iter().map(user_to_proto).collect(),
            next_cursor,
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("pid", &req.pid)?;

        let result = Users::delete_many()
            .filter(users::Column::Pid.eq(req.pid))
            .exec(self.db())
            .await
            .map_err(internal_error)?;

        if result.rows_affected == 0 {
            return Err(Status::not_found("user not found"));
        }

        Ok(Response::new(DeleteUserResponse {}))
    }

    async fn create_api_key(
        &self,
        request: Request<CreateApiKeyRequest>,
    ) -> Result<Response<CreateApiKeyResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("name", &req.name)?;
        let role_str = role_to_db(ApiKeyRole::try_from(req.role).unwrap_or(ApiKeyRole::Unspecified))?;

        let generated = generate_api_key().map_err(internal_error)?;
        let secret_hash = hash_password(&generated.secret).map_err(internal_error)?;

        let new_key = api_keys::ActiveModel {
            key_id: Set(generated.key_id.clone()),
            key_hash: Set(secret_hash),
            name: Set(req.name),
            role: Set(role_str.to_string()),
            org_id: Set(req.org_id),
            ..Default::default()
        };
        let inserted = new_key.insert(self.db()).await.map_err(internal_error)?;

        tracing::info!(
            api_key.pid = %inserted.pid,
            api_key.key_id = %inserted.key_id,
            "api key created"
        );

        Ok(Response::new(CreateApiKeyResponse {
            key: Some(api_key_to_proto(&inserted)),
            plaintext: generated.plaintext,
        }))
    }

    async fn get_api_key(
        &self,
        request: Request<GetApiKeyRequest>,
    ) -> Result<Response<GetApiKeyResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("pid", &req.pid)?;

        let row = ApiKeys::find()
            .filter(api_keys::Column::Pid.eq(req.pid))
            .one(self.db())
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found("api key not found"))?;

        Ok(Response::new(GetApiKeyResponse {
            key: Some(api_key_to_proto(&row)),
        }))
    }

    async fn list_api_keys(
        &self,
        request: Request<ListApiKeysRequest>,
    ) -> Result<Response<ListApiKeysResponse>, Status> {
        let req = request.into_inner();
        let limit = resolve_limit(req.limit);
        let after = decode_cursor(req.cursor.as_deref())?;

        let mut query = ApiKeys::find().order_by_asc(api_keys::Column::Id);
        if let Some(id) = after {
            query = query.filter(api_keys::Column::Id.gt(id));
        }
        if let Some(status) = req.status {
            let status_enum =
                ApiKeyStatus::try_from(status).unwrap_or(ApiKeyStatus::Unspecified);
            if let Some(s) = status_to_db(status_enum) {
                query = query.filter(api_keys::Column::Status.eq(s));
            }
        }

        let mut rows = query
            .limit(u64::from(limit as u32) + 1)
            .all(self.db())
            .await
            .map_err(internal_error)?;

        let next_cursor = if rows.len() > limit as usize {
            rows.pop().map(|row| encode_cursor(row.id))
        } else {
            None
        };

        Ok(Response::new(ListApiKeysResponse {
            keys: rows.iter().map(api_key_to_proto).collect(),
            next_cursor,
        }))
    }

    async fn rotate_api_key(
        &self,
        request: Request<RotateApiKeyRequest>,
    ) -> Result<Response<RotateApiKeyResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("pid", &req.pid)?;

        let existing = ApiKeys::find()
            .filter(api_keys::Column::Pid.eq(req.pid))
            .one(self.db())
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found("api key not found"))?;

        let generated = generate_api_key().map_err(internal_error)?;
        let new_hash = hash_password(&generated.secret).map_err(internal_error)?;

        let mut active: api_keys::ActiveModel = existing.into();
        active.key_id = Set(generated.key_id.clone());
        active.key_hash = Set(new_hash);
        let updated = active.update(self.db()).await.map_err(internal_error)?;

        tracing::info!(api_key.pid = %updated.pid, "api key rotated");

        Ok(Response::new(RotateApiKeyResponse {
            key: Some(api_key_to_proto(&updated)),
            plaintext: generated.plaintext,
        }))
    }

    async fn revoke_api_key(
        &self,
        request: Request<RevokeApiKeyRequest>,
    ) -> Result<Response<RevokeApiKeyResponse>, Status> {
        let req = request.into_inner();
        validate_non_empty("pid", &req.pid)?;

        let existing = ApiKeys::find()
            .filter(api_keys::Column::Pid.eq(req.pid))
            .one(self.db())
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found("api key not found"))?;

        let mut active: api_keys::ActiveModel = existing.into();
        active.status = Set(STATUS_REVOKED.to_string());
        let updated = active.update(self.db()).await.map_err(internal_error)?;

        tracing::info!(api_key.pid = %updated.pid, "api key revoked");

        Ok(Response::new(RevokeApiKeyResponse {}))
    }
}

// ─── helpers ────────────────────────────────────────────────────────────────

fn validate_non_empty(field: &str, value: &str) -> Result<(), Status> {
    if value.is_empty() {
        return Err(Status::invalid_argument(format!("{field} must be non-empty")));
    }
    Ok(())
}

fn resolve_limit(requested: i32) -> i32 {
    if requested <= 0 {
        DEFAULT_LIST_LIMIT
    } else {
        requested.min(MAX_LIST_LIMIT)
    }
}

fn encode_cursor(id: i64) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(id.to_be_bytes())
}

fn decode_cursor(cursor: Option<&str>) -> Result<Option<i64>, Status> {
    let Some(c) = cursor else {
        return Ok(None);
    };
    if c.is_empty() {
        return Ok(None);
    }
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(c)
        .map_err(|_| Status::invalid_argument("invalid cursor"))?;
    let arr: [u8; 8] = bytes
        .try_into()
        .map_err(|_| Status::invalid_argument("invalid cursor"))?;
    Ok(Some(i64::from_be_bytes(arr)))
}

fn role_to_db(role: ApiKeyRole) -> Result<&'static str, Status> {
    match role {
        ApiKeyRole::Admin => Ok(ROLE_ADMIN),
        ApiKeyRole::Integration => Ok(ROLE_INTEGRATION),
        ApiKeyRole::Unspecified => Err(Status::invalid_argument("role must be specified")),
    }
}

fn role_from_db(role: &str) -> ApiKeyRole {
    match role {
        ROLE_ADMIN => ApiKeyRole::Admin,
        ROLE_INTEGRATION => ApiKeyRole::Integration,
        _ => ApiKeyRole::Unspecified,
    }
}

fn status_to_db(status: ApiKeyStatus) -> Option<&'static str> {
    match status {
        ApiKeyStatus::Active => Some(STATUS_ACTIVE),
        ApiKeyStatus::Revoked => Some(STATUS_REVOKED),
        ApiKeyStatus::Unspecified => None,
    }
}

fn status_from_db(status: &str) -> ApiKeyStatus {
    match status {
        STATUS_ACTIVE => ApiKeyStatus::Active,
        STATUS_REVOKED => ApiKeyStatus::Revoked,
        _ => ApiKeyStatus::Unspecified,
    }
}

/// Logs the source error and returns a generic gRPC `Internal` status.
///
/// Never leak SQL or implementation details to clients. Operators see the
/// underlying error in structured logs; callers see only "internal error".
fn internal_error<E: std::fmt::Display>(err: E) -> Status {
    tracing::error!(error.message = %err, "internal error in auth handler");
    Status::internal("internal error")
}

fn ts_to_proto(ts: DateTime<FixedOffset>) -> pbjson_types::Timestamp {
    pbjson_types::Timestamp {
        seconds: ts.timestamp(),
        nanos: ts.timestamp_subsec_nanos() as i32,
    }
}

fn user_to_proto(row: &users::Model) -> User {
    User {
        pid: row.pid.clone(),
        username: row.username.clone(),
        is_admin: row.is_admin,
        created_at: Some(ts_to_proto(row.created_at)),
        updated_at: Some(ts_to_proto(row.updated_at)),
    }
}

fn api_key_to_proto(row: &api_keys::Model) -> ApiKey {
    ApiKey {
        pid: row.pid.clone(),
        key_id: row.key_id.clone(),
        name: row.name.clone(),
        role: role_from_db(&row.role) as i32,
        org_id: row.org_id.clone(),
        status: status_from_db(&row.status) as i32,
        created_at: Some(ts_to_proto(row.created_at)),
        updated_at: Some(ts_to_proto(row.updated_at)),
        last_used_at: row.last_used_at.map(ts_to_proto),
    }
}
