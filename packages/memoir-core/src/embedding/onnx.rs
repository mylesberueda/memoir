//! [`EmbeddingModel`] implementation backed by `fastembed`.

use std::sync::{Arc, Mutex};

use super::{EmbeddingError, EmbeddingModel};

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
        let model = fastembed::TextEmbedding::try_new(options)
            .map_err(|e| EmbeddingError::Init(e.to_string()))?;
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

    #[test]
    fn should_report_onnx_dimensions_as_384() {
        assert_eq!(ONNX_DIMENSIONS, 384);
    }
}
