//! Text-to-vector embedding primitive.
//!
//! Defines [`EmbeddingModel`], implemented by `OnnxEmbedding` (ticket 0003)
//! and by callers who want to plug in their own embedder.

/// Failure modes for [`EmbeddingModel`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("embedding model initialization failed: {0}")]
    Init(String),

    #[error("embedding failed: {0}")]
    Embed(String),
}

/// Produces a fixed-dimension float vector from a text input.
///
/// Implementations must be deterministic: embedding the same input twice
/// returns identical vectors. The returned vector's length must equal
/// [`Self::dimensions`].
pub trait EmbeddingModel: Send + Sync + 'static {
    /// Embeds `text` into a [`Self::dimensions`]-length vector.
    ///
    /// # Errors
    ///
    /// Returns [`EmbeddingError::Embed`] when inference fails. Init-time
    /// failures surface from the implementation's constructor.
    fn embed(&self, text: &str) -> impl std::future::Future<Output = Result<Vec<f32>, EmbeddingError>> + Send;

    /// Returns the dimension of vectors produced by [`Self::embed`].
    fn dimensions(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubEmbedding {
        dim: usize,
    }

    impl EmbeddingModel for StubEmbedding {
        async fn embed(&self, _text: &str) -> Result<Vec<f32>, EmbeddingError> {
            Ok(vec![0.1; self.dim])
        }

        fn dimensions(&self) -> usize {
            self.dim
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_implement_trait_with_in_test_stub() {
        let model = StubEmbedding { dim: 4 };

        let vector = model.embed("hello").await.unwrap();

        assert_eq!(vector.len(), 4);
        assert_eq!(model.dimensions(), 4);
    }

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
