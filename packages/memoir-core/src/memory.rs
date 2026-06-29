//! Memory domain types.

use bon::bon;
use chrono::{DateTime, FixedOffset};

/// Reserved-value prefix memoir uses for internal scope sentinels.
///
/// No consumer-supplied scope value may begin with this prefix; doing so would
/// let a real org or agent collide with memoir's "unscoped" markers. The
/// builder rejects any such value (see [`Scope::builder`]).
pub(crate) const RESERVED_SCOPE_PREFIX: &str = "__MEMOIR_";

/// Stored `org_id` for a memory written without an org (`org_id` unset).
pub(crate) const NO_ORG_SENTINEL: &str = "__MEMOIR_NO_ORG__";

/// Stored `agent_id` for a memory written without an agent (`agent_id` unset).
pub(crate) const NO_AGENT_SENTINEL: &str = "__MEMOIR_NO_AGENT__";

/// User, plus optional org and agent, that a memory belongs to.
///
/// `user_id` is the required floor — every read and write is confined to one
/// user, so memories never cross users. `org_id` and `agent_id` are optional:
/// `None` means the memory is *unscoped* on that dimension (a genuine address,
/// e.g. a user-wide preference that belongs to no single project).
///
/// `None` is read and written **asymmetrically**, and this asymmetry is
/// deliberate:
///
/// - On a **write**, an unset `org_id`/`agent_id` is a concrete unscoped
///   address. The storage layer normalizes it to an internal sentinel so the
///   underlying columns stay non-null.
/// - On a **read**, an unset `org_id`/`agent_id` is *unconstrained*: that
///   dimension is not filtered, so the read matches any value. Omitting a
///   constraint widens; supplying one narrows. No field ever means "all".
///
/// A `Scope` is always valid by construction (see [`Scope::builder`]); the
/// fields are private so an invalid scope cannot exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    agent_id: Option<String>,
    org_id: Option<String>,
    user_id: String,
}

/// Reasons a [`Scope`] cannot be constructed.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ScopeError {
    /// `user_id` was empty. It is the required floor and must be non-empty.
    #[error("scope: user_id must be non-empty")]
    EmptyUserId,

    /// An `org_id` or `agent_id` was supplied as the empty string. Use an unset
    /// value to mean "unscoped"; an empty string is a bug, not an intent.
    #[error("scope: {field} must be a non-empty value or unset, not an empty string")]
    EmptyValue {
        /// The offending field name (`org_id` or `agent_id`).
        field: &'static str,
    },

    /// A supplied value used memoir's reserved sentinel prefix. Consumers may
    /// never pass a value beginning with [`RESERVED_SCOPE_PREFIX`]; an unset
    /// value is the only way to express "unscoped".
    #[error("scope: {field} must not begin with the reserved prefix {RESERVED_SCOPE_PREFIX:?}")]
    Reserved {
        /// The offending field name (`org_id` or `agent_id`).
        field: &'static str,
    },
}

#[bon]
impl Scope {
    /// Builds a validated [`Scope`] from a required user and optional org/agent.
    ///
    /// Omit `org`/`agent` to write or read an *unscoped* memory on that
    /// dimension. See the [`Scope`] type docs for the write-vs-read meaning of
    /// an unset field.
    ///
    /// # Examples
    ///
    /// ```
    /// use memoir_core::memory::Scope;
    ///
    /// let user_global = Scope::builder().user_id("user-42").build()?;
    /// let scoped = Scope::builder()
    ///     .user_id("user-42")
    ///     .org("acme")
    ///     .agent("support-bot")
    ///     .build()?;
    /// # Ok::<(), memoir_core::memory::ScopeError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ScopeError::EmptyUserId`] when `user_id` is empty,
    /// [`ScopeError::EmptyValue`] when `org`/`agent` is supplied as an empty
    /// string, and [`ScopeError::Reserved`] when `org`/`agent` begins with
    /// memoir's reserved sentinel prefix.
    #[builder(start_fn = builder, finish_fn = build)]
    pub fn new(
        #[builder(into)] user_id: String,
        #[builder(into)] org: Option<String>,
        #[builder(into)] agent: Option<String>,
    ) -> Result<Self, ScopeError> {
        if user_id.is_empty() {
            return Err(ScopeError::EmptyUserId);
        }
        Self::check_optional("org_id", org.as_deref())?;
        Self::check_optional("agent_id", agent.as_deref())?;
        Ok(Self {
            agent_id: agent,
            org_id: org,
            user_id,
        })
    }
}

impl Scope {
    /// The required user this scope is confined to.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// The org, or `None` when the memory is unscoped on the org dimension.
    pub fn org_id(&self) -> Option<&str> {
        self.org_id.as_deref()
    }

    /// The agent, or `None` when the memory is unscoped on the agent dimension.
    pub fn agent_id(&self) -> Option<&str> {
        self.agent_id.as_deref()
    }

    /// Reconstructs a [`Scope`] from already-stored column values.
    ///
    /// The inverse of the `*_or_sentinel` write projection: a stored sentinel
    /// becomes `None`, so the sentinel never leaks back to a consumer. Storage
    /// values are trusted and may legitimately equal a sentinel, so this
    /// bypasses the public builder's validation rather than rejecting them.
    pub(crate) fn from_storage(
        agent_id: impl Into<String>,
        org_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Self {
        let unsentinel = |value: String, sentinel: &str| (value != sentinel).then_some(value);
        Self {
            agent_id: unsentinel(agent_id.into(), NO_AGENT_SENTINEL),
            org_id: unsentinel(org_id.into(), NO_ORG_SENTINEL),
            user_id: user_id.into(),
        }
    }

    /// The org as a storage value: the real org, or the unscoped sentinel.
    ///
    /// This is the **write** projection — an unscoped org is materialized as
    /// [`NO_ORG_SENTINEL`] so storage columns stay non-null. Never use this to
    /// build a read filter; on a read an unset org means "any", not the
    /// sentinel (see the [`Scope`] type docs).
    pub(crate) fn org_id_or_sentinel(&self) -> &str {
        self.org_id.as_deref().unwrap_or(NO_ORG_SENTINEL)
    }

    /// The agent as a storage value: the real agent, or the unscoped sentinel.
    ///
    /// The **write** projection, mirroring [`Self::org_id_or_sentinel`]. Not for
    /// read filters.
    pub(crate) fn agent_id_or_sentinel(&self) -> &str {
        self.agent_id.as_deref().unwrap_or(NO_AGENT_SENTINEL)
    }

    /// Whether `self` (a stored scope) is selected by `filter` (a read scope).
    ///
    /// `user_id` must match exactly (the floor). An unset `org_id`/`agent_id` on
    /// the filter is *unconstrained* — it matches any stored value — while a set
    /// one must match exactly. This is the read-widen rule: omitting a
    /// constraint widens, supplying one narrows. Equality (`==`) is the wrong
    /// test for reads, as it would force the filter's unset dimensions to mean
    /// "unscoped only" instead of "any".
    #[cfg(test)]
    pub(crate) fn matches_read_filter(&self, filter: &Scope) -> bool {
        self.user_id == filter.user_id
            && filter
                .agent_id
                .as_ref()
                .is_none_or(|a| Some(a) == self.agent_id.as_ref())
            && filter.org_id.as_ref().is_none_or(|o| Some(o) == self.org_id.as_ref())
    }

    /// Rejects an optional value that is empty or uses the reserved prefix.
    fn check_optional(field: &'static str, value: Option<&str>) -> Result<(), ScopeError> {
        match value {
            None => Ok(()),
            Some("") => Err(ScopeError::EmptyValue { field }),
            Some(v) if v.starts_with(RESERVED_SCOPE_PREFIX) => Err(ScopeError::Reserved { field }),
            Some(_) => Ok(()),
        }
    }
}

/// Kind of memory written to or read from storage.
///
/// The two kinds form memoir's source-and-projection model: episodic rows are
/// the verbatim record a consumer writes; semantic rows are facts a worker
/// derives from them. Semantic content is **never hand-written or edited** —
/// it is always re-derived from its episodic source, so a wrong semantic fact
/// is corrected by teaching ([`crate::client::Client::feedback`]) or by editing
/// the source ([`crate::client::Client::edit`]), never by writing the fact
/// directly. See the crate-root docs' "Correction" section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum MemoryKind {
    /// Conversational memory; written by `Client::remember`.
    Episodic,

    /// Structured fact extracted from episodic memory by an LLM (epic 0006).
    ///
    /// Always derived, never authored directly: there is no API to set a
    /// semantic row's content. Corrections flow through re-derivation.
    Semantic,
}

/// Why a memory was retired by the correction model (epic 0011 Track B).
///
/// A retired memory is hidden from every read and its vector is evicted, so
/// it can no longer surface or pollute reprocessing — but the row is kept (it
/// is the reprocess "don't re-derive this" guard and the accuracy-metric
/// record). The reason distinguishes an extraction error from a non-error:
/// only [`Self::Rejected`] counts against extraction accuracy.
///
/// Distinct from supersession (the `superseded_by` column + events table),
/// which models "a newer fact won" — a normal lifecycle event, not a
/// correction. "Active" means neither superseded nor retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum RetirementReason {
    /// The extraction was wrong; the user corrected it via feedback. This is
    /// an extraction error — the numerator of the accuracy metric.
    Rejected,

    /// The episodic source was edited or deleted, so this derived semantic no
    /// longer reflects it. The model did not err; the source changed.
    Stale,
}

/// Optional scope-subset filter for an aggregate read.
///
/// Each field narrows the aggregate to memories matching it; an unset field
/// imposes no constraint. Distinct from [`Scope`], which requires all three
/// fields — this is a partial filter, so a caller can aggregate org-wide
/// (`org_id` only), per-agent, or across the whole store (all unset).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct StatsFilter {
    pub agent_id: Option<String>,
    pub org_id: Option<String>,
    pub user_id: Option<String>,
}

/// Extraction-accuracy tally for one `(provider, model)` pair within a slice.
///
/// `total` counts every semantic row the pair produced (active or retired, any
/// reason); `rejected` counts only those retired as [`RetirementReason::Rejected`]
/// — a wrong extraction the user corrected. Rows retired as
/// [`RetirementReason::Stale`] (the source changed) and superseded rows (a newer
/// fact won) are in `total` but never in `rejected`: they are not model errors.
/// See [`Self::accuracy`] for the derived ratio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionStat {
    pub provider: String,
    pub model: String,
    pub total: u64,
    pub rejected: u64,
}

impl ExtractionStat {
    /// Returns the extraction accuracy as `1 − rejected/total` in `[0.0, 1.0]`.
    ///
    /// A pair with zero extractions returns `1.0`: there is nothing to have
    /// gotten wrong, so the identity value is "no errors."
    #[must_use]
    pub fn accuracy(&self) -> f64 {
        if self.total == 0 {
            return 1.0;
        }
        1.0 - (self.rejected as f64 / self.total as f64)
    }
}

/// A memory's confidence as a 0-100 percentage.
///
/// A newtype over `i8` whose only constructor clamps into `[0, 100]`, so an
/// out-of-range value is unrepresentable. This is the single home for the
/// scale-and-clamp logic: the extraction LLM emits an `f32` (occasionally
/// `> 1.0`), which [`Confidence::from_unit_scale`] scales by 100 and clamps.
///
/// # Examples
///
/// ```
/// use memoir_core::memory::Confidence;
///
/// assert_eq!(Confidence::new(73).get(), 73);
/// assert_eq!(Confidence::new(120).get(), 100); // clamped
/// assert_eq!(Confidence::from_unit_scale(0.42).get(), 42);
/// assert_eq!(Confidence::from_unit_scale(1.7).get(), 100); // clamped
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Confidence(i8);

impl Confidence {
    /// Largest valid confidence: fully certain.
    pub const MAX: Confidence = Confidence(100);

    /// Smallest valid confidence: no certainty.
    pub const MIN: Confidence = Confidence(0);

    /// Creates a confidence from a percentage, clamping into `[0, 100]`.
    ///
    /// Clamping is the defined behavior, not an error: callers (and the
    /// extraction LLM) occasionally produce out-of-range values, and the
    /// intent is always "as confident as possible / not at all," never a
    /// failure. Hence this is infallible.
    #[must_use]
    pub fn new(percent: i8) -> Self {
        Self(percent.clamp(0, 100))
    }

    /// Creates a confidence from a unit-scale score, scaling ×100 and clamping.
    ///
    /// The extraction LLM emits a per-fact score in `[0.0, 1.0]` (but may
    /// exceed `1.0`). This scales to a percentage and clamps into `[0, 100]`.
    /// `NaN` maps to [`Confidence::MIN`].
    #[must_use]
    pub fn from_unit_scale(score: f32) -> Self {
        if score.is_nan() {
            return Self::MIN;
        }
        // Round before clamping so e.g. 0.005 -> 1, not 0.
        let percent = (score * 100.0).round();
        Self(percent.clamp(0.0, 100.0) as i8)
    }

    /// Returns the percentage value in `[0, 100]`.
    #[must_use]
    pub fn get(self) -> i8 {
        self.0
    }
}

impl Default for Confidence {
    /// Defaults to fully certain (`100`), matching the `memories.confidence`
    /// column default — episodic writes are certain by construction.
    fn default() -> Self {
        Self::MAX
    }
}

impl std::fmt::Display for Confidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Selects which memory kinds a read includes.
///
/// Each field gates inclusion of one kind. Default ([`Self::default`]) has
/// every field `true` — retrieve all kinds. A field set to `false` filters
/// that kind out. Constructing with all fields `false` is legal and yields an
/// empty result.
///
/// Designed so that adding a new kind later is additive: a new `pub bool`
/// field with default `true` does not break existing constructors that use
/// `..Default::default()` or named-field init.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KindSelector {
    pub episodic: bool,
    pub semantic: bool,
}

impl Default for KindSelector {
    fn default() -> Self {
        Self {
            episodic: true,
            semantic: true,
        }
    }
}

impl KindSelector {
    /// Returns the kinds this selector includes, in canonical order.
    pub fn included_kinds(&self) -> Vec<MemoryKind> {
        let mut out = Vec::with_capacity(2);
        if self.episodic {
            out.push(MemoryKind::Episodic);
        }
        if self.semantic {
            out.push(MemoryKind::Semantic);
        }
        out
    }

    /// Returns `true` when every defined kind is included.
    pub fn includes_all(&self) -> bool {
        self.episodic && self.semantic
    }

    /// Returns `true` when no kind is included.
    pub fn is_empty(&self) -> bool {
        !self.episodic && !self.semantic
    }
}

/// A stored memory, with optional similarity score from vector search.
///
/// Carries three distinct timestamps that should not be confused:
/// `created_at` (when memoir was told), `updated_at` (last in-place edit),
/// and `event_at` (when the remembered event actually occurred). The first
/// two are wall-clock; the third is event-time and may predate `created_at`
/// by arbitrary amounts.
///
/// Soft-deletion via [`SupersessionInfo`] keeps superseded rows in the
/// store, but [`crate::client::Client::search`] filters them out by
/// default. They remain reachable via [`crate::client::Client::recall`].
#[derive(Debug, Clone)]
pub struct Memory {
    /// Public id; opaque, stable for the lifetime of the row.
    pub pid: String,

    /// Tenant + agent + user partition. See [`Scope`].
    pub scope: Scope,

    /// Raw text of the memory.
    pub content: String,

    /// Arbitrary JSON attached at write time; round-trips unchanged.
    pub metadata: serde_json::Value,

    /// Episodic (raw utterance) or semantic (LLM-extracted fact).
    pub kind: MemoryKind,

    /// Originating episodic pid for semantic rows; `None` for episodic.
    ///
    /// Enforced at the database with `ON DELETE CASCADE`: forgetting the
    /// source automatically removes derived semantic memories.
    pub source_pid: Option<String>,

    /// Soft-deletion marker; `None` when active.
    ///
    /// Populated by contradiction-detection passes or operator action.
    /// The nested type ties winner pid and decision time together so
    /// neither can exist without the other.
    pub supersession: Option<SupersessionInfo>,

    /// Wall-clock time memoir received the utterance.
    pub created_at: DateTime<FixedOffset>,

    /// Wall-clock time of the row's last in-place mutation.
    ///
    /// Auto-bumped by the database trigger on every UPDATE. Equals
    /// `created_at` for memories never edited via
    /// [`crate::client::Client::edit`].
    pub updated_at: DateTime<FixedOffset>,

    /// Event-time of the thing being remembered; `None` when unknown.
    ///
    /// Distinct from `created_at`: "the deployment happened Friday" said
    /// today carries `event_at = Friday`, `created_at = today`. Set by
    /// consumers via `RememberBuilder::event_at` or by LLM extraction.
    /// `None` is appropriate when no event-time is meaningful
    /// (preferences, identity facts).
    pub event_at: Option<DateTime<FixedOffset>>,

    /// Cosine similarity score; `Some` only on vector-search results.
    pub score: Option<f32>,

    /// Processing lifecycle state of the row's vector index.
    ///
    /// `Pending` immediately after a write (embedding + vector upsert in
    /// flight), `Indexed` once searchable, `Failed` if embedding errored.
    /// Mirrors the `memories.qdrant_status` column. Consumers use this as the
    /// canonical "is this memory fully processed yet" signal.
    pub status: crate::store::IndexStatus,

    /// How sure memoir is that this memory is true, as a 0-100 percentage.
    ///
    /// Episodic memories are `100` by construction — the user said it.
    /// Semantic memories carry the extraction LLM's scaled per-fact score
    /// (populated by the extract worker). See [`Confidence`]. Feeds the
    /// selection blend as a signal (normalized to `[0, 1]`) and the
    /// `min_confidence` hard filter — see [`crate::client::BlendWeights`].
    pub confidence: Confidence,

    /// Categorization label, or `None` until the categorize worker runs.
    ///
    /// Populated asynchronously by the NLI categorize stage. A `None`
    /// category is unfiltered, not rejected — absence means "not yet
    /// classified," not "no category applies." The value set (taxonomy) is
    /// owned by the categorize worker, so this stays an open `String` here;
    /// the v1 labels are `preference`, `identity`, `workflow`, `factual`,
    /// `transient` (see `crate::client::categorize`). Drives the
    /// category-bonus term of the selection blend ([`crate::client::BlendWeights`])
    /// and the `category` hard filter on search/query.
    pub category: Option<String>,

    /// Why this memory was retired, or `None` when active (epic 0011).
    ///
    /// Set by the correction model ([`crate::client::Client::reject`] /
    /// `mark_stale`). A `Some(_)` row is hidden from all reads and its vector
    /// is evicted; the row is kept for the reprocess guard and the
    /// extraction-accuracy metric ([`crate::client::Client::extraction_stats`]),
    /// where only [`RetirementReason::Rejected`] counts as an error. Distinct
    /// from [`Self::supersession`]. "Active" requires both this and
    /// `supersession` to be `None`.
    pub retirement: Option<RetirementReason>,
}

/// Latest supersession state for a [`Memory`] — winner pid and decision time.
///
/// Reflects only the current state. Full supersession history, including
/// reversals, lives in the `supersession_events` audit table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersessionInfo {
    /// Pid of the memory that supersedes this one.
    pub winner_pid: String,

    /// Wall-clock time the supersession decision was made.
    pub at: DateTime<FixedOffset>,
}

/// One supersede or unsupersede decision against a memory.
///
/// Mirrors one row of the `supersession_events` audit table. A `winner_pid`
/// of `None` is an unsupersede — the memory was restored to active.
///
/// Returned in chronological order by
/// [`crate::store::MemoryStore::supersession_history`] and surfaced by
/// [`crate::client::Client::supersession_history`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersessionEvent {
    /// Pid of the memory that took precedence; `None` for an unsupersede event.
    pub winner_pid: Option<String>,

    /// Wall-clock time the decision was recorded.
    pub decided_at: DateTime<FixedOffset>,
}

/// Target of a forget operation: a single memory or a whole scope.
#[derive(Debug, Clone)]
pub enum ForgetTarget {
    /// Forget exactly one memory by its public id.
    Pid(String),

    /// Forget every memory matching the scope tuple.
    Scope(Scope),
}

/// A list of memories and an optional LLM-facing system prompt section.
///
/// Returned by [`crate::client::Client::remember`]. Implements [`Display`]
/// for direct injection into a system prompt and [`Deref`] to `[Memory]`
/// for iteration.
///
/// When `system_prompt` is `Some`, [`Display`] emits the prompt followed by
/// a bullet list of memory content. When `None`, only the bullet list is
/// emitted — the caller takes responsibility for instructing the LLM.
///
/// [`Display`]: std::fmt::Display
/// [`Deref`]: std::ops::Deref
#[derive(Debug, Clone)]
pub struct Memories {
    list: Vec<Memory>,
    system_prompt: Option<String>,
    graph: crate::graph::GraphContext,
}

impl Memories {
    /// Builds a `Memories` from a list and an optional system prompt section.
    ///
    /// The graph context starts empty; populate it with
    /// [`Self::with_graph_context`] when a search opts into enrichment.
    pub fn new(list: Vec<Memory>, system_prompt: Option<String>) -> Self {
        Self {
            list,
            system_prompt,
            graph: crate::graph::GraphContext::default(),
        }
    }

    /// Attaches the graph neighborhood produced by an enriched search.
    #[must_use]
    pub fn with_graph_context(mut self, graph: crate::graph::GraphContext) -> Self {
        self.graph = graph;
        self
    }

    /// Returns the contained memories as a slice.
    pub fn list(&self) -> &[Memory] {
        &self.list
    }

    /// Returns the configured system-prompt section, if any.
    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    /// Returns the graph neighborhood from an enriched search.
    ///
    /// Empty unless the search opted in via `.with_graph()`. This is read-only
    /// context for the consumer to format as they choose; [`Display`] renders
    /// only the memories, leaving graph-context injection to the caller.
    ///
    /// [`Display`]: std::fmt::Display
    pub fn graph(&self) -> &crate::graph::GraphContext {
        &self.graph
    }
}

impl std::fmt::Display for Memories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prompt) = &self.system_prompt {
            writeln!(f, "{prompt}")?;
        }
        for memory in &self.list {
            writeln!(f, "- {}", memory.content)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Memories {
    type Target = [Memory];

    fn deref(&self) -> &[Memory] {
        &self.list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn fixture(content: &str) -> Memory {
        let now: DateTime<FixedOffset> = Utc::now().into();
        Memory {
            pid: "test".into(),
            scope: Scope::builder()
                .user_id("u")
                .org("o")
                .agent("a")
                .build()
                .expect("fixture scope is valid"),
            content: content.into(),
            metadata: serde_json::json!({}),
            kind: MemoryKind::Episodic,
            source_pid: None,
            supersession: None,
            created_at: now,
            updated_at: now,
            event_at: None,
            score: None,
            status: crate::store::IndexStatus::Pending,
            confidence: Confidence::default(),
            category: None,
            retirement: None,
        }
    }

    #[test]
    fn should_render_memory_kind_as_lowercase_string() {
        assert_eq!(MemoryKind::Episodic.as_ref(), "episodic");
        assert_eq!(MemoryKind::Semantic.as_ref(), "semantic");
    }

    #[test]
    fn should_display_memory_kind_matching_as_ref() {
        assert_eq!(MemoryKind::Episodic.to_string(), "episodic");
        assert_eq!(MemoryKind::Semantic.to_string(), "semantic");
    }

    #[test]
    fn should_render_retirement_reason_as_lowercase_string() {
        assert_eq!(RetirementReason::Rejected.as_ref(), "rejected");
        assert_eq!(RetirementReason::Stale.as_ref(), "stale");
    }

    #[test]
    fn should_round_trip_retirement_reason_through_str() {
        use std::str::FromStr as _;
        assert_eq!(
            RetirementReason::from_str("rejected").unwrap(),
            RetirementReason::Rejected
        );
        assert_eq!(RetirementReason::from_str("stale").unwrap(), RetirementReason::Stale);
        assert!(RetirementReason::from_str("superseded").is_err());
        assert!(RetirementReason::from_str("nonsense").is_err());
    }

    #[test]
    fn should_compute_accuracy_as_one_minus_rejected_over_total() {
        let stat = ExtractionStat {
            provider: "ollama".to_string(),
            model: "qwen3:14b".to_string(),
            total: 100,
            rejected: 3,
        };
        assert!((stat.accuracy() - 0.97).abs() < f64::EPSILON);
    }

    #[test]
    fn should_report_perfect_accuracy_when_no_extractions() {
        let stat = ExtractionStat {
            provider: String::new(),
            model: String::new(),
            total: 0,
            rejected: 0,
        };
        assert_eq!(stat.accuracy(), 1.0, "zero extractions means nothing to get wrong");
    }

    #[test]
    fn should_parse_memory_kind_from_str() {
        use std::str::FromStr as _;
        assert_eq!(MemoryKind::from_str("episodic").unwrap(), MemoryKind::Episodic);
        assert_eq!(MemoryKind::from_str("semantic").unwrap(), MemoryKind::Semantic);
        assert!(MemoryKind::from_str("nonsense").is_err());
    }

    #[test]
    fn should_keep_in_range_confidence_unchanged() {
        assert_eq!(Confidence::new(0).get(), 0);
        assert_eq!(Confidence::new(73).get(), 73);
        assert_eq!(Confidence::new(100).get(), 100);
    }

    #[test]
    fn should_clamp_out_of_range_confidence() {
        assert_eq!(Confidence::new(127).get(), 100);
        assert_eq!(Confidence::new(-1).get(), 0);
        assert_eq!(Confidence::new(-128).get(), 0);
    }

    #[test]
    fn should_scale_unit_confidence_to_percentage() {
        assert_eq!(Confidence::from_unit_scale(0.0).get(), 0);
        assert_eq!(Confidence::from_unit_scale(0.42).get(), 42);
        assert_eq!(Confidence::from_unit_scale(1.0).get(), 100);
    }

    #[test]
    fn should_clamp_unit_confidence_above_one() {
        // The extraction LLM occasionally emits scores > 1.0.
        assert_eq!(Confidence::from_unit_scale(1.7).get(), 100);
        assert_eq!(Confidence::from_unit_scale(-0.5).get(), 0);
    }

    #[test]
    fn should_map_nan_confidence_to_min() {
        assert_eq!(Confidence::from_unit_scale(f32::NAN), Confidence::MIN);
    }

    #[test]
    fn should_default_confidence_to_max() {
        assert_eq!(Confidence::default(), Confidence::MAX);
        assert_eq!(Confidence::default().get(), 100);
    }

    #[test]
    fn should_display_memories_with_system_prompt_and_bullets() {
        let memories = Memories::new(vec![fixture("first"), fixture("second")], Some("Context:".into()));

        assert_eq!(memories.to_string(), "Context:\n- first\n- second\n");
    }

    #[test]
    fn should_display_memories_without_system_prompt_as_bullets_only() {
        let memories = Memories::new(vec![fixture("only")], None);

        assert_eq!(memories.to_string(), "- only\n");
    }

    #[test]
    fn should_display_empty_memories_as_empty_string() {
        let memories = Memories::new(Vec::new(), None);
        assert_eq!(memories.to_string(), "");
    }

    #[test]
    fn should_deref_memories_to_slice() {
        let memories = Memories::new(vec![fixture("a"), fixture("b")], None);
        assert_eq!(memories.len(), 2);
        assert_eq!(memories[0].content, "a");
    }

    #[test]
    fn should_default_event_at_to_none_in_fixture() {
        let memory = fixture("hello");
        assert!(
            memory.event_at.is_none(),
            "fixture default event_at must be None — most memories have no meaningful event-time"
        );
    }

    #[test]
    fn should_accept_when_org_and_agent_are_none() {
        let scope = Scope::builder().user_id("u").build();
        let scope = scope.expect("user-only scope is valid");
        assert_eq!(scope.user_id(), "u");
        assert_eq!(scope.org_id(), None);
        assert_eq!(scope.agent_id(), None);
    }

    #[test]
    fn should_accept_when_org_and_agent_are_supplied() {
        let scope = Scope::builder().user_id("u").org("o").agent("a").build();
        let scope = scope.expect("fully scoped scope is valid");
        assert_eq!(scope.org_id(), Some("o"));
        assert_eq!(scope.agent_id(), Some("a"));
    }

    #[test]
    fn should_reject_when_user_id_empty() {
        let result = Scope::builder().user_id("").build();
        assert_eq!(result, Err(ScopeError::EmptyUserId));
    }

    #[test]
    fn should_reject_when_org_id_is_empty_string() {
        let result = Scope::builder().user_id("u").org("").build();
        assert_eq!(result, Err(ScopeError::EmptyValue { field: "org_id" }));
    }

    #[test]
    fn should_reject_when_agent_id_is_empty_string() {
        let result = Scope::builder().user_id("u").agent("").build();
        assert_eq!(result, Err(ScopeError::EmptyValue { field: "agent_id" }));
    }

    #[test]
    fn should_reject_when_org_id_is_reserved_sentinel() {
        let result = Scope::builder().user_id("u").org(NO_ORG_SENTINEL).build();
        assert_eq!(result, Err(ScopeError::Reserved { field: "org_id" }));
    }

    #[test]
    fn should_reject_when_agent_id_uses_reserved_prefix_with_unknown_suffix() {
        let result = Scope::builder().user_id("u").agent("__MEMOIR_FUTURE__").build();
        assert_eq!(result, Err(ScopeError::Reserved { field: "agent_id" }));
    }
}
