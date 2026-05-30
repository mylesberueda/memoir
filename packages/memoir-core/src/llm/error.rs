/// Failure modes for [`crate::llm::LlmProvider`] and its consumers.
///
/// `Connection` / `Provider` cover transport and server-side errors;
/// `Parse` covers failures turning an LLM's raw text reply into a structured
/// type (the extraction parser is the canonical user).
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("llm provider connection failed: {0}")]
    Connection(String),

    #[error("llm provider error: {0}")]
    Provider(String),

    #[error("llm output parse failed: {0}")]
    Parse(String),
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

    #[test]
    fn should_render_parse_error_with_message() {
        let err = LlmError::Parse("invalid json at len=512".to_string());
        assert_eq!(err.to_string(), "llm output parse failed: invalid json at len=512");
    }
}
