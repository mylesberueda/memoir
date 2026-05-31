//! Proto ↔ library type conversions for the gRPC boundary.
//!
//! Centralizes every shape mismatch between `memoir_sdk::memoir::v1::*`
//! (wire types, prost-generated) and `memoir_core::*` (library types). The
//! handler bodies in `services/memory.rs` stay short by delegating every
//! conversion here.
//!
//! ## Conventions
//!
//! - **Boundary validation is at the proto side**, not the library side. A
//!   `proto::Scope` with empty fields is rejected here with
//!   `Status::invalid_argument`. memoir-core's own `validate_scope` is a
//!   defense-in-depth fallback, not the primary gate.
//! - **Conversions that can fail** return `Result<_, tonic::Status>`
//!   directly, so handler bodies can `?` them. The status surfaces a
//!   safe-to-wire message; never the inner error's `Display`.
//! - **Conversions from library → proto are infallible** (`From` impls) —
//!   library values are already validated.
//! - **Metadata round-trips through serde.** `pbjson_types::Struct` and
//!   `serde_json::Value` both implement `Serialize`/`Deserialize`; the
//!   `serde_json::to_value` / `serde_json::from_value` pair is the
//!   canonical conversion path per the pbjson-types maintainer.

use memoir_core::client::{ClientError, ReconcileSummary};
use memoir_core::jobs::{FailedJob as LibFailedJob, JobKind as LibJobKind, JobsError};
use memoir_core::memory::{ForgetTarget, Memory as LibMemory, Scope as LibScope};
use memoir_core::store::StoreError;
use memoir_core::vector::{
    FilterCondition as LibFilterCondition, MatchValue as LibMatchValue, MatchValues as LibMatchValues,
    MemoryFilter as LibMemoryFilter, NumericRange as LibNumericRange, VectorError,
};
use memoir_sdk::memoir::v1::{
    FailedJob as ProtoFailedJob, FilterCondition as ProtoFilterCondition, ForgetRequest, JobKind as ProtoJobKind,
    MatchValue as ProtoMatchValue, MatchValues as ProtoMatchValues, Memory as ProtoMemory,
    MemoryFilter as ProtoMemoryFilter, MemoryStatus, NumericRange as ProtoNumericRange, ReconcileResponse,
    Scope as ProtoScope, filter_condition, forget_request, match_value, match_values,
};
use tonic::Status;

/// Converts a wire `Scope` into the library shape, rejecting empty fields.
///
/// Empty `agent_id` / `org_id` / `user_id` is a misuse from a misconfigured
/// caller — proto3 fills unset string fields with `""` on the wire, so a
/// client that "forgot to set" a scope field arrives here with empties.
/// We reject with `InvalidArgument` rather than letting memoir-core's
/// storage layer silently treat empty as a literal scope value.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the scope is unset or any
/// field is empty.
pub(crate) fn scope_from_proto(scope: Option<ProtoScope>) -> Result<LibScope, Status> {
    let scope = scope.ok_or_else(|| Status::invalid_argument("scope: required"))?;
    if scope.agent_id.is_empty() || scope.org_id.is_empty() || scope.user_id.is_empty() {
        return Err(Status::invalid_argument(
            "scope: agent_id, org_id, and user_id must all be non-empty",
        ));
    }
    Ok(LibScope {
        agent_id: scope.agent_id,
        org_id: scope.org_id,
        user_id: scope.user_id,
    })
}

/// Converts a library `Scope` back to the wire shape. Infallible.
pub(crate) fn scope_to_proto(scope: LibScope) -> ProtoScope {
    ProtoScope {
        agent_id: scope.agent_id,
        org_id: scope.org_id,
        user_id: scope.user_id,
    }
}

/// Converts a wire metadata `Struct` into the library's `serde_json::Value`.
///
/// `None` (proto3 unset) maps to `serde_json::json!({})` to match the
/// library's column default. Non-object payloads (arrays, scalars) are
/// accepted — JSONB doesn't constrain shape — but malformed inputs
/// (`NaN`, infinity) fail at serde and surface as `InvalidArgument`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the struct cannot be
/// represented as JSON (e.g., contains `NaN`).
pub(crate) fn metadata_from_proto(meta: Option<pbjson_types::Struct>) -> Result<serde_json::Value, Status> {
    let Some(struct_value) = meta else {
        return Ok(serde_json::json!({}));
    };
    serde_json::to_value(struct_value).map_err(|err| {
        tracing::warn!(error.message = %err, "rejected RememberRequest with malformed metadata");
        Status::invalid_argument("metadata: not representable as JSON")
    })
}

/// Converts library `serde_json::Value` metadata back to the wire `Struct`.
///
/// Falls back to an empty `Struct` if the value is not a JSON object —
/// the proto field's `Struct` type can only encode objects, so a scalar
/// or array metadata value is mapped to `{}` on the way out. This case
/// should not occur in practice (the library writes JSON objects) but
/// is handled defensively.
pub(crate) fn metadata_to_proto(value: serde_json::Value) -> pbjson_types::Struct {
    serde_json::from_value::<pbjson_types::Struct>(value).unwrap_or_default()
}

/// Converts a library `Memory` into the wire shape.
///
/// `processed_at` is always `None` in v0.1 — memoir-core does not yet
/// track a dedicated processed timestamp (see `memory.proto` field
/// comment). The `status` field is set to PENDING for newly-written
/// rows; a future library schema migration will let this surface
/// PROCESSED/FAILED based on `qdrant_status`.
pub(crate) fn memory_to_proto(memory: LibMemory) -> ProtoMemory {
    ProtoMemory {
        pid: memory.pid,
        scope: Some(scope_to_proto(memory.scope)),
        content: memory.content,
        metadata: Some(metadata_to_proto(memory.metadata)),
        created_at: Some(timestamp_from_chrono(memory.created_at)),
        processed_at: None,
        status: MemoryStatus::Pending as i32,
    }
}

/// Converts `ForgetRequest.target` (oneof) into the library `ForgetTarget`.
///
/// Proto3 oneof maps to `Option<Target>` in the generated code. An unset
/// oneof is a client bug — the proto contract requires "exactly one"
/// of pid or scope. Reject with `InvalidArgument`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the oneof is unset, when a
/// `Pid` payload is empty, or when a `Scope` payload has any empty field.
pub(crate) fn forget_target_from_proto(request: ForgetRequest) -> Result<ForgetTarget, Status> {
    let target = request
        .target
        .ok_or_else(|| Status::invalid_argument("target: required (set either pid or scope)"))?;
    match target {
        forget_request::Target::Pid(pid) if pid.is_empty() => {
            Err(Status::invalid_argument("target.pid: must be non-empty"))
        }
        forget_request::Target::Pid(pid) => Ok(ForgetTarget::Pid(pid)),
        forget_request::Target::Scope(scope) => {
            let lib_scope = scope_from_proto(Some(scope))?;
            Ok(ForgetTarget::Scope(lib_scope))
        }
    }
}

/// Converts a `chrono::DateTime<FixedOffset>` to a `pbjson_types::Timestamp`.
///
/// Lossless: `chrono::DateTime` carries nanosecond precision, `Timestamp`
/// carries seconds + nanos. The proto type cannot represent negative
/// nanos, which `chrono` also disallows for the unix-epoch-anchored
/// representation we use here — so the conversion never loses data.
pub(crate) fn timestamp_from_chrono(dt: chrono::DateTime<chrono::FixedOffset>) -> pbjson_types::Timestamp {
    pbjson_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

/// Maps a [`memoir_core::client::ClientError`] to a [`tonic::Status`].
///
/// The mapping follows gRPC semantics:
///
/// | Library error                                        | gRPC code         |
/// |------------------------------------------------------|-------------------|
/// | `Store(StoreError::NotFound)` / `Jobs(JobsError::NotFound)` | `NotFound`       |
/// | `Store(StoreError::InvalidScope)`                    | `InvalidArgument` |
/// | `Vector(VectorError::NotFound)`                      | `NotFound`        |
/// | `Vector(VectorError::BadRequest)`                    | `InvalidArgument` |
/// | `Vector(VectorError::Connection)` / `Database`       | `Unavailable`     |
/// | `Store(StoreError::Database)` / `Jobs(...Database)`  | `Internal`        |
/// | `Embedding(_)` / `Llm(_)` / `Migration(_)`           | `Internal`        |
///
/// Wire messages are opaque ("internal error", "memory not found", etc.)
/// — never the inner error's `Display`. Full error context is logged
/// server-side at WARN/ERROR with structured fields so operators can
/// triage without exposing internals to callers.
pub(crate) fn client_error_to_status(err: ClientError) -> Status {
    match &err {
        ClientError::Store(StoreError::NotFound(pid)) => {
            tracing::debug!(error.kind = "store.not_found", memory.pid = %pid, "client error mapped to NOT_FOUND");
            Status::not_found("memory not found")
        }
        ClientError::Store(StoreError::InvalidScope(detail)) => {
            tracing::warn!(error.kind = "store.invalid_scope", error.detail = %detail, "client error mapped to INVALID_ARGUMENT");
            Status::invalid_argument("scope: agent_id, org_id, and user_id must all be non-empty")
        }
        ClientError::Store(StoreError::Database(detail)) => {
            tracing::error!(error.kind = "store.database", error.detail = %detail, "client error mapped to INTERNAL");
            Status::internal("internal error")
        }
        ClientError::Jobs(JobsError::NotFound(id)) => {
            tracing::debug!(error.kind = "jobs.not_found", job.id = %id, "client error mapped to NOT_FOUND");
            Status::not_found("job not found")
        }
        ClientError::Jobs(JobsError::Database(detail)) => {
            tracing::error!(error.kind = "jobs.database", error.detail = %detail, "client error mapped to INTERNAL");
            Status::internal("internal error")
        }
        ClientError::Vector(VectorError::NotFound(detail)) => {
            tracing::debug!(error.kind = "vector.not_found", error.detail = %detail, "client error mapped to NOT_FOUND");
            Status::not_found("vector index entry not found")
        }
        ClientError::Vector(VectorError::BadRequest(detail)) => {
            tracing::warn!(error.kind = "vector.bad_request", error.detail = %detail, "client error mapped to INVALID_ARGUMENT");
            Status::invalid_argument("invalid request to vector backend")
        }
        ClientError::Vector(VectorError::Connection(detail)) => {
            tracing::error!(error.kind = "vector.connection", error.detail = %detail, "client error mapped to UNAVAILABLE");
            Status::unavailable("vector backend unavailable")
        }
        ClientError::Database(detail) => {
            tracing::error!(error.kind = "database", error.detail = %detail, "client error mapped to UNAVAILABLE");
            Status::unavailable("database unavailable")
        }
        ClientError::Embedding(detail) => {
            tracing::error!(error.kind = "embedding", error.detail = %detail, "client error mapped to INTERNAL");
            Status::internal("internal error")
        }
        ClientError::Llm(detail) => {
            tracing::error!(error.kind = "llm", error.detail = %detail, "client error mapped to INTERNAL");
            Status::internal("internal error")
        }
        ClientError::Migration(detail) => {
            tracing::error!(error.kind = "migration", error.detail = %detail, "client error mapped to INTERNAL");
            Status::internal("internal error")
        }
    }
}

// ─── AdminService conversions ──────────────────────────────────────────────

/// Converts a library `JobKind` to the wire enum's discriminant.
pub(crate) fn job_kind_to_proto(kind: LibJobKind) -> ProtoJobKind {
    match kind {
        LibJobKind::Embed => ProtoJobKind::Embed,
        LibJobKind::Extract => ProtoJobKind::Extract,
    }
}

/// Converts an optional wire-side `JobKind` filter to the library shape.
///
/// `None` and `Some(JOB_KIND_UNSPECIFIED)` both mean "no filter, retry all
/// kinds" — matching the library's `RetryBuilder` default. Unknown
/// discriminants (e.g., a forward-compatible proto carrying a value this
/// build doesn't recognize) are rejected with `InvalidArgument`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the discriminant is set but
/// not a known [`ProtoJobKind`].
pub(crate) fn job_kind_filter_from_proto(value: Option<i32>) -> Result<Option<LibJobKind>, Status> {
    let Some(discriminant) = value else {
        return Ok(None);
    };
    let proto_kind = ProtoJobKind::try_from(discriminant)
        .map_err(|_| Status::invalid_argument(format!("of_kind: unknown JobKind discriminant {discriminant}")))?;
    match proto_kind {
        ProtoJobKind::Unspecified => Ok(None),
        ProtoJobKind::Embed => Ok(Some(LibJobKind::Embed)),
        ProtoJobKind::Extract => Ok(Some(LibJobKind::Extract)),
    }
}

/// Converts a library `FailedJob` row to the wire shape.
///
/// Mirrors the library type exactly per `admin.proto`'s `FailedJob`
/// message — id, source pid, kind, attempts, optional failure reason,
/// last update. The original job payload and the referenced memory's
/// content are **never** included; that PII boundary is enforced at
/// the library by the type's shape.
pub(crate) fn failed_job_to_proto(job: LibFailedJob) -> ProtoFailedJob {
    ProtoFailedJob {
        id: job.id,
        source_pid: job.source_pid,
        kind: job_kind_to_proto(job.kind) as i32,
        attempts: job.attempts,
        failure_reason: job.failure_reason,
        updated_at: Some(timestamp_from_chrono(job.updated_at)),
    }
}

/// Converts the library's `ReconcileSummary` to the wire `ReconcileResponse`.
///
/// All three counters are `usize` in the library; `int64` on the wire.
/// `usize as i64` is lossy only when `usize > i64::MAX` (impossible on
/// any practical 64-bit deployment — would require reconciling more
/// than 9 quintillion rows), so a saturating cast is safe enough; we
/// use `i64::try_from` defensively and return zero on overflow with a
/// loud warning so operators see something is wrong.
pub(crate) fn reconcile_summary_to_proto(summary: ReconcileSummary) -> ReconcileResponse {
    ReconcileResponse {
        failed_retried: usize_to_i64_saturating(summary.failed_retried, "failed_retried"),
        failed_recovered: usize_to_i64_saturating(summary.failed_recovered, "failed_recovered"),
        orphans_deleted: usize_to_i64_saturating(summary.orphans_deleted, "orphans_deleted"),
    }
}

/// Converts a library `u64` (e.g., `pending_jobs_count`) to wire `int64`.
///
/// # Errors
///
/// Returns [`Status::internal`] when the count exceeds `i64::MAX`
/// (physically impossible on any practical 64-bit deployment; surfaces
/// as a loud server-side error so operators can investigate any future
/// schema corruption that produces an absurd count).
pub(crate) fn u64_count_to_proto(count: u64, field: &'static str) -> Result<i64, Status> {
    i64::try_from(count).map_err(|_| {
        tracing::error!(
            error.kind = "count.overflow",
            field = field,
            count = count,
            "u64 count exceeds i64::MAX — wire conversion failed"
        );
        Status::internal("internal error")
    })
}

/// Internal helper: convert `usize → i64`, saturating + warning on overflow.
fn usize_to_i64_saturating(value: usize, field: &'static str) -> i64 {
    match i64::try_from(value) {
        Ok(v) => v,
        Err(_) => {
            tracing::warn!(
                error.kind = "count.overflow",
                field = field,
                value = value,
                "usize counter exceeds i64::MAX — wire conversion saturated to 0"
            );
            0
        }
    }
}

// ─── SearchBuilder filter conversions ──────────────────────────────────────

/// Converts a wire `MemoryFilter` into the library shape.
///
/// `None` (proto3 unset) maps to `None` on the library side — the search
/// path treats that as "no caller-supplied filter." A `Some` with all
/// sections empty round-trips as an inert filter; memoir-core handles both
/// the same way.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when any nested `FilterCondition`
/// has an unset condition oneof, an empty `field`, a `MatchValue` whose
/// `value` oneof is unset, or a `MatchValues` whose `values` oneof is unset.
pub(crate) fn metadata_filter_from_proto(filter: Option<ProtoMemoryFilter>) -> Result<Option<LibMemoryFilter>, Status> {
    let Some(filter) = filter else {
        return Ok(None);
    };
    let must = translate_condition_list(filter.must)?;
    let must_not = translate_condition_list(filter.must_not)?;
    let should = translate_condition_list(filter.should)?;
    Ok(Some(LibMemoryFilter { must, must_not, should }))
}

fn translate_condition_list(conditions: Vec<ProtoFilterCondition>) -> Result<Vec<LibFilterCondition>, Status> {
    conditions.into_iter().map(filter_condition_from_proto).collect()
}

fn filter_condition_from_proto(cond: ProtoFilterCondition) -> Result<LibFilterCondition, Status> {
    if cond.field.is_empty() {
        return Err(Status::invalid_argument(
            "metadata_filter: condition.field must be non-empty",
        ));
    }
    let inner = cond
        .condition
        .ok_or_else(|| Status::invalid_argument("metadata_filter: condition.condition oneof must be set"))?;
    Ok(match inner {
        filter_condition::Condition::Equals(value) => LibFilterCondition::Equals {
            field: cond.field,
            value: match_value_from_proto(value)?,
        },
        filter_condition::Condition::InValues(values) => LibFilterCondition::In {
            field: cond.field,
            values: match_values_from_proto(values)?,
        },
        filter_condition::Condition::Range(range) => LibFilterCondition::Range {
            field: cond.field,
            range: numeric_range_from_proto(range),
        },
    })
}

fn match_value_from_proto(value: ProtoMatchValue) -> Result<LibMatchValue, Status> {
    let inner = value
        .value
        .ok_or_else(|| Status::invalid_argument("metadata_filter: MatchValue.value oneof must be set"))?;
    Ok(match inner {
        match_value::Value::Keyword(s) => LibMatchValue::Keyword(s),
        match_value::Value::Integer(i) => LibMatchValue::Integer(i),
        match_value::Value::Boolean(b) => LibMatchValue::Bool(b),
    })
}

fn match_values_from_proto(values: ProtoMatchValues) -> Result<LibMatchValues, Status> {
    let inner = values
        .values
        .ok_or_else(|| Status::invalid_argument("metadata_filter: MatchValues.values oneof must be set"))?;
    Ok(match inner {
        match_values::Values::Keywords(list) => LibMatchValues::Keywords(list.values),
        match_values::Values::Integers(list) => LibMatchValues::Integers(list.values),
    })
}

fn numeric_range_from_proto(range: ProtoNumericRange) -> LibNumericRange {
    LibNumericRange {
        lt: range.lt,
        lte: range.lte,
        gt: range.gt,
        gte: range.gte,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_reject_unset_scope() {
        let err = scope_from_proto(None).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_scope_with_empty_agent_id() {
        let proto = ProtoScope {
            agent_id: String::new(),
            org_id: "o".into(),
            user_id: "u".into(),
        };
        let err = scope_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_round_trip_scope() {
        let original = LibScope {
            agent_id: "a".into(),
            org_id: "o".into(),
            user_id: "u".into(),
        };
        let proto = scope_to_proto(original.clone());
        let back = scope_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_default_unset_metadata_to_empty_object() {
        let result = metadata_from_proto(None).unwrap();
        assert_eq!(result, serde_json::json!({}));
    }

    #[test]
    fn should_round_trip_metadata_string_values_unchanged() {
        // Strings round-trip exactly through pbjson_types::Struct.
        let original = serde_json::json!({ "source": "test", "tag": "rust" });
        let proto = metadata_to_proto(original.clone());
        let back = metadata_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_round_trip_metadata_integers_as_floats() {
        // Proto3 `google.protobuf.Value.number_value` is `double` per spec.
        // Integers round-trip through `f64`, so `42` (integer) comes back as
        // `42.0` (float). This is a wire-level behavior — consumers needing
        // integer fidelity must encode them as strings.
        let original = serde_json::json!({ "count": 42 });
        let proto = metadata_to_proto(original);
        let back = metadata_from_proto(Some(proto)).unwrap();
        assert_eq!(back, serde_json::json!({ "count": 42.0 }));
    }

    #[test]
    fn should_preserve_nested_object_structure_through_metadata_round_trip() {
        // Deeply-nested objects retain their shape; only the leaf number
        // representation is affected (per the previous test).
        let original = serde_json::json!({
            "source": "test",
            "tags": ["one", "two"],
            "flags": { "enabled": true },
        });
        let proto = metadata_to_proto(original.clone());
        let back = metadata_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_reject_forget_request_with_unset_target() {
        let request = ForgetRequest {
            target: None,
            hard_delete: false,
        };
        let err = forget_target_from_proto(request).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_forget_request_with_empty_pid() {
        let request = ForgetRequest {
            target: Some(forget_request::Target::Pid(String::new())),
            hard_delete: false,
        };
        let err = forget_target_from_proto(request).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_accept_forget_request_with_pid() {
        let request = ForgetRequest {
            target: Some(forget_request::Target::Pid("abc".into())),
            hard_delete: false,
        };
        let target = forget_target_from_proto(request).unwrap();
        assert!(matches!(target, ForgetTarget::Pid(p) if p == "abc"));
    }

    #[test]
    fn should_map_store_not_found_to_grpc_not_found() {
        let err = ClientError::Store(StoreError::NotFound("abc".into()));
        let status = client_error_to_status(err);
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[test]
    fn should_map_store_invalid_scope_to_grpc_invalid_argument() {
        let err = ClientError::Store(StoreError::InvalidScope("empty".into()));
        let status = client_error_to_status(err);
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_vector_connection_to_grpc_unavailable() {
        let err = ClientError::Vector(VectorError::Connection("dial failed".into()));
        let status = client_error_to_status(err);
        assert_eq!(status.code(), tonic::Code::Unavailable);
    }

    #[test]
    fn should_map_jobs_not_found_to_grpc_not_found() {
        let err = ClientError::Jobs(JobsError::NotFound("42".into()));
        let status = client_error_to_status(err);
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[test]
    fn should_never_echo_database_error_detail_to_status_message() {
        let err = ClientError::Store(StoreError::Database(
            "connection string: postgres://user:s3cret@host/db".into(),
        ));
        let status = client_error_to_status(err);
        let message = status.message();
        assert!(!message.contains("s3cret"));
        assert!(!message.contains("postgres://"));
    }

    // ─── Admin conversion tests ────────────────────────────────────────────

    #[test]
    fn should_round_trip_job_kind_embed() {
        let lib = LibJobKind::Embed;
        let proto = job_kind_to_proto(lib);
        let back = job_kind_filter_from_proto(Some(proto as i32)).unwrap();
        assert_eq!(back, Some(LibJobKind::Embed));
    }

    #[test]
    fn should_round_trip_job_kind_extract() {
        let lib = LibJobKind::Extract;
        let proto = job_kind_to_proto(lib);
        let back = job_kind_filter_from_proto(Some(proto as i32)).unwrap();
        assert_eq!(back, Some(LibJobKind::Extract));
    }

    #[test]
    fn should_treat_unset_job_kind_filter_as_no_filter() {
        assert_eq!(job_kind_filter_from_proto(None).unwrap(), None);
    }

    #[test]
    fn should_treat_unspecified_job_kind_as_no_filter() {
        // JOB_KIND_UNSPECIFIED == 0; library treats this as "no filter".
        assert_eq!(job_kind_filter_from_proto(Some(0)).unwrap(), None);
    }

    #[test]
    fn should_reject_unknown_job_kind_discriminant() {
        let err = job_kind_filter_from_proto(Some(99)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_convert_failed_job_preserving_fields() {
        use chrono::{DateTime, FixedOffset};
        let updated_at: DateTime<FixedOffset> = "2026-05-19T10:00:00+00:00".parse().unwrap();
        let lib = LibFailedJob {
            id: 42,
            source_pid: "src_abc".into(),
            kind: LibJobKind::Embed,
            attempts: 3,
            failure_reason: Some("timeout".into()),
            updated_at,
        };
        let proto = failed_job_to_proto(lib);
        assert_eq!(proto.id, 42);
        assert_eq!(proto.source_pid, "src_abc");
        assert_eq!(proto.kind, ProtoJobKind::Embed as i32);
        assert_eq!(proto.attempts, 3);
        assert_eq!(proto.failure_reason.as_deref(), Some("timeout"));
        assert!(proto.updated_at.is_some());
    }

    #[test]
    fn should_convert_reconcile_summary_with_all_zeros() {
        let summary = ReconcileSummary {
            failed_retried: 0,
            failed_recovered: 0,
            orphans_deleted: 0,
        };
        let proto = reconcile_summary_to_proto(summary);
        assert_eq!(proto.failed_retried, 0);
        assert_eq!(proto.failed_recovered, 0);
        assert_eq!(proto.orphans_deleted, 0);
    }

    #[test]
    fn should_convert_reconcile_summary_preserving_counts() {
        let summary = ReconcileSummary {
            failed_retried: 5,
            failed_recovered: 3,
            orphans_deleted: 12,
        };
        let proto = reconcile_summary_to_proto(summary);
        assert_eq!(proto.failed_retried, 5);
        assert_eq!(proto.failed_recovered, 3);
        assert_eq!(proto.orphans_deleted, 12);
    }

    #[test]
    fn should_convert_u64_count_within_i64_range() {
        let count = u64_count_to_proto(1_000_000, "pending").unwrap();
        assert_eq!(count, 1_000_000);
    }

    #[test]
    fn should_reject_u64_count_overflowing_i64() {
        let err = u64_count_to_proto(u64::MAX, "pending").unwrap_err();
        assert_eq!(err.code(), tonic::Code::Internal);
    }

    #[test]
    fn should_reject_filter_condition_with_empty_field() {
        let proto = ProtoMemoryFilter {
            must: vec![ProtoFilterCondition {
                field: String::new(),
                condition: Some(filter_condition::Condition::Equals(ProtoMatchValue {
                    value: Some(match_value::Value::Keyword("x".into())),
                })),
            }],
            ..ProtoMemoryFilter::default()
        };
        let err = metadata_filter_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_filter_condition_with_unset_oneof() {
        let proto = ProtoMemoryFilter {
            must: vec![ProtoFilterCondition {
                field: "x".into(),
                condition: None,
            }],
            ..ProtoMemoryFilter::default()
        };
        let err = metadata_filter_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_match_value_with_unset_oneof() {
        let proto = ProtoMemoryFilter {
            must: vec![ProtoFilterCondition {
                field: "x".into(),
                condition: Some(filter_condition::Condition::Equals(ProtoMatchValue { value: None })),
            }],
            ..ProtoMemoryFilter::default()
        };
        let err = metadata_filter_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }
}
