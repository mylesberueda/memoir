//! Reconciling extracted triples against semantic facts before commit.
//!
//! Synthesis is the fan-in of memoir's two LLM-derived siblings: the relational
//! triples ([`TripleExtractor`](super::TripleExtractor)) and the flat semantic
//! facts (the extraction worker). A [`Synthesizer`] reconciles them — vetoing
//! triples the semantic facts do not corroborate (precision: kills hallucinated
//! edges) and, where an implementation chooses, contributing relationships the
//! triple pass missed (recall). Its output is the canonical triple set the
//! commit path writes.
//!
//! Like every stage of how memoir's memory works, this is a swappable seam: the
//! trait is model-agnostic, and implementations range from a cheap default
//! ([`EmbeddingSynthesizer`], reuse the embedder already in hand) through a
//! disable switch ([`PassthroughSynthesizer`]) to consumer-supplied or
//! LLM-backed reconcilers. The default is cheap by deliberate choice for a
//! library; an expensive reconciler is a valid alternative behind the same
//! trait, not a thing ruled out.

use std::future::Future;

use crate::embedding::{EmbeddingError, EmbeddingModel};

use super::cosine::cosine_similarity;
use super::{Triple, TripleSet};

/// Minimum cosine similarity for a semantic fact to corroborate a triple.
///
/// A triple scoring below this against every semantic fact is treated as
/// uncorroborated — likely a hallucinated edge — and vetoed. Mirrors the
/// conservative-default reasoning of `MIN_ENTITY_SIMILARITY` /
/// `MIN_CATEGORY_SCORE`: tuned to drop weakly-supported facts rather than admit
/// them, since a spurious edge pollutes traversal more than a missing one.
pub const MIN_CORROBORATION_SIMILARITY: f32 = 0.6;

/// A semantic fact a [`Synthesizer`] reconciles triples against.
///
/// Carries the fact's text — the corroboration signal. An implementation that
/// needs vectors embeds the text itself (the embedder is the impl's concern, not
/// the trait's), so this stays a plain, backend-agnostic input.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticFact {
    /// The fact's content sentence, as the extraction worker produced it.
    pub content: String,
}

/// Reconciles extracted triples against semantic facts into a committable set.
///
/// The fifth trait seam of the knowledge-graph pipeline. Implementations decide
/// which initial triples survive (corroborated) and may add triples the
/// extractor missed; the result is what the commit path writes. Swapping one
/// implementation for another (passthrough, embedding, a future LLM-backed or
/// consumer impl) requires no caller change, which is what lets the pipeline be
/// reconfigured and benchmarked.
pub trait Synthesizer: Send + Sync + 'static {
    /// Reconciles `triples` against `facts` into the set to commit.
    ///
    /// # Errors
    ///
    /// Returns [`SynthesisError`] when the implementation's own machinery fails
    /// (e.g. embedding a triple or fact).
    fn synthesize(
        &self,
        triples: TripleSet,
        facts: &[SemanticFact],
    ) -> impl Future<Output = Result<TripleSet, SynthesisError>> + Send;
}

/// Failure modes for [`Synthesizer`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum SynthesisError {
    /// Embedding a triple or fact during corroboration failed.
    #[error("synthesis embedding failed: {0}")]
    Embed(#[from] EmbeddingError),
}

/// Commits the extracted triples unchanged — the disable switch and floor.
///
/// Performs no reconciliation: every initial triple passes through. This is the
/// "synthesis off" configuration and the benchmark floor an active reconciler is
/// measured against. Never fails and never calls a model.
#[derive(Debug, Default, Clone, Copy)]
pub struct PassthroughSynthesizer;

impl PassthroughSynthesizer {
    /// Creates a passthrough synthesizer.
    pub fn new() -> Self {
        Self
    }
}

impl Synthesizer for PassthroughSynthesizer {
    async fn synthesize(&self, triples: TripleSet, _facts: &[SemanticFact]) -> Result<TripleSet, SynthesisError> {
        Ok(triples)
    }
}

/// Vetoes triples no semantic fact corroborates, by embedding similarity.
///
/// The cheap default: reuses the [`EmbeddingModel`] already configured on the
/// client (no extra model call class). Each triple is rendered to text and
/// embedded; it survives if its cosine similarity to some semantic fact's
/// embedding is at least [`MIN_CORROBORATION_SIMILARITY`], and is vetoed
/// otherwise. This is the precision half of synthesis — the recall half
/// (deriving triples for facts no triple covered) is left to richer
/// implementations.
///
/// Generic over the embedder so tests inject a stub.
pub struct EmbeddingSynthesizer<E> {
    embedder: E,
    min_similarity: f32,
}

impl<E: EmbeddingModel> EmbeddingSynthesizer<E> {
    /// Builds a synthesizer over `embedder` using the default corroboration floor.
    pub fn new(embedder: E) -> Self {
        Self {
            embedder,
            min_similarity: MIN_CORROBORATION_SIMILARITY,
        }
    }

    /// Overrides the minimum corroboration similarity.
    #[must_use]
    pub fn with_min_similarity(mut self, min_similarity: f32) -> Self {
        self.min_similarity = min_similarity;
        self
    }
}

impl<E: EmbeddingModel> Synthesizer for EmbeddingSynthesizer<E> {
    async fn synthesize(&self, triples: TripleSet, facts: &[SemanticFact]) -> Result<TripleSet, SynthesisError> {
        if facts.is_empty() {
            return Ok(TripleSet::default());
        }

        let mut fact_embeddings = Vec::with_capacity(facts.len());
        for fact in facts {
            fact_embeddings.push(self.embedder.embed(&fact.content).await?);
        }

        let mut kept = Vec::new();
        for triple in triples {
            let rendered = render_triple(&triple);
            let triple_embedding = self.embedder.embed(&rendered).await?;
            let corroborated = fact_embeddings
                .iter()
                .filter_map(|fact| cosine_similarity(&triple_embedding, fact))
                .any(|score| score >= self.min_similarity);
            if corroborated {
                kept.push(triple);
            }
        }

        Ok(kept.into_iter().collect())
    }
}

/// Renders a triple to the text embedded for corroboration.
fn render_triple(triple: &Triple) -> String {
    format!("{} {} {}", triple.subject, triple.relation, triple.object)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triple(subject: &str, relation: &str, object: &str) -> Triple {
        Triple {
            subject: subject.to_string(),
            relation: relation.to_string(),
            object: object.to_string(),
            confidence: 0.9,
        }
    }

    fn triples(items: Vec<Triple>) -> TripleSet {
        items.into_iter().collect()
    }

    fn fact(content: &str) -> SemanticFact {
        SemanticFact {
            content: content.to_string(),
        }
    }

    /// Embeds corroborated text to one vector and everything else orthogonally,
    /// so a triple "matches" a fact iff both render to the corroborated token.
    struct FakeEmbedding;

    impl EmbeddingModel for FakeEmbedding {
        async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
            let vector = if text.contains("Acme") {
                vec![1.0, 0.0, 0.0]
            } else if text.contains("Globex") {
                vec![0.0, 1.0, 0.0]
            } else {
                vec![0.0, 0.0, 1.0]
            };
            Ok(vector)
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_pass_all_triples_through_passthrough() {
        let synth = PassthroughSynthesizer::new();
        let input = triples(vec![triple("Alice", "works at", "Acme"), triple("Bob", "likes", "tea")]);

        let out = synth.synthesize(input.clone(), &[]).await.unwrap();

        assert_eq!(out.len(), 2);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_keep_corroborated_triple() {
        let synth = EmbeddingSynthesizer::new(FakeEmbedding);
        let input = triples(vec![triple("Alice", "works at", "Acme")]);

        let out = synth.synthesize(input, &[fact("Alice works at Acme Corp")]).await.unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].object, "Acme");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_veto_uncorroborated_triple() {
        // The triple says Globex; the only fact is about Acme -> orthogonal -> vetoed.
        let synth = EmbeddingSynthesizer::new(FakeEmbedding);
        let input = triples(vec![triple("Alice", "works at", "Globex")]);

        let out = synth.synthesize(input, &[fact("Alice works at Acme Corp")]).await.unwrap();

        assert!(out.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_veto_everything_when_no_facts() {
        // No semantic facts means nothing corroborates -> all triples vetoed.
        let synth = EmbeddingSynthesizer::new(FakeEmbedding);
        let input = triples(vec![triple("Alice", "works at", "Acme")]);

        let out = synth.synthesize(input, &[]).await.unwrap();

        assert!(out.is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_keep_only_corroborated_among_mixed() {
        let synth = EmbeddingSynthesizer::new(FakeEmbedding);
        let input = triples(vec![
            triple("Alice", "works at", "Acme"),
            triple("Alice", "works at", "Globex"),
        ]);

        let out = synth.synthesize(input, &[fact("Alice works at Acme")]).await.unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].object, "Acme");
    }

    #[test]
    fn should_render_triple_as_subject_relation_object() {
        assert_eq!(render_triple(&triple("Alice", "works at", "Acme")), "Alice works at Acme");
    }
}
