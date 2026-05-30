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

use memoir_core::client::{DEFAULT_QUERY_LIMIT, DecayFn, MemoryContext, RankingStrategy, ReconcileSummary};
use memoir_core::jobs::{FailedJob as LibFailedJob, JobKind as LibJobKind};
use memoir_core::memory::{
    ForgetTarget, KindSelector as LibKindSelector, Memory as LibMemory, Scope as LibScope,
    SupersessionEvent as LibSupersessionEvent,
};
use memoir_core::store::{AsOfParams, DEFAULT_TIMELINE_LIMIT, TimelineDirection, TimelineParams};
use memoir_core::vector::{
    FilterCondition as LibFilterCondition, MatchValue as LibMatchValue, MatchValues as LibMatchValues,
    MemoryFilter as LibMemoryFilter, NumericRange as LibNumericRange,
};
use memoir_sdk::memoir::v1::{
    FailedJob as ProtoFailedJob, FilterCondition as ProtoFilterCondition, ForgetRequest, JobKind as ProtoJobKind,
    KindSelector as ProtoKindSelector, MatchValue as ProtoMatchValue, MatchValues as ProtoMatchValues,
    Decay as ProtoDecay, DecayBucket as ProtoDecayBucket, EditRequest, ExponentialDecay, Hybrid as ProtoHybrid,
    Memory as ProtoMemory, MemoryFilter as ProtoMemoryFilter, NumericRange as ProtoNumericRange, QueryHit,
    QueryRequest, QueryResponse, Ranking as ProtoRanking, RecallAsOfRequest, RecallAsOfResponse, ReconcileResponse,
    ReciprocalDecay, Scope as ProtoScope, StepDecay, SupersessionEvent as ProtoSupersessionEvent,
    SupersessionHistoryRequest, TimelineRequest, TimelineResponse, decay, filter_condition, forget_request,
    match_value, match_values, ranking,
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
/// Shim delegating to [`crate::services::wire::WireMemory`] — the canonical
/// conversion. Retained so the not-yet-migrated handlers (search / recall /
/// remember) keep compiling; ticket 0015 repoints them to `WireMemory` and
/// removes this shim.
pub(crate) fn memory_to_proto(memory: LibMemory) -> ProtoMemory {
    crate::services::wire::WireMemory::from(memory).0
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

/// Converts a wire `Timestamp` to a `chrono::DateTime<FixedOffset>` (UTC offset).
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the seconds/nanos do not form a
/// representable instant (out-of-range or invalid nanos).
fn timestamp_to_chrono(ts: pbjson_types::Timestamp) -> Result<chrono::DateTime<chrono::FixedOffset>, Status> {
    chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
        .map(|dt| dt.fixed_offset())
        .ok_or_else(|| Status::invalid_argument("timestamp: not a representable instant"))
}

/// Converts a wire `KindSelector` into the library shape.
///
/// `None` (omitted) and a both-false selector both map to
/// [`LibKindSelector::default`] — all kinds — matching the library's
/// "toggled neither = retrieve both" rule. Setting exactly one field filters
/// to that kind.
fn kind_selector_from_proto(kinds: Option<ProtoKindSelector>) -> LibKindSelector {
    match kinds {
        None
        | Some(ProtoKindSelector {
            episodic: false,
            semantic: false,
        }) => LibKindSelector::default(),
        Some(ProtoKindSelector { episodic, semantic }) => LibKindSelector { episodic, semantic },
    }
}

/// A validated `Timeline` request: scope plus query parameters.
///
/// `TryFrom<TimelineRequest>` performs boundary validation — the request's
/// shared-vocabulary fields (scope, timestamps, kinds) are checked and
/// converted here. The handler destructures this rather than juggling a
/// tuple.
#[derive(Debug)]
pub(crate) struct TimelineArgs {
    pub scope: LibScope,
    pub params: TimelineParams,
}

impl TryFrom<TimelineRequest> for TimelineArgs {
    type Error = Status;

    fn try_from(request: TimelineRequest) -> Result<Self, Status> {
        let limit = if request.limit > 0 {
            request.limit as usize
        } else {
            DEFAULT_TIMELINE_LIMIT
        };
        let direction = if request.ascending {
            TimelineDirection::Ascending
        } else {
            TimelineDirection::Descending
        };
        Ok(Self {
            scope: scope_from_proto(request.scope)?,
            params: TimelineParams {
                kinds: kind_selector_from_proto(request.kinds),
                created_after: request.created_after.map(timestamp_to_chrono).transpose()?,
                created_before: request.created_before.map(timestamp_to_chrono).transpose()?,
                event_at_after: request.event_at_after.map(timestamp_to_chrono).transpose()?,
                event_at_before: request.event_at_before.map(timestamp_to_chrono).transpose()?,
                include_superseded: !request.exclude_superseded,
                limit,
                direction,
            },
        })
    }
}

/// Wraps a library timeline result in the wire response shape.
pub(crate) fn timeline_response(memories: Vec<LibMemory>) -> TimelineResponse {
    use crate::services::wire::WireMemory;
    TimelineResponse {
        memories: memories.into_iter().map(|m| WireMemory::from(m).0).collect(),
    }
}

/// A validated `RecallAsOf` request: scope plus point-in-time parameters.
#[derive(Debug)]
pub(crate) struct RecallAsOfArgs {
    pub scope: LibScope,
    pub params: AsOfParams,
}

impl TryFrom<RecallAsOfRequest> for RecallAsOfArgs {
    type Error = Status;

    fn try_from(request: RecallAsOfRequest) -> Result<Self, Status> {
        let as_of = request
            .as_of
            .ok_or_else(|| Status::invalid_argument("as_of: required"))?;
        let limit = if request.limit > 0 {
            request.limit as usize
        } else {
            DEFAULT_TIMELINE_LIMIT
        };
        Ok(Self {
            scope: scope_from_proto(request.scope)?,
            params: AsOfParams {
                as_of: timestamp_to_chrono(as_of)?,
                kinds: kind_selector_from_proto(request.kinds),
                limit,
            },
        })
    }
}

/// Wraps a library recall-as-of result in the wire response shape.
pub(crate) fn recall_as_of_response(memories: Vec<LibMemory>) -> RecallAsOfResponse {
    use crate::services::wire::WireMemory;
    RecallAsOfResponse {
        memories: memories.into_iter().map(|m| WireMemory::from(m).0).collect(),
    }
}

/// Converts a wire `Duration` to a `chrono::Duration` at seconds granularity.
///
/// The library's decay functions operate on whole seconds (`num_seconds`),
/// so sub-second precision is intentionally dropped.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the value is out of
/// `chrono::Duration`'s representable range.
fn duration_to_chrono(d: pbjson_types::Duration) -> Result<chrono::Duration, Status> {
    chrono::Duration::try_seconds(d.seconds)
        .ok_or_else(|| Status::invalid_argument("duration: out of representable range"))
}

/// Converts a `chrono::Duration` to a wire `Duration` (seconds, no nanos).
fn duration_to_proto(d: chrono::Duration) -> pbjson_types::Duration {
    pbjson_types::Duration {
        seconds: d.num_seconds(),
        nanos: 0,
    }
}

/// Converts a wire `Ranking` to the library [`RankingStrategy`].
///
/// `None` (unset) maps to [`RankingStrategy::default_hybrid`], matching the
/// library's "no strategy = default hybrid" behavior.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when a `Hybrid` lacks its `decay`,
/// a `Decay` lacks its function, or a duration is out of range.
fn ranking_from_proto(proto: Option<ProtoRanking>) -> Result<RankingStrategy, Status> {
    let Some(ranking) = proto.and_then(|r| r.strategy) else {
        return Ok(RankingStrategy::default_hybrid());
    };
    match ranking {
        ranking::Strategy::Hybrid(h) => {
            let decay = h
                .decay
                .and_then(|d| d.function)
                .ok_or_else(|| Status::invalid_argument("ranking.hybrid.decay: required"))?;
            let decay = match decay {
                decay::Function::Exponential(e) => DecayFn::Exponential {
                    half_life: duration_to_chrono(
                        e.half_life
                            .ok_or_else(|| Status::invalid_argument("decay.exponential.half_life: required"))?,
                    )?,
                },
                decay::Function::Reciprocal(r) => DecayFn::Reciprocal {
                    scale: duration_to_chrono(
                        r.scale
                            .ok_or_else(|| Status::invalid_argument("decay.reciprocal.scale: required"))?,
                    )?,
                },
                decay::Function::Step(s) => {
                    let mut thresholds = Vec::with_capacity(s.buckets.len());
                    for bucket in s.buckets {
                        let boundary = duration_to_chrono(
                            bucket
                                .boundary
                                .ok_or_else(|| Status::invalid_argument("decay.step.bucket.boundary: required"))?,
                        )?;
                        thresholds.push((boundary, bucket.value));
                    }
                    DecayFn::Step { thresholds }
                }
            };
            Ok(RankingStrategy::Hybrid { alpha: h.alpha, decay })
        }
    }
}

/// Converts a library [`RankingStrategy`] back to the wire `Ranking`.
fn ranking_to_proto(strategy: &RankingStrategy) -> ProtoRanking {
    let strategy = match strategy {
        RankingStrategy::Hybrid { alpha, decay } => ranking::Strategy::Hybrid(ProtoHybrid {
            alpha: *alpha,
            decay: Some(ProtoDecay {
                function: Some(decay_fn_to_proto(decay)),
            }),
        }),
        // `RankingStrategy` is #[non_exhaustive]: a core build newer than this
        // service could carry a variant we can't represent. Treat it as
        // version skew and echo the default hybrid so the response stays
        // well-formed.
        other => {
            tracing::warn!(strategy = ?other, "unknown RankingStrategy variant; echoing default hybrid");
            return ranking_to_proto(&RankingStrategy::default_hybrid());
        }
    };
    ProtoRanking { strategy: Some(strategy) }
}

/// Converts a library [`DecayFn`] to the wire `Decay`'s oneof function.
fn decay_fn_to_proto(decay: &DecayFn) -> decay::Function {
    match decay {
        DecayFn::Exponential { half_life } => decay::Function::Exponential(ExponentialDecay {
            half_life: Some(duration_to_proto(*half_life)),
        }),
        DecayFn::Reciprocal { scale } => decay::Function::Reciprocal(ReciprocalDecay {
            scale: Some(duration_to_proto(*scale)),
        }),
        DecayFn::Step { thresholds } => decay::Function::Step(StepDecay {
            buckets: thresholds
                .iter()
                .map(|(boundary, value)| ProtoDecayBucket {
                    boundary: Some(duration_to_proto(*boundary)),
                    value: *value,
                })
                .collect(),
        }),
        // #[non_exhaustive] forward-compat: fall back to the default decay.
        other => {
            tracing::warn!(decay = ?other, "unknown DecayFn variant; echoing default exponential");
            decay::Function::Exponential(ExponentialDecay {
                half_life: Some(duration_to_proto(chrono::Duration::days(7))),
            })
        }
    }
}

/// A validated `Query` request: scope, query text, and all builder knobs.
#[derive(Debug)]
pub(crate) struct QueryArgs {
    pub query: String,
    pub scope: LibScope,
    pub limit: usize,
    pub kinds: memoir_core::memory::KindSelector,
    pub metadata_filter: Option<LibMemoryFilter>,
    pub min_similarity: Option<f32>,
    pub created_after: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub created_before: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub event_at_after: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub event_at_before: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub ranking: RankingStrategy,
}

impl TryFrom<QueryRequest> for QueryArgs {
    type Error = Status;

    fn try_from(request: QueryRequest) -> Result<Self, Status> {
        if request.query.is_empty() {
            return Err(Status::invalid_argument("query: required"));
        }
        let limit = if request.limit > 0 {
            request.limit as usize
        } else {
            DEFAULT_QUERY_LIMIT
        };
        Ok(Self {
            query: request.query,
            scope: scope_from_proto(request.scope)?,
            limit,
            kinds: kind_selector_from_proto(request.kinds),
            metadata_filter: metadata_filter_from_proto(request.metadata_filter)?,
            min_similarity: request.min_similarity,
            created_after: request.created_after.map(timestamp_to_chrono).transpose()?,
            created_before: request.created_before.map(timestamp_to_chrono).transpose()?,
            event_at_after: request.event_at_after.map(timestamp_to_chrono).transpose()?,
            event_at_before: request.event_at_before.map(timestamp_to_chrono).transpose()?,
            ranking: ranking_from_proto(request.ranking)?,
        })
    }
}

/// Wraps a library `MemoryContext` in the wire `QueryResponse`.
///
/// Each hit carries its hybrid score (from `Memory.score`, populated by
/// `query`). `ranking_used` echoes the strategy that produced the result.
pub(crate) fn query_response(context: MemoryContext) -> QueryResponse {
    use crate::services::wire::WireMemory;
    let ranking_used = Some(ranking_to_proto(context.strategy_used()));
    let hits = context
        .memories()
        .iter()
        .cloned()
        .map(|m| {
            let score = m.score.unwrap_or(0.0);
            QueryHit {
                memory: Some(WireMemory::from(m).0),
                score,
            }
        })
        .collect();
    QueryResponse { hits, ranking_used }
}

/// A validated `Edit` request: the target pid plus the optional field edits.
///
/// Each `Option` is "untouched when `None`, overwrite when `Some`", mirroring
/// the library's `EditBuilder`. Reserved-metadata-key validation runs
/// library-side in `Client::edit`; the handler maps the resulting error.
#[derive(Debug)]
pub(crate) struct EditArgs {
    pub pid: String,
    pub content: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub event_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl TryFrom<EditRequest> for EditArgs {
    type Error = Status;

    fn try_from(request: EditRequest) -> Result<Self, Status> {
        if request.pid.is_empty() {
            return Err(Status::invalid_argument("pid: required"));
        }
        let metadata = request
            .metadata
            .map(serde_json::to_value)
            .transpose()
            .map_err(|err| {
                tracing::warn!(error.message = %err, "rejected EditRequest with malformed metadata");
                Status::invalid_argument("metadata: not representable as JSON")
            })?;
        Ok(Self {
            pid: request.pid,
            content: request.content,
            metadata,
            event_at: request.event_at.map(timestamp_to_chrono).transpose()?,
        })
    }
}

// `ClientError → Status` lives in memoir-core behind the `grpc` feature.
// Call sites use `.map_err(Status::from)` or `?` against tonic boundaries.

/// A validated `SupersessionHistory` request: just the target pid.
#[derive(Debug)]
pub(crate) struct SupersessionHistoryArgs {
    pub pid: String,
}

impl TryFrom<SupersessionHistoryRequest> for SupersessionHistoryArgs {
    type Error = Status;

    fn try_from(request: SupersessionHistoryRequest) -> Result<Self, Status> {
        if request.pid.is_empty() {
            return Err(Status::invalid_argument("pid: required"));
        }
        Ok(Self { pid: request.pid })
    }
}

/// Wire form of a [`LibSupersessionEvent`]. Build via `WireSupersessionEvent::from(event)`.
pub(crate) struct WireSupersessionEvent(pub ProtoSupersessionEvent);

impl From<LibSupersessionEvent> for WireSupersessionEvent {
    fn from(event: LibSupersessionEvent) -> Self {
        Self(ProtoSupersessionEvent {
            winner_pid: event.winner_pid,
            decided_at: Some(timestamp_from_chrono(event.decided_at)),
        })
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
    use memoir_core::client::ClientError;
    use memoir_core::jobs::JobsError;
    use memoir_core::store::StoreError;
    use memoir_core::vector::VectorError;

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
        let status = crate::services::wire::WireError::into_status(err);
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[test]
    fn should_map_store_invalid_scope_to_grpc_invalid_argument() {
        let err = ClientError::Store(StoreError::InvalidScope("empty".into()));
        let status = crate::services::wire::WireError::into_status(err);
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_vector_connection_to_grpc_unavailable() {
        let err = ClientError::Vector(VectorError::Connection("dial failed".into()));
        let status = crate::services::wire::WireError::into_status(err);
        assert_eq!(status.code(), tonic::Code::Unavailable);
    }

    #[test]
    fn should_map_jobs_not_found_to_grpc_not_found() {
        let err = ClientError::Jobs(JobsError::NotFound("42".into()));
        let status = crate::services::wire::WireError::into_status(err);
        assert_eq!(status.code(), tonic::Code::NotFound);
    }

    #[test]
    fn should_never_echo_database_error_detail_to_status_message() {
        let err = ClientError::Store(StoreError::Database(sea_orm::DbErr::Custom(
            "connection string: postgres://user:s3cret@host/db".into(),
        )));
        let status = crate::services::wire::WireError::into_status(err);
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

    fn timeline_scope() -> ProtoScope {
        ProtoScope {
            agent_id: "a".into(),
            org_id: "o".into(),
            user_id: "u".into(),
        }
    }

    /// Test helper: convert a request and project to its `TimelineParams`.
    fn timeline_args(request: TimelineRequest) -> Result<TimelineParams, Status> {
        TimelineArgs::try_from(request).map(|args| args.params)
    }

    #[test]
    fn should_map_omitted_kind_selector_to_all_kinds() {
        let selector = kind_selector_from_proto(None);
        assert!(selector.episodic && selector.semantic, "omitted selector = all kinds");
    }

    #[test]
    fn should_map_both_false_kind_selector_to_all_kinds() {
        let selector = kind_selector_from_proto(Some(ProtoKindSelector {
            episodic: false,
            semantic: false,
        }));
        assert!(
            selector.episodic && selector.semantic,
            "both-false selector = all kinds, matching the library's neither-toggled rule",
        );
    }

    #[test]
    fn should_map_single_kind_selector_to_that_kind_only() {
        let selector = kind_selector_from_proto(Some(ProtoKindSelector {
            episodic: true,
            semantic: false,
        }));
        assert!(selector.episodic && !selector.semantic);
    }

    #[test]
    fn should_default_include_superseded_when_exclude_flag_false() {
        let params = timeline_args(TimelineRequest {
            scope: Some(timeline_scope()),
            exclude_superseded: false,
            ..TimelineRequest::default()
        })
        .unwrap();
        assert!(
            params.include_superseded,
            "exclude_superseded=false must map to include_superseded=true (audit-view default)",
        );
    }

    #[test]
    fn should_exclude_superseded_when_exclude_flag_true() {
        let params = timeline_args(TimelineRequest {
            scope: Some(timeline_scope()),
            exclude_superseded: true,
            ..TimelineRequest::default()
        })
        .unwrap();
        assert!(!params.include_superseded);
    }

    #[test]
    fn should_default_to_descending_when_ascending_flag_false() {
        let params = timeline_args(TimelineRequest {
            scope: Some(timeline_scope()),
            ascending: false,
            ..TimelineRequest::default()
        })
        .unwrap();
        assert_eq!(params.direction, TimelineDirection::Descending);
    }

    #[test]
    fn should_map_ascending_flag_to_ascending_direction() {
        let params = timeline_args(TimelineRequest {
            scope: Some(timeline_scope()),
            ascending: true,
            ..TimelineRequest::default()
        })
        .unwrap();
        assert_eq!(params.direction, TimelineDirection::Ascending);
    }

    #[test]
    fn should_map_zero_limit_to_library_default() {
        let params = timeline_args(TimelineRequest {
            scope: Some(timeline_scope()),
            limit: 0,
            ..TimelineRequest::default()
        })
        .unwrap();
        assert_eq!(params.limit, DEFAULT_TIMELINE_LIMIT);
    }

    #[test]
    fn should_reject_timeline_with_unset_scope() {
        let err = TimelineArgs::try_from(TimelineRequest::default()).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    fn as_of_ts() -> pbjson_types::Timestamp {
        pbjson_types::Timestamp { seconds: 1_900_000_000, nanos: 0 }
    }

    #[test]
    fn should_reject_recall_as_of_with_unset_as_of() {
        let err = RecallAsOfArgs::try_from(RecallAsOfRequest {
            scope: Some(timeline_scope()),
            as_of: None,
            ..RecallAsOfRequest::default()
        })
        .unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_recall_as_of_with_unset_scope() {
        let err = RecallAsOfArgs::try_from(RecallAsOfRequest {
            scope: None,
            as_of: Some(as_of_ts()),
            ..RecallAsOfRequest::default()
        })
        .unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_recall_as_of_zero_limit_to_library_default() {
        let args = RecallAsOfArgs::try_from(RecallAsOfRequest {
            scope: Some(timeline_scope()),
            as_of: Some(as_of_ts()),
            limit: 0,
            ..RecallAsOfRequest::default()
        })
        .unwrap();
        assert_eq!(args.params.limit, DEFAULT_TIMELINE_LIMIT);
    }

    #[test]
    fn should_map_recall_as_of_both_false_kinds_to_all() {
        let args = RecallAsOfArgs::try_from(RecallAsOfRequest {
            scope: Some(timeline_scope()),
            as_of: Some(as_of_ts()),
            kinds: Some(ProtoKindSelector { episodic: false, semantic: false }),
            ..RecallAsOfRequest::default()
        })
        .unwrap();
        assert!(args.params.kinds.episodic && args.params.kinds.semantic);
    }

    #[test]
    fn should_default_ranking_to_hybrid_when_unset() {
        let strategy = ranking_from_proto(None).unwrap();
        assert_eq!(strategy, RankingStrategy::default_hybrid());
    }

    #[test]
    fn should_round_trip_hybrid_exponential_ranking() {
        let original = RankingStrategy::Hybrid {
            alpha: 0.6,
            decay: DecayFn::Exponential {
                half_life: chrono::Duration::days(3),
            },
        };
        let proto = ranking_to_proto(&original);
        let back = ranking_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_round_trip_hybrid_step_ranking() {
        let original = RankingStrategy::Hybrid {
            alpha: 0.4,
            decay: DecayFn::Step {
                thresholds: vec![
                    (chrono::Duration::hours(1), 1.0),
                    (chrono::Duration::days(1), 0.5),
                ],
            },
        };
        let proto = ranking_to_proto(&original);
        let back = ranking_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_reject_hybrid_ranking_missing_decay() {
        let proto = ProtoRanking {
            strategy: Some(ranking::Strategy::Hybrid(ProtoHybrid { alpha: 0.5, decay: None })),
        };
        let err = ranking_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_query_with_empty_query_string() {
        let err = QueryArgs::try_from(QueryRequest {
            scope: Some(timeline_scope()),
            query: String::new(),
            ..QueryRequest::default()
        })
        .unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_query_zero_limit_to_library_default() {
        let args = QueryArgs::try_from(QueryRequest {
            scope: Some(timeline_scope()),
            query: "hello".into(),
            limit: 0,
            ..QueryRequest::default()
        })
        .unwrap();
        assert_eq!(args.limit, DEFAULT_QUERY_LIMIT);
        assert_eq!(args.ranking, RankingStrategy::default_hybrid());
    }

    #[test]
    fn should_reject_edit_with_empty_pid() {
        let err = EditArgs::try_from(EditRequest {
            pid: String::new(),
            content: Some("x".into()),
            ..EditRequest::default()
        })
        .unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_empty_edit_request_to_all_none_patch() {
        let args = EditArgs::try_from(EditRequest {
            pid: "p1".into(),
            ..EditRequest::default()
        })
        .unwrap();
        assert!(args.content.is_none() && args.metadata.is_none() && args.event_at.is_none());
    }

    #[test]
    fn should_pass_through_edit_fields_when_set() {
        let args = EditArgs::try_from(EditRequest {
            pid: "p1".into(),
            content: Some("new".into()),
            event_at: Some(as_of_ts()),
            ..EditRequest::default()
        })
        .unwrap();
        assert_eq!(args.content.as_deref(), Some("new"));
        assert!(args.event_at.is_some());
    }

    #[test]
    fn should_reject_supersession_history_request_with_empty_pid() {
        let err = SupersessionHistoryArgs::try_from(SupersessionHistoryRequest { pid: String::new() }).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_map_unsupersede_event_to_proto_with_unset_winner() {
        let event = LibSupersessionEvent {
            winner_pid: None,
            decided_at: chrono::Utc::now().into(),
        };
        let wire: WireSupersessionEvent = event.into();
        assert!(wire.0.winner_pid.is_none(), "unsupersede preserves None on the wire");
        assert!(wire.0.decided_at.is_some());
    }
}
