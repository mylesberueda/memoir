//! Worker stage that reconciles staged triples and commits the graph.
//!
//! Dispatched by the worker loop ([`super::worker`]) when a job's kind is
//! [`crate::jobs::JobKind::Synthesize`]. This is the two-parent fan-in: it runs
//! exactly once per source, after both the semantic ([`super::extract`]) and
//! relational ([`super::relational`]) siblings succeed (enforced by the atomic
//! [`crate::jobs::MemoryJobsStore::enqueue_synthesis_if_ready`] guard).
//!
//! It reads the staged triples ([`crate::graph::TripleStaging`]) and the
//! source's semantic facts, reconciles them through a [`crate::graph::Synthesizer`]
//! (vetoing uncorroborated triples), commits the reconciled set to the graph —
//! the single graph write for this source — and clears the staging row.
//!
//! Only compiled with the `knowledge-graph` feature.

use std::sync::Arc;

use tracing::{Instrument, Level, event, info_span};

use crate::graph::{
    CardinalityPolicy, CommitContext, EmbeddingEntityResolver, EmbeddingSynthesizer, FalkorEdgeCatalog,
    FalkorEntityCatalog, SemanticFact, Synthesizer, TemporalEdgeResolver, commit_triples,
};
use crate::jobs::Job;
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

/// Failure modes for the synthesize worker stage.
#[derive(Debug, thiserror::Error)]
pub(super) enum SynthesizeError {
    /// Loading the episodic source or its semantic facts hit the database.
    #[error("source lookup failed: {0}")]
    SourceLookup(String),

    /// Reading or clearing the triple staging store failed.
    #[error("triple staging access failed: {0}")]
    Staging(String),

    /// The synthesizer's reconciliation failed.
    #[error("synthesis failed: {0}")]
    Synthesis(String),

    /// Committing the reconciled triples to the graph failed.
    #[error("graph commit failed: {0}")]
    Commit(String),
}

impl ClientInner {
    /// Runs the synthesis fan-in for one claimed job.
    ///
    /// Reconciles the source's staged triples against its semantic facts and
    /// commits the result to the graph. A source that vanished, has no staged
    /// triples, or no configured graph is a no-op success.
    pub(super) async fn run_synthesize(self: &Arc<Self>, job: Job) -> Result<(), SynthesizeError> {
        let span = info_span!("memoir.synthesize", source_pid = %job.source_pid);
        async move { self.run_synthesize_inner(job).await }.instrument(span).await
    }

    async fn run_synthesize_inner(self: &Arc<Self>, job: Job) -> Result<(), SynthesizeError> {
        let pid = job.source_pid.clone();

        let Some(graph) = self.graph.clone() else {
            event!(
                name: "memoir.synthesize.no_graph",
                Level::WARN,
                source_pid = %pid,
                "no graph store configured; treating job as no-op",
            );
            return Ok(());
        };

        let Some(triples) = self
            .triple_staging
            .take_pending(&pid)
            .await
            .map_err(|err| SynthesizeError::Staging(err.to_string()))?
        else {
            event!(
                name: "memoir.synthesize.no_staged_triples",
                Level::WARN,
                source_pid = %pid,
                "no staged triples for source; treating job as no-op",
            );
            return Ok(());
        };

        let source = match self.store.recall(&pid).await {
            Ok(source) => source,
            Err(StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.synthesize.source_missing",
                    Level::WARN,
                    source_pid = %pid,
                    "episodic source vanished before synthesis; clearing staging and treating job as no-op",
                );
                self.triple_staging
                    .clear(&pid)
                    .await
                    .map_err(|err| SynthesizeError::Staging(err.to_string()))?;
                return Ok(());
            }
            Err(err) => return Err(SynthesizeError::SourceLookup(err.to_string())),
        };

        let facts = self
            .store
            .active_semantics_for_source(&pid)
            .await
            .map_err(|err| SynthesizeError::SourceLookup(err.to_string()))?
            .into_iter()
            .map(|memory| SemanticFact { content: memory.content })
            .collect::<Vec<_>>();

        let synthesizer = EmbeddingSynthesizer::new(self.embedder.clone());
        let reconciled = synthesizer
            .synthesize(triples, &facts)
            .await
            .map_err(|err| SynthesizeError::Synthesis(err.to_string()))?;

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

        let committed = commit_triples(graph.as_ref(), &self.embedder, &entities, &edges, &ctx, &reconciled)
            .await
            .map_err(|err| SynthesizeError::Commit(err.to_string()))?;

        self.triple_staging
            .clear(&pid)
            .await
            .map_err(|err| SynthesizeError::Staging(err.to_string()))?;

        event!(
            name: "memoir.synthesize.committed",
            Level::DEBUG,
            source_pid = %source.pid,
            fact_count = facts.len(),
            committed,
            "synthesized and committed triples to the graph",
        );

        Ok(())
    }
}
