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

impl ClientInner {
    /// Spawns a detached embed-and-index task for the freshly written row.
    ///
    /// Fire-and-forget: callers do not await it. Progress and outcome are
    /// observable via the `memoir.embed.*` tracing events.
    pub(super) fn spawn_embed_for_write(self: &Arc<Self>, written: Memory) {
        let inner = self.clone();
        let span = info_span!("memoir.embed", pid = %written.pid);
        tokio::spawn(inner.run_embed(written).instrument(span));
    }

    async fn run_embed(self: Arc<Self>, written: Memory) {
        let pid = written.pid.as_str();

        let vector = match self.embedder.embed(&written.content).await {
            Ok(v) => v,
            Err(err) => {
                event!(
                    name: "memoir.embed.embed_failed",
                    Level::WARN,
                    pid = %pid,
                    error = %err,
                    "embed step failed for {{pid}}",
                );
                self.record_failed(pid).await;
                return;
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
                error = %err,
                "vector upsert failed for {{pid}}",
            );
            self.record_failed(pid).await;
            return;
        }

        if let Err(err) = self.store.set_index_status(pid, IndexStatus::Indexed).await {
            event!(
                name: "memoir.embed.index_status_failed",
                Level::WARN,
                pid = %pid,
                error = %err,
                "set_index_status(indexed) failed for {{pid}} — row stays pending until reconciliation",
            );
            return;
        }

        event!(
            name: "memoir.embed.success",
            Level::INFO,
            pid = %pid,
            "{{pid}} indexed",
        );
    }

    async fn record_failed(&self, pid: &str) {
        if let Err(err) = self.store.set_index_status(pid, IndexStatus::Failed).await {
            event!(
                name: "memoir.embed.index_status_failed",
                Level::WARN,
                pid = %pid,
                error = %err,
                "set_index_status(failed) failed for {{pid}} — row stays pending until reconciliation",
            );
        }
    }
}
