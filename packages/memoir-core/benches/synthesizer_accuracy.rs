//! Accuracy benchmark for [`Synthesizer`] implementations.
//!
//! Scores the keep/veto decisions of a synthesizer against a gold corpus: each
//! case gives candidate triples (some corroborated by the semantic facts, some
//! hallucinated) and the facts, then checks which triples survived.
//!
//! Synthesis is a precision/recall tradeoff on triples — a corroborated triple
//! should survive, a hallucinated one should be vetoed — so the metric names
//! both error modes: false-keep (a hallucination let through, the precision
//! failure synthesis exists to prevent) and false-veto (a real triple killed,
//! the recall cost). It compares the floor [`PassthroughSynthesizer`] (keeps
//! everything) to the production [`EmbeddingSynthesizer`] (cosine-corroboration
//! against the real fastembed embedder).
//!
//! Like the resolver bench it needs **no LLM provider** — it runs offline against
//! the production embedder (`OnnxEmbedding`, ~50MB downloaded once), so it
//! reflects the synthesizer memoir actually ships. Corroboration quality *is*
//! embedding quality, so these numbers are the most likely to surprise: a gold
//! `survive` set that disagrees with the real embeddings is a signal the 0.6
//! corroboration floor is mis-tuned, not necessarily a bad corpus.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;

use memoir_core::embedding::OnnxEmbedding;
use memoir_core::graph::{EmbeddingSynthesizer, PassthroughSynthesizer, SemanticFact, Synthesizer, Triple, TripleSet};
use serde::Deserialize;

/// Where the synthesizer accuracy table is committed, relative to the crate root.
const RESULTS_DIR: &str = "benches/results";

/// The committed synthesizer corpus (triples + facts + gold surviving set).
const SYNTHESIZER_CORPUS: &str = include_str!("corpus/synthesizer.json");

/// One synthesis case: candidate triples, the facts, and which triples survive.
#[derive(Debug, Deserialize)]
struct Case {
    id: String,
    facts: Vec<FactSpec>,
    triples: Vec<TripleSpec>,
    survive: Vec<TripleSpec>,
}

/// A semantic fact's content.
#[derive(Debug, Deserialize)]
struct FactSpec {
    content: String,
}

/// A triple's match key (no confidence — not graded).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
struct TripleSpec {
    subject: String,
    relation: String,
    object: String,
}

/// The corpus file shape.
#[derive(Debug, Deserialize)]
struct Corpus {
    cases: Vec<Case>,
}

/// Running keep/veto verdict counts for one synthesizer.
///
/// `false_keep` is the precision failure (a hallucination survived);
/// `false_veto` is the recall cost (a real triple was killed).
#[derive(Debug, Default)]
struct Tally {
    correct_keep: usize,
    correct_veto: usize,
    false_keep: usize,
    false_veto: usize,
}

impl Tally {
    /// Tallies one triple's outcome: did it survive, and should it have?
    fn add(&mut self, survived: bool, should_survive: bool) {
        match (survived, should_survive) {
            (true, true) => self.correct_keep += 1,
            (false, false) => self.correct_veto += 1,
            (true, false) => self.false_keep += 1,
            (false, true) => self.false_veto += 1,
        }
    }

    /// Share of keep/veto decisions that matched gold.
    fn accuracy(&self) -> f64 {
        let total = self.correct_keep + self.correct_veto + self.false_keep + self.false_veto;
        if total == 0 {
            1.0
        } else {
            (self.correct_keep + self.correct_veto) as f64 / total as f64
        }
    }
}

/// Builds a [`Triple`] from a corpus spec (confidence is irrelevant to scoring).
fn triple_of(spec: &TripleSpec) -> Triple {
    Triple {
        subject: spec.subject.clone(),
        relation: spec.relation.clone(),
        object: spec.object.clone(),
        confidence: 1.0,
    }
}

/// Tallies one synthesizer's output for a case against the gold surviving set.
fn tally_case(output: &TripleSet, case: &Case, tally: &mut Tally, label: &str) {
    let survived: HashSet<TripleSpec> = output
        .iter()
        .map(|triple| TripleSpec {
            subject: triple.subject.clone(),
            relation: triple.relation.clone(),
            object: triple.object.clone(),
        })
        .collect();
    let gold: HashSet<&TripleSpec> = case.survive.iter().collect();

    for spec in &case.triples {
        let did_survive = survived.contains(spec);
        let should_survive = gold.contains(spec);
        if did_survive != should_survive {
            let verb = if did_survive { "false-keep" } else { "false-veto" };
            println!("  [{label}] {} → {verb}: {} {} {}", case.id, spec.subject, spec.relation, spec.object);
        }
        tally.add(did_survive, should_survive);
    }
}

fn main() {
    let corpus: Corpus = serde_json::from_str(SYNTHESIZER_CORPUS).expect("synthesizer corpus is valid JSON");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("building the bench tokio runtime");

    let embedder = Arc::new(OnnxEmbedding::new().expect("initializing the fastembed model (downloads ~50MB on first use)"));

    let passthrough = PassthroughSynthesizer::new();
    let embedding = EmbeddingSynthesizer::new(embedder);

    let mut passthrough_tally = Tally::default();
    let mut embedding_tally = Tally::default();

    for case in &corpus.cases {
        let facts: Vec<SemanticFact> = case
            .facts
            .iter()
            .map(|fact| SemanticFact {
                content: fact.content.clone(),
            })
            .collect();

        let pass_out = runtime
            .block_on(passthrough.synthesize(input_triples(case), &facts))
            .expect("passthrough synthesis never errors");
        tally_case(&pass_out, case, &mut passthrough_tally, "passthrough");

        let embed_out = runtime
            .block_on(embedding.synthesize(input_triples(case), &facts))
            .unwrap_or_else(|err| panic!("embedding synthesis failed for {}: {err}", case.id));
        tally_case(&embed_out, case, &mut embedding_tally, "embedding");
    }

    let table = render_table(&[("passthrough", passthrough_tally), ("embedding", embedding_tally)]);
    print!("{table}");
    write_results(&table);
}

/// Builds the candidate triple set for a case (fresh per synthesizer call).
fn input_triples(case: &Case) -> TripleSet {
    case.triples.iter().map(triple_of).collect()
}

/// Renders the per-synthesizer keep/veto breakdown as a markdown table.
fn render_table(results: &[(&str, Tally)]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# synthesizer_accuracy");
    let _ = writeln!(out);
    let _ = writeln!(out, "Triple keep/veto decisions vs gold; real fastembed embedder.");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "| synthesizer | accuracy | correct_keep | correct_veto | false_keep | false_veto |"
    );
    let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: | ---: |");
    for (label, tally) in results {
        let _ = writeln!(
            out,
            "| {} | {:.3} | {} | {} | {} | {} |",
            label,
            tally.accuracy(),
            tally.correct_keep,
            tally.correct_veto,
            tally.false_keep,
            tally.false_veto
        );
    }
    out
}

/// Writes the rendered table to the committed results file.
fn write_results(table: &str) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dir = Path::new(manifest_dir).join(RESULTS_DIR);
    std::fs::create_dir_all(&dir).expect("creating the benches/results directory");
    let path = dir.join("synthesizer_accuracy.md");
    std::fs::write(&path, table).unwrap_or_else(|err| panic!("writing results to {}: {err}", path.display()));
    println!("wrote {}", path.display());
}
