//! Accuracy benchmark for [`EntityResolver`] implementations.
//!
//! Scores the merge/no-merge decisions of an entity resolver against a gold
//! corpus: each case seeds a catalog of existing entities, resolves one query
//! string, and checks whether the resolver merged into the right node or
//! correctly created a new one.
//!
//! Unlike the extractor benches, this one needs **no LLM provider** — it runs
//! offline against the real production embedder (fastembed `OnnxEmbedding`,
//! downloaded once on first use). It therefore runs on a plain `cargo bench`,
//! and compares the two shipped resolvers head to head:
//!
//! - [`ExactStringResolver`] — the floor (exact canonical-name match only).
//! - [`EmbeddingEntityResolver`] — production (cosine over name embeddings).
//!
//! The metric is decision accuracy, split into the categories that matter:
//! correct merge / correct split, and the three error modes (false merge — the
//! dangerous one, merging entities that should stay distinct; false split —
//! missing a merge; wrong merge — merging into the wrong node).

use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;

use memoir_core::embedding::{EmbeddingModel, OnnxEmbedding};
use memoir_core::graph::{
    EmbeddingEntityResolver, EntityResolver, EntityVector, ExactStringResolver, InMemoryEntityCatalog, Resolution,
};
use memoir_core::memory::Scope;
use serde::Deserialize;

/// Where the resolver accuracy table is committed, relative to the crate root.
const RESULTS_DIR: &str = "benches/results";

/// The committed resolver corpus (catalog state + query + expected outcome).
const RESOLVER_CORPUS: &str = include_str!("corpus/resolver.json");

/// One resolution case: a seeded catalog, a query, and the expected outcome.
#[derive(Debug, Deserialize)]
struct Case {
    id: String,
    catalog: Vec<CatalogEntity>,
    query: String,
    expected: Expected,
}

/// An entity already in the graph for a case — name is embedded at bench time.
#[derive(Debug, Deserialize)]
struct CatalogEntity {
    key: String,
    name: String,
}

/// The gold outcome: merge into `key`, or create a new node.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum Expected {
    /// The query should merge into the existing node identified by `key`.
    Existing { key: String },
    /// The query should not merge with any catalog entity.
    New,
}

/// The corpus file shape.
#[derive(Debug, Deserialize)]
struct Corpus {
    cases: Vec<Case>,
}

/// How one resolution outcome compares to the expected one.
///
/// `FalseMerge` is the dangerous error (two distinct entities collapsed);
/// `FalseSplit` is the recoverable one (a duplicate); `WrongMerge` is merging
/// into a real node that is the wrong one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    CorrectMerge,
    CorrectSplit,
    FalseMerge,
    FalseSplit,
    WrongMerge,
}

impl Verdict {
    /// Whether the resolution matched the gold outcome.
    fn is_correct(self) -> bool {
        matches!(self, Self::CorrectMerge | Self::CorrectSplit)
    }
}

/// Classifies a resolution against its expected outcome.
fn judge(resolution: &Resolution, expected: &Expected) -> Verdict {
    match (resolution, expected) {
        (Resolution::Existing { key, .. }, Expected::Existing { key: want }) => {
            if key == want {
                Verdict::CorrectMerge
            } else {
                Verdict::WrongMerge
            }
        }
        (Resolution::New { .. }, Expected::New) => Verdict::CorrectSplit,
        (Resolution::Existing { .. }, Expected::New) => Verdict::FalseMerge,
        (Resolution::New { .. }, Expected::Existing { .. }) => Verdict::FalseSplit,
    }
}

/// Running counts of each verdict for one resolver.
#[derive(Debug, Default)]
struct Tally {
    correct_merge: usize,
    correct_split: usize,
    false_merge: usize,
    false_split: usize,
    wrong_merge: usize,
}

impl Tally {
    /// Folds one verdict into the tally.
    fn add(&mut self, verdict: Verdict) {
        match verdict {
            Verdict::CorrectMerge => self.correct_merge += 1,
            Verdict::CorrectSplit => self.correct_split += 1,
            Verdict::FalseMerge => self.false_merge += 1,
            Verdict::FalseSplit => self.false_split += 1,
            Verdict::WrongMerge => self.wrong_merge += 1,
        }
    }

    /// Total cases tallied.
    fn total(&self) -> usize {
        self.correct_merge + self.correct_split + self.false_merge + self.false_split + self.wrong_merge
    }

    /// Share of cases resolved correctly.
    fn accuracy(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            1.0
        } else {
            (self.correct_merge + self.correct_split) as f64 / total as f64
        }
    }
}

/// The one scope every case shares.
fn scope() -> Scope {
    Scope {
        agent_id: "bench".to_string(),
        org_id: "bench".to_string(),
        user_id: "bench".to_string(),
    }
}

/// Builds a fresh catalog for a case, embedding each entity name with `embedder`.
async fn catalog_for(case: &Case, embedder: &Arc<OnnxEmbedding>) -> InMemoryEntityCatalog {
    let catalog = InMemoryEntityCatalog::new();
    for entity in &case.catalog {
        let embedding = embedder
            .embed(&entity.name)
            .await
            .unwrap_or_else(|err| panic!("embedding catalog entity {} failed: {err}", entity.name));
        catalog.insert(
            &scope(),
            EntityVector {
                key: entity.key.clone(),
                name: entity.name.clone(),
                embedding,
            },
        );
    }
    catalog
}

fn main() {
    let corpus: Corpus = serde_json::from_str(RESOLVER_CORPUS).expect("resolver corpus is valid JSON");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building the bench tokio runtime");

    let embedder = Arc::new(OnnxEmbedding::new().expect("initializing the fastembed model (downloads ~50MB on first use)"));

    let exact = score(&runtime, &corpus, &embedder, "exact-string", |_, catalog| {
        Resolver::Exact(ExactStringResolver::new(catalog))
    });
    let embedding = score(&runtime, &corpus, &embedder, "embedding", |embedder, catalog| {
        Resolver::Embedding(EmbeddingEntityResolver::new(embedder, catalog))
    });

    let table = render_table(&[("exact-string", exact), ("embedding", embedding)]);
    print!("{table}");
    write_results(&table);
}

/// The two resolvers under test, unified so the scoring loop drives both.
///
/// `ExactStringResolver` and `EmbeddingEntityResolver` have different generic
/// parameters, so a small enum is the simplest way to run both through one loop
/// without boxing. The embedding variant holds an `Arc<OnnxEmbedding>`, which
/// implements [`EmbeddingModel`] via the blanket `Arc` impl.
enum Resolver {
    Exact(ExactStringResolver<InMemoryEntityCatalog>),
    Embedding(EmbeddingEntityResolver<Arc<OnnxEmbedding>, InMemoryEntityCatalog>),
}

impl Resolver {
    async fn resolve(&self, scope: &Scope, query: &str) -> Resolution {
        match self {
            Self::Exact(resolver) => resolver.resolve(scope, query).await,
            Self::Embedding(resolver) => resolver.resolve(scope, query).await,
        }
        .unwrap_or_else(|err| panic!("resolution failed: {err}"))
    }
}

/// Runs one resolver over the whole corpus and tallies the verdicts.
///
/// `build` receives a clone of the shared embedder (for the embedding resolver;
/// the exact resolver ignores it) plus the per-case catalog.
fn score(
    runtime: &tokio::runtime::Runtime,
    corpus: &Corpus,
    embedder: &Arc<OnnxEmbedding>,
    label: &str,
    build: impl Fn(Arc<OnnxEmbedding>, InMemoryEntityCatalog) -> Resolver,
) -> Tally {
    let mut tally = Tally::default();
    for case in &corpus.cases {
        let (verdict, nearest) = runtime.block_on(async {
            let catalog = catalog_for(case, embedder).await;
            let nearest = nearest_by_cosine(case, embedder).await;
            let resolver = build(embedder.clone(), catalog);
            let resolution = resolver.resolve(&scope(), &case.query).await;
            (judge(&resolution, &case.expected), nearest)
        });
        let marker = if verdict.is_correct() { "ok" } else { "MISS" };
        match nearest {
            Some((name, cosine)) => {
                println!("  [{label}] {} {marker} {verdict:?} (nearest: {name} @ cosine {cosine:.3})", case.id)
            }
            None => println!("  [{label}] {} {marker} {verdict:?}", case.id),
        }
        tally.add(verdict);
    }
    tally
}

/// Finds the catalog entity nearest to the case's query by embedding cosine.
///
/// The diagnostic that turns a failed verdict into a threshold decision: a
/// `FalseSplit` at cosine 0.81 says "lower [`MIN_ENTITY_SIMILARITY`] below
/// 0.81," whereas the verdict alone only says "wrong."
///
/// [`MIN_ENTITY_SIMILARITY`]: memoir_core::graph::MIN_ENTITY_SIMILARITY
async fn nearest_by_cosine(case: &Case, embedder: &Arc<OnnxEmbedding>) -> Option<(String, f32)> {
    let query = embedder
        .embed(&case.query)
        .await
        .unwrap_or_else(|err| panic!("embedding query {} failed: {err}", case.query));

    let mut best: Option<(String, f32)> = None;
    for entity in &case.catalog {
        let embedding = embedder
            .embed(&entity.name)
            .await
            .unwrap_or_else(|err| panic!("embedding catalog entity {} failed: {err}", entity.name));
        let cosine = cosine(&query, &embedding);
        if best.as_ref().is_none_or(|(_, best_cosine)| cosine > *best_cosine) {
            best = Some((entity.name.clone(), cosine));
        }
    }
    best
}

/// Cosine similarity between two equal-length vectors.
fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

/// Renders the per-resolver verdict breakdown as a markdown table.
fn render_table(results: &[(&str, Tally)]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# resolver_accuracy");
    let _ = writeln!(out);
    let _ = writeln!(out, "Entity-resolution decisions vs gold; real fastembed embedder.");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| resolver | accuracy | correct_merge | correct_split | false_merge | false_split | wrong_merge |"
    );
    let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (label, tally) in results {
        let _ = writeln!(
            out,
            "| {} | {:.3} | {} | {} | {} | {} | {} |",
            label,
            tally.accuracy(),
            tally.correct_merge,
            tally.correct_split,
            tally.false_merge,
            tally.false_split,
            tally.wrong_merge
        );
    }
    out
}

/// Writes the rendered table to the committed results file.
fn write_results(table: &str) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(manifest_dir).join(RESULTS_DIR);
    std::fs::create_dir_all(&dir).expect("creating the benches/results directory");
    let path = dir.join("resolver_accuracy.md");
    std::fs::write(&path, table).unwrap_or_else(|err| panic!("writing results to {}: {err}", path.display()));
    println!("wrote {}", path.display());
}
