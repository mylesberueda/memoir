//! Worker stage that categorizes a semantic memory via zero-shot NLI.
//!
//! Dispatched by the worker loop ([`super::worker`]) when a job's kind is
//! [`crate::jobs::JobKind::Categorize`]. Loads the target semantic memory,
//! runs the NLI classifier against the [v1 taxonomy](CATEGORY_LABELS), and
//! writes the winning label to the row's `category` column. Confidence is
//! deliberately untouched here — the extraction LLM owns that column
//! (ticket 0006); NLI's softmax measures label-fit, a different quantity.
//!
//! ## Resilience
//!
//! Categorization is best-effort. A row whose source vanished between enqueue
//! and claim, or that scores below [`MIN_CATEGORY_SCORE`], is handled without
//! failing the job — a `NULL`/`transient` category is unfiltered, not
//! rejected. Only real infrastructure failures flip the job to `failed`.

use std::sync::Arc;

use tracing::{Instrument, Level, event, info_span};

use crate::jobs::Job;
use crate::store::{MemoryStore, StoreError};

use super::ClientInner;

/// The v1 category taxonomy: a single functional axis (epic 0011 ticket 0005).
///
/// Functional, not topical: each label means something distinct to the
/// selection layer (ticket 0008) at recall time — durable preferences and
/// identity resist decay, transient observations drop first. See the design
/// memory `project-memoir-epic-0011-taxonomy` for the full rationale.
pub const CATEGORY_LABELS: &[&str] = &["preference", "identity", "workflow", "factual", "transient"];

/// The category a low-confidence classification falls back to.
///
/// When the top NLI score is below [`MIN_CATEGORY_SCORE`], the row is labeled
/// `transient` rather than guessed into a confident bucket: selection treats
/// transient as drop-first, so an unsure classification degrades gracefully.
const FALLBACK_CATEGORY: &str = "transient";

/// Minimum top NLI score to accept a category; below this falls back.
///
/// Mirrors committee-ai's 0.5 threshold. Below it the entailment signal is too
/// weak to trust, so [`FALLBACK_CATEGORY`] applies.
const MIN_CATEGORY_SCORE: f32 = 0.5;

/// The hypothesis template filled per label before NLI entailment scoring.
///
/// `{}` is replaced with each label, e.g. `"This memory is about preference."`.
/// The "is about" framing matches committee-ai's semantic-type pass.
const HYPOTHESIS_TEMPLATE: &str = "This memory is about a {}.";

/// Failure modes for the categorize worker stage.
///
/// Only these reach the worker as failures that flip the job to `failed`.
/// Source-not-found and low-score are handled as successes inside the handler.
#[derive(Debug, thiserror::Error)]
pub(super) enum CategorizeError {
    /// Loading the target memory hit the database.
    #[error("source lookup failed: {0}")]
    SourceLookup(#[from] StoreError),

    /// Writing the category column back failed.
    #[error("persist failed: {0}")]
    Persist(String),

    /// The NLI classifier returned an error.
    #[error("classification failed: {0}")]
    Classify(String),
}

impl ClientInner {
    /// Runs the categorize pipeline for one claimed categorize job.
    ///
    /// Returns `Ok(())` on success, including the no-op cases (source missing,
    /// no classifier configured, score below threshold). Returns `Err` only
    /// for real failures that should flip the job to `failed`.
    pub(super) async fn run_categorize(self: &Arc<Self>, job: Job) -> Result<(), CategorizeError> {
        let span = info_span!("memoir.categorize", source_pid = %job.source_pid);
        async move { self.run_categorize_inner(job).await }
            .instrument(span)
            .await
    }

    async fn run_categorize_inner(self: &Arc<Self>, job: Job) -> Result<(), CategorizeError> {
        let pid = job.source_pid.clone();

        // No classifier configured → no-op success. The enqueue seam only
        // fires when one is present, but defending in depth catches a
        // misconfiguration where stale categorize jobs outlive the config.
        let Some(classifier) = self.nli.clone() else {
            event!(
                name: "memoir.categorize.skipped",
                Level::WARN,
                source_pid = %pid,
                "no NLI classifier configured; treating job as no-op",
            );
            return Ok(());
        };

        // Load the target. NotFound is the cascade-delete race (forgotten
        // between enqueue and claim) — a no-op success, mirroring extract.
        let memory = match self.store.recall(&pid).await {
            Ok(memory) => memory,
            Err(StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.categorize.source_missing",
                    Level::INFO,
                    source_pid = %pid,
                    "target memory absent for {{source_pid}} (cascade delete race); skipping",
                );
                return Ok(());
            }
            Err(err) => return Err(CategorizeError::SourceLookup(err)),
        };

        // NLI inference is sync + CPU-bound; run it off the async worker via
        // spawn_blocking (mirrors how the embedder is driven). The classifier
        // is Send + Sync behind Arc, so the clone moves cleanly into the task.
        let content = memory.content.clone();
        let scored =
            tokio::task::spawn_blocking(move || classifier.classify(&content, CATEGORY_LABELS, HYPOTHESIS_TEMPLATE))
                .await
                .map_err(|join_err| CategorizeError::Classify(format!("classify task panicked: {join_err}")))?
                .map_err(|nli_err| CategorizeError::Classify(nli_err.to_string()))?;

        // `classify` returns labels sorted desc; the head is the winner.
        // Below threshold (or empty), fall back to `transient`.
        let category = match scored.first() {
            Some(top) if top.score >= MIN_CATEGORY_SCORE => top.label.clone(),
            _ => FALLBACK_CATEGORY.to_string(),
        };

        self.store
            .set_category(&pid, &category)
            .await
            .map_err(|err| CategorizeError::Persist(err.to_string()))?;

        event!(
            name: "memoir.categorize.done",
            Level::INFO,
            source_pid = %pid,
            category = %category,
            "categorized {{source_pid}} as {{category}}",
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_five_distinct_category_labels() {
        assert_eq!(CATEGORY_LABELS.len(), 5);
        let unique: std::collections::HashSet<_> = CATEGORY_LABELS.iter().collect();
        assert_eq!(unique.len(), 5, "category labels must be distinct");
    }

    #[test]
    fn should_include_fallback_in_taxonomy() {
        assert!(
            CATEGORY_LABELS.contains(&FALLBACK_CATEGORY),
            "the fallback category must be a valid taxonomy label"
        );
    }

    #[test]
    fn should_fill_hypothesis_template_per_label() {
        assert_eq!(
            HYPOTHESIS_TEMPLATE.replace("{}", "preference"),
            "This memory is about a preference."
        );
    }
}
