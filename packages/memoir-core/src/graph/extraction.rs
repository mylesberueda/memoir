//! Turning episodic text into relational triples.
//!
//! Defines [`TripleExtractor`], the seam that produces `(subject, relation,
//! object)` triples from an episodic memory's content, and [`LlmExtractor`],
//! the LLM-backed implementation that is the production default. The relation
//! vocabulary is open: the model names relations in its own words, which suits
//! agent memory's idiosyncratic relations (`prefers`, `blocked_by`) better than
//! a fixed taxonomy.
//!
//! Entities are bare strings here; entity-type labels (`:Person`, `:Org`) are a
//! concern of the resolution and commit path, not extraction.

use std::future::Future;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::llm::{LlmError, LlmProvider};

/// One extracted relationship: `subject` is related to `object` via `relation`.
///
/// `confidence` is the extractor's stated certainty on the 0.0-1.0 scale.
/// Entities are bare strings; typing them is the resolver's concern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triple {
    pub subject: String,
    pub relation: String,
    pub object: String,
    #[serde(default = "default_confidence")]
    pub confidence: f32,
}

fn default_confidence() -> f32 {
    1.0
}

/// The triples extracted from one episodic memory, in extraction order.
///
/// Also the deserialization target for an LLM reply: the JSON shape is
/// `{"triples": [...]}`, so the field is named `triples`. Construct one from a
/// raw reply with [`TripleSet::try_new`]; read it as a slice via [`Deref`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TripleSet {
    #[serde(default)]
    triples: Vec<Triple>,
}

impl TripleSet {
    /// Parses an LLM's raw reply into a [`TripleSet`].
    ///
    /// Locates the first balanced JSON object in the reply (tolerating markdown
    /// fences and surrounding prose) and deserializes it.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError::Parse`] when the reply is empty, exceeds
    /// [`TRIPLE_REPLY_MAX_CHARS`], contains no balanced JSON object, or does not
    /// deserialize. The message carries length and reason, never the raw text.
    pub fn try_new(raw: &str) -> Result<Self, LlmError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(LlmError::Parse("empty llm reply".to_string()));
        }
        if trimmed.len() > TRIPLE_REPLY_MAX_CHARS {
            return Err(LlmError::Parse(format!(
                "reply too long: len={} > max={TRIPLE_REPLY_MAX_CHARS}",
                trimmed.len()
            )));
        }

        let json_slice = crate::llm::locate_json_object(trimmed)
            .ok_or_else(|| LlmError::Parse(format!("no balanced json object found in len={}", trimmed.len())))?;

        serde_json::from_str(json_slice)
            .map_err(|err| LlmError::Parse(format!("json deserialize failed at len={}: {err}", json_slice.len())))
    }

    /// Consumes the set, returning the owned triples.
    pub fn into_inner(self) -> Vec<Triple> {
        self.triples
    }
}

impl Deref for TripleSet {
    type Target = [Triple];

    fn deref(&self) -> &Self::Target {
        &self.triples
    }
}

impl IntoIterator for TripleSet {
    type Item = Triple;
    type IntoIter = std::vec::IntoIter<Triple>;

    fn into_iter(self) -> Self::IntoIter {
        self.triples.into_iter()
    }
}

impl<'a> IntoIterator for &'a TripleSet {
    type Item = &'a Triple;
    type IntoIter = std::slice::Iter<'a, Triple>;

    fn into_iter(self) -> Self::IntoIter {
        self.triples.iter()
    }
}

impl FromIterator<Triple> for TripleSet {
    fn from_iter<I: IntoIterator<Item = Triple>>(iter: I) -> Self {
        Self {
            triples: iter.into_iter().collect(),
        }
    }
}

/// System prompt steering an LLM toward a triple-extraction JSON reply.
pub const DEFAULT_TRIPLE_PROMPT: &str = "\
You extract relationships from text as subject-relation-object triples.
Return ONLY a JSON object of the form:
{\"triples\": [{\"subject\": \"...\", \"relation\": \"...\", \"object\": \"...\", \"confidence\": 0.0}]}
Rules:
- subject and object are concrete entities (people, places, organizations, things).
- relation is a short verb phrase in your own words (e.g. \"works at\", \"prefers\", \"lives in\").
- confidence is your certainty from 0.0 to 1.0.
- Extract only relationships the text actually states. Emit an empty list if there are none.
- Do not add commentary outside the JSON object.";

/// Maximum reply length [`LlmExtractor`] will attempt to parse, in bytes.
pub const TRIPLE_REPLY_MAX_CHARS: usize = 100_000;

/// Extracts relational triples from episodic text.
///
/// Implementations turn an episodic memory's content into a [`TripleSet`].
/// Swapping one implementation for another (LLM, syntactic, purpose-built)
/// requires no caller change, which is what lets the benchmark compare them.
pub trait TripleExtractor: Send + Sync + 'static {
    /// Extracts triples from `content`.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError`] (or an impl-specific error mapped onto it) when the
    /// backend call or the parse of its reply fails.
    fn extract(&self, content: &str) -> impl Future<Output = Result<TripleSet, LlmError>> + Send;
}

/// LLM-backed [`TripleExtractor`] — the production default.
///
/// Wraps any [`LlmProvider`] and prompts it for a structured triple reply, then
/// parses the JSON. Generic over the provider so a test can inject a stub.
pub struct LlmExtractor<P> {
    provider: P,
    prompt: String,
}

impl<P: LlmProvider> LlmExtractor<P> {
    /// Builds an extractor over `provider` using [`DEFAULT_TRIPLE_PROMPT`].
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            prompt: DEFAULT_TRIPLE_PROMPT.to_string(),
        }
    }

    /// Overrides the system prompt steering the extraction.
    #[must_use]
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }
}

impl<P: LlmProvider> TripleExtractor for LlmExtractor<P> {
    async fn extract(&self, content: &str) -> Result<TripleSet, LlmError> {
        let raw = self.provider.extract(&self.prompt, content).await?;
        TripleSet::try_new(&raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_well_formed_triple_reply() {
        let raw = r#"{"triples":[{"subject":"Alice","relation":"works at","object":"Acme","confidence":0.9}]}"#;
        let triples = TripleSet::try_new(raw).unwrap();
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].subject, "Alice");
        assert_eq!(triples[0].relation, "works at");
        assert_eq!(triples[0].object, "Acme");
        assert_eq!(triples[0].confidence, 0.9);
    }

    #[test]
    fn should_parse_reply_wrapped_in_prose_and_fences() {
        let raw = "Here are the triples:\n```json\n{\"triples\":[{\"subject\":\"Bob\",\"relation\":\"lives in\",\"object\":\"Paris\"}]}\n```\nDone.";
        let triples = TripleSet::try_new(raw).unwrap();
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].object, "Paris");
    }

    #[test]
    fn should_default_confidence_when_absent() {
        let raw = r#"{"triples":[{"subject":"Bob","relation":"likes","object":"tea"}]}"#;
        let triples = TripleSet::try_new(raw).unwrap();
        assert_eq!(triples[0].confidence, 1.0);
    }

    #[test]
    fn should_return_empty_set_for_empty_triple_list() {
        let triples = TripleSet::try_new(r#"{"triples":[]}"#).unwrap();
        assert!(triples.is_empty());
    }

    #[test]
    fn should_reject_empty_reply() {
        assert!(TripleSet::try_new("   ").is_err());
    }

    #[test]
    fn should_reject_reply_with_no_json() {
        assert!(TripleSet::try_new("no json here").is_err());
    }

    struct StubProvider {
        reply: String,
    }

    impl LlmProvider for StubProvider {
        async fn extract(&self, _preamble: &str, _content: &str) -> Result<String, LlmError> {
            Ok(self.reply.clone())
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_extract_triples_through_the_trait() {
        let provider = StubProvider {
            reply: r#"{"triples":[{"subject":"Alice","relation":"works at","object":"Acme","confidence":0.8}]}"#
                .to_string(),
        };
        let extractor = LlmExtractor::new(provider);

        let triples = extractor.extract("Alice works at Acme.").await.unwrap();

        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].subject, "Alice");
        assert_eq!(triples[0].relation, "works at");
    }
}
