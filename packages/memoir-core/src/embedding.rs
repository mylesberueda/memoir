//! Text-to-vector embedding primitive.
//!
//! Defines [`EmbeddingModel`], implemented by [`OnnxEmbedding`] and by callers
//! who want to plug in their own embedder.

use std::sync::{Arc, Mutex};

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

const ONNX_DIMENSIONS: usize = 384;

/// Default [`EmbeddingModel`] backed by `fastembed`'s BGE-small-en-v1.5.
///
/// Produces 384-dimension vectors. The model file is downloaded on first
/// construction (~50 MB) and cached locally by `fastembed`.
pub struct OnnxEmbedding {
    model: Arc<Mutex<fastembed::TextEmbedding>>,
}

impl std::fmt::Debug for OnnxEmbedding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnnxEmbedding").finish_non_exhaustive()
    }
}

impl OnnxEmbedding {
    /// Initializes the embedder, downloading the model file if not cached.
    ///
    /// # Errors
    ///
    /// Returns [`EmbeddingError::Init`] when the model cannot be loaded —
    /// typically a download failure on first use or a corrupted cache.
    pub fn new() -> Result<Self, EmbeddingError> {
        let options = fastembed::InitOptions::new(fastembed::EmbeddingModel::BGESmallENV15);
        let model = fastembed::TextEmbedding::try_new(options).map_err(|e| EmbeddingError::Init(e.to_string()))?;
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }
}

impl EmbeddingModel for OnnxEmbedding {
    fn embed(&self, text: &str) -> impl std::future::Future<Output = Result<Vec<f32>, EmbeddingError>> + Send {
        let model = self.model.clone();
        let text = text.to_owned();
        async move {
            tokio::task::spawn_blocking(move || {
                let mut guard = model
                    .lock()
                    .map_err(|e| EmbeddingError::Embed(format!("model lock poisoned: {e}")))?;
                let mut results = guard
                    .embed(vec![&text], None)
                    .map_err(|e| EmbeddingError::Embed(e.to_string()))?;
                results
                    .pop()
                    .ok_or_else(|| EmbeddingError::Embed("empty result from model".into()))
            })
            .await
            .map_err(|e| EmbeddingError::Embed(format!("join error: {e}")))?
        }
    }

    fn dimensions(&self) -> usize {
        ONNX_DIMENSIONS
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

    #[test]
    fn should_report_onnx_dimensions_as_384() {
        assert_eq!(ONNX_DIMENSIONS, 384);
    }
}
