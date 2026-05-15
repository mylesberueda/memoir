//! Memoir memory substrate as an embeddable Rust library.

pub mod client;
pub mod embedding;
pub mod jobs;
pub mod memory;
pub mod store;
pub mod vector;

#[doc(inline)]
pub use client::{Client, DEFAULT_SYSTEM_PROMPT};
