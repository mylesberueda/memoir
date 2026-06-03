//! Memoir memory substrate as an embeddable Rust library.
//!
//! Memoir stores what an agent is told and derives durable facts from it. A
//! write is an *episodic* memory (the raw utterance); a background worker runs
//! LLM extraction over it to produce *semantic* memories (the facts). See
//! [`memory::MemoryKind`]. Recall reads the semantic layer; the episodic layer
//! is the audit trail and the source the facts are re-derived from.
//!
//! Two contracts govern how that derived knowledge is ranked and corrected.
//!
//! # Selection
//!
//! Retrieval is not just cosine similarity. Each semantic memory carries a
//! [`memory::Confidence`] (how sure the extractor was) and, once the categorize
//! worker runs, a category label. [`client::Client::query`] blends these signals
//! — cosine, confidence, recency, and a category bonus — via a
//! [`client::RankingStrategy`] with tunable [`client::BlendWeights`]. Use
//! [`client::Client::search`] for raw nearest-neighbor hits, `query` when you
//! want ranked, prompt-shaped context. Hard filters (`min_confidence`,
//! `category`) exclude rows; the blend *weights* them — distinct mechanisms.
//!
//! # Correction
//!
//! Semantic memory is **always derived, never hand-written** — there is no API
//! to edit a semantic row's content. A consumer corrects a wrong fact by
//! *teaching*: [`client::Client::feedback`] supplies the correction, and memoir
//! re-derives from the episodic source. Editing the episodic source itself
//! ([`client::Client::edit`]) cascades the same way. Both retire the stale
//! derived rows and re-extract — see [`memory::RetirementReason`] for why a
//! retirement is `Rejected` (a wrong extraction, counts against extraction
//! accuracy) versus `Stale` (the source changed, does not). Retired rows are
//! kept (not deleted) so accuracy is measurable via
//! [`client::Client::extraction_stats`].
//!
//! # Categorization
//!
//! The category label is populated by a zero-shot NLI classifier
//! ([`nli::NliClassifier`]), opt-in via [`nli::NliConfig`] on the client builder.
//! Without it, categorization is skipped and the category-bonus blend term is
//! inert.

pub mod client;
pub mod embedding;
pub mod jobs;
pub mod llm;
pub mod memory;
pub mod migration;
pub mod nli;
pub mod store;
pub mod vector;

#[doc(inline)]
pub use client::{Client, DEFAULT_SYSTEM_PROMPT};

#[doc(inline)]
pub use nli::{ExecutionProvider, NliClassifier, NliConfig, NliError, ScoredLabel};

#[doc(inline)]
pub use llm::DEFAULT_EXTRACTION_PROMPT;

/// Default Postgres schema for memoir-core's tables.
///
/// Consumers configuring a custom schema via `Client::builder().schema(...)`
/// can fall back to this when no override is supplied.
pub use migration::DEFAULT_SCHEMA;
