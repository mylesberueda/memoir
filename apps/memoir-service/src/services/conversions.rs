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
//! - **Metadata bridges serde and the wire `Struct`.** `pbjson_types::Struct`
//!   and `serde_json::Value` both implement `Serialize`/`Deserialize`; the
//!   `serde_json::to_value` / `serde_json::from_value` pair is the canonical
//!   conversion path per the pbjson-types maintainer. Inbound metadata flows
//!   through [`Metadata`], which additionally narrows the integers proto3
//!   widens to `f64`; outbound is a direct `from_value` at the wire layer.

use std::ops::Deref;

use memoir_core::client::{DEFAULT_QUERY_LIMIT, DecayFn, MemoryContext, RankingStrategy, ReconcileSummary};
use memoir_core::graph::GraphContext as LibGraphContext;
use memoir_core::graph::GraphSnapshot as LibGraphSnapshot;
use memoir_core::jobs::{FailedJob as LibFailedJob, JobKind as LibJobKind};
use memoir_core::memory::{
    ExtractionStat as LibExtractionStat, ForgetTarget, KindSelector as LibKindSelector, Memory as LibMemory,
    Scope as LibScope, StatsFilter, SupersessionEvent as LibSupersessionEvent,
};
use memoir_core::store::{AsOfParams, DEFAULT_TIMELINE_LIMIT, TimelineDirection, TimelineParams};
use memoir_core::vector::{
    FilterCondition as LibFilterCondition, MatchValue as LibMatchValue, MatchValues as LibMatchValues,
    MemoryFilter as LibMemoryFilter, NumericRange as LibNumericRange,
};
use memoir_sdk::memoir::v1::{
    BlendWeights as ProtoBlendWeights, Blended as ProtoBlended, Decay as ProtoDecay, DecayBucket as ProtoDecayBucket,
    EditRequest, ExponentialDecay, ExtractionStat as ProtoExtractionStat, ExtractionStatsRequest,
    FailedJob as ProtoFailedJob, FeedbackRequest, FilterCondition as ProtoFilterCondition, ForgetRequest,
    GraphEdge as ProtoGraphEdge, GraphEnrichment as ProtoGraphEnrichment, GraphEntity as ProtoGraphEntity,
    GraphNode as ProtoGraphNode, GraphRelationship as ProtoGraphRelationship, Hybrid as ProtoHybrid,
    InspectGraphResponse, JobKind as ProtoJobKind, KindSelector as ProtoKindSelector, MatchValue as ProtoMatchValue,
    MatchValues as ProtoMatchValues, MemoryFilter as ProtoMemoryFilter, NumericRange as ProtoNumericRange, QueryHit,
    QueryRequest, QueryResponse, Ranking as ProtoRanking, RecallAsOfRequest, RecallAsOfResponse, ReciprocalDecay,
    ReconcileResponse, RetryFailedJobsRequest, Scope as ProtoScope, StepDecay,
    SupersessionEvent as ProtoSupersessionEvent, SupersessionHistoryRequest, TimelineRequest, TimelineResponse, decay,
    filter_condition, forget_request, match_value, match_values, ranking,
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

/// Consumer-supplied memory metadata at the gRPC boundary.
///
/// A thin wrapper over `serde_json::Value` — metadata is opaque, consumer-owned
/// JSON, so this newtype carries no schema. It exists solely to own the one
/// invariant the wire boundary needs: `google.protobuf.Struct` encodes every
/// number as an IEEE-754 double, so an inbound integer (`conversation_id: 1`)
/// arrives as `1.0`. Stored that way, an integer payload filter would never
/// match it. [`Metadata::try_from`] normalizes whole-number doubles back to
/// integers so consumer filters behave as written. `Deref`s to the inner
/// value so call sites treat it as a plain `serde_json::Value`.
#[derive(Debug)]
pub(crate) struct Metadata(serde_json::Value);

impl Deref for Metadata {
    type Target = serde_json::Value;

    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}

impl Metadata {
    /// Consumes the wrapper, yielding the inner value for the library call.
    pub(crate) fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

impl TryFrom<Option<pbjson_types::Struct>> for Metadata {
    type Error = Status;

    /// Converts inbound wire metadata, normalizing widened integers.
    ///
    /// `None` (proto3 unset) maps to `{}` to match the library's column
    /// default. Non-object payloads (arrays, scalars) are accepted — JSONB
    /// doesn't constrain shape.
    ///
    /// # Errors
    ///
    /// Returns [`Status::invalid_argument`] when the struct cannot be
    /// represented as JSON (e.g., contains `NaN`).
    fn try_from(meta: Option<pbjson_types::Struct>) -> Result<Self, Status> {
        let Some(struct_value) = meta else {
            return Ok(Self(serde_json::json!({})));
        };
        let mut value = serde_json::to_value(struct_value).map_err(|err| {
            tracing::warn!(error.message = %err, "rejected metadata that is not representable as JSON");
            Status::invalid_argument("metadata: not representable as JSON")
        })?;
        narrow_whole_number_doubles(&mut value);
        Ok(Self(value))
    }
}

/// Rewrites every top-level whole-number `f64` in a JSON object to an `i64`.
///
/// Only top-level keys are flattened into the Qdrant payload and become
/// filterable (see `qdrant::QdrantIndex::upsert`), so only they need to match
/// the integer type a consumer filters with. Nested numbers are left as serde
/// parsed them. A non-object value (array/scalar) is a no-op.
fn narrow_whole_number_doubles(value: &mut serde_json::Value) {
    let Some(object) = value.as_object_mut() else {
        return;
    };
    for field in object.values_mut() {
        let Some(float) = field.as_f64() else { continue };
        if field.is_f64() && float.fract() == 0.0 && i64::try_from(float as i128).is_ok() {
            *field = serde_json::Value::from(float as i64);
        }
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
            let decay = decay_from_proto(h.decay, "ranking.hybrid.decay")?;
            Ok(RankingStrategy::Hybrid { alpha: h.alpha, decay })
        }
        ranking::Strategy::Blended(b) => {
            let decay = decay_from_proto(b.decay, "ranking.blended.decay")?;
            let w = b
                .weights
                .ok_or_else(|| Status::invalid_argument("ranking.blended.weights: required"))?;
            let weights = memoir_core::client::BlendWeights {
                cosine: w.cosine,
                confidence: w.confidence,
                recency: w.recency,
                category_bonus: w.category_bonus,
                preferred_categories: w.preferred_categories,
            };
            Ok(RankingStrategy::Blended { weights, decay })
        }
    }
}

/// Parses a wire `Decay` (required) into a library [`DecayFn`].
///
/// `field` names the parent for error messages (e.g. `ranking.hybrid.decay`).
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the decay or its function is
/// unset, a duration field is missing, or a duration is out of range.
fn decay_from_proto(decay: Option<ProtoDecay>, field: &str) -> Result<DecayFn, Status> {
    let function = decay
        .and_then(|d| d.function)
        .ok_or_else(|| Status::invalid_argument(format!("{field}: required")))?;
    Ok(match function {
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
    })
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
        RankingStrategy::Blended { weights, decay } => ranking::Strategy::Blended(ProtoBlended {
            weights: Some(ProtoBlendWeights {
                cosine: weights.cosine,
                confidence: weights.confidence,
                recency: weights.recency,
                category_bonus: weights.category_bonus,
                preferred_categories: weights.preferred_categories.clone(),
            }),
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
    ProtoRanking {
        strategy: Some(strategy),
    }
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
    pub with_graph_enrichment: bool,
    pub graph_depth: Option<usize>,
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
            metadata_filter: request
                .metadata_filter
                .map(WireMemoryFilter::try_from)
                .transpose()?
                .map(WireMemoryFilter::into_inner),
            min_similarity: request.min_similarity,
            created_after: request.created_after.map(timestamp_to_chrono).transpose()?,
            created_before: request.created_before.map(timestamp_to_chrono).transpose()?,
            event_at_after: request.event_at_after.map(timestamp_to_chrono).transpose()?,
            event_at_before: request.event_at_before.map(timestamp_to_chrono).transpose()?,
            ranking: ranking_from_proto(request.ranking)?,
            with_graph_enrichment: request.with_graph_enrichment,
            graph_depth: (request.graph_depth > 0).then_some(request.graph_depth as usize),
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
    let enrichment = WireGraphEnrichment::from(context.graph()).into_inner();
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
    QueryResponse {
        hits,
        ranking_used,
        enrichment,
    }
}

/// Wire form of a graph [`LibGraphContext`]. Build via `WireGraphEnrichment::from(context)`.
///
/// The inner `Option` is `None` for an empty context (no graph configured,
/// enrichment not requested, or no neighbors found) so the wire field stays
/// absent rather than carrying an empty message — the one converter that
/// collapses empty to absent. The entity/relationship shapes map one-to-one. The
/// newtype satisfies coherence — both `LibGraphContext` and
/// `Option<ProtoGraphEnrichment>` are foreign to this crate, so the `From` impl
/// needs a local anchor (same pattern as [`WireInspectGraphResponse`]).
pub(crate) struct WireGraphEnrichment(pub Option<ProtoGraphEnrichment>);

impl From<&LibGraphContext> for WireGraphEnrichment {
    fn from(context: &LibGraphContext) -> Self {
        if context.is_empty() {
            return Self(None);
        }
        let entities = context
            .entities
            .iter()
            .map(|entity| ProtoGraphEntity {
                name: entity.name.clone(),
            })
            .collect();
        let relationships = context
            .relationships
            .iter()
            .map(|relationship| ProtoGraphRelationship {
                subject: relationship.subject.clone(),
                relation: relationship.relation.clone(),
                object: relationship.object.clone(),
                confidence: relationship.confidence,
            })
            .collect();
        Self(Some(ProtoGraphEnrichment {
            entities,
            relationships,
        }))
    }
}

impl Deref for WireGraphEnrichment {
    type Target = Option<ProtoGraphEnrichment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireGraphEnrichment {
    /// Consumes the wrapper, yielding the inner wire enrichment.
    pub(crate) fn into_inner(self) -> Option<ProtoGraphEnrichment> {
        self.0
    }
}

/// Wire form of a [`LibGraphSnapshot`]. Build via `WireInspectGraphResponse::from(snapshot)`.
///
/// The whole snapshot maps one-to-one — an empty graph is a valid response, so
/// there is no empty-to-`None` collapse (unlike [`WireGraphEnrichment`], whose
/// inner `Option` goes absent for an empty context). Library `Option<String>`
/// timestamps map to proto `optional string` directly. The newtype satisfies
/// coherence — both `LibGraphSnapshot` and `InspectGraphResponse` are foreign to
/// this crate, so the `From` impl needs a local anchor (same pattern as
/// [`WireReconcileResponse`]).
pub(crate) struct WireInspectGraphResponse(pub InspectGraphResponse);

impl From<LibGraphSnapshot> for WireInspectGraphResponse {
    fn from(snapshot: LibGraphSnapshot) -> Self {
        let nodes = snapshot
            .nodes
            .into_iter()
            .map(|node| ProtoGraphNode {
                name: node.name,
                memory_pids: node.memory_pids,
                first_seen_at: node.first_seen_at,
            })
            .collect();
        let edges = snapshot
            .edges
            .into_iter()
            .map(|edge| ProtoGraphEdge {
                subject: edge.subject,
                relation: edge.relation,
                object: edge.object,
                confidence: edge.confidence,
                valid_from: edge.valid_from,
                valid_to: edge.valid_to,
                memory_pids: edge.memory_pids,
            })
            .collect();
        Self(InspectGraphResponse {
            nodes,
            edges,
            truncated: snapshot.truncated,
        })
    }
}

impl Deref for WireInspectGraphResponse {
    type Target = InspectGraphResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireInspectGraphResponse {
    /// Consumes the wrapper, yielding the inner wire response.
    pub(crate) fn into_inner(self) -> InspectGraphResponse {
        self.0
    }
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
            .map(|struct_value| Metadata::try_from(Some(struct_value)).map(Metadata::into_inner))
            .transpose()?;
        Ok(Self {
            pid: request.pid,
            content: request.content,
            metadata,
            event_at: request.event_at.map(timestamp_to_chrono).transpose()?,
        })
    }
}

// `ClientError → Status` is owned by `services/wire::WireError`.
// Call sites use `.map_err(WireError::into_status)?`.

/// A validated `Feedback` request: the wrong semantic pid + optional correction.
///
/// Target-kind validation (must be a semantic row with an episodic source)
/// runs library-side in `Client::feedback`; the handler maps the resulting
/// `NotCorrectable` error.
#[derive(Debug)]
pub(crate) struct FeedbackArgs {
    pub pid: String,
    pub correction: Option<String>,
}

impl TryFrom<FeedbackRequest> for FeedbackArgs {
    type Error = Status;

    fn try_from(request: FeedbackRequest) -> Result<Self, Status> {
        if request.pid.is_empty() {
            return Err(Status::invalid_argument("pid: required"));
        }
        Ok(Self {
            pid: request.pid,
            correction: request.correction,
        })
    }
}

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

/// Wire form of a [`LibJobKind`]. Build via `WireJobKind::from(kind)`.
pub(crate) struct WireJobKind(pub ProtoJobKind);

impl From<LibJobKind> for WireJobKind {
    fn from(kind: LibJobKind) -> Self {
        Self(match kind {
            LibJobKind::Embed => ProtoJobKind::Embed,
            LibJobKind::Extract => ProtoJobKind::Extract,
            LibJobKind::Categorize => ProtoJobKind::Categorize,
            LibJobKind::Reprocess => ProtoJobKind::Reprocess,
            LibJobKind::RelationalExtract => ProtoJobKind::RelationalExtract,
            LibJobKind::Synthesize => ProtoJobKind::Synthesize,
        })
    }
}

/// Validated args for `Client::retry_failed_jobs`. Build via `WireRetryArgs::try_from(request)`.
///
/// `of_kind` resolves the wire discriminant to an optional library filter:
/// `None` and `Some(JOB_KIND_UNSPECIFIED)` both mean "no filter, retry all
/// kinds" (matching the library's `RetryBuilder` default); an unknown
/// discriminant (a forward-compatible proto carrying a value this build doesn't
/// recognize) is rejected. `dry_run` passes through. Anchors on the request
/// message like [`QueryArgs`]/[`TimelineArgs`].
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when `of_kind` is set but not a known
/// [`ProtoJobKind`].
#[derive(Debug)]
pub(crate) struct WireRetryArgs {
    pub of_kind: Option<LibJobKind>,
    pub dry_run: bool,
}

impl TryFrom<RetryFailedJobsRequest> for WireRetryArgs {
    type Error = Status;

    fn try_from(request: RetryFailedJobsRequest) -> Result<Self, Status> {
        let of_kind = match request.of_kind {
            None => None,
            Some(discriminant) => {
                let proto_kind = ProtoJobKind::try_from(discriminant).map_err(|_| {
                    Status::invalid_argument(format!("of_kind: unknown JobKind discriminant {discriminant}"))
                })?;
                match proto_kind {
                    ProtoJobKind::Unspecified => None,
                    ProtoJobKind::Embed => Some(LibJobKind::Embed),
                    ProtoJobKind::Extract => Some(LibJobKind::Extract),
                    ProtoJobKind::Categorize => Some(LibJobKind::Categorize),
                    ProtoJobKind::Reprocess => Some(LibJobKind::Reprocess),
                    ProtoJobKind::RelationalExtract => Some(LibJobKind::RelationalExtract),
                    ProtoJobKind::Synthesize => Some(LibJobKind::Synthesize),
                }
            }
        };
        Ok(Self {
            of_kind,
            dry_run: request.dry_run,
        })
    }
}

/// Wire form of a [`LibFailedJob`]. Build via `WireFailedJob::from(job)`.
///
/// Mirrors the library type exactly per `admin.proto`'s `FailedJob`
/// message. The original job payload and the referenced memory's content
/// are **never** included; that PII boundary is enforced at the library
/// by the type's shape.
pub(crate) struct WireFailedJob(pub ProtoFailedJob);

impl From<LibFailedJob> for WireFailedJob {
    fn from(job: LibFailedJob) -> Self {
        Self(ProtoFailedJob {
            id: job.id,
            source_pid: job.source_pid,
            kind: WireJobKind::from(job.kind).0 as i32,
            attempts: job.attempts,
            failure_reason: job.failure_reason,
            updated_at: Some(timestamp_from_chrono(job.updated_at)),
        })
    }
}

/// Wire form of a [`ReconcileSummary`]. Build via `WireReconcileResponse::from(summary)`.
///
/// All three counters are `usize` in the library; `int64` on the wire.
/// `usize as i64` is lossy only when `usize > i64::MAX` (impossible on
/// any practical 64-bit deployment — would require reconciling more
/// than 9 quintillion rows), so a saturating cast is safe enough; we
/// use `i64::try_from` defensively and return zero on overflow with a
/// loud warning so operators see something is wrong.
pub(crate) struct WireReconcileResponse(pub ReconcileResponse);

impl From<ReconcileSummary> for WireReconcileResponse {
    fn from(summary: ReconcileSummary) -> Self {
        Self(ReconcileResponse {
            failed_retried: usize_to_i64_saturating(summary.failed_retried, "failed_retried"),
            failed_recovered: usize_to_i64_saturating(summary.failed_recovered, "failed_recovered"),
            orphans_deleted: usize_to_i64_saturating(summary.orphans_deleted, "orphans_deleted"),
        })
    }
}

/// Wire form of a [`StatsFilter`]. Build via `WireStatsFilter::from(request)`.
///
/// Each proto `optional string` maps one-to-one to an `Option<String>` filter
/// dimension; there is nothing to reject (an all-unset request is the valid
/// "aggregate the whole store" case), so the conversion is infallible. The
/// newtype exists to satisfy coherence — both `ExtractionStatsRequest` and
/// `StatsFilter` are foreign to this crate, so the `From` impl needs a local
/// anchor (same pattern as [`WireExtractionStat`]).
pub(crate) struct WireStatsFilter(pub StatsFilter);

impl From<ExtractionStatsRequest> for WireStatsFilter {
    fn from(request: ExtractionStatsRequest) -> Self {
        Self(StatsFilter {
            agent_id: request.agent_id,
            org_id: request.org_id,
            user_id: request.user_id,
        })
    }
}

/// Wire form of a [`LibExtractionStat`]. Build via `WireExtractionStat::try_from(stat)`.
///
/// `total` and `rejected` are `u64` in the library, `int64` on the wire — the
/// fallible `u64_count_to_proto` guards the (physically impossible) overflow,
/// so the conversion is `TryFrom`, not `From` (the infallible-`From` sibling
/// newtypes like [`WireFailedJob`] have no such cast). `accuracy` is recomputed
/// library-side and echoed for wire consumers.
pub(crate) struct WireExtractionStat(pub ProtoExtractionStat);

impl TryFrom<LibExtractionStat> for WireExtractionStat {
    type Error = Status;

    fn try_from(stat: LibExtractionStat) -> Result<Self, Status> {
        let accuracy = stat.accuracy();
        Ok(Self(ProtoExtractionStat {
            provider: stat.provider,
            model: stat.model,
            total: u64_count_to_proto(stat.total, "extraction_stats.total")?,
            rejected: u64_count_to_proto(stat.rejected, "extraction_stats.rejected")?,
            accuracy,
        }))
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

/// Wire form of a [`LibMemoryFilter`]. Build via `WireMemoryFilter::try_from(filter)`.
///
/// A caller's unset filter (`None`) is the caller's concern — `.map().transpose()`
/// the `Option` at the call site; this converts a present `ProtoMemoryFilter`. A
/// filter with all sections empty round-trips as an inert filter; memoir-core
/// handles that the same as no filter. The newtype anchors the impl on the
/// foreign `ProtoMemoryFilter` message (same pattern as [`WireFailedJob`]).
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when any nested `FilterCondition`
/// has an unset condition oneof, an empty `field`, a `MatchValue` whose
/// `value` oneof is unset, or a `MatchValues` whose `values` oneof is unset.
#[derive(Debug)]
pub(crate) struct WireMemoryFilter(pub LibMemoryFilter);

impl TryFrom<ProtoMemoryFilter> for WireMemoryFilter {
    type Error = Status;

    fn try_from(filter: ProtoMemoryFilter) -> Result<Self, Status> {
        let translate = |conds: Vec<ProtoFilterCondition>| {
            conds
                .into_iter()
                .map(|c| WireFilterCondition::try_from(c).map(WireFilterCondition::into_inner))
                .collect::<Result<Vec<_>, _>>()
        };
        Ok(Self(LibMemoryFilter {
            must: translate(filter.must)?,
            must_not: translate(filter.must_not)?,
            should: translate(filter.should)?,
        }))
    }
}

impl Deref for WireMemoryFilter {
    type Target = LibMemoryFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireMemoryFilter {
    /// Consumes the wrapper, yielding the inner library filter.
    pub(crate) fn into_inner(self) -> LibMemoryFilter {
        self.0
    }
}

/// Wire form of a [`LibFilterCondition`]. Build via `WireFilterCondition::try_from(cond)`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when `field` is empty or the
/// `condition` oneof is unset, or when a nested value fails to convert.
struct WireFilterCondition(LibFilterCondition);

impl TryFrom<ProtoFilterCondition> for WireFilterCondition {
    type Error = Status;

    fn try_from(cond: ProtoFilterCondition) -> Result<Self, Status> {
        if cond.field.is_empty() {
            return Err(Status::invalid_argument(
                "metadata_filter: condition.field must be non-empty",
            ));
        }
        let inner = cond
            .condition
            .ok_or_else(|| Status::invalid_argument("metadata_filter: condition.condition oneof must be set"))?;
        Ok(Self(match inner {
            filter_condition::Condition::Equals(value) => LibFilterCondition::Equals {
                field: cond.field,
                value: WireMatchValue::try_from(value)?.into_inner(),
            },
            filter_condition::Condition::InValues(values) => LibFilterCondition::In {
                field: cond.field,
                values: WireMatchValues::try_from(values)?.into_inner(),
            },
            filter_condition::Condition::Range(range) => LibFilterCondition::Range {
                field: cond.field,
                range: WireNumericRange::from(range).into_inner(),
            },
        }))
    }
}

impl Deref for WireFilterCondition {
    type Target = LibFilterCondition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireFilterCondition {
    /// Consumes the wrapper, yielding the inner library condition.
    fn into_inner(self) -> LibFilterCondition {
        self.0
    }
}

/// Wire form of a [`LibMatchValue`]. Build via `WireMatchValue::try_from(value)`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the `value` oneof is unset.
struct WireMatchValue(LibMatchValue);

impl TryFrom<ProtoMatchValue> for WireMatchValue {
    type Error = Status;

    fn try_from(value: ProtoMatchValue) -> Result<Self, Status> {
        let inner = value
            .value
            .ok_or_else(|| Status::invalid_argument("metadata_filter: MatchValue.value oneof must be set"))?;
        Ok(Self(match inner {
            match_value::Value::Keyword(s) => LibMatchValue::Keyword(s),
            match_value::Value::Integer(i) => LibMatchValue::Integer(i),
            match_value::Value::Boolean(b) => LibMatchValue::Bool(b),
        }))
    }
}

impl Deref for WireMatchValue {
    type Target = LibMatchValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireMatchValue {
    /// Consumes the wrapper, yielding the inner library match value.
    fn into_inner(self) -> LibMatchValue {
        self.0
    }
}

/// Wire form of a [`LibMatchValues`]. Build via `WireMatchValues::try_from(values)`.
///
/// # Errors
///
/// Returns [`Status::invalid_argument`] when the `values` oneof is unset.
struct WireMatchValues(LibMatchValues);

impl TryFrom<ProtoMatchValues> for WireMatchValues {
    type Error = Status;

    fn try_from(values: ProtoMatchValues) -> Result<Self, Status> {
        let inner = values
            .values
            .ok_or_else(|| Status::invalid_argument("metadata_filter: MatchValues.values oneof must be set"))?;
        Ok(Self(match inner {
            match_values::Values::Keywords(list) => LibMatchValues::Keywords(list.values),
            match_values::Values::Integers(list) => LibMatchValues::Integers(list.values),
        }))
    }
}

impl Deref for WireMatchValues {
    type Target = LibMatchValues;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireMatchValues {
    /// Consumes the wrapper, yielding the inner library match values.
    fn into_inner(self) -> LibMatchValues {
        self.0
    }
}

/// Wire form of a [`LibNumericRange`]. Build via `WireNumericRange::from(range)`. Infallible.
struct WireNumericRange(LibNumericRange);

impl From<ProtoNumericRange> for WireNumericRange {
    fn from(range: ProtoNumericRange) -> Self {
        Self(LibNumericRange {
            lt: range.lt,
            lte: range.lte,
            gt: range.gt,
            gte: range.gte,
        })
    }
}

impl Deref for WireNumericRange {
    type Target = LibNumericRange;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WireNumericRange {
    /// Consumes the wrapper, yielding the inner library range.
    fn into_inner(self) -> LibNumericRange {
        self.0
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

    /// Builds a wire `Struct` from a JSON value the way the proto layer
    /// delivers one — proto3 widens every number to `f64` in the process.
    fn proto_struct(value: serde_json::Value) -> pbjson_types::Struct {
        serde_json::from_value(value).expect("value encodes as a Struct")
    }

    #[test]
    fn should_default_unset_metadata_to_empty_object() {
        let result = Metadata::try_from(None).unwrap();
        assert_eq!(*result, serde_json::json!({}));
    }

    #[test]
    fn should_preserve_string_values_when_converting_metadata() {
        let proto = proto_struct(serde_json::json!({ "source": "test", "tag": "rust" }));
        let result = Metadata::try_from(Some(proto)).unwrap();
        assert_eq!(*result, serde_json::json!({ "source": "test", "tag": "rust" }));
    }

    #[test]
    fn should_narrow_whole_number_doubles_to_integers() {
        let proto = proto_struct(serde_json::json!({ "conversation_id": 42 }));
        let result = Metadata::try_from(Some(proto)).unwrap();
        assert_eq!(*result, serde_json::json!({ "conversation_id": 42 }));
        assert!(
            result["conversation_id"].is_i64(),
            "whole number must land as an integer"
        );
    }

    #[test]
    fn should_keep_fractional_doubles_as_floats() {
        let proto = proto_struct(serde_json::json!({ "score": 0.5 }));
        let result = Metadata::try_from(Some(proto)).unwrap();
        assert_eq!(*result, serde_json::json!({ "score": 0.5 }));
        assert!(result["score"].is_f64(), "fractional number must stay a float");
    }

    #[test]
    fn should_leave_nested_numbers_unnarrowed() {
        let proto = proto_struct(serde_json::json!({ "nested": { "count": 1 } }));
        let result = Metadata::try_from(Some(proto)).unwrap();
        assert!(result["nested"]["count"].is_f64(), "nested numbers are not narrowed",);
    }

    #[test]
    fn should_preserve_nested_object_structure_when_converting_metadata() {
        let proto = proto_struct(serde_json::json!({
            "source": "test",
            "tags": ["one", "two"],
            "flags": { "enabled": true },
        }));
        let result = Metadata::try_from(Some(proto)).unwrap();
        assert_eq!(
            *result,
            serde_json::json!({
                "source": "test",
                "tags": ["one", "two"],
                "flags": { "enabled": true },
            })
        );
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

    fn retry_args(of_kind: Option<i32>) -> Result<WireRetryArgs, Status> {
        WireRetryArgs::try_from(RetryFailedJobsRequest {
            of_kind,
            dry_run: false,
        })
    }

    #[test]
    fn should_round_trip_job_kind_embed() {
        let proto = WireJobKind::from(LibJobKind::Embed).0;
        let args = retry_args(Some(proto as i32)).unwrap();
        assert_eq!(args.of_kind, Some(LibJobKind::Embed));
    }

    #[test]
    fn should_round_trip_job_kind_extract() {
        let proto = WireJobKind::from(LibJobKind::Extract).0;
        let args = retry_args(Some(proto as i32)).unwrap();
        assert_eq!(args.of_kind, Some(LibJobKind::Extract));
    }

    #[test]
    fn should_treat_unset_job_kind_filter_as_no_filter() {
        assert_eq!(retry_args(None).unwrap().of_kind, None);
    }

    #[test]
    fn should_treat_unspecified_job_kind_as_no_filter() {
        assert_eq!(retry_args(Some(0)).unwrap().of_kind, None);
    }

    #[test]
    fn should_reject_unknown_job_kind_discriminant() {
        let err = retry_args(Some(99)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_pass_dry_run_through_retry_args() {
        let args = WireRetryArgs::try_from(RetryFailedJobsRequest {
            of_kind: None,
            dry_run: true,
        })
        .unwrap();
        assert!(args.dry_run);
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
        let proto = WireFailedJob::from(lib).0;
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
            graph_rebuild_enqueued: 0,
        };
        let proto = WireReconcileResponse::from(summary).0;
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
            graph_rebuild_enqueued: 0,
        };
        let proto = WireReconcileResponse::from(summary).0;
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
        let err = WireMemoryFilter::try_from(proto).unwrap_err();
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
        let err = WireMemoryFilter::try_from(proto).unwrap_err();
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
        let err = WireMemoryFilter::try_from(proto).unwrap_err();
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
        pbjson_types::Timestamp {
            seconds: 1_900_000_000,
            nanos: 0,
        }
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
            kinds: Some(ProtoKindSelector {
                episodic: false,
                semantic: false,
            }),
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
                thresholds: vec![(chrono::Duration::hours(1), 1.0), (chrono::Duration::days(1), 0.5)],
            },
        };
        let proto = ranking_to_proto(&original);
        let back = ranking_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_round_trip_blended_ranking_with_preferred_categories() {
        let original = RankingStrategy::Blended {
            weights: memoir_core::client::BlendWeights {
                cosine: 0.4,
                confidence: 0.3,
                recency: 0.3,
                category_bonus: 0.05,
                preferred_categories: vec!["preference".to_string(), "identity".to_string()],
            },
            decay: DecayFn::Exponential {
                half_life: chrono::Duration::days(7),
            },
        };
        let proto = ranking_to_proto(&original);
        let back = ranking_from_proto(Some(proto)).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn should_reject_blended_ranking_missing_weights() {
        let proto = ProtoRanking {
            strategy: Some(ranking::Strategy::Blended(ProtoBlended {
                weights: None,
                decay: Some(ProtoDecay {
                    function: Some(decay::Function::Exponential(ExponentialDecay {
                        half_life: Some(duration_to_proto(chrono::Duration::days(7))),
                    })),
                }),
            })),
        };
        let err = ranking_from_proto(Some(proto)).unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[test]
    fn should_reject_hybrid_ranking_missing_decay() {
        let proto = ProtoRanking {
            strategy: Some(ranking::Strategy::Hybrid(ProtoHybrid {
                alpha: 0.5,
                decay: None,
            })),
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
