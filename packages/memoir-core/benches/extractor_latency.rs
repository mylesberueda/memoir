//! Latency benchmark for [`TripleExtractor`] implementations.
//!
//! The *timing* half of ticket 0012-0011 (criterion), distinct from the accuracy
//! bench: it measures how long one `extract` call takes against the configured
//! provider. It says nothing about correctness — averaging correctness across
//! criterion's repeated runs would be meaningless, which is why accuracy lives in
//! a separate bench.
//!
//! ## Running
//!
//! Same provider gating as `extractor_accuracy`: a plain `cargo bench` with no
//! `BENCH_PROVIDER` set prints a skip notice and exits 0 (no API call, no
//! failure). Configure a provider to time it:
//!
//! - `BENCH_PROVIDER=ollama` (free + local), `openai`, or `anthropic`.
//! - Optional: `BENCH_MODEL`, `BENCH_OLLAMA_URL`.
//!
//! ## Cache posture (a ticket 0012-0011 decision)
//!
//! Unlike the accuracy pass, the latency pass does **not** cache-bust by default:
//! it measures warm-deployment steady-state latency (the common production case).
//! Busting the cache on every criterion iteration would both change the question
//! (cold-start instead of steady-state) and multiply token spend against the cost
//! ceiling. Set `BENCH_BUST_CACHE=1` to measure cold-start instead.

mod common;

use std::env;
use std::time::Duration;

use common::{cache_busted, provider_from_env};
use criterion::Criterion;
use memoir_core::graph::{LlmExtractor, TripleExtractor};
use memoir_core::llm::RigLlmProvider;

/// A representative utterance to time extraction against.
///
/// One fixed, mid-complexity sentence (two triples) — latency is a property of
/// the provider/model + prompt, not of corpus breadth, so a single stable input
/// keeps the timing comparable across providers.
const SAMPLE: &str = "Sarah owns the billing service and reports to Tom.";

/// Builds the per-call input, optionally cache-busted (cold-start measurement).
fn input() -> String {
    if env::var("BENCH_BUST_CACHE").is_ok_and(|v| v == "1") {
        cache_busted(SAMPLE)
    } else {
        SAMPLE.to_string()
    }
}

fn main() {
    let Some(config) = provider_from_env() else {
        println!(
            "extractor_latency: no provider configured — skipping (expected for a plain `cargo bench`).\n\
             Set BENCH_PROVIDER=ollama|openai|anthropic to run. See the file header for details."
        );
        return;
    };

    let label = format!("{}-{}", config.kind(), config.model());
    let provider = RigLlmProvider::new(config).expect("building the bench LLM provider from env config");
    let extractor = LlmExtractor::new(provider);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building the bench tokio runtime");

    // Network round-trips dominate, so keep the sample small and the measurement
    // window generous — the defaults assume microsecond-scale work, not LLM calls.
    let mut criterion = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(30))
        .configure_from_args();

    criterion.bench_function(&format!("extract/{label}"), |b| {
        b.iter(|| {
            runtime
                .block_on(extractor.extract(&input()))
                .expect("extraction failed during latency benchmark")
        });
    });

    criterion.final_summary();
}
