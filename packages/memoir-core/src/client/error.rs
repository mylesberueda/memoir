use crate::embedding::EmbeddingError;
use crate::jobs::JobsError;
use crate::llm::LlmError;
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

    #[error("jobs failed: {0}")]
    Jobs(#[from] JobsError),

    #[error("llm provider failed: {0}")]
    Llm(#[from] LlmError),

    #[error("migration failed: {0}")]
    Migration(#[from] crate::migration::MigrationError),

    #[error("database connection failed: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error(
        "metadata uses reserved key '{key}'; reserved keys are owned by memoir-core's payload schema and cannot be set via metadata"
    )]
    ReservedMetadataKey { key: String },

    /// NLI classifier initialization failed (epic 0011).
    #[error("nli classifier failed: {0}")]
    Nli(String),

    /// Feedback targeted a memory that cannot be corrected (epic 0011).
    ///
    /// Feedback corrects a wrong *extraction*, so its target must be a
    /// semantic row derived from an episodic source. An episodic target
    /// (correct it via [`crate::client::Client::edit`] instead) or a semantic
    /// row with no `source_pid` cannot be reprocessed.
    #[error("memory {pid} is not correctable via feedback: {reason}")]
    NotCorrectable { pid: String, reason: String },
}
