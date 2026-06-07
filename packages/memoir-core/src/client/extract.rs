//! Worker stage that runs LLM extraction against an episodic memory.
//!
//! Dispatched by the worker loop ([`super::worker`]) when a job's kind is
//! [`crate::jobs::JobKind::Extract`]. Fetches the source memory, calls the
//! configured extraction LLM, parses the reply, persists derived semantic
//! memories, and enqueues follow-on embed jobs for each.
//!
//! ## Trust boundary
//!
//! The LLM call carries user-supplied content into a third-party service and
//! ingests untrusted output back. Tracing fields are deliberately limited to
//! ids, counts, and provider identifiers — never raw content or raw LLM
//! output. The downstream `MemoryStore::remember` path uses parameterized
//! SQL, so a poisoned LLM cannot escape into the database.

use std::sync::Arc;

use tracing::{Instrument, Level, event, info_span};

use crate::jobs::{Job, JobsError, MemoryJobsStore};
use crate::llm::{
    AcceptAllEventAt, DEFAULT_EXTRACTION_PROMPT, EventAtValidator, ExtractionOutput, LlmError, LlmKind, LlmProvider,
    LlmRole, MAX_CONTENT_CHARS, build_extraction_content, parse_extraction,
};
use crate::memory::MemoryKind;
use crate::store::{MemoryStore, StoreError};

use super::ClientInner;

/// Failure modes for the extract worker stage.
///
/// Only the variants in this enum reach the worker as failures that flip the
/// job to `failed`. Source-not-found is handled as a no-op success inside
/// the handler (the source was forgotten between enqueue and claim).
#[derive(Debug, thiserror::Error)]
pub(super) enum ExtractError {
    /// Loading the source memory hit the database.
    #[error("source lookup failed: {0}")]
    SourceLookup(#[from] StoreError),

    /// LLM call failed mid-flight.
    #[error("llm call failed: {0}")]
    LlmCall(LlmError),

    /// LLM returned a reply we could not parse.
    #[error("llm output parse failed: {0}")]
    Parse(LlmError),

    /// Persisting a semantic row or enqueuing a followup job failed.
    #[error("persist failed: {0}")]
    Persist(String),
}

impl ClientInner {
    /// Runs the extract pipeline for one claimed extract job.
    ///
    /// Returns `Ok(())` on success (including the source-not-found no-op);
    /// returns `Err` only for real failures that should flip the job to
    /// `failed`.
    pub(super) async fn run_extract(self: &Arc<Self>, job: Job) -> Result<(), ExtractError> {
        let span = info_span!("memoir.extraction", source_pid = %job.source_pid);
        async move { self.run_extract_inner(job).await }.instrument(span).await
    }

    async fn run_extract_inner(self: &Arc<Self>, job: Job) -> Result<(), ExtractError> {
        let source_pid = job.source_pid.clone();

        event!(
            name: "memoir.extraction.started",
            Level::INFO,
            source_pid = %source_pid,
            "extraction started for {{source_pid}}",
        );

        // Step 1: fetch source. NotFound is the expected cascade-delete race
        // (operator called forget between enqueue and claim). Treat as success.
        let source = match self.store.recall(&source_pid).await {
            Ok(memory) => memory,
            Err(StoreError::NotFound(_)) => {
                event!(
                    name: "memoir.extraction.source_missing",
                    Level::INFO,
                    source_pid = %source_pid,
                    "source memory absent for {{source_pid}} (cascade delete race); skipping",
                );
                return Ok(());
            }
            Err(err) => return Err(ExtractError::SourceLookup(err)),
        };

        // First-pass extraction carries no correction.
        self.re_extract_source(&source, None).await
    }

    /// Re-runs extraction over an already-loaded source, optionally corrected.
    ///
    /// The post-recall half of the extraction pipeline, shared by first-pass
    /// extraction ([`Self::run_extract_inner`], `correction = None`) and the
    /// reprocess engine ([`Self::run_reprocess`], `correction = Some(text)`).
    /// Picks the extraction LLM, runs it over the source content (with the
    /// correction woven into the prompt when present), parses the reply, and
    /// persists one semantic row per fact plus its follow-on embed and
    /// categorize jobs.
    ///
    /// A missing extraction LLM is a no-op success, mirroring the dispatch-time
    /// skip. The caller owns retiring any prior derived rows before calling.
    ///
    /// # Errors
    ///
    /// Returns [`ExtractError::LlmCall`] / [`ExtractError::Parse`] on LLM
    /// failures and [`ExtractError::Persist`] when a row or follow-on job
    /// cannot be written.
    pub(super) async fn re_extract_source(
        self: &Arc<Self>,
        source: &crate::memory::Memory,
        correction: Option<&str>,
    ) -> Result<(), ExtractError> {
        let source_pid = source.pid.clone();

        // Step 2: pick the configured extraction LLM. If none is wired up,
        // skip rather than fail — the worker has already filtered at dispatch
        // time, but defending in depth catches misconfiguration.
        let Some(provider) = self.llms.get(LlmRole::Extraction) else {
            event!(
                name: "memoir.extraction.skipped",
                Level::WARN,
                source_pid = %source_pid,
                "no extraction llm configured; treating job as no-op",
            );
            return Ok(());
        };

        // Step 3: LLM call. Date relative facts ("last Friday") off the source's
        // event-time when it has one, falling back to write-time — so an
        // event_at edit (epic 0011 ticket 0012) actually shifts the derived
        // event-times rather than re-deriving the same dates.
        let content_len = source.content.len();
        let reference = source.event_at.unwrap_or(source.created_at);
        let extraction_content = build_extraction_content(reference, &source.content, correction);
        let raw = match provider.extract(DEFAULT_EXTRACTION_PROMPT, &extraction_content).await {
            Ok(raw) => raw,
            Err(err) => {
                event!(
                    name: "memoir.extraction.llm_failed",
                    Level::WARN,
                    source_pid = %source_pid,
                    provider = %provider.kind(),
                    model = %provider.model(),
                    content_len = content_len,
                    error.message = %err,
                    "extraction llm call failed for {{source_pid}}: {{error.message}}",
                );
                return Err(ExtractError::LlmCall(err));
            }
        };

        // Step 4: parse. Defensive checks: an empty fact list is success.
        let parsed: ExtractionOutput = parse_extraction(&raw).map_err(|err| {
            event!(
                name: "memoir.extraction.parse_failed",
                Level::WARN,
                source_pid = %source_pid,
                provider = %provider.kind(),
                content_len = content_len,
                error.message = %err,
                "extraction parse failed for {{source_pid}}: {{error.message}}",
            );
            ExtractError::Parse(err)
        })?;

        event!(
            name: "memoir.extraction.parsed",
            Level::INFO,
            source_pid = %source_pid,
            fact_count = parsed.facts.len(),
            "extraction yielded {{fact_count}} fact(s) for {{source_pid}}",
        );

        if parsed.facts.is_empty() {
            return Ok(());
        }

        // Step 5: persist semantic rows + enqueue embed jobs.
        let provider_kind = provider.kind();
        let provider_model = provider.model().to_string();
        let validator = AcceptAllEventAt;
        let mut persisted_pids: Vec<String> = Vec::with_capacity(parsed.facts.len());

        for fact in parsed.facts {
            let content_chars = fact.content.chars().count();
            if fact.content.is_empty() || content_chars > MAX_CONTENT_CHARS {
                continue;
            }

            let metadata = build_semantic_metadata(provider_kind, &provider_model);

            let event_at = fact
                .event_at
                .and_then(|candidate| validator.validate(reference, candidate));

            // Confidence is now a first-class column, sourced from the LLM's
            // per-fact score (scaled f32[0,1] → i8[0,100], clamped). It is no
            // longer stuffed into the metadata blob.
            let confidence = crate::memory::Confidence::from_unit_scale(fact.confidence);

            let written = self
                .store
                .remember(crate::store::NewMemory {
                    scope: source.scope.clone(),
                    content: fact.content,
                    metadata,
                    kind: MemoryKind::Semantic,
                    source_pid: Some(source_pid.clone()),
                    event_at,
                    confidence,
                })
                .await
                .map_err(|err| ExtractError::Persist(err.to_string()))?;

            self.jobs
                .enqueue(
                    crate::jobs::JobKind::Embed,
                    written.pid.clone(),
                    serde_json::json!({ "origin": "extraction" }),
                )
                .await
                .map_err(|err: JobsError| ExtractError::Persist(err.to_string()))?;

            // Enqueue categorize only when a classifier is configured —
            // otherwise the job would sit unclaimable (mirrors how remember
            // only enqueues Extract when an extraction LLM is present). The
            // semantic row provably exists at this line, so this is a data
            // dependency satisfied by construction, not a scheduling one.
            if self.nli.is_some() {
                self.jobs
                    .enqueue(
                        crate::jobs::JobKind::Categorize,
                        written.pid.clone(),
                        serde_json::json!({ "origin": "extraction" }),
                    )
                    .await
                    .map_err(|err: JobsError| ExtractError::Persist(err.to_string()))?;
            }

            persisted_pids.push(written.pid);
        }

        event!(
            name: "memoir.extraction.persisted",
            Level::INFO,
            source_pid = %source_pid,
            semantic_count = persisted_pids.len(),
            "extraction persisted {{semantic_count}} semantic row(s) for {{source_pid}}",
        );

        // Semantic extraction is one of the two synthesis parents. On success,
        // try to fire synthesis: it only fires once the relational sibling is
        // also done (the guard is a no-op while it is still pending), and only
        // when a graph is configured (otherwise no relational sibling was ever
        // enqueued, so there is nothing to synthesize).
        #[cfg(feature = "knowledge-graph")]
        if self.graph.is_some() {
            self.jobs
                .enqueue_synthesis_if_ready(&source_pid)
                .await
                .map_err(|err: JobsError| ExtractError::Persist(err.to_string()))?;
        }

        Ok(())
    }
}

/// Builds the JSON metadata stored on each extracted semantic row.
///
/// Includes the provider identifier, model identifier, fact confidence, and
/// a marker that this row was machine-generated. Operators inspecting a
/// semantic row can see which LLM produced it without joining other tables.
fn build_semantic_metadata(provider: LlmKind, model: &str) -> serde_json::Value {
    serde_json::json!({
        "origin": "extraction",
        "provider": provider.as_ref(),
        "model": model,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_semantic_metadata_with_expected_shape() {
        let meta = build_semantic_metadata(LlmKind::Ollama, "llama3.2");
        assert_eq!(meta["origin"], "extraction");
        assert_eq!(meta["provider"], "ollama");
        assert_eq!(meta["model"], "llama3.2");
    }

    #[test]
    fn should_not_record_confidence_in_metadata() {
        // Confidence moved to a first-class column (ticket 0006); the metadata
        // blob must no longer carry it, so the two cannot drift.
        let meta = build_semantic_metadata(LlmKind::Ollama, "llama3.2");
        assert!(meta.get("confidence").is_none());
    }

    #[test]
    fn should_extract_error_chain_via_from_store_error() {
        let store_err = StoreError::NotFound("pid".to_string());
        let extract_err: ExtractError = store_err.into();
        assert!(matches!(extract_err, ExtractError::SourceLookup(_)));
    }
}
