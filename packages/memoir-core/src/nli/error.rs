/// Failure modes for the [`crate::nli::NliClassifier`].
#[derive(Debug, thiserror::Error)]
pub enum NliError {
    /// The model or tokenizer file could not be downloaded from HuggingFace.
    #[error("failed to download NLI model: {0}")]
    Download(String),

    /// The ONNX model could not be loaded into an inference session.
    #[error("failed to load NLI model: {0}")]
    ModelLoad(String),

    /// The tokenizer file could not be parsed.
    #[error("failed to load NLI tokenizer: {0}")]
    TokenizerLoad(String),

    /// Tokenization or model inference failed at classification time.
    #[error("NLI inference error: {0}")]
    Inference(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_model_load_error_with_message() {
        let err = NliError::ModelLoad("session init failed".to_string());

        assert_eq!(err.to_string(), "failed to load NLI model: session init failed");
    }

    #[test]
    fn should_render_inference_error_with_message() {
        let err = NliError::Inference("tokenization failed".to_string());

        assert_eq!(err.to_string(), "NLI inference error: tokenization failed");
    }
}
