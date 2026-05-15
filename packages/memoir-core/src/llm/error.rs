/// Failure modes for [`crate::llm::LlmProvider`] implementations.
///
/// Parser-level failures (malformed JSON in the LLM's reply, missing required
/// fields) live on `LlmProvider`'s consumers (ticket 0005's
/// `parse_extraction`), not here — this enum covers transport and provider
/// errors only.
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("llm provider connection failed: {0}")]
    Connection(String),

    #[error("llm provider error: {0}")]
    Provider(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_connection_error_with_message() {
        let err = LlmError::Connection("dial refused".to_string());
        assert_eq!(err.to_string(), "llm provider connection failed: dial refused");
    }

    #[test]
    fn should_render_provider_error_with_message() {
        let err = LlmError::Provider("model not found".to_string());
        assert_eq!(err.to_string(), "llm provider error: model not found");
    }
}
