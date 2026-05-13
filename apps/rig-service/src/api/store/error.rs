#[derive(Debug, thiserror::Error)]
pub(crate) enum StoreError {
    #[error("redis error: {0}")]
    Redis(#[from] fred::error::Error),
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<StoreError> for tonic::Status {
    fn from(error: StoreError) -> Self {
        tracing::error!(%error, "store operation failed");
        match error {
            StoreError::Redis(error) => tonic::Status::internal(error.to_string()),
            StoreError::Database(db_err) => tonic::Status::internal(db_err.to_string()),
            StoreError::Serialization(error) => tonic::Status::internal(error.to_string()),
            StoreError::Internal(error) => tonic::Status::internal(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Code;

    mod store_error_to_tonic_status {
        use super::*;

        #[test]
        fn should_convert_database_error_to_internal_status() {
            let error = StoreError::Database(sea_orm::DbErr::Custom("test error".into()));
            let status: tonic::Status = error.into();

            assert_eq!(status.code(), Code::Internal);
            assert!(status.message().contains("test error"));
        }

        #[test]
        fn should_convert_serialization_error_to_internal_status() {
            let json_err = serde_json::from_str::<String>("not valid json").unwrap_err();
            let error = StoreError::Serialization(json_err);
            let status: tonic::Status = error.into();

            assert_eq!(status.code(), Code::Internal);
        }

        #[test]
        fn should_convert_internal_error_to_internal_status() {
            let error = StoreError::Internal("something went wrong".into());
            let status: tonic::Status = error.into();

            assert_eq!(status.code(), Code::Internal);
            assert!(status.message().contains("something went wrong"));
        }
    }
}
