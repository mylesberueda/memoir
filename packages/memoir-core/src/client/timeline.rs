//! Per-call builder for [`Client::timeline`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use chrono::{DateTime, FixedOffset};

use crate::memory::{KindSelector, Memory, Scope};
use crate::store::{DEFAULT_TIMELINE_LIMIT, MemoryStore, TimelineDirection, TimelineParams};

use super::{Client, ClientError};

/// Per-call builder returned by [`Client::timeline`].
///
/// Awaiting the builder runs a Postgres-only chronological read of memories
/// in `scope`. No embedding, no Qdrant. Includes superseded rows by default
/// — opt out via [`Self::exclude_superseded`]. Default order is newest-first
/// and default limit is [`DEFAULT_TIMELINE_LIMIT`].
///
/// Kind toggles match [`crate::client::SearchBuilder`]: toggling neither
/// retrieves both kinds; toggling either filters retrieval to that kind;
/// toggling both is equivalent to toggling neither.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # use memoir_core::memory::Scope;
/// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
/// let recent = client.timeline(scope).limit(20).await?;
/// for m in &recent {
///     println!("{}: {}", m.created_at, m.content);
/// }
/// # Ok(())
/// # }
/// ```
#[must_use = "timeline(..) returns a builder that must be awaited"]
pub struct TimelineBuilder<'a> {
    client: &'a Client,
    scope: Scope,
    episodic: bool,
    semantic: bool,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    event_at_after: Option<DateTime<FixedOffset>>,
    event_at_before: Option<DateTime<FixedOffset>>,
    include_superseded: bool,
    limit: usize,
    direction: TimelineDirection,
}

impl<'a> TimelineBuilder<'a> {
    pub(super) fn new(client: &'a Client, scope: Scope) -> Self {
        Self {
            client,
            scope,
            episodic: false,
            semantic: false,
            created_after: None,
            created_before: None,
            event_at_after: None,
            event_at_before: None,
            include_superseded: true,
            limit: DEFAULT_TIMELINE_LIMIT,
            direction: TimelineDirection::Descending,
        }
    }

    /// Caps the number of returned rows. Defaults to [`DEFAULT_TIMELINE_LIMIT`].
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Restricts retrieval to episodic memories. See builder doc for kind-toggle semantics.
    pub fn episodic(mut self) -> Self {
        self.episodic = true;
        self
    }

    /// Restricts retrieval to semantic memories. See builder doc for kind-toggle semantics.
    pub fn semantic(mut self) -> Self {
        self.semantic = true;
        self
    }

    /// Restricts to memories written at or after `at` (inclusive).
    pub fn created_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_after = Some(at.into());
        self
    }

    /// Restricts to memories written strictly before `at` (exclusive).
    pub fn created_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.created_before = Some(at.into());
        self
    }

    /// Restricts to memories whose `event_at` is at or after `at` (inclusive).
    ///
    /// Memories with no `event_at` set are excluded by this filter.
    pub fn event_at_after(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_after = Some(at.into());
        self
    }

    /// Restricts to memories whose `event_at` is strictly before `at` (exclusive).
    ///
    /// Memories with no `event_at` set are excluded by this filter.
    pub fn event_at_before(mut self, at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at_before = Some(at.into());
        self
    }

    /// Drops superseded rows from the result.
    ///
    /// Default behavior includes them — timeline is the audit view.
    pub fn exclude_superseded(mut self) -> Self {
        self.include_superseded = false;
        self
    }

    /// Returns rows in oldest-first order. Default is newest-first.
    pub fn ascending(mut self) -> Self {
        self.direction = TimelineDirection::Ascending;
        self
    }
}

fn kind_selector(episodic: bool, semantic: bool) -> KindSelector {
    match (episodic, semantic) {
        (false, false) => KindSelector::default(),
        (episodic, semantic) => KindSelector { episodic, semantic },
    }
}

impl<'a> IntoFuture for TimelineBuilder<'a> {
    type Output = Result<Vec<Memory>, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: TimelineBuilder<'_>) -> Result<Vec<Memory>, ClientError> {
    let kinds = kind_selector(builder.episodic, builder.semantic);
    let TimelineBuilder {
        client,
        scope,
        created_after,
        created_before,
        event_at_after,
        event_at_before,
        include_superseded,
        limit,
        direction,
        ..
    } = builder;

    let params = TimelineParams {
        kinds,
        created_after,
        created_before,
        event_at_after,
        event_at_before,
        include_superseded,
        limit,
        direction,
    };

    let memories = client.inner.store.timeline(scope, params).await?;
    Ok(memories)
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
    fn should_select_only_episodic_when_only_episodic_toggled() {
        let selector = kind_selector(true, false);
        assert!(selector.episodic);
        assert!(!selector.semantic);
    }
}
