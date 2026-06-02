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

/// How [`Client::query`] orders the candidate set.
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
        match strategy {
            RankingStrategy::Hybrid { alpha, decay } => {
                assert!((alpha - DEFAULT_HYBRID_ALPHA).abs() < f32::EPSILON);
                assert_eq!(
                    decay,
                    DecayFn::Exponential {
                        half_life: chrono::Duration::days(DEFAULT_HYBRID_HALF_LIFE_DAYS as i64)
                    }
                );
            }
        }
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
}
