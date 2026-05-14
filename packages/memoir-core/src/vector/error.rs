/// Failure modes for [`crate::vector::VectorIndex`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum VectorError {
    #[error("vector backend connection failed: {0}")]
    Connection(String),

    #[error("invalid request to vector backend: {0}")]
    BadRequest(String),

    #[error("vector backend resource not found: {0}")]
    NotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_connection_error_with_message() {
        let err = VectorError::Connection("dial failed".to_string());
        assert_eq!(err.to_string(), "vector backend connection failed: dial failed");
    }
}
