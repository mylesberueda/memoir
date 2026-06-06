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

use crate::jobs::Job;
use crate::store::{MemoryStore, StoreError};

use super::ClientInner;

/// Failure modes for the relational-extract worker stage.
#[derive(Debug, thiserror::Error)]
pub(super) enum RelationalExtractError {
    /// Loading the episodic source hit the database.
    #[error("source lookup failed: {0}")]
    SourceLookup(String),
}

impl ClientInner {
    /// Runs the relational-extract pipeline for one claimed job.
    ///
    /// Loads the episodic source named by the job and returns `Ok(())`. Triple
    /// extraction and graph commit are not wired yet, so this is currently a
    /// no-op beyond the source load; a source that vanished between enqueue and
    /// claim is a no-op success, not a failure.
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

        event!(
            name: "memoir.relational.claimed",
            Level::DEBUG,
            source_pid = %source.pid,
            "relational extract claimed; triple extraction not yet wired",
        );

        Ok(())
    }
}
