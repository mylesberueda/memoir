use crate::embedding::EmbeddingError;
use crate::store::StoreError;
use crate::vector::VectorError;

/// Failure modes for [`crate::client::Client`] construction and operations.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("embedding model failed: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("vector index failed: {0}")]
    Vector(#[from] VectorError),

    #[error("store failed: {0}")]
    Store(#[from] StoreError),

    #[error("migration failed: {0}")]
    Migration(#[from] memoir_core_migration::MigrationError),
}
