//! Per-call builder for [`Client::search`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use crate::embedding::EmbeddingModel;
use crate::memory::{KindSelector, Memories, Scope};
use crate::store::MemoryStore;
use crate::vector::VectorIndex;

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
/// # async fn example(client: &Client, scope: Scope) -> Result<(), Box<dyn std::error::Error>> {
/// let memories = client
///     .search("what did the user just say?", scope)
///     .limit(5)
///     .episodic()
///     .await?;
/// for m in memories.list() {
///     println!("{}", m.content);
/// }
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
        ..
    } = builder;

    let inner = client.inner.clone();

    let query_vector = inner.embedder.embed(&query).await?;
    let hits = inner.index.search(scope, query_vector, limit, kinds).await?;

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
