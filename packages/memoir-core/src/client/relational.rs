//! Worker stage that derives relational triples from an episodic memory.
//!
//! Dispatched by the worker loop ([`super::worker`]) when a job's kind is
//! [`crate::jobs::JobKind::RelationalExtract`]. Parallel to the semantic
//! extraction stage ([`super::extract`]): both fan out from the same episodic
//! write. This stage loads the episodic source and (once ticket 0004 lands the
//! [`crate::graph::GraphStore`] write path) extracts `(subject, relation,
//! object)` triples and commits them to the graph.
//!
//! Only compiled with the `knowledge-graph` feature.

use std::sync::Arc;

use tracing::{Instrument, Level, event, info_span};

use crate::graph::{
    CardinalityPolicy, CommitContext, EmbeddingEntityResolver, FalkorEdgeCatalog, FalkorEntityCatalog, LlmExtractor,
    TemporalEdgeResolver, TripleExtractor, commit_triples,
};
use crate::jobs::Job;
use crate::llm::LlmRole;
use crate::store::{MemoryStore, StoreError};

use super::ClientInner;

/// Relations treated as single-valued by the temporal edge resolver (v1).
///
/// A single-valued relation holds one live object at a time, so a newer fact
/// supersedes the old (Alice's employer, residence). Every other relation
/// defaults to multi-valued and coexists — see [`CardinalityPolicy`]. This v1
/// set is deliberately small; relations not listed append rather than supersede,
/// the safe default when extraction uses an open vocabulary.
const SINGLE_VALUED_RELATIONS: &[&str] = &["works at", "lives in", "located in", "reports to"];

/// Failure modes for the relational-extract worker stage.
#[derive(Debug, thiserror::Error)]
pub(super) enum RelationalExtractError {
    /// Loading the episodic source hit the database.
    #[error("source lookup failed: {0}")]
    SourceLookup(String),

    /// The triple extractor's LLM call or reply parse failed.
    #[error("triple extraction failed: {0}")]
    Extraction(String),

    /// Resolving or committing triples to the graph failed.
    #[error("graph commit failed: {0}")]
    Commit(String),
}

impl ClientInner {
    /// Runs the relational-extract pipeline for one claimed job.
    ///
    /// Loads the episodic source, extracts relational triples from its content,
    /// resolves their entities and edges, and commits them to the graph. A source
    /// that vanished between enqueue and claim, no configured relational LLM, or
    /// no configured graph store is a no-op success rather than a failure.
    pub(super) async fn run_relational_extract(self: &Arc<Self>, job: Job) -> Result<(), RelationalExtractError> {
        let span = info_span!("memoir.relational", source_pid = %job.source_pid);
        async move { self.run_relational_extract_inner(job).await }
            .instrument(span)
            .await
    }

    async fn run_relational_extract_inner(self: &Arc<Self>, job: Job) -> Result<(), RelationalExtractError> {
        let pid = job.source_pid.clone();

        let source = match self.store.recall(&pid).await {
            Ok(source) => source,
            Err(StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.relational.source_missing",
                    Level::WARN,
                    source_pid = %pid,
                    "episodic source vanished before relational extract; treating job as no-op",
                );
                return Ok(());
            }
            Err(err) => return Err(RelationalExtractError::SourceLookup(err.to_string())),
        };

        let Some(provider) = self.llms.get(LlmRole::Relational) else {
            event!(
                name: "memoir.relational.skipped",
                Level::WARN,
                source_pid = %pid,
                "no relational llm configured; treating job as no-op",
            );
            return Ok(());
        };

        let Some(graph) = self.graph.clone() else {
            event!(
                name: "memoir.relational.no_graph",
                Level::WARN,
                source_pid = %pid,
                "no graph store configured; treating job as no-op",
            );
            return Ok(());
        };

        let extractor = LlmExtractor::new(provider.clone());
        let triples = extractor
            .extract(&source.content)
            .await
            .map_err(|err| RelationalExtractError::Extraction(err.to_string()))?;

        let entities = EmbeddingEntityResolver::new(self.embedder.clone(), FalkorEntityCatalog::new(graph.clone()));
        let edges = TemporalEdgeResolver::new(
            FalkorEdgeCatalog::new(graph.clone()),
            CardinalityPolicy::with_single_valued(SINGLE_VALUED_RELATIONS.iter().copied()),
        );
        let ctx = CommitContext {
            scope: source.scope.clone(),
            memory_pid: source.pid.clone(),
            valid_from: source.event_at.unwrap_or(source.created_at),
        };

        let committed = commit_triples(graph.as_ref(), &self.embedder, &entities, &edges, &ctx, &triples)
            .await
            .map_err(|err| RelationalExtractError::Commit(err.to_string()))?;

        event!(
            name: "memoir.relational.committed",
            Level::DEBUG,
            source_pid = %source.pid,
            triple_count = triples.len(),
            committed,
            "committed relational triples to the graph",
        );

        Ok(())
    }
}
