use crate::embedding::EmbeddingError;
use crate::vector::VectorError;

/// Failure modes for [`crate::client::Client`] construction and lifecycle.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("embedding model initialization failed: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("vector index bootstrap failed: {0}")]
    Vector(#[from] VectorError),

    #[error("migration failed: {0}")]
    Migration(#[from] memoir_core_migration::MigrationError),
}
