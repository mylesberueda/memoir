//! Shared setup for the extractor benchmarks (`extractor_accuracy`,
//! `extractor_latency`).
//!
//! Both benches select the same provider the same way and share the cache-bust
//! marker format, so that logic lives here rather than being copied per bench.

use std::env;

use memoir_core::llm::{
    DEFAULT_ANTHROPIC_MODEL, DEFAULT_OLLAMA_MODEL, DEFAULT_OLLAMA_URL, DEFAULT_OPENAI_MODEL, LlmConfig,
};

/// Builds the provider config the benches score, from environment, or `None`.
///
/// `None` means no `BENCH_PROVIDER` was set — the caller skips (a plain
/// `cargo bench` then neither calls an LLM nor fails). The same corpus runs
/// against any provider by swapping this one env var: the cross-provider
/// comparison rides the `LlmProvider` seam, no code change.
///
/// # Panics
///
/// Panics when `BENCH_PROVIDER` is set to a paid provider without its API
/// key, or to an unrecognized value — a misconfigured run should fail loudly,
/// not silently skip.
pub fn provider_from_env() -> Option<LlmConfig> {
    let kind = env::var("BENCH_PROVIDER").ok()?;
    let model_override = env::var("BENCH_MODEL").ok();
    match kind.to_lowercase().as_str() {
        "ollama" => {
            let url = env::var("BENCH_OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string());
            let model = model_override.unwrap_or_else(|| DEFAULT_OLLAMA_MODEL.to_string());
            Some(LlmConfig::ollama(url, model))
        }
        "openai" => {
            let api_key = env::var("OPENAI_API_KEY").expect("BENCH_PROVIDER=openai requires OPENAI_API_KEY");
            let model = model_override.unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
            Some(LlmConfig::openai(api_key, model))
        }
        "anthropic" => {
            let api_key =
                env::var("ANTHROPIC_API_KEY").expect("BENCH_PROVIDER=anthropic requires ANTHROPIC_API_KEY");
            let model = model_override.unwrap_or_else(|| DEFAULT_ANTHROPIC_MODEL.to_string());
            Some(LlmConfig::anthropic(api_key, model))
        }
        other => panic!("unknown BENCH_PROVIDER={other:?} (expected ollama|openai|anthropic)"),
    }
}

/// Prepends a cache-busting marker so each prompt is a fresh sample.
///
/// Caching replays the prefix *computation*, not the answer, so this is for
/// sample-independence (making extraction temperature variance visible), not
/// anti-inflation. The marker leads the content — placed to break the cacheable
/// prefix — and is labeled so the model treats it as ignorable.
pub fn cache_busted(text: &str) -> String {
    format!("[ignore this id: {}]\n{text}", nanoid::nanoid!())
}
