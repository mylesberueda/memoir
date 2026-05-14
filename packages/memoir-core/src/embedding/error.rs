/// Failure modes for [`crate::embedding::EmbeddingModel`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("embedding model initialization failed: {0}")]
    Init(String),

    #[error("embedding failed: {0}")]
    Embed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_init_error_with_message() {
        let err = EmbeddingError::Init("model file missing".to_string());

        assert_eq!(
            err.to_string(),
            "embedding model initialization failed: model file missing"
        );
    }

    #[test]
    fn should_render_embed_error_with_message() {
        let err = EmbeddingError::Embed("input too long".to_string());

        assert_eq!(err.to_string(), "embedding failed: input too long");
    }
}
