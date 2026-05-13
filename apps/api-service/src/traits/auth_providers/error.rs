#[derive(Debug, thiserror::Error)]
pub(crate) enum AuthMetadataError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error ({status}): {body}")]
    Api { status: u16, body: String },
    #[allow(dead_code)]
    #[error("User not found: {user_id}")]
    UserNotFound { user_id: String },
    #[allow(dead_code)]
    #[error("Configuration error: {0}")]
    Configuration(String),
}
