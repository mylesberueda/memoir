mod error;
pub(crate) mod onnx;
use crate::api::embedding::onnx::OnnxEmbedding;
pub(crate) use error::EmbeddingError;
use std::{future::Future, pin::Pin};

pub(crate) type DefaultEmbedding = OnnxEmbedding;
pub(crate) type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for generating text embeddings. Implementations must be Send + Sync
/// for use across the actor system via Arc.
pub(crate) trait EmbeddingModel: Send + Sync + 'static {
    /// Generate an embedding vector for a single text input.
    fn embed(&self, text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>>;

    /// Generate embedding vectors for multiple text inputs.
    /// Default: sequential calls to [`embed`]. Implementations should override
    /// for batch-optimized inference.
    #[allow(dead_code)] // Used in task 03 (compaction)
    fn embed_batch<'a>(&'a self, texts: &'a [&'a str]) -> BoxFuture<'a, Result<Vec<Vec<f32>>, EmbeddingError>> {
        Box::pin(async move {
            let mut results = Vec::with_capacity(texts.len());
            for text in texts {
                results.push(self.embed(text).await?);
            }
            Ok(results)
        })
    }

    /// The dimensionality of the embedding vectors produced by this model.
    #[allow(dead_code)] // Used in task 03 (compaction)
    fn dimensions(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal implementation that only provides `embed` + `dimensions`,
    /// relying on the trait's default `embed_batch` (sequential delegation).
    struct StubEmbeddingModel;

    impl EmbeddingModel for StubEmbeddingModel {
        fn embed(&self, text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            let len = text.len();
            Box::pin(async move { Ok(vec![len as f32; 4]) })
        }

        fn dimensions(&self) -> usize {
            4
        }
    }

    /// Mock that always returns an error, for testing error propagation.
    struct FailingEmbeddingModel;

    impl EmbeddingModel for FailingEmbeddingModel {
        fn embed(&self, _text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
            Box::pin(async { Err(EmbeddingError::Embed("model unavailable".into())) })
        }

        fn dimensions(&self) -> usize {
            4
        }
    }

    #[tokio::test]
    async fn should_use_default_embed_batch_via_sequential_embed() {
        let model = StubEmbeddingModel;
        let texts = vec!["hi", "hello", "hey"];
        let results = model.embed_batch(&texts).await.expect("batch");

        assert_eq!(results.len(), 3);
        // StubEmbeddingModel encodes text length — verify each result matches
        assert!(results[0].iter().all(|&v| v == 2.0)); // "hi" = 2 chars
        assert!(results[1].iter().all(|&v| v == 5.0)); // "hello" = 5 chars
        assert!(results[2].iter().all(|&v| v == 3.0)); // "hey" = 3 chars
    }

    #[tokio::test]
    async fn should_return_empty_vec_from_default_embed_batch_with_no_inputs() {
        let model = StubEmbeddingModel;
        let texts: Vec<&str> = vec![];
        let results = model.embed_batch(&texts).await.expect("batch empty");
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn should_propagate_embed_error_from_failing_model() {
        let model = FailingEmbeddingModel;
        let result = model.embed("anything").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, EmbeddingError::Embed(ref msg) if msg == "model unavailable"),
            "expected Embed error, got: {err}"
        );
    }

    #[tokio::test]
    async fn should_propagate_error_through_default_embed_batch() {
        let model = FailingEmbeddingModel;
        let texts = vec!["a", "b"];
        let result = model.embed_batch(&texts).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, EmbeddingError::Embed(ref msg) if msg == "model unavailable"),
            "expected Embed error from batch, got: {err}"
        );
    }
}
