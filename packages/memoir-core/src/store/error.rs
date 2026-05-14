/// Failure modes for [`crate::store::MemoryStore`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("memory not found: {0}")]
    NotFound(String),

    #[error("invalid scope: {0}")]
    InvalidScope(String),

    #[error("database error: {0}")]
    Database(String),
}
