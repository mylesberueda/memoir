//! Worker stage that re-derives semantic memories from a corrected source.
//!
//! Dispatched by the worker loop ([`super::worker`]) when a job's kind is
//! [`crate::jobs::JobKind::Reprocess`]. This is the correction engine with
//! one shape and two triggers: feedback (`reason = rejected`, carrying the
//! user's correction text) and episodic edits (`reason = stale`). It retires
//! the semantic rows derived from the source, then re-runs extraction so
//! fresh, corrected rows replace them.
//!
//! ## Why re-derive, not just retire
//!
//! Retiring the wrong row (epic 0011 ticket 0009) erases it but never writes
//! the corrected fact. Semantic memory is the recall workhorse; a retire-only
//! correction leaves a hole where a usable fact should be. So the engine
//! retires *and* re-extracts, preserving the invariant "semantic = always
//! derived, never hand-written."
//!
//! ## Neighborhood
//!
//! A correction rarely concerns exactly one source. The handler embeds the
//! feedback text and gathers the near-vector *episodic* neighborhood within
//! the same scope (a similarity floor plus a top-k cap), reprocessing each.
//! The named source is always included even if it falls below the floor — it
//! is the row the user pointed at.
//!
//! ## Resilience
//!
//! Mirrors extract: a source forgotten between enqueue and claim is a no-op
//! success, and a missing extraction LLM skips rather than fails. A failed
//! vector eviction is logged at WARN and does not roll back the retirement —
//! reconciliation cleans the orphan.

use std::sync::Arc;

use tracing::{Instrument, Level, event, info_span};

use crate::embedding::EmbeddingModel;
use crate::jobs::{Job, JobKind, JobsError, MemoryJobsStore};
use crate::memory::{KindSelector, Memory, RetirementReason, Scope};
use crate::store::{MemoryStore, NewMemory, StoreError};
use crate::vector::VectorIndex;

use super::extract::ExtractError;
use super::ClientInner;

/// Similarity floor for the reprocess neighborhood search.
///
/// Episodic rows scoring below this against the embedded feedback are not
/// reprocessed. Cosine similarity in `[-1, 1]`; mirrors the categorize
/// threshold. Overridable via the job payload's `min_similarity`.
const DEFAULT_NEIGHBORHOOD_FLOOR: f32 = 0.5;

/// Top-k cap on the reprocess neighborhood.
///
/// A blast-radius bound: at most this many episodic sources are reprocessed
/// per job, even if more clear the floor. Overridable via the job payload's
/// `top_k`.
const DEFAULT_NEIGHBORHOOD_TOP_K: usize = 10;

/// Failure modes for the reprocess worker stage.
///
/// Only these reach the worker as failures that flip the job to `failed`.
/// Source-not-found, an empty neighborhood, and a missing LLM are handled as
/// successes inside the handler.
#[derive(Debug, thiserror::Error)]
pub(super) enum ReprocessError {
    /// The job payload was missing a parseable retirement reason.
    #[error("invalid reprocess payload: {0}")]
    Payload(String),

    /// A store read or write failed.
    #[error("store failed: {0}")]
    Store(#[from] StoreError),

    /// Embedding the feedback text failed.
    #[error("embedding failed: {0}")]
    Embedding(String),

    /// The neighborhood search failed.
    #[error("neighborhood search failed: {0}")]
    Search(String),

    /// Enqueuing a follow-on job failed.
    #[error("enqueue failed: {0}")]
    Enqueue(String),

    /// Re-extraction failed.
    #[error("re-extraction failed: {0}")]
    Extract(#[from] ExtractError),
}

/// The correction text + reason parsed out of a reprocess job payload.
struct ReprocessRequest {
    reason: RetirementReason,
    feedback: Option<String>,
    min_similarity: f32,
    top_k: usize,
}

impl ReprocessRequest {
    /// Parses a reprocess job payload into its typed request.
    ///
    /// The payload shape is `{ "reason": "rejected"|"stale", "feedback"?: str,
    /// "min_similarity"?: f32, "top_k"?: usize }`. The reason is required; the
    /// rest fall back to defaults.
    fn from_payload(payload: &serde_json::Value) -> Result<Self, ReprocessError> {
        let reason = payload
            .get("reason")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ReprocessError::Payload("missing 'reason'".to_string()))?
            .parse::<RetirementReason>()
            .map_err(|err| ReprocessError::Payload(format!("bad 'reason': {err}")))?;

        let feedback = payload
            .get("feedback")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .filter(|s| !s.trim().is_empty());

        let min_similarity = payload
            .get("min_similarity")
            .and_then(serde_json::Value::as_f64)
            .map_or(DEFAULT_NEIGHBORHOOD_FLOOR, |v| v as f32);

        let top_k = payload
            .get("top_k")
            .and_then(serde_json::Value::as_u64)
            .map_or(DEFAULT_NEIGHBORHOOD_TOP_K, |v| v as usize);

        Ok(Self {
            reason,
            feedback,
            min_similarity,
            top_k,
        })
    }
}

impl ClientInner {
    /// Runs the reprocess pipeline for one claimed reprocess job.
    ///
    /// Returns `Ok(())` on success, including the no-op cases (source missing,
    /// no extraction LLM). Returns `Err` only for real failures that should
    /// flip the job to `failed`.
    ///
    /// # Errors
    ///
    /// Returns [`ReprocessError`] for a malformed payload or for store,
    /// embedding, search, enqueue, or re-extraction failures.
    pub(super) async fn run_reprocess(self: &Arc<Self>, job: Job) -> Result<(), ReprocessError> {
        let span = info_span!("memoir.reprocess", source_pid = %job.source_pid);
        async move { self.run_reprocess_inner(job).await }.instrument(span).await
    }

    async fn run_reprocess_inner(self: &Arc<Self>, job: Job) -> Result<(), ReprocessError> {
        let source_pid = job.source_pid.clone();
        let request = ReprocessRequest::from_payload(&job.payload)?;

        event!(
            name: "memoir.reprocess.started",
            Level::INFO,
            source_pid = %source_pid,
            reason = %request.reason,
            has_feedback = request.feedback.is_some(),
            "reprocess started for {{source_pid}} ({{reason}})",
        );

        // Load the named source. NotFound is the cascade-delete race (forgotten
        // between enqueue and claim) — a no-op success, mirroring extract.
        let source = match self.store.recall(&source_pid).await {
            Ok(memory) => memory,
            Err(StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.reprocess.source_missing",
                    Level::INFO,
                    source_pid = %source_pid,
                    "source memory absent for {{source_pid}} (cascade delete race); skipping",
                );
                return Ok(());
            }
            Err(err) => return Err(ReprocessError::Store(err)),
        };

        // Persist the feedback itself as an episodic row tagged `origin:
        // feedback` — it was said, so it belongs in the verbatim record, and it
        // anchors the corrected derivation's provenance. Done before the
        // neighborhood search so the feedback is part of the scope's vectors.
        if let Some(feedback) = request.feedback.as_deref() {
            self.persist_feedback_row(&source.scope, feedback).await?;
        }

        // Gather the neighborhood: the named source, plus near-vector episodic
        // rows in the same scope. The named source is always reprocessed even
        // if its own similarity falls below the floor.
        let neighborhood = self
            .reprocess_neighborhood(&source, request.feedback.as_deref(), request.min_similarity, request.top_k)
            .await?;

        event!(
            name: "memoir.reprocess.neighborhood",
            Level::INFO,
            source_pid = %source_pid,
            neighborhood_size = neighborhood.len(),
            "reprocessing {{neighborhood_size}} source(s)",
        );

        // For each source: retire its active derived semantics with the reason,
        // then re-extract fresh rows with the correction woven in.
        for member in &neighborhood {
            let derived = self.store.active_semantics_for_source(&member.pid).await?;
            for row in &derived {
                self.retire_and_evict_internal(&row.pid, request.reason).await;
            }
            self.re_extract_source(member, request.feedback.as_deref()).await?;
        }

        event!(
            name: "memoir.reprocess.done",
            Level::INFO,
            source_pid = %source_pid,
            "reprocess complete for {{source_pid}}",
        );

        Ok(())
    }

    /// Writes the user's correction as an `origin: feedback` episodic row.
    ///
    /// Enqueues an embed job so the row joins the scope's vector index. The
    /// row is episodic (it was said) and pinned to maximum confidence.
    async fn persist_feedback_row(self: &Arc<Self>, scope: &Scope, feedback: &str) -> Result<(), ReprocessError> {
        let written = self
            .store
            .remember(NewMemory {
                scope: scope.clone(),
                content: feedback.to_string(),
                metadata: serde_json::json!({ "origin": "feedback" }),
                kind: crate::memory::MemoryKind::Episodic,
                source_pid: None,
                event_at: None,
                confidence: crate::memory::Confidence::MAX,
            })
            .await?;

        self.jobs
            .enqueue(JobKind::Embed, written.pid, serde_json::json!({ "origin": "feedback" }))
            .await
            .map_err(|err: JobsError| ReprocessError::Enqueue(err.to_string()))?;

        Ok(())
    }

    /// Returns the episodic sources to reprocess: the named source + neighbors.
    ///
    /// Embeds the feedback (falling back to the source content when no feedback
    /// is supplied), runs an episodic-only similarity search in scope bounded
    /// by `min_similarity` and `top_k`, and guarantees the named source is in
    /// the result. Deduplicates by pid.
    async fn reprocess_neighborhood(
        self: &Arc<Self>,
        source: &Memory,
        feedback: Option<&str>,
        min_similarity: f32,
        top_k: usize,
    ) -> Result<Vec<Memory>, ReprocessError> {
        let query = feedback.unwrap_or(&source.content);
        let query_vector = self
            .embedder
            .embed(query)
            .await
            .map_err(|err| ReprocessError::Embedding(err.to_string()))?;

        let hits = self
            .index
            .search(
                source.scope.clone(),
                query_vector,
                top_k,
                KindSelector {
                    episodic: true,
                    semantic: false,
                },
                None,
                Some(min_similarity),
            )
            .await
            .map_err(|err| ReprocessError::Search(err.to_string()))?;

        let mut pids: Vec<String> = hits.into_iter().map(|(pid, _)| pid).collect();
        if !pids.iter().any(|pid| pid == &source.pid) {
            pids.push(source.pid.clone());
        }

        let pid_refs: Vec<&str> = pids.iter().map(String::as_str).collect();
        let rows = self.store.find_by_pids(&pid_refs).await?;
        Ok(rows)
    }

    /// Retires a memory and evicts its vector, logging eviction failures.
    ///
    /// The shared store-mark-then-evict primitive behind
    /// [`crate::client::Client::reject`] / `mark_stale` and the reprocess
    /// engine. Mirrors [`crate::client::Client::forget`]'s ordering: Postgres
    /// is the source of truth, and a transient Qdrant failure leaves a
    /// searchable orphan for reconciliation rather than rolling back.
    ///
    /// # Errors
    ///
    /// Returns [`StoreError::NotFound`] when no memory matches `pid`,
    /// [`StoreError::Database`] for database failures. A vector-eviction
    /// failure is logged at WARN and does not fail the call.
    pub(crate) async fn retire_and_evict(&self, pid: &str, reason: RetirementReason) -> Result<(), super::ClientError> {
        self.store.retire(pid, reason).await?;
        self.evict_after_retire(pid, reason).await;
        Ok(())
    }

    /// Reprocess-internal retire: surfaces store errors via [`ReprocessError`].
    ///
    /// Identical mark-then-evict behavior to [`Self::retire_and_evict`] but
    /// folded into the reprocess flow; logs and swallows a store failure on a
    /// single row so one bad row does not abort the whole neighborhood.
    async fn retire_and_evict_internal(&self, pid: &str, reason: RetirementReason) {
        if let Err(err) = self.store.retire(pid, reason).await {
            event!(
                name: "memoir.reprocess.retire_failed",
                Level::WARN,
                pid = %pid,
                reason = %reason,
                error.message = %err,
                "retire failed for {{pid}} during reprocess: {{error.message}}; continuing",
            );
            return;
        }
        self.evict_after_retire(pid, reason).await;
    }

    /// Evicts a retired row's vector, logging failures at WARN.
    async fn evict_after_retire(&self, pid: &str, reason: RetirementReason) {
        if let Err(err) = self.index.delete_by_pids(&[pid]).await {
            event!(
                name: "memoir.retire.index_delete_failed",
                Level::WARN,
                pid = %pid,
                reason = %reason,
                error.message = %err,
                "vector evict failed for {{pid}} ({{reason}}): {{error.message}} — reconciliation will clean up the orphan",
            );
        } else {
            event!(
                name: "memoir.retire.success",
                Level::INFO,
                pid = %pid,
                reason = %reason,
                "retired {{pid}} as {{reason}}",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_reason_from_payload() {
        let payload = serde_json::json!({ "reason": "rejected" });
        let req = ReprocessRequest::from_payload(&payload).unwrap();
        assert_eq!(req.reason, RetirementReason::Rejected);
        assert!(req.feedback.is_none());
    }

    #[test]
    fn should_default_floor_and_top_k_when_payload_omits_them() {
        let payload = serde_json::json!({ "reason": "stale" });
        let req = ReprocessRequest::from_payload(&payload).unwrap();
        assert_eq!(req.min_similarity, DEFAULT_NEIGHBORHOOD_FLOOR);
        assert_eq!(req.top_k, DEFAULT_NEIGHBORHOOD_TOP_K);
    }

    #[test]
    fn should_carry_feedback_and_overrides_from_payload() {
        let payload = serde_json::json!({
            "reason": "rejected",
            "feedback": "they actually love green",
            "min_similarity": 0.7,
            "top_k": 3,
        });
        let req = ReprocessRequest::from_payload(&payload).unwrap();
        assert_eq!(req.feedback.as_deref(), Some("they actually love green"));
        assert_eq!(req.min_similarity, 0.7);
        assert_eq!(req.top_k, 3);
    }

    #[test]
    fn should_treat_blank_feedback_as_none() {
        let payload = serde_json::json!({ "reason": "stale", "feedback": "   " });
        let req = ReprocessRequest::from_payload(&payload).unwrap();
        assert!(req.feedback.is_none());
    }

    #[test]
    fn should_error_when_reason_missing() {
        let payload = serde_json::json!({ "feedback": "x" });
        assert!(ReprocessRequest::from_payload(&payload).is_err());
    }

    #[test]
    fn should_error_when_reason_unparseable() {
        let payload = serde_json::json!({ "reason": "nonsense" });
        assert!(ReprocessRequest::from_payload(&payload).is_err());
    }
}

// Rust guideline compliant 2026-02-21
