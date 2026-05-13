#![cfg(test)]

pub mod mocks;

#[cfg(feature = "integration")]
pub mod context;

#[expect(
    unused_imports,
    reason = "These mocks are re-exported for test modules across the crate"
)]
pub use mocks::{MockEmbeddingModel, MockStore};

#[cfg(feature = "integration")]
pub use context::{TestContext, init_test_crypto};
