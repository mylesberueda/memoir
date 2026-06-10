//! Text-to-vector embedding primitive.
//!
//! Defines [`EmbeddingModel`], implemented by [`OnnxEmbedding`] and by callers
//! who want to plug in their own embedder.

mod error;
pub mod onnx;

pub use error::EmbeddingError;
pub use onnx::OnnxEmbedding;

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

impl<T: EmbeddingModel> EmbeddingModel for std::sync::Arc<T> {
    fn embed(&self, text: &str) -> impl std::future::Future<Output = Result<Vec<f32>, EmbeddingError>> + Send {
        (**self).embed(text)
    }

    fn dimensions(&self) -> usize {
        (**self).dimensions()
    }
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
}
