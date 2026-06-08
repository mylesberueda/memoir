//! Accuracy benchmark for [`EdgeResolver`] implementations.
//!
//! Scores the supersede/coexist decisions of an edge resolver against a gold
//! corpus: each case seeds the graph's current edges, applies a cardinality
//! policy, resolves one new edge, and checks whether the resolver closed exactly
//! the edges it should.
//!
//! Edge resolution is **deterministic pure logic** — no model, no provider — so
//! this runs on a plain `cargo bench` and its "accuracy" is the correctness of
//! the temporal/cardinality rule, scored against gold the same way the other
//! seams are. It compares the floor [`NaiveAppendResolver`] (closes nothing) to
//! the production [`TemporalEdgeResolver`]: the gap is how often append-only
//! would leave a stale, contradicting edge live.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::future::Future;
use std::path::Path;
use std::sync::Mutex;

use chrono::{DateTime, FixedOffset};
use memoir_core::graph::{
    CardinalityPolicy, Edge, EdgeCatalog, EdgeError, EdgeResolver, EdgeResolution, ExistingEdge, NaiveAppendResolver,
    TemporalEdgeResolver,
};
use memoir_core::memory::Scope;
use serde::Deserialize;

/// Where the edge accuracy table is committed, relative to the crate root.
const RESULTS_DIR: &str = "benches/results";

/// The committed edge corpus (current edges + policy + new edge + gold close-set).
const EDGE_CORPUS: &str = include_str!("corpus/edge.json");

/// One edge case: seeded graph state, the new edge, and which edges should close.
#[derive(Debug, Deserialize)]
struct Case {
    id: String,
    single_valued: Vec<String>,
    existing: Vec<ExistingSpec>,
    new: NewSpec,
    expected_close: Vec<String>,
}

/// An edge already current in the graph (all are current — `valid_to` is `None`).
#[derive(Debug, Deserialize)]
struct ExistingSpec {
    key: String,
    subject: String,
    relation: String,
    object: String,
}

/// The new edge to resolve; `day` is the day-of-month for its `valid_from`.
#[derive(Debug, Deserialize)]
struct NewSpec {
    subject: String,
    relation: String,
    object: String,
    day: u32,
}

/// The corpus file shape.
#[derive(Debug, Deserialize)]
struct Corpus {
    cases: Vec<Case>,
}

/// Running counts for one resolver: correct vs incorrect close-decisions.
#[derive(Debug, Default)]
struct Tally {
    correct: usize,
    incorrect: usize,
}

impl Tally {
    /// Folds one case outcome into the tally.
    fn add(&mut self, correct: bool) {
        if correct {
            self.correct += 1;
        } else {
            self.incorrect += 1;
        }
    }

    /// Share of cases whose close-set exactly matched gold.
    fn accuracy(&self) -> f64 {
        let total = self.correct + self.incorrect;
        if total == 0 {
            1.0
        } else {
            self.correct as f64 / total as f64
        }
    }
}

/// In-memory [`EdgeCatalog`] for the bench, yielding only current edges.
///
/// The production `EdgeCatalog` reads FalkorDB; this bench-local one mirrors the
/// in-graph contract (current edges in scope matching subject + relation) so the
/// resolver logic runs against real candidates with no live backend.
#[derive(Default)]
struct BenchEdgeCatalog {
    edges: Mutex<Vec<ExistingEdge>>,
}

impl BenchEdgeCatalog {
    /// Builds a catalog holding `edges` as the current graph state.
    fn with(edges: Vec<ExistingEdge>) -> Self {
        Self {
            edges: Mutex::new(edges),
        }
    }
}

impl EdgeCatalog for BenchEdgeCatalog {
    fn current_edges(
        &self,
        _scope: &Scope,
        subject_key: &str,
        relation: &str,
    ) -> impl Future<Output = Result<Vec<ExistingEdge>, EdgeError>> + Send {
        let matched = self
            .edges
            .lock()
            .expect("edge catalog mutex poisoned")
            .iter()
            .filter(|existing| {
                existing.valid_to.is_none() && existing.subject_key == subject_key && existing.relation == relation
            })
            .cloned()
            .collect();
        async move { Ok(matched) }
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

/// Builds a `valid_from` timestamp from a day-of-month in a fixed month.
fn at(day: u32) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(&format!("2026-06-{day:02}T00:00:00Z")).expect("valid corpus date")
}

/// Maps a case's existing-edge specs to current `ExistingEdge`s.
fn existing_edges(case: &Case) -> Vec<ExistingEdge> {
    case.existing
        .iter()
        .map(|spec| ExistingEdge {
            key: spec.key.clone(),
            subject_key: spec.subject.clone(),
            relation: spec.relation.clone(),
            object_key: spec.object.clone(),
            valid_to: None,
        })
        .collect()
}

/// Maps a case's new-edge spec to an [`Edge`].
fn new_edge(case: &Case) -> Edge {
    Edge {
        subject_key: case.new.subject.clone(),
        relation: case.new.relation.clone(),
        object_key: case.new.object.clone(),
        confidence: 1.0,
        valid_from: at(case.new.day),
    }
}

/// Whether a resolution's close-set equals the gold close-set (order-independent).
fn close_matches(resolution: &EdgeResolution, expected: &[String]) -> bool {
    let got: HashSet<&String> = resolution.close.iter().collect();
    let want: HashSet<&String> = expected.iter().collect();
    got == want
}

fn main() {
    let corpus: Corpus = serde_json::from_str(EDGE_CORPUS).expect("edge corpus is valid JSON");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building the bench tokio runtime");

    let mut naive = Tally::default();
    let mut temporal = Tally::default();

    for case in &corpus.cases {
        let expected = &case.expected_close;

        let naive_resolution = runtime.block_on(NaiveAppendResolver::new().resolve(&scope(), new_edge(case)));
        let naive_ok = close_matches(&naive_resolution.expect("naive resolve never errors"), expected);
        if !naive_ok {
            println!("  [naive] {} → close mismatch", case.id);
        }
        naive.add(naive_ok);

        let catalog = BenchEdgeCatalog::with(existing_edges(case));
        let policy = CardinalityPolicy::with_single_valued(case.single_valued.iter());
        let resolver = TemporalEdgeResolver::new(catalog, policy);
        let temporal_resolution = runtime.block_on(resolver.resolve(&scope(), new_edge(case)));
        let temporal_ok = close_matches(&temporal_resolution.expect("temporal resolve over in-memory catalog"), expected);
        if !temporal_ok {
            println!("  [temporal] {} → close mismatch", case.id);
        }
        temporal.add(temporal_ok);
    }

    let table = render_table(&[("naive-append", naive), ("temporal", temporal)]);
    print!("{table}");
    write_results(&table);
}

/// Renders the per-resolver close-decision accuracy as a markdown table.
fn render_table(results: &[(&str, Tally)]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# edge_accuracy");
    let _ = writeln!(out);
    let _ = writeln!(out, "Edge supersede/coexist decisions vs gold; deterministic, no model.");
    let _ = writeln!(out);
    let _ = writeln!(out, "| resolver | accuracy | correct | incorrect |");
    let _ = writeln!(out, "| --- | ---: | ---: | ---: |");
    for (label, tally) in results {
        let _ = writeln!(
            out,
            "| {} | {:.3} | {} | {} |",
            label,
            tally.accuracy(),
            tally.correct,
            tally.incorrect
        );
    }
    out
}

/// Writes the rendered table to the committed results file.
fn write_results(table: &str) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(manifest_dir).join(RESULTS_DIR);
    std::fs::create_dir_all(&dir).expect("creating the benches/results directory");
    let path = dir.join("edge_accuracy.md");
    std::fs::write(&path, table).unwrap_or_else(|err| panic!("writing results to {}: {err}", path.display()));
    println!("wrote {}", path.display());
}
