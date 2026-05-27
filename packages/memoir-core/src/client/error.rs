use crate::embedding::EmbeddingError;
use crate::jobs::JobsError;
use crate::llm::LlmError;
use crate::store::StoreError;
use crate::vector::VectorError;

/// Failure modes for [`crate::client::Client`] construction and operations.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("embedding model failed: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("vector index failed: {0}")]
    Vector(#[from] VectorError),

    #[error("store failed: {0}")]
    Store(#[from] StoreError),

    #[error("jobs failed: {0}")]
    Jobs(#[from] JobsError),

    #[error("llm provider failed: {0}")]
    Llm(#[from] LlmError),

    #[error("migration failed: {0}")]
    Migration(#[from] crate::migration::MigrationError),

    #[error("database connection failed: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error(
        "metadata uses reserved key '{key}'; reserved keys are owned by memoir-core's payload schema and cannot be set via metadata"
    )]
    ReservedMetadataKey { key: String },
}

#[cfg(feature = "grpc")]
mod grpc {
    use tonic::{Code, Status};
    use tracing::Level;

    use crate::jobs::JobsError;
    use crate::memory::MemoryKind;
    use crate::store::StoreError;
    use crate::vector::VectorError;

    use super::ClientError;

    /// Server-facing classification of a [`ClientError`] for gRPC responses.
    ///
    /// Each variant decides its gRPC `Code`, the tracing level the error
    /// deserves, and the opaque message that goes on the wire. The actual
    /// `Display`-formatted error context stays server-side via the trace
    /// emitted in [`Status::from`] — never on the wire.
    struct Classification {
        code: Code,
        level: Level,
        kind: &'static str,
        message: String,
    }

    impl ClientError {
        fn classify(&self) -> Classification {
            match self {
                Self::Store(StoreError::NotFound(_)) => Classification {
                    code: Code::NotFound,
                    level: Level::DEBUG,
                    kind: "store.not_found",
                    message: "memory not found".into(),
                },
                Self::Store(StoreError::InvalidScope(_)) => Classification {
                    code: Code::InvalidArgument,
                    level: Level::WARN,
                    kind: "store.invalid_scope",
                    message: "scope: agent_id, org_id, and user_id must all be non-empty".into(),
                },
                Self::Store(StoreError::UnsupportedEdit { kind, .. }) => Classification {
                    code: Code::FailedPrecondition,
                    level: Level::WARN,
                    kind: "store.unsupported_edit",
                    message: format!("edit not supported for memory kind {}", MemoryKind::as_ref(kind)),
                },
                Self::Store(StoreError::Database(_)) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "store.database",
                    message: "internal error".into(),
                },
                Self::Store(StoreError::CacheInvariant(_)) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "store.cache_invariant",
                    message: "internal error".into(),
                },
                Self::Jobs(JobsError::NotFound(_)) => Classification {
                    code: Code::NotFound,
                    level: Level::DEBUG,
                    kind: "jobs.not_found",
                    message: "job not found".into(),
                },
                Self::Jobs(JobsError::Database(_)) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "jobs.database",
                    message: "internal error".into(),
                },
                Self::Vector(VectorError::NotFound(_)) => Classification {
                    code: Code::NotFound,
                    level: Level::DEBUG,
                    kind: "vector.not_found",
                    message: "vector index entry not found".into(),
                },
                Self::Vector(VectorError::BadRequest(_)) => Classification {
                    code: Code::InvalidArgument,
                    level: Level::WARN,
                    kind: "vector.bad_request",
                    message: "invalid request to vector backend".into(),
                },
                Self::Vector(VectorError::Connection(_)) => Classification {
                    code: Code::Unavailable,
                    level: Level::ERROR,
                    kind: "vector.connection",
                    message: "vector backend unavailable".into(),
                },
                Self::Database(_) => Classification {
                    code: Code::Unavailable,
                    level: Level::ERROR,
                    kind: "database",
                    message: "database unavailable".into(),
                },
                Self::Embedding(_) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "embedding",
                    message: "internal error".into(),
                },
                Self::Llm(_) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "llm",
                    message: "internal error".into(),
                },
                Self::Migration(_) => Classification {
                    code: Code::Internal,
                    level: Level::ERROR,
                    kind: "migration",
                    message: "internal error".into(),
                },
                Self::ReservedMetadataKey { key } => Classification {
                    code: Code::InvalidArgument,
                    level: Level::WARN,
                    kind: "client.reserved_metadata_key",
                    message: format!("metadata key '{key}' is reserved by memoir-core's payload schema"),
                },
            }
        }
    }

    impl From<ClientError> for Status {
        fn from(err: ClientError) -> Self {
            let Classification {
                code,
                level,
                kind,
                message,
            } = err.classify();
            match level {
                Level::ERROR => {
                    tracing::error!(error.kind = kind, error.detail = %err, "client error mapped to gRPC status")
                }
                Level::WARN => {
                    tracing::warn!(error.kind = kind, error.detail = %err, "client error mapped to gRPC status")
                }
                _ => tracing::debug!(error.kind = kind, error.detail = %err, "client error mapped to gRPC status"),
            }
            Status::new(code, message)
        }
    }
}
