use super::{BoxFuture, EmbeddingError, EmbeddingModel};
use fastembed::{InitOptions, TextEmbedding};
use std::sync::Mutex;

/// Local ONNX-based embedding using BGE-small-en-v1.5 (384 dimensions).
/// Downloads the model on first use (~50MB) and caches locally.
pub(crate) struct OnnxEmbedding {
    model: Mutex<TextEmbedding>,
}

impl std::fmt::Debug for OnnxEmbedding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OnnxEmbedding").finish_non_exhaustive()
    }
}

impl OnnxEmbedding {
    pub(crate) fn new() -> Result<Self, EmbeddingError> {
        let options = InitOptions::new(fastembed::EmbeddingModel::BGESmallENV15).with_show_download_progress(true);
        let model = TextEmbedding::try_new(options).map_err(|e| EmbeddingError::Init(e.to_string()))?;

        Ok(Self {
            model: Mutex::new(model),
        })
    }
}

impl EmbeddingModel for OnnxEmbedding {
    fn embed(&self, text: &str) -> BoxFuture<'_, Result<Vec<f32>, EmbeddingError>> {
        let text = text.to_owned();
        Box::pin(async move {
            let mut model = self.model.lock().map_err(|e| EmbeddingError::Embed(e.to_string()))?;
            let results = model
                .embed(vec![&text], None)
                .map_err(|e| EmbeddingError::Embed(e.to_string()))?;

            results
                .into_iter()
                .next()
                .ok_or_else(|| EmbeddingError::Embed("empty result from model".into()))
        })
    }

    fn embed_batch<'a>(&'a self, texts: &'a [&'a str]) -> BoxFuture<'a, Result<Vec<Vec<f32>>, EmbeddingError>> {
        Box::pin(async move {
            let docs: Vec<&str> = texts.to_vec();
            let mut model = self.model.lock().map_err(|e| EmbeddingError::Embed(e.to_string()))?;
            model
                .embed(docs, None)
                .map_err(|e| EmbeddingError::Embed(e.to_string()))
        })
    }

    fn dimensions(&self) -> usize {
        384
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn should_return_384_dimensional_vector() {
        let model = OnnxEmbedding::new().expect("model init");
        let embedding = model.embed("hello world").await.expect("embed");
        assert_eq!(embedding.len(), 384);
    }

    #[tokio::test]
    async fn should_report_384_dimensions() {
        let model = OnnxEmbedding::new().expect("model init");
        assert_eq!(model.dimensions(), 384);
    }

    #[tokio::test]
    async fn should_embed_batch_of_texts() {
        let model = OnnxEmbedding::new().expect("model init");
        let texts = vec!["hello", "world", "test"];
        let embeddings = model.embed_batch(&texts).await.expect("batch embed");
        assert_eq!(embeddings.len(), 3);
        assert!(embeddings.iter().all(|e| e.len() == 384));
    }

    #[tokio::test]
    async fn should_embed_empty_string() {
        let model = OnnxEmbedding::new().expect("model init");
        let embedding = model.embed("").await.expect("embed empty");
        assert_eq!(embedding.len(), 384);
    }

    #[tokio::test]
    async fn should_return_empty_vec_for_empty_batch() {
        let model = OnnxEmbedding::new().expect("model init");
        let texts: Vec<&str> = vec![];
        let embeddings = model.embed_batch(&texts).await.expect("batch embed empty");
        assert!(embeddings.is_empty());
    }
}
