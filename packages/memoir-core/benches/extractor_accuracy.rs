//! Accuracy benchmark for [`TripleExtractor`] implementations.
//!
//! This is the *accuracy* half of ticket 0012-0011 (the `harness = false` bench,
//! distinct from the criterion *timing* bench): it scores an extractor against a
//! gold corpus and reports precision/recall per provenance slice, with no LLM
//! judge — correctness is normalized exact set membership on `(subject,
//! relation, object)`. It does not measure latency.
//!
//! ## Running
//!
//! A plain `cargo bench` does **not** call any LLM: with no provider configured
//! the run prints how to enable one and exits 0 (so CI and an offline `cargo
//! bench` neither hit a paid API nor fail). Configure a provider via env:
//!
//! - `MEMOIR_BENCH_PROVIDER=ollama` (default URL/model, free + local), or
//! - `MEMOIR_BENCH_PROVIDER=openai` with `OPENAI_API_KEY`, or
//! - `MEMOIR_BENCH_PROVIDER=anthropic` with `ANTHROPIC_API_KEY`.
//!
//! Optional overrides: `MEMOIR_BENCH_MODEL`, `MEMOIR_BENCH_OLLAMA_URL`.
//!
//! The same corpus runs against any provider by swapping `MEMOIR_BENCH_PROVIDER`
//! — the cross-provider comparison rides the `LlmProvider` seam, no code change.

mod common;

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::Path;

use common::{cache_busted, provider_from_env};
use memoir_core::graph::{LlmExtractor, Triple, TripleExtractor};
use memoir_core::llm::RigLlmProvider;
use serde::Deserialize;

/// Where the accuracy table is committed, relative to the crate root.
///
/// Written deterministically (sorted slices, fixed precision, no timestamp) so a
/// re-run against the same provider/model yields a minimal git diff and score
/// drift is reviewable. Keyed by provider+model so swapping providers does not
/// clobber another provider's recorded numbers.
const RESULTS_DIR: &str = "benches/results";

/// The two committed corpus slices: synthesized agent-memory (60%, open-vocab)
/// and CaRB (40%, encyclopedic, MIT-vendored). Loaded via `include_str!` so the
/// bench is hermetic — the gold data needs no runtime fs or network.
const CORPUS_SLICES: [&str; 2] = [include_str!("corpus/synthesized.json"), include_str!("corpus/carb.json")];

/// One labeled corpus item: input text and its gold triples.
#[derive(Debug, Deserialize)]
struct CorpusItem {
    id: String,
    provenance: String,
    text: String,
    gold: Vec<GoldTriple>,
}

/// A gold triple — only the match key, no confidence (confidence is not graded).
#[derive(Debug, Deserialize)]
struct GoldTriple {
    subject: String,
    relation: String,
    object: String,
}

/// The corpus file shape: a comment field and the items.
#[derive(Debug, Deserialize)]
struct Corpus {
    items: Vec<CorpusItem>,
}

/// Precision/recall tallies for one provenance slice.
#[derive(Debug, Default)]
struct Tally {
    /// Predicted triples that matched a gold triple.
    true_positives: usize,
    /// Predicted triples with no gold match (over-extraction).
    false_positives: usize,
    /// Gold triples the extractor missed.
    false_negatives: usize,
}

impl Tally {
    /// `tp / (tp + fp)` — share of predictions that were correct. 1.0 when
    /// nothing was predicted (vacuously precise).
    fn precision(&self) -> f64 {
        let predicted = self.true_positives + self.false_positives;
        if predicted == 0 {
            1.0
        } else {
            self.true_positives as f64 / predicted as f64
        }
    }

    /// `tp / (tp + fn)` — share of gold triples recovered. 1.0 when there were
    /// no gold triples (nothing to miss).
    fn recall(&self) -> f64 {
        let relevant = self.true_positives + self.false_negatives;
        if relevant == 0 {
            1.0
        } else {
            self.true_positives as f64 / relevant as f64
        }
    }

    /// Harmonic mean of precision and recall.
    fn f1(&self) -> f64 {
        let (p, r) = (self.precision(), self.recall());
        if p + r == 0.0 { 0.0 } else { 2.0 * p * r / (p + r) }
    }

    /// Folds one item's matched/predicted/gold counts into the running tally.
    fn add(&mut self, matched: usize, predicted: usize, gold: usize) {
        self.true_positives += matched;
        self.false_positives += predicted - matched;
        self.false_negatives += gold - matched;
    }
}

/// Normalizes a string for matching: lowercase + collapse whitespace + light
/// suffix stemming (`-s`/`-ed`/`-ing`) on the final word.
///
/// A fixed normalizer (ticket 0012-0011 decision: not swappable in v1). It folds
/// the tense/plural variation the extractor's "own words" relation phrasing
/// produces ("works at"/"worked at") so a real match is not scored a miss. A
/// true lemmatizer would be more correct; this avoids a Rust-lemmatizer
/// dependency and is the flagged v1 fallback.
fn normalize(text: &str) -> String {
    let lowered = text.to_lowercase();
    let mut words: Vec<&str> = lowered.split_whitespace().collect();
    if let Some(last) = words.last_mut() {
        *last = stem(last);
    }
    words.join(" ")
}

/// Trims a single trailing inflection from one word.
fn stem(word: &str) -> &str {
    for suffix in ["ing", "ed", "es", "s"] {
        if let Some(base) = word.strip_suffix(suffix) {
            if base.len() >= 3 {
                return base;
            }
        }
    }
    word
}

/// The normalized match key for a triple — what set membership compares.
fn key(subject: &str, relation: &str, object: &str) -> (String, String, String) {
    (normalize(subject), normalize(relation), normalize(object))
}

/// Counts how many predicted triples match a gold triple, by normalized key.
///
/// Each gold key is consumed at most once, so duplicate predictions do not
/// double-count against a single gold triple.
fn count_matches(predicted: &[Triple], gold: &[GoldTriple]) -> usize {
    let mut gold_keys: HashSet<(String, String, String)> =
        gold.iter().map(|g| key(&g.subject, &g.relation, &g.object)).collect();
    let mut matched = 0;
    for triple in predicted {
        let triple_key = key(&triple.subject, &triple.relation, &triple.object);
        if gold_keys.remove(&triple_key) {
            matched += 1;
        }
    }
    matched
}

fn main() {
    let Some(config) = provider_from_env() else {
        println!(
            "extractor_accuracy: no provider configured — skipping (this is expected for a plain `cargo bench`).\n\
             Set MEMOIR_BENCH_PROVIDER=ollama|openai|anthropic to run. See the file header for details."
        );
        return;
    };

    let label = format!("{}-{}", config.kind(), config.model());
    let items: Vec<CorpusItem> = CORPUS_SLICES
        .iter()
        .flat_map(|raw| serde_json::from_str::<Corpus>(raw).expect("corpus slice is valid JSON").items)
        .collect();
    let provider = RigLlmProvider::new(config).expect("building the bench LLM provider from env config");
    let extractor = LlmExtractor::new(provider);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building the bench tokio runtime");

    let mut by_slice: std::collections::BTreeMap<String, Tally> = std::collections::BTreeMap::new();
    let mut overall = Tally::default();

    for item in &items {
        let predicted = runtime
            .block_on(extractor.extract(&cache_busted(&item.text)))
            .unwrap_or_else(|err| panic!("extraction failed for {}: {err}", item.id));

        let matched = count_matches(&predicted, &item.gold);
        let slice = by_slice.entry(item.provenance.clone()).or_default();
        slice.add(matched, predicted.len(), item.gold.len());
        overall.add(matched, predicted.len(), item.gold.len());
    }

    let table = render_table(&label, &by_slice, &overall);
    print!("{table}");
    write_results(&label, &table);
}

/// Writes the rendered table to a per-provider committed results file.
///
/// Filename is the sanitized provider+model label, so each provider keeps its
/// own tracked numbers. A write failure is fatal — a benchmark that silently
/// fails to record its results is worse than one that stops.
fn write_results(label: &str, table: &str) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(manifest_dir).join(RESULTS_DIR);
    std::fs::create_dir_all(&dir).expect("creating the benches/results directory");

    let file_stem: String = label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let path = dir.join(format!("{file_stem}.md"));
    std::fs::write(&path, table).unwrap_or_else(|err| panic!("writing results to {}: {err}", path.display()));
    println!("wrote {}", path.display());
}

/// Renders per-slice and overall precision/recall/F1 as a markdown table.
///
/// Deterministic: slices are iterated in sorted (`BTreeMap`) order at fixed
/// precision with no timestamp, so committing the output produces a minimal diff
/// across runs and score drift is what shows up in review.
fn render_table(label: &str, by_slice: &std::collections::BTreeMap<String, Tally>, overall: &Tally) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# extractor_accuracy — {label}");
    let _ = writeln!(out);
    let _ = writeln!(out, "Normalized exact-match on (subject, relation, object); no LLM judge.");
    let _ = writeln!(out);
    let _ = writeln!(out, "| slice | precision | recall | f1 |");
    let _ = writeln!(out, "| --- | ---: | ---: | ---: |");
    for (slice, tally) in by_slice {
        let _ = writeln!(
            out,
            "| {} | {:.3} | {:.3} | {:.3} |",
            slice,
            tally.precision(),
            tally.recall(),
            tally.f1()
        );
    }
    let _ = writeln!(
        out,
        "| **overall** | {:.3} | {:.3} | {:.3} |",
        overall.precision(),
        overall.recall(),
        overall.f1()
    );
    out
}
