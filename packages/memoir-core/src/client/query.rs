//! Per-call builder for [`Client::query`] plus the public ranking enum and
//! the opaque [`MemoryContext`] result type.

use std::future::{Future, IntoFuture};
use std::ops::Deref;
use std::pin::Pin;

use chrono::{DateTime, FixedOffset, Utc};

use crate::embedding::EmbeddingModel;
use crate::memory::{KindSelector, Memory, Scope};
use crate::store::MemoryStore;
use crate::vector::{FilterCondition, MemoryFilter, NumericRange, VectorIndex};

use super::{Client, ClientError};

/// Default page size when the caller does not specify `limit`.
pub const DEFAULT_QUERY_LIMIT: usize = 10;

/// Default `alpha` for [`RankingStrategy::Hybrid`] — weight on cosine.
pub const DEFAULT_HYBRID_ALPHA: f32 = 0.7;

/// Default half-life (in days) for the exponential recency decay.
pub const DEFAULT_HYBRID_HALF_LIFE_DAYS: f32 = 7.0;

/// Recency-decay function used by [`RankingStrategy::Hybrid`].
///
/// All variants take an age in seconds and produce a value in `[0.0, 1.0]`
/// (loosely — `Step` may exceed `1.0` if thresholds are configured that
/// way). The blended hybrid score is
/// `alpha * cosine + (1 - alpha) * decay(age)`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum DecayFn {
    /// `exp(-ln(2) * age / half_life)`. Reaches `0.5` at `half_life`.
    Exponential {
        /// Half-life as a [`chrono::Duration`].
        half_life: chrono::Duration,
    },

    /// `1 / (1 + age / scale)`. Slower tail than exponential.
    Reciprocal {
        /// Scale as a [`chrono::Duration`]; reaches `0.5` at `scale`.
        scale: chrono::Duration,
    },

    /// Bucketed decay. `thresholds` is a list of `(boundary, value)` pairs
    /// sorted by `boundary` ascending; ages within `[prev_boundary, boundary]`
    /// take the listed value. Ages past the last boundary take the last value.
    Step {
        /// `(age_boundary, decay_value)` pairs.
        thresholds: Vec<(chrono::Duration, f32)>,
    },
}

impl DecayFn {
    fn evaluate(&self, age: chrono::Duration) -> f32 {
        let age_secs = age.num_seconds().max(0) as f32;
        match self {
            DecayFn::Exponential { half_life } => {
                let hl = (half_life.num_seconds().max(1)) as f32;
                (-std::f32::consts::LN_2 * age_secs / hl).exp()
            }
            DecayFn::Reciprocal { scale } => {
                let s = (scale.num_seconds().max(1)) as f32;
                1.0 / (1.0 + age_secs / s)
            }
            DecayFn::Step { thresholds } => {
                for (boundary, value) in thresholds {
                    if age <= *boundary {
                        return *value;
                    }
                }
                thresholds.last().map(|(_, v)| *v).unwrap_or(0.0)
            }
        }
    }
}

/// Weights for [`RankingStrategy::Blended`]'s linear score.
///
/// The three signal weights (`cosine`, `confidence`, `recency`) are blended
/// additively; they need not sum to `1.0` but doing so keeps the score in
/// `[0, 1]` for interpretability. `category_bonus` is added on top when a
/// memory's category is in `preferred_categories` — a soft nudge, not a
/// filter (for a hard category filter, see [`crate::client::SearchBuilder`]).
/// When `preferred_categories` is empty the bonus never applies, so the
/// category term is inert by default.
///
/// Use a named preset ([`Self::relevance_first`], [`Self::trust_first`],
/// [`Self::balanced`]) or construct directly for custom weights. Presets are
/// v1 starting points and **may be retuned pre-1.0**; pin a `Blended` with
/// explicit weights for frozen behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct BlendWeights {
    /// Weight on cosine similarity (the `[0, 1]`-ish vector match).
    pub cosine: f32,
    /// Weight on confidence (the memory's `0-100` certainty, normalized to `[0, 1]`).
    pub confidence: f32,
    /// Weight on recency (the decay-of-age term in `[0, 1]`).
    pub recency: f32,
    /// Additive bonus when a memory's category is preferred.
    pub category_bonus: f32,
    /// Categories that earn `category_bonus`. Empty disables the bonus.
    pub preferred_categories: Vec<String>,
}

impl BlendWeights {
    /// Favors vector relevance; closest to pure-cosine behavior.
    ///
    /// Cosine dominates, with confidence and recency as light tiebreakers.
    /// Use when the query is a specific information need and the best answer
    /// is whatever matches the wording most closely.
    #[must_use]
    pub fn relevance_first() -> Self {
        Self {
            cosine: 0.7,
            confidence: 0.15,
            recency: 0.15,
            category_bonus: 0.05,
            preferred_categories: Vec::new(),
        }
    }

    /// Favors durable, high-confidence facts over raw relevance.
    ///
    /// Confidence carries the most weight, so a corrected/asserted fact
    /// outranks a slightly-closer transient match. Use when building a
    /// system-prompt persona where being *right* matters more than being
    /// lexically closest.
    #[must_use]
    pub fn trust_first() -> Self {
        Self {
            cosine: 0.4,
            confidence: 0.45,
            recency: 0.15,
            category_bonus: 0.05,
            preferred_categories: Vec::new(),
        }
    }

    /// Even-handed blend of relevance, confidence, and recency.
    #[must_use]
    pub fn balanced() -> Self {
        Self {
            cosine: 0.4,
            confidence: 0.3,
            recency: 0.3,
            category_bonus: 0.05,
            preferred_categories: Vec::new(),
        }
    }

    /// Returns a copy with `categories` set as the preferred set.
    ///
    /// Memories whose `category` is in this set earn `category_bonus` at
    /// ranking time. With an empty set the bonus is inert.
    #[must_use]
    pub fn prefer_categories(mut self, categories: impl IntoIterator<Item = String>) -> Self {
        self.preferred_categories = categories.into_iter().collect();
        self
    }
}

/// How [`Client::query`] orders the candidate set.
///
/// Reach for [`Self::Hybrid`] when relevance and recency are the only signals
/// that matter (it ignores confidence and category); reach for [`Self::Blended`]
/// to also reward high-confidence facts and preferred categories — the typical
/// choice once the categorize/confidence pipeline is populated. `Blended` is a
/// strict superset, so `Hybrid` is just the `w_conf = 0`, no-category special
/// case. For raw nearest-neighbor hits with no re-ranking at all, use
/// [`Client::search`] instead of `query`.
///
/// Constructing a variant explicitly is the caller's opt-in to that
/// specific behavior — those parameter values become part of the stability
/// contract for the caller's code. The *default* behavior (when no strategy
/// is passed) is allowed to drift pre-1.0.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum RankingStrategy {
    /// Blend of cosine similarity and recency.
    ///
    /// `score = alpha * cosine + (1 - alpha) * decay(age)`. `alpha` in
    /// `[0.0, 1.0]`; `0.0` is pure recency, `1.0` is pure cosine. Age is
    /// computed against `event_at` when present, falling back to
    /// `created_at`.
    Hybrid {
        /// Weight on cosine; `1 - alpha` weights recency.
        alpha: f32,
        /// Decay shape applied to the memory's age.
        decay: DecayFn,
    },

    /// Linear blend of cosine, confidence, recency, and a category bonus.
    ///
    /// `score = w_cos·cosine + w_conf·(confidence/100) + w_rec·decay(age)`,
    /// plus `category_bonus` when the memory's category is preferred. The
    /// superset of [`Self::Hybrid`] (which is the `w_conf = 0`, no-category
    /// case). Confidence is normalized from `0-100` to `[0, 1]` before
    /// weighting so no single signal dominates by scale. Pure math + indexed
    /// lookups — no inference in the read path.
    Blended {
        /// Signal weights and the optional category preference.
        weights: BlendWeights,
        /// Decay shape applied to the memory's age.
        decay: DecayFn,
    },
}

impl RankingStrategy {
    /// The default Hybrid configuration `query()` uses when no strategy is
    /// passed. **Parameter values may drift pre-1.0.**
    pub fn default_hybrid() -> Self {
        Self::Hybrid {
            alpha: DEFAULT_HYBRID_ALPHA,
            decay: DecayFn::Exponential {
                half_life: chrono::Duration::days(DEFAULT_HYBRID_HALF_LIFE_DAYS as i64),
            },
        }
    }

    /// A [`Self::Blended`] strategy with the given weight preset and the
    /// default exponential recency decay.
    ///
    /// Convenience over constructing `Blended { weights, decay }` by hand
    /// when the default half-life is fine:
    /// `RankingStrategy::blended(BlendWeights::trust_first())`.
    #[must_use]
    pub fn blended(weights: BlendWeights) -> Self {
        Self::Blended {
            weights,
            decay: DecayFn::Exponential {
                half_life: chrono::Duration::days(DEFAULT_HYBRID_HALF_LIFE_DAYS as i64),
            },
        }
    }
}

/// Opaque result returned by [`Client::query`].
///
/// Holds the ranked memories plus the strategy used to rank them.
/// Implements [`Display`] for direct injection into a system prompt and
/// [`Deref`] to `[Memory]` for iteration. Each memory's `score` field
/// carries its hybrid score (not raw cosine), so callers ranking results
/// themselves see the same numbers `query()` used.
///
/// When `system_prompt` is `Some` (populated from
/// [`crate::client::Client::system_prompt`] at build time), [`Display`]
/// emits the prompt followed by a bullet list. When `None`, only the
/// bullets are emitted.
///
/// Each bullet is rendered as `[YYYY-MM-DD, N units ago] content`. The
/// absolute date is the canonical reference; the relative-time label is
/// computed at render time and may become stale if the rendered string is
/// cached and re-fed to an LLM later — callers should re-call `query()`
/// rather than persist its output.
///
/// [`Display`]: std::fmt::Display
#[derive(Debug, Clone)]
pub struct MemoryContext {
    memories: Vec<Memory>,
    system_prompt: Option<String>,
    strategy: RankingStrategy,
}

impl MemoryContext {
    pub(super) fn new(
        memories: Vec<Memory>,
        system_prompt: Option<String>,
        strategy: RankingStrategy,
    ) -> Self {
        Self {
            memories,
            system_prompt,
            strategy,
        }
    }

    /// Returns the ranked memories.
    #[must_use]
    pub fn memories(&self) -> &[Memory] {
        &self.memories
    }

    /// Returns the strategy that produced this ranking.
    #[must_use]
    pub fn strategy_used(&self) -> &RankingStrategy {
        &self.strategy
    }

    /// Returns the system-prompt preamble, if any.
    #[must_use]
    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }
}

impl Deref for MemoryContext {
    type Target = [Memory];

    fn deref(&self) -> &[Memory] {
        &self.memories
    }
}

impl std::fmt::Display for MemoryContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(prompt) = &self.system_prompt {
            writeln!(f, "{prompt}")?;
        }
        let now = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
        for memory in &self.memories {
            let anchor = memory.event_at.unwrap_or(memory.created_at);
            let date = anchor.format("%Y-%m-%d");
            let relative = relative_label(now - anchor);
            writeln!(f, "- [{date}, {relative}] {}", memory.content)?;
        }
        Ok(())
    }
}

fn relative_label(delta: chrono::Duration) -> String {
    let secs = delta.num_seconds();
    if secs < 0 {
        return "in the future".to_string();
    }
    if secs < 60 {
        return "just now".to_string();
    }
    let mins = delta.num_minutes();
    if mins < 60 {
        return format!("{mins} minute{} ago", if mins == 1 { "" } else { "s" });
    }
    let hours = delta.num_hours();
    if hours < 24 {
        return format!("{hours} hour{} ago", if hours == 1 { "" } else { "s" });
    }
    let days = delta.num_days();
    if days < 30 {
        return format!("{days} day{} ago", if days == 1 { "" } else { "s" });
    }
    let months = days / 30;
    if months < 12 {
        return format!("{months} month{} ago", if months == 1 { "" } else { "s" });
    }
    let years = days / 365;
    format!("{years} year{} ago", if years == 1 { "" } else { "s" })
}

/// Per-call builder returned by [`Client::query`].
///
/// Awaiting the builder retrieves candidate memories via vector search,
/// re-ranks them using the configured [`RankingStrategy`] (default:
/// [`RankingStrategy::default_hybrid`]), and returns a [`MemoryContext`]
/// suitable for dropping into a system prompt.
///
/// **The default ranking parameters may drift pre-1.0.** Callers who need
/// stable behavior across memoir-core versions should pass an explicit
/// `RankingStrategy::Hybrid { .. }` via [`Self::ranking`].
///
/// Kind toggles (`episodic`/`semantic`) and metadata/time-range filters
/// match [`crate::client::SearchBuilder`]'s shape and semantics.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # use memoir_core::memory::Scope;
/// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
/// let context = client.query("what did the user say about deployments?", scope).await?;
/// println!("{context}");
/// # Ok(())
/// # }
/// ```
///
/// Opt into an explicit strategy:
///
/// ```no_run
/// # use memoir_core::client::{Client, RankingStrategy, DecayFn};
/// # use memoir_core::memory::Scope;
/// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
/// let context = client
///     .query("recent deployments", scope)
///     .ranking(RankingStrategy::Hybrid {
///         alpha: 0.5,
///         decay: DecayFn::Exponential { half_life: chrono::Duration::days(3) },
///     })
///     .await?;
/// # Ok(())
/// # }
/// ```
#[must_use = "query(..) returns a builder that must be awaited"]
pub struct QueryBuilder<'a> {
    client: &'a Client,
    query: String,
    scope: Scope,
    limit: usize,
    episodic: bool,
    semantic: bool,
    metadata_filter: Option<MemoryFilter>,
    min_similarity: Option<f32>,
    created_at_range: NumericRange,
    event_at_range: NumericRange,
    ranking: Option<RankingStrategy>,
}

impl<'a> QueryBuilder<'a> {
    pub(super) fn new(client: &'a Client, query: String, scope: Scope) -> Self {
        Self {
            client,
            query,
            scope,
            limit: DEFAULT_QUERY_LIMIT,
            episodic: false,
            semantic: false,
            metadata_filter: None,
            min_similarity: None,
            created_at_range: NumericRange::default(),
            event_at_range: NumericRange::default(),
            ranking: None,
        }
    }

    /// Caps the number of returned memories. Defaults to [`DEFAULT_QUERY_LIMIT`].
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Restricts retrieval to episodic memories. See [`crate::client::SearchBuilder`] for kind-toggle semantics.
    pub fn episodic(mut self) -> Self {
        self.episodic = true;
        self
    }

    /// Restricts retrieval to semantic memories. See [`crate::client::SearchBuilder`] for kind-toggle semantics.
    pub fn semantic(mut self) -> Self {
        self.semantic = true;
        self
    }

    /// Applies a caller-supplied metadata filter alongside the scope+kind filter.
    pub fn metadata_filter(mut self, filter: MemoryFilter) -> Self {
        self.metadata_filter = Some(filter);
        self
    }

    /// Drops candidates whose raw cosine score is below `threshold`.
    pub fn min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = Some(threshold);
        self
    }

    /// Restricts to memories written at or after `at` (inclusive).
    pub fn created_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_at_range.gte = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts to memories written strictly before `at` (exclusive).
    pub fn created_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_at_range.lt = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts to memories whose `event_at` is at or after `at` (inclusive).
    pub fn event_at_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_range.gte = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts to memories whose `event_at` is strictly before `at` (exclusive).
    pub fn event_at_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_range.lt = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Selects the ranking strategy. Defaults to [`RankingStrategy::default_hybrid`].
    pub fn ranking(mut self, strategy: RankingStrategy) -> Self {
        self.ranking = Some(strategy);
        self
    }
}

fn kind_selector(episodic: bool, semantic: bool) -> KindSelector {
    match (episodic, semantic) {
        (false, false) => KindSelector::default(),
        (episodic, semantic) => KindSelector { episodic, semantic },
    }
}

fn combine_filter(
    metadata_filter: Option<MemoryFilter>,
    created_at: NumericRange,
    event_at: NumericRange,
) -> Option<MemoryFilter> {
    if metadata_filter.is_none() && created_at.is_unbounded() && event_at.is_unbounded() {
        return None;
    }
    let mut combined = metadata_filter.unwrap_or_default();
    if !created_at.is_unbounded() {
        combined.must.push(FilterCondition::Range {
            field: "created_at".to_string(),
            range: created_at,
        });
    }
    if !event_at.is_unbounded() {
        combined.must.push(FilterCondition::Range {
            field: "event_at".to_string(),
            range: event_at,
        });
    }
    Some(combined)
}

fn rank_score(strategy: &RankingStrategy, cosine: f32, memory: &Memory, now: DateTime<FixedOffset>) -> f32 {
    match strategy {
        RankingStrategy::Hybrid { alpha, decay } => {
            let anchor = memory.event_at.unwrap_or(memory.created_at);
            let age = now - anchor;
            let recency = decay.evaluate(age);
            alpha * cosine + (1.0 - alpha) * recency
        }
        RankingStrategy::Blended { weights, decay } => {
            let anchor = memory.event_at.unwrap_or(memory.created_at);
            let recency = decay.evaluate(now - anchor);
            // Normalize confidence 0-100 → [0, 1] so it blends on the same
            // scale as cosine and recency; otherwise its magnitude would
            // dominate regardless of weight.
            let confidence = f32::from(memory.confidence.get()) / 100.0;
            let category_bonus = match &memory.category {
                Some(category) if weights.preferred_categories.iter().any(|c| c == category) => {
                    weights.category_bonus
                }
                _ => 0.0,
            };
            weights.cosine * cosine
                + weights.confidence * confidence
                + weights.recency * recency
                + category_bonus
        }
    }
}

impl<'a> IntoFuture for QueryBuilder<'a> {
    type Output = Result<MemoryContext, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: QueryBuilder<'_>) -> Result<MemoryContext, ClientError> {
    let kinds = kind_selector(builder.episodic, builder.semantic);
    let strategy = builder.ranking.unwrap_or_else(RankingStrategy::default_hybrid);
    let QueryBuilder {
        client,
        query,
        scope,
        limit,
        metadata_filter,
        min_similarity,
        created_at_range,
        event_at_range,
        ..
    } = builder;

    let combined_filter = combine_filter(metadata_filter, created_at_range, event_at_range);
    let candidate_limit = limit.saturating_mul(3).max(limit);
    let inner = client.inner.clone();

    let query_vector = inner.embedder.embed(&query).await?;
    let hits = inner
        .index
        .search(scope, query_vector, candidate_limit, kinds, combined_filter, min_similarity)
        .await?;

    let pids: Vec<&str> = hits.iter().map(|(pid, _)| pid.as_str()).collect();
    let mut rows = inner.store.find_by_pids(&pids).await?;

    let cosine: std::collections::HashMap<&str, f32> = hits
        .iter()
        .map(|(pid, score)| (pid.as_str(), *score))
        .collect();

    let now: DateTime<FixedOffset> = Utc::now().into();
    let mut scored: Vec<(f32, Memory)> = rows
        .drain(..)
        .filter_map(|m| {
            let raw = *cosine.get(m.pid.as_str())?;
            let score = rank_score(&strategy, raw, &m, now);
            Some((score, m))
        })
        .collect();

    scored.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(limit);

    let memories: Vec<Memory> = scored
        .into_iter()
        .map(|(score, mut m)| {
            m.score = Some(score);
            m
        })
        .collect();

    Ok(MemoryContext::new(memories, inner.system_prompt.clone(), strategy))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_default_hybrid_use_documented_alpha_and_decay() {
        let strategy = RankingStrategy::default_hybrid();
        let RankingStrategy::Hybrid { alpha, decay } = strategy else {
            panic!("default_hybrid must return the Hybrid variant; got {strategy:?}");
        };
        assert!((alpha - DEFAULT_HYBRID_ALPHA).abs() < f32::EPSILON);
        assert_eq!(
            decay,
            DecayFn::Exponential {
                half_life: chrono::Duration::days(DEFAULT_HYBRID_HALF_LIFE_DAYS as i64)
            }
        );
    }

    #[test]
    fn should_exponential_decay_be_half_at_half_life() {
        let decay = DecayFn::Exponential {
            half_life: chrono::Duration::days(7),
        };
        let v = decay.evaluate(chrono::Duration::days(7));
        assert!((v - 0.5).abs() < 1e-3, "exp decay at half-life should be ~0.5, got {v}");
    }

    #[test]
    fn should_reciprocal_decay_be_half_at_scale() {
        let decay = DecayFn::Reciprocal {
            scale: chrono::Duration::days(7),
        };
        let v = decay.evaluate(chrono::Duration::days(7));
        assert!((v - 0.5).abs() < 1e-3, "reciprocal decay at scale should be 0.5, got {v}");
    }

    #[test]
    fn should_step_decay_apply_first_matching_bucket() {
        let decay = DecayFn::Step {
            thresholds: vec![
                (chrono::Duration::hours(1), 1.0),
                (chrono::Duration::days(1), 0.5),
                (chrono::Duration::days(7), 0.1),
            ],
        };
        assert_eq!(decay.evaluate(chrono::Duration::minutes(30)), 1.0);
        assert_eq!(decay.evaluate(chrono::Duration::hours(12)), 0.5);
        assert_eq!(decay.evaluate(chrono::Duration::days(3)), 0.1);
        assert_eq!(decay.evaluate(chrono::Duration::days(30)), 0.1);
    }

    #[test]
    fn should_relative_label_render_minutes_and_days() {
        assert_eq!(relative_label(chrono::Duration::seconds(30)), "just now");
        assert_eq!(relative_label(chrono::Duration::minutes(5)), "5 minutes ago");
        assert_eq!(relative_label(chrono::Duration::minutes(1)), "1 minute ago");
        assert_eq!(relative_label(chrono::Duration::hours(3)), "3 hours ago");
        assert_eq!(relative_label(chrono::Duration::days(2)), "2 days ago");
    }

    /// A memory at `now` (zero age) with the given confidence and category.
    fn scored_fixture(now: DateTime<FixedOffset>, confidence: i8, category: Option<&str>) -> Memory {
        Memory {
            pid: "p".into(),
            scope: Scope {
                agent_id: "a".into(),
                org_id: "o".into(),
                user_id: "u".into(),
            },
            content: "c".into(),
            metadata: serde_json::json!({}),
            kind: crate::memory::MemoryKind::Semantic,
            source_pid: None,
            supersession: None,
            created_at: now,
            updated_at: now,
            event_at: None,
            score: None,
            status: crate::store::IndexStatus::Indexed,
            confidence: crate::memory::Confidence::new(confidence),
            category: category.map(str::to_string),
            retirement: None,
        }
    }

    fn balanced_blend() -> RankingStrategy {
        RankingStrategy::blended(BlendWeights::balanced())
    }

    #[test]
    fn should_rank_high_confidence_above_low_at_equal_cosine() {
        // Ticket 0008 verification: a high-confidence row outranks a
        // low-confidence row of equal cosine + equal (zero) age.
        let now = Utc::now().into();
        let strategy = balanced_blend();
        let high = rank_score(&strategy, 0.8, &scored_fixture(now, 95, None), now);
        let low = rank_score(&strategy, 0.8, &scored_fixture(now, 10, None), now);
        assert!(high > low, "high confidence ({high}) must outrank low ({low}) at equal cosine");
    }

    #[test]
    fn should_keep_recency_moving_ranking_at_equal_cosine_and_confidence() {
        // A recent row outranks an old row of equal cosine + confidence.
        let now: DateTime<FixedOffset> = Utc::now().into();
        let strategy = balanced_blend();
        let mut old = scored_fixture(now, 80, None);
        old.created_at = now - chrono::Duration::days(60);
        let recent = scored_fixture(now, 80, None);
        let recent_score = rank_score(&strategy, 0.8, &recent, now);
        let old_score = rank_score(&strategy, 0.8, &old, now);
        assert!(
            recent_score > old_score,
            "recent ({recent_score}) must outrank old ({old_score}) at equal cosine+confidence"
        );
    }

    #[test]
    fn should_apply_category_bonus_only_to_preferred_categories() {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let strategy = RankingStrategy::blended(BlendWeights::balanced().prefer_categories(["preference".to_string()]));
        let preferred = rank_score(&strategy, 0.8, &scored_fixture(now, 80, Some("preference")), now);
        let other = rank_score(&strategy, 0.8, &scored_fixture(now, 80, Some("transient")), now);
        let uncategorized = rank_score(&strategy, 0.8, &scored_fixture(now, 80, None), now);
        assert!(preferred > other, "preferred category must earn the bonus");
        assert!(
            (other - uncategorized).abs() < f32::EPSILON,
            "non-preferred and uncategorized rows must score identically (no bonus)"
        );
    }

    #[test]
    fn should_blend_be_inert_on_category_when_no_preference_set() {
        // With an empty preferred set, category never moves the score.
        let now: DateTime<FixedOffset> = Utc::now().into();
        let strategy = balanced_blend();
        let with_cat = rank_score(&strategy, 0.8, &scored_fixture(now, 80, Some("preference")), now);
        let without = rank_score(&strategy, 0.8, &scored_fixture(now, 80, None), now);
        assert!((with_cat - without).abs() < f32::EPSILON);
    }

    #[test]
    fn should_preset_weights_differ_in_confidence_emphasis() {
        // trust_first weights confidence more heavily than relevance_first.
        assert!(BlendWeights::trust_first().confidence > BlendWeights::relevance_first().confidence);
        assert!(BlendWeights::relevance_first().cosine > BlendWeights::trust_first().cosine);
    }
}
