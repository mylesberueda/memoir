//! Per-call builder for [`Client::edit`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use chrono::{DateTime, FixedOffset};

use crate::jobs::MemoryJobsStore;
use crate::memory::Memory;
use crate::store::{EditPatch, IndexStatus, MemoryStore};

use super::{Client, ClientError};

/// Per-call builder returned by [`Client::edit`].
///
/// Awaiting the builder applies the patch, flips the row's `qdrant_status`
/// back to `pending`, and enqueues a re-embed job so the row drops out of
/// search until the worker re-upserts it. Only fields the caller touches
/// are written; untouched fields keep their existing values. The patch is
/// no-op when nothing was set — awaiting the builder still works and
/// returns the current row unchanged.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # async fn example(client: &Client, pid: &str) -> Result<(), Box<dyn std::error::Error>> {
/// let corrected = client
///     .edit(pid)
///     .content("the user said hello, not goodbye")
///     .await?;
/// println!("updated_at = {}", corrected.updated_at);
/// # Ok(())
/// # }
/// ```
#[must_use = "edit(..) returns a builder that must be awaited"]
pub struct EditBuilder<'a> {
    client: &'a Client,
    pid: String,
    content: Option<String>,
    metadata: Option<serde_json::Value>,
    event_at: Option<Option<DateTime<FixedOffset>>>,
}

impl<'a> EditBuilder<'a> {
    pub(super) fn new(client: &'a Client, pid: String) -> Self {
        Self {
            client,
            pid,
            content: None,
            metadata: None,
            event_at: None,
        }
    }

    /// Overwrites the memory's content.
    ///
    /// Replaces the existing text wholesale; the old content is not preserved.
    /// Triggers re-embedding because the vector is no longer representative
    /// of the new text.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Overwrites the memory's metadata blob.
    ///
    /// Replaces the existing JSON wholesale; partial-merge is not supported
    /// at this layer. Reserved payload keys (`pid`, scope fields, `kind`,
    /// `created_at`, `event_at`) are rejected at execute time, matching
    /// [`Client::remember`].
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets the memory's event-time.
    ///
    /// The wrapping `Option` on the builder field distinguishes "leave
    /// untouched" from "explicitly clear" — this builder method only
    /// covers the "set to a concrete value" case. Clearing event_at via
    /// edit() is not exposed today; consumers who need to clear an
    /// event-time should re-`remember()` from scratch.
    pub fn event_at(mut self, event_at: impl Into<DateTime<FixedOffset>>) -> Self {
        self.event_at = Some(Some(event_at.into()));
        self
    }
}

impl<'a> IntoFuture for EditBuilder<'a> {
    type Output = Result<Memory, ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: EditBuilder<'_>) -> Result<Memory, ClientError> {
    let EditBuilder {
        client,
        pid,
        content,
        metadata,
        event_at,
    } = builder;

    if let Some(obj) = metadata.as_ref().and_then(|m| m.as_object()) {
        for key in obj.keys() {
            if crate::vector::qdrant::RESERVED_PAYLOAD_KEYS
                .iter()
                .any(|reserved| reserved == key)
            {
                return Err(ClientError::ReservedMetadataKey { key: key.clone() });
            }
        }
    }

    let content_changed = content.is_some();
    let patch = EditPatch {
        content,
        metadata,
        event_at,
    };

    let inner = client.inner.clone();
    let updated = inner.store.edit(&pid, patch).await?;

    if content_changed {
        inner.store.set_index_status(&pid, IndexStatus::Pending).await?;
        inner
            .jobs
            .enqueue(
                crate::jobs::JobKind::Embed,
                pid.clone(),
                serde_json::json!({ "origin": "edit" }),
            )
            .await?;
    }

    Ok(updated)
}
