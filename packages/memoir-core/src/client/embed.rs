//! Background embed-on-write substrate for [`super::Client`].
//!
//! Methods on [`super::ClientInner`] spawn a detached `tokio::spawn` task per
//! Remember that embeds the row's content, upserts the resulting vector into
//! the configured index, and flips the row's lifecycle from `pending` to
//! `indexed` (or `failed` on error).
//!
//! Failure semantics: a task that fails at any step records the failure on
//! the row's `qdrant_status` and emits a structured `tracing` event. The
//! reconciliation sweep (ticket 0012) owns retries; this module does not
//! retry inline. Tasks do not survive process restarts — rows stuck at
//! `pending` are also reconciliation's problem.

use std::sync::Arc;

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
    /// Spawns a detached embed-and-index task for the freshly written row.
    ///
    /// Fire-and-forget: callers do not await it. Progress and outcome are
    /// observable via the `memoir.embed.*` tracing events.
    pub(super) fn spawn_embed_for_write(self: &Arc<Self>, written: Memory) {
        let inner = self.clone();
        let span = info_span!("memoir.embed", pid = %written.pid);
        tokio::spawn(async move { inner.embed_and_index(written).await }.instrument(span));
    }

    /// Embeds the row, upserts the vector, and flips its lifecycle state.
    ///
    /// Shared between the write-time `tokio::spawn` path
    /// ([`Self::spawn_embed_for_write`]) and the reconciliation sweep's
    /// retry pass. Returns whether the row reached `indexed` or got flipped
    /// to `failed`.
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
