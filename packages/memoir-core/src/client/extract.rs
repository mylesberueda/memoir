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
    DEFAULT_EXTRACTION_PROMPT, ExtractionOutput, LlmError, LlmKind, LlmProvider, LlmRole,
    MAX_CONTENT_CHARS, parse_extraction,
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
        async move { self.run_extract_inner(job).await }
            .instrument(span)
            .await
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

        // Step 3: LLM call.
        let content_len = source.content.len();
        let raw = match provider.extract(DEFAULT_EXTRACTION_PROMPT, &source.content).await {
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
        let mut persisted_pids: Vec<String> = Vec::with_capacity(parsed.facts.len());

        for fact in parsed.facts {
            // Skip empty / over-length facts at the boundary. The LLM may
            // emit either; we do not want to write them.
            let content_chars = fact.content.chars().count();
            if fact.content.is_empty() || content_chars > MAX_CONTENT_CHARS {
                continue;
            }

            let metadata = build_semantic_metadata(provider_kind, &provider_model, fact.confidence);

            let written = self
                .store
                .remember(
                    source.scope.clone(),
                    fact.content,
                    metadata,
                    MemoryKind::Semantic,
                    Some(source_pid.clone()),
                )
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

            persisted_pids.push(written.pid);
        }

        event!(
            name: "memoir.extraction.persisted",
            Level::INFO,
            source_pid = %source_pid,
            semantic_count = persisted_pids.len(),
            "extraction persisted {{semantic_count}} semantic row(s) for {{source_pid}}",
        );

        Ok(())
    }
}

/// Builds the JSON metadata stored on each extracted semantic row.
///
/// Includes the provider identifier, model identifier, fact confidence, and
/// a marker that this row was machine-generated. Operators inspecting a
/// semantic row can see which LLM produced it without joining other tables.
fn build_semantic_metadata(
    provider: LlmKind,
    model: &str,
    confidence: f32,
) -> serde_json::Value {
    serde_json::json!({
        "origin": "extraction",
        "provider": provider.as_str(),
        "model": model,
        "confidence": confidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_semantic_metadata_with_expected_shape() {
        let meta = build_semantic_metadata(LlmKind::Ollama, "llama3.2", 0.87);
        assert_eq!(meta["origin"], "extraction");
        assert_eq!(meta["provider"], "ollama");
        assert_eq!(meta["model"], "llama3.2");
        // f32 → f64 → serde_json::Value::Number; compare via approx.
        let confidence = meta["confidence"].as_f64().unwrap();
        assert!((confidence - 0.87).abs() < 1e-5);
    }

    #[test]
    fn should_extract_error_chain_via_from_store_error() {
        let store_err = StoreError::NotFound("pid".to_string());
        let extract_err: ExtractError = store_err.into();
        assert!(matches!(extract_err, ExtractError::SourceLookup(_)));
    }
}
