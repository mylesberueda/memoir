#[derive(Debug, thiserror::Error)]
pub(crate) enum EmbeddingError {
    #[error("embedding model initialization failed: {0}")]
    Init(String),
    #[allow(dead_code)] // Will be wired next task
    #[error("embedding generation failed: {0}")]
    Embed(String),
}
