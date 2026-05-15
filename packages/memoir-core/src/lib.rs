//! Memoir memory substrate as an embeddable Rust library.

pub mod client;
pub mod embedding;
pub mod jobs;
pub mod llm;
pub mod memory;
pub mod store;
pub mod vector;

#[doc(inline)]
pub use client::{Client, DEFAULT_SYSTEM_PROMPT};

#[doc(inline)]
pub use llm::DEFAULT_EXTRACTION_PROMPT;
