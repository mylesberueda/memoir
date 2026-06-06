// Rust guideline compliant 2026-02-21

/// Failure modes for [`crate::graph::GraphStore`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("graph backend connection failed: {0}")]
    Connection(String),

    #[error("graph query failed: {0}")]
    Query(String),

    #[error("invalid request to graph backend: {0}")]
    BadRequest(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_connection_error_with_message() {
        let err = GraphError::Connection("dial failed".to_string());
        assert_eq!(err.to_string(), "graph backend connection failed: dial failed");
    }
}
