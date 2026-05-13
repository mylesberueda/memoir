#[derive(Debug, thiserror::Error)]
pub(crate) enum AgentServiceError {
    #[error("db error")]
    DbError(#[from] sea_orm::DbErr),
    #[error("model not found")]
    ModelNotFound,
    #[error("provider not found")]
    ProviderNotFound,
}

impl From<AgentServiceError> for tonic::Status {
    fn from(error: AgentServiceError) -> Self {
        match error {
            AgentServiceError::DbError(db_err) => tonic::Status::internal(db_err.to_string()),
            AgentServiceError::ModelNotFound => tonic::Status::not_found("Model not found."),
            AgentServiceError::ProviderNotFound => tonic::Status::not_found("Provider not found."),
        }
    }
}
