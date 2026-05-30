/// Failure modes returned by [`crate::jobs::MemoryJobsStore`] methods.
///
/// `NotFound` indicates a logic mismatch (e.g. completing a job another
/// worker holds); `Database` wraps an upstream sea-orm error.
#[derive(Debug, thiserror::Error)]
pub enum JobsError {
    #[error("job not found: {0}")]
    NotFound(String),

    #[error("database error: {0}")]
    Database(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_not_found_with_id() {
        let err = JobsError::NotFound("42".to_string());
        assert_eq!(err.to_string(), "job not found: 42");
    }

    #[test]
    fn should_render_database_error_with_message() {
        let err = JobsError::Database("connection refused".to_string());
        assert_eq!(err.to_string(), "database error: connection refused");
    }
}
