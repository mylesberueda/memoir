//! Embed-and-index substrate for [`super::Client`].
//!
//! Memories that need their content embedded and upserted into the vector
//! index flow through here. Two paths converge on [`ClientInner::embed_and_index`]:
//!
//! 1. **Worker-driven** ([`ClientInner::run_embed_job`]) — called by the
//!    queue worker after claiming a `JobKind::Embed` row. This is the only
//!    path on the happy `Client::remember` → enqueue → worker → indexed
//!    write loop.
//! 2. **Reconciliation-driven** ([`super::reconcile`]) — retries `failed`
//!    rows by hydrating each one and re-running the same pipeline.
//!
//! Failure semantics: per-stage errors are recorded on the row's
//! `qdrant_status` and emitted as structured `tracing` events. The
//! reconciliation sweep owns retries; this module does not retry inline.

use tracing::{Instrument, Level, event, info_span};

use crate::embedding::EmbeddingModel;
use crate::memory::Memory;
use crate::store::{IndexStatus, MemoryStore};
use crate::vector::VectorIndex;

use super::ClientInner;

/// Outcome of an inline (awaited) embed-and-index pass.
///
/// Returned by [`ClientInner::embed_and_index`] so callers like the
/// reconciliation sweep can count successes vs. failures without re-parsing
/// tracing output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EmbedOutcome {
    Indexed,
    Failed,
}

impl ClientInner {
    /// Runs an embed job claimed from the queue.
    ///
    /// Loads the source memory by pid, then delegates to
    /// [`Self::embed_and_index`]. Returns `Ok(())` for success and the
    /// no-op cascade-delete-race case (source already forgotten); returns
    /// `Err` only when a real failure should flip the job to `failed`.
    pub(super) async fn run_embed_job(
        &self,
        source_pid: &str,
    ) -> Result<(), crate::store::StoreError> {
        let written = match self.store.recall(source_pid).await {
            Ok(memory) => memory,
            Err(crate::store::StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.embed.source_missing",
                    Level::INFO,
                    pid = %source_pid,
                    "source memory absent for {{pid}} (cascade delete race); skipping",
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        };

        let span = info_span!("memoir.embed", pid = %written.pid);
        async move {
            // embed_and_index returns its own outcome; per-stage failures are
            // logged via tracing events inside it. Either way the job is
            // complete from the worker's perspective.
            let _ = self.embed_and_index(written).await;
        }
        .instrument(span)
        .await;

        Ok(())
    }

    /// Embeds the row, upserts the vector, and flips its lifecycle state.
    ///
    /// Shared between the worker-driven path
    /// ([`Self::run_embed_job`]) and the reconciliation sweep's retry pass.
    /// Returns whether the row reached `indexed` or got flipped to `failed`.
    pub(super) async fn embed_and_index(&self, written: Memory) -> EmbedOutcome {
        let pid = written.pid.as_str();

        let vector = match self.embedder.embed(&written.content).await {
            Ok(v) => v,
            Err(err) => {
                event!(
                    name: "memoir.embed.embed_failed",
                    Level::WARN,
                    pid = %pid,
                    error.message = %err,
                    "embed step failed for {{pid}}: {{error.message}}",
                );
                self.record_failed(pid).await;
                return EmbedOutcome::Failed;
            }
        };

        if let Err(err) = self
            .index
            .upsert(&written.pid, &written.scope, written.kind, vector)
            .await
        {
            event!(
                name: "memoir.embed.upsert_failed",
                Level::WARN,
                pid = %pid,
                error.message = %err,
                "vector upsert failed for {{pid}}: {{error.message}}",
            );
            self.record_failed(pid).await;
            return EmbedOutcome::Failed;
        }

        if let Err(err) = self.store.set_index_status(pid, IndexStatus::Indexed).await {
            event!(
                name: "memoir.embed.index_status_failed",
                Level::WARN,
                pid = %pid,
                error.message = %err,
                "set_index_status(indexed) failed for {{pid}}: {{error.message}} — row stays pending until reconciliation",
            );
            return EmbedOutcome::Failed;
        }

        event!(
            name: "memoir.embed.success",
            Level::INFO,
            pid = %pid,
            "{{pid}} indexed",
        );
        EmbedOutcome::Indexed
    }

    async fn record_failed(&self, pid: &str) {
        if let Err(err) = self.store.set_index_status(pid, IndexStatus::Failed).await {
            event!(
                name: "memoir.embed.index_status_failed",
                Level::WARN,
                pid = %pid,
                error.message = %err,
                "set_index_status(failed) failed for {{pid}}: {{error.message}} — row stays pending until reconciliation",
            );
        }
    }
}
