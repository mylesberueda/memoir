use crate::agents::{MessageConversionError, error::AgentError};
use common_rs::crypto::CryptoError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ActorError {
    #[error("failed to load conversation: {0}")]
    ConversationLoadFailed(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("provider error: {0}")]
    Provider(#[from] tonic::Status),
    #[error("store error: {0}")]
    Store(#[from] crate::api::store::StoreError),
    #[error("failed to decrypt secret")]
    CryptoError(#[from] CryptoError),
    #[error("db error: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("conversion error: {0}")]
    ConversionError(#[from] MessageConversionError),
    #[error("agent error: {0}")]
    Agent(#[from] AgentError),
}

impl From<ActorError> for tonic::Status {
    fn from(error: ActorError) -> Self {
        match error {
            ActorError::ConversationLoadFailed(err) => tonic::Status::internal(err),
            ActorError::Unauthorized(err) => tonic::Status::permission_denied(err),
            ActorError::Provider(status) => status,
            ActorError::Store(store_error) => store_error.into(),
            ActorError::CryptoError(e) => tonic::Status::invalid_argument(e.to_string()),
            ActorError::DbError(err) => tonic::Status::internal(err.to_string()),
            ActorError::ConversionError(e) => tonic::Status::failed_precondition(e.to_string()),
            ActorError::Agent(e) => tonic::Status::internal(e.to_string()),
        }
    }
}
