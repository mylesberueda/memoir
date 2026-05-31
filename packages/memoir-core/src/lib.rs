//! Memoir memory substrate as an embeddable Rust library.

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
pub use nli::{ExecutionProvider, NliClassifier, NliError, ScoredLabel};

#[doc(inline)]
pub use llm::DEFAULT_EXTRACTION_PROMPT;

/// Default Postgres schema for memoir-core's tables.
///
/// Consumers configuring a custom schema via `Client::builder().schema(...)`
/// can fall back to this when no override is supplied.
pub use migration::DEFAULT_SCHEMA;
