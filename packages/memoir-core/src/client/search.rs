//! Per-call builder for [`Client::search`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use chrono::{DateTime, FixedOffset};

use crate::embedding::EmbeddingModel;
use crate::memory::{KindSelector, Memories, Scope};
use crate::store::MemoryStore;
use crate::vector::{FilterCondition, MemoryFilter, NumericRange, VectorIndex};

use super::{Client, ClientError};

/// Default page size when the caller does not specify `limit`.
pub const DEFAULT_LIMIT: usize = 10;

/// Per-call builder returned by [`Client::search`].
///
/// Awaiting the builder embeds the query, runs a vector search against the
/// scope's indexed memories, and returns the matching rows wrapped in
/// [`Memories`]. The kind toggles [`Self::episodic`] and [`Self::semantic`]
/// are independent: toggling neither retrieves both kinds; toggling either
/// filters retrieval to that kind; toggling both is equivalent to toggling
/// neither.
///
/// Only rows whose vector index entry has reached `indexed` are eligible.
/// Recently-written memories that are still `pending` are filtered out —
/// inspect them via [`Client::recall`] when their pid is known.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # use memoir_core::memory::Scope;
/// # use memoir_core::vector::{FilterCondition, MatchValue, MemoryFilter};
/// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
/// let exclude_current_conversation = MemoryFilter {
///     must_not: vec![FilterCondition::Equals {
///         field: "conversation_id".to_string(),
///         value: MatchValue::Integer(42),
///     }],
///     ..MemoryFilter::default()
/// };
/// let memories = client
///     .search("what did the user just say?", scope)
///     .limit(5)
///     .episodic()
///     .metadata_filter(exclude_current_conversation)
///     .min_similarity(0.3)
///     .await?;
/// for m in memories.list() {
///     println!("{}", m.content);
/// }
/// # Ok(())
/// # }
/// ```
///
/// Filter by time windows on either `created_at` (write time) or `event_at`
/// (event time). Bounds are half-open: `after` is inclusive (`>=`), `before`
/// is exclusive (`<`), matching Rust's `start..end` convention. Filtering by
/// `event_at` implicitly excludes memories with no known event-time.
///
/// ```no_run
/// # use chrono::{DateTime, Utc};
/// # use memoir_core::client::Client;
/// # use memoir_core::memory::Scope;
/// # async fn example(client: &Client, scope: Scope, week_ago: DateTime<Utc>, now: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
/// let memories = client
///     .search("deployment status", scope)
///     .created_after(week_ago)
///     .created_before(now)
///     .await?;
/// # Ok(())
/// # }
/// ```
#[must_use = "search(..) returns a builder that must be awaited"]
pub struct SearchBuilder<'a> {
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
    confidence_range: NumericRange,
    category: Option<String>,
}

impl<'a> SearchBuilder<'a> {
    pub(super) fn new(client: &'a Client, query: String, scope: Scope) -> Self {
        Self {
            client,
            query,
            scope,
            limit: DEFAULT_LIMIT,
            episodic: false,
            semantic: false,
            metadata_filter: None,
            min_similarity: None,
            created_at_range: NumericRange::default(),
            event_at_range: NumericRange::default(),
            confidence_range: NumericRange::default(),
            category: None,
        }
    }

    /// Caps the number of retrieved memories. Defaults to [`DEFAULT_LIMIT`].
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Restricts retrieval to episodic memories.
    ///
    /// Calling this without [`Self::semantic`] excludes semantic memories from
    /// the result. Calling both (or calling neither) retrieves both kinds.
    pub fn episodic(mut self) -> Self {
        self.episodic = true;
        self
    }

    /// Restricts retrieval to semantic memories.
    ///
    /// Calling this without [`Self::episodic`] excludes episodic memories from
    /// the result. Calling both (or calling neither) retrieves both kinds.
    pub fn semantic(mut self) -> Self {
        self.semantic = true;
        self
    }

    /// Applies a caller-supplied metadata filter alongside the scope+kind filter.
    ///
    /// AND-joined with the scope conditions: caller-supplied filter cannot
    /// widen scope. Multiple calls replace (last wins). See [`MemoryFilter`]
    /// for the shape.
    pub fn metadata_filter(mut self, filter: MemoryFilter) -> Self {
        self.metadata_filter = Some(filter);
        self
    }

    /// Drops hits whose similarity score is below `threshold`.
    ///
    /// `threshold` is in the same range as [`crate::memory::Memory::score`]:
    /// cosine similarity in `[-1.0, 1.0]`, where higher = closer. The
    /// vector backend applies the floor before results are returned;
    /// memoir-core does not post-filter. Multiple calls replace (last wins).
    pub fn min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = Some(threshold);
        self
    }

    /// Restricts retrieval to memories written at or after `at` (inclusive).
    ///
    /// Filters on the `created_at` payload key. Combine with
    /// [`Self::created_before`] for a half-open window. Multiple calls
    /// replace (last wins). Accepts any value convertible to
    /// `DateTime<FixedOffset>`, including `DateTime<Utc>`.
    pub fn created_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_at_range.gte = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts retrieval to memories written strictly before `at` (exclusive).
    ///
    /// Filters on the `created_at` payload key. Half-open semantics
    /// (`< at`) match Rust's `start..end` convention: `.created_after(jan_1)`
    /// `.created_before(feb_1)` retrieves January's memories. Multiple calls
    /// replace (last wins).
    pub fn created_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_at_range.lt = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts retrieval to memories whose event-time is at or after `at` (inclusive).
    ///
    /// Filters on the `event_at` payload key. Memories with no known
    /// event-time (the key is absent from the payload) are excluded by
    /// this filter — Qdrant treats missing range-target keys as
    /// non-matches. Combine with [`Self::event_at_before`] for a window.
    pub fn event_at_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_range.gte = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts retrieval to memories whose event-time is strictly before `at` (exclusive).
    ///
    /// Filters on the `event_at` payload key. Half-open semantics; see
    /// [`Self::created_before`] for the rationale. Memories with no known
    /// event-time are excluded.
    pub fn event_at_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_range.lt = Some(at.into().timestamp_millis() as f64);
        self
    }

    /// Restricts retrieval to memories whose confidence is at or above `min`.
    ///
    /// A hard floor on the `confidence` payload key (0-100 percentage):
    /// rows below `min` are excluded by the vector backend. This is a hard
    /// filter, distinct from the soft confidence *weighting* the selection
    /// layer applies (epic 0011 ticket 0008). Multiple calls replace (last
    /// wins). Out-of-range values are clamped via [`crate::memory::Confidence`].
    pub fn min_confidence(mut self, min: i8) -> Self {
        self.confidence_range.gte = Some(f64::from(crate::memory::Confidence::new(min).get()));
        self
    }

    /// Restricts retrieval to memories with exactly this category.
    ///
    /// A hard equality match on the `category` payload key. Rows with a
    /// different category — or none yet assigned — are excluded (Qdrant
    /// treats the missing key as a non-match). This is a hard filter; the
    /// soft "prefer this category" ranking signal is the selection layer's
    /// concern (epic 0011 ticket 0008). Multiple calls replace (last wins).
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    fn kind_selector(&self) -> KindSelector {
        kind_selector(self.episodic, self.semantic)
    }
}

fn kind_selector(episodic: bool, semantic: bool) -> KindSelector {
    match (episodic, semantic) {
        (false, false) => KindSelector::default(),
        (episodic, semantic) => KindSelector { episodic, semantic },
    }
}

/// Folds the time-range bounds into the caller-supplied metadata filter.
///
/// Time-range conditions are AND-joined with `metadata_filter.must` — both
/// represent "must match" predicates. `must_not` and `should` from the
/// caller-supplied filter are passed through unchanged. Returns `None` when
/// neither the caller filter nor any time-range bound is set, so the
/// vector backend skips filter translation entirely.
fn combine_filter(
    metadata_filter: Option<MemoryFilter>,
    created_at: NumericRange,
    event_at: NumericRange,
    confidence: NumericRange,
    category: Option<String>,
) -> Option<MemoryFilter> {
    if metadata_filter.is_none()
        && created_at.is_unbounded()
        && event_at.is_unbounded()
        && confidence.is_unbounded()
        && category.is_none()
    {
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
    if !confidence.is_unbounded() {
        combined.must.push(FilterCondition::Range {
            field: "confidence".to_string(),
            range: confidence,
        });
    }
    if let Some(category) = category {
        combined.must.push(FilterCondition::Equals {
            field: "category".to_string(),
            value: crate::vector::MatchValue::Keyword(category),
        });
    }
    Some(combined)
}

impl<'a> IntoFuture for SearchBuilder<'a> {
    type Output = Result<Memories, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: SearchBuilder<'_>) -> Result<Memories, ClientError> {
    let kinds = builder.kind_selector();
    let SearchBuilder {
        client,
        query,
        scope,
        limit,
        metadata_filter,
        min_similarity,
        created_at_range,
        event_at_range,
        confidence_range,
        category,
        ..
    } = builder;

    let combined_filter = combine_filter(
        metadata_filter,
        created_at_range,
        event_at_range,
        confidence_range,
        category,
    );

    let inner = client.inner.clone();

    let query_vector = inner.embedder.embed(&query).await?;
    let hits = inner
        .index
        .search(scope, query_vector, limit, kinds, combined_filter, min_similarity)
        .await?;

    let pids: Vec<&str> = hits.iter().map(|(pid, _)| pid.as_str()).collect();
    let mut rows = inner.store.find_by_pids(&pids).await?;

    let order: std::collections::HashMap<&str, (usize, f32)> = hits
        .iter()
        .enumerate()
        .map(|(idx, (pid, score))| (pid.as_str(), (idx, *score)))
        .collect();
    rows.sort_by_key(|m| order.get(m.pid.as_str()).map(|(idx, _)| *idx).unwrap_or(usize::MAX));
    for memory in &mut rows {
        memory.score = order.get(memory.pid.as_str()).map(|(_, score)| *score);
    }

    Ok(Memories::new(rows, inner.system_prompt.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_select_all_kinds_when_no_kind_toggled() {
        let selector = kind_selector(false, false);
        assert!(selector.episodic);
        assert!(selector.semantic);
    }

    #[test]
    fn should_select_all_kinds_when_both_kinds_toggled() {
        let selector = kind_selector(true, true);
        assert!(selector.episodic);
        assert!(selector.semantic);
    }

    #[test]
    fn should_select_only_episodic_when_only_episodic_toggled() {
        let selector = kind_selector(true, false);
        assert!(selector.episodic);
        assert!(!selector.semantic);
    }

    #[test]
    fn should_select_only_semantic_when_only_semantic_toggled() {
        let selector = kind_selector(false, true);
        assert!(!selector.episodic);
        assert!(selector.semantic);
    }
}
