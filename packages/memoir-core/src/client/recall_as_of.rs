//! Per-call builder for [`Client::recall_as_of`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use chrono::{DateTime, FixedOffset};

use crate::memory::{KindSelector, Memory, Scope};
use crate::store::{AsOfParams, DEFAULT_TIMELINE_LIMIT, MemoryStore};

use super::{Client, ClientError};

/// Per-call builder returned by [`Client::recall_as_of`].
///
/// Awaiting the builder returns the memories in `scope` that were active at
/// `as_of`: created on or before `as_of` AND, considering only supersession
/// events with `decided_at <= as_of`, not currently superseded.
///
/// Pure Postgres read; no Qdrant, no embedder, no LLM. Ordering is
/// newest-first by `created_at`. Default limit is
/// [`DEFAULT_TIMELINE_LIMIT`].
///
/// # Examples
///
/// ```no_run
/// # use chrono::{DateTime, Utc};
/// # use memoir_core::client::Client;
/// # use memoir_core::memory::Scope;
/// # async fn example(client: &Client, scope: Scope, last_tuesday: DateTime<Utc>) -> Result<(), Box<dyn std::error::Error>> {
/// let known_then = client.recall_as_of(scope, last_tuesday).await?;
/// for m in &known_then {
///     println!("{}: {}", m.created_at, m.content);
/// }
/// # Ok(())
/// # }
/// ```
#[must_use = "recall_as_of(..) returns a builder that must be awaited"]
pub struct RecallAsOfBuilder<'a> {
    client: &'a Client,
    scope: Scope,
    as_of: DateTime<FixedOffset>,
    episodic: bool,
    semantic: bool,
    limit: usize,
}

impl<'a> RecallAsOfBuilder<'a> {
    pub(super) fn new(client: &'a Client, scope: Scope, as_of: DateTime<FixedOffset>) -> Self {
        Self {
            client,
            scope,
            as_of,
            episodic: false,
            semantic: false,
            limit: DEFAULT_TIMELINE_LIMIT,
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
}

fn kind_selector(episodic: bool, semantic: bool) -> KindSelector {
    match (episodic, semantic) {
        (false, false) => KindSelector::default(),
        (episodic, semantic) => KindSelector { episodic, semantic },
    }
}

impl<'a> IntoFuture for RecallAsOfBuilder<'a> {
    type Output = Result<Vec<Memory>, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: RecallAsOfBuilder<'_>) -> Result<Vec<Memory>, ClientError> {
    let kinds = kind_selector(builder.episodic, builder.semantic);
    let RecallAsOfBuilder {
        client,
        scope,
        as_of,
        limit,
        ..
    } = builder;

    let params = AsOfParams { as_of, kinds, limit };
    let memories = client.inner.store.memories_as_of(scope, params).await?;
    Ok(memories)
}
