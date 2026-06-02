//! Newtype wrappers bridging library types and generated wire types.
//!
//! The orphan rule forbids `impl From<LibType> for ProtoType` in this crate
//! (both are foreign). Wrapping the wire type in a crate-local newtype makes
//! the impl legal: the wrapper is local, so `From` / `TryFrom` apply. Each
//! wrapper `Deref`s to its inner proto type, so handlers read the converted
//! value with no `.0` noise.
//!
//! Only the wrappers shared across multiple RPC handlers live here
//! (`WireMemory`, `WireError`). Per-RPC request/response conversions
//! co-locate with their handler. The remaining free-function conversions in
//! `conversions.rs` migrate to this pattern under ticket 0015.

use std::ops::Deref;

use memoir_core::client::ClientError;
use memoir_core::memory::{Memory as LibMemory, MemoryKind as LibMemoryKind};
use memoir_core::store::IndexStatus;
use memoir_sdk::memoir::v1::{
    Memory as ProtoMemory, MemoryKind as ProtoMemoryKind, MemoryStatus, Supersession as ProtoSupersession,
};
use tonic::Status;

use super::conversions::{metadata_to_proto, scope_to_proto, timestamp_from_chrono};

/// Wire form of a [`LibMemory`]. Build via `WireMemory::from(memory)`.
pub(crate) struct WireMemory(pub ProtoMemory);

impl Deref for WireMemory {
    type Target = ProtoMemory;

    fn deref(&self) -> &ProtoMemory {
        &self.0
    }
}

impl From<LibMemory> for WireMemory {
    fn from(memory: LibMemory) -> Self {
        Self(ProtoMemory {
            pid: memory.pid,
            scope: Some(scope_to_proto(memory.scope)),
            content: memory.content,
            metadata: Some(metadata_to_proto(memory.metadata)),
            created_at: Some(timestamp_from_chrono(memory.created_at)),
            processed_at: None,
            status: match memory.status {
                IndexStatus::Pending => MemoryStatus::Pending as i32,
                IndexStatus::Indexed => MemoryStatus::Processed as i32,
                IndexStatus::Failed => MemoryStatus::Failed as i32,
            },
            updated_at: Some(timestamp_from_chrono(memory.updated_at)),
            event_at: memory.event_at.map(timestamp_from_chrono),
            supersession: memory.supersession.map(|s| ProtoSupersession {
                winner_pid: s.winner_pid,
                at: Some(timestamp_from_chrono(s.at)),
            }),
            kind: match memory.kind {
                LibMemoryKind::Episodic => ProtoMemoryKind::Episodic as i32,
                LibMemoryKind::Semantic => ProtoMemoryKind::Semantic as i32,
            },
        })
    }
}

/// Wire form of a [`ClientError`]. Build via `WireError::from(err)` then
/// `.into()` for a [`Status`], or use `.map_err(WireError::into_status)`.
///
/// Replaces the `impl From<ClientError> for Status` that memoir-core carried
/// behind its `grpc` feature — moving the mapping here lets that feature
/// (and core's optional `tonic` dependency) be removed under ticket 0015.
pub(crate) struct WireError(pub ClientError);

impl WireError {
    /// Maps a [`ClientError`] directly to a [`Status`] for `.map_err`.
    pub(crate) fn into_status(err: ClientError) -> Status {
        Self(err).into()
    }
}

impl From<WireError> for Status {
    fn from(err: WireError) -> Self {
        use memoir_core::jobs::JobsError;
        use memoir_core::memory::MemoryKind;
        use memoir_core::store::StoreError;
        use memoir_core::vector::VectorError;
        use tonic::Code;

        let (code, kind, message): (Code, &str, String) = match &err.0 {
            ClientError::Store(StoreError::NotFound(_)) => {
                (Code::NotFound, "store.not_found", "memory not found".into())
            }
            ClientError::Store(StoreError::InvalidScope(_)) => (
                Code::InvalidArgument,
                "store.invalid_scope",
                "scope: agent_id, org_id, and user_id must all be non-empty".into(),
            ),
            ClientError::Store(StoreError::UnsupportedEdit { kind, .. }) => (
                Code::FailedPrecondition,
                "store.unsupported_edit",
                format!("edit not supported for memory kind {}", MemoryKind::as_ref(kind)),
            ),
            ClientError::Store(StoreError::Database(_)) => (Code::Internal, "store.database", "internal error".into()),
            ClientError::Store(StoreError::CacheInvariant(_)) => {
                (Code::Internal, "store.cache_invariant", "internal error".into())
            }
            ClientError::Jobs(JobsError::NotFound(_)) => (Code::NotFound, "jobs.not_found", "job not found".into()),
            ClientError::Jobs(JobsError::Database(_)) => (Code::Internal, "jobs.database", "internal error".into()),
            ClientError::Vector(VectorError::NotFound(_)) => (
                Code::NotFound,
                "vector.not_found",
                "vector index entry not found".into(),
            ),
            ClientError::Vector(VectorError::BadRequest(_)) => (
                Code::InvalidArgument,
                "vector.bad_request",
                "invalid request to vector backend".into(),
            ),
            ClientError::Vector(VectorError::Connection(_)) => (
                Code::Unavailable,
                "vector.connection",
                "vector backend unavailable".into(),
            ),
            ClientError::Database(_) => (Code::Unavailable, "database", "database unavailable".into()),
            ClientError::Embedding(_) => (Code::Internal, "embedding", "internal error".into()),
            ClientError::Llm(_) => (Code::Internal, "llm", "internal error".into()),
            ClientError::Migration(_) => (Code::Internal, "migration", "internal error".into()),
            ClientError::ReservedMetadataKey { key } => (
                Code::InvalidArgument,
                "client.reserved_metadata_key",
                format!("metadata key '{key}' is reserved by memoir-core's payload schema"),
            ),
        };

        match code {
            Code::Internal | Code::Unavailable => {
                tracing::error!(error.kind = kind, error.detail = %err.0, "client error mapped to gRPC status")
            }
            Code::InvalidArgument | Code::FailedPrecondition => {
                tracing::warn!(error.kind = kind, error.detail = %err.0, "client error mapped to gRPC status")
            }
            _ => tracing::debug!(error.kind = kind, error.detail = %err.0, "client error mapped to gRPC status"),
        }

        Status::new(code, message)
    }
}
