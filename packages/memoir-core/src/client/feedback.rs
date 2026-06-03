//! Per-call builder for [`Client::feedback`].

use std::future::{Future, IntoFuture};
use std::pin::Pin;

use crate::jobs::{JobKind, MemoryJobsStore};
use crate::memory::{MemoryKind, RetirementReason};
use crate::store::MemoryStore;

use super::{Client, ClientError};

/// Per-call builder returned by [`Client::feedback`].
///
/// Awaiting the builder enqueues a reprocess of the wrong fact's episodic
/// source: the derived semantic rows are retired as `rejected` and re-derived
/// with the correction in context (epic 0011). The user corrects a wrong
/// *extraction* by teaching, never by hand-writing a semantic row — semantic
/// memory stays always-derived. The target is the wrong semantic pid the user
/// saw in recall; memoir resolves its episodic source to anchor the reprocess.
///
/// Fire-and-forget: the call returns once the reprocess job is enqueued, like
/// every other write behind the worker queue. Re-`recall` later to see the
/// corrected rows.
///
/// To correct the *episodic* record itself (the verbatim source was wrong, or
/// the world changed) use [`Client::edit`] — that is a different correction
/// (the `stale` path), not a wrong extraction.
///
/// # Examples
///
/// ```no_run
/// # use memoir_core::client::Client;
/// # async fn example(client: &Client, wrong_pid: &str) -> Result<(), Box<dyn std::error::Error>> {
/// client
///     .feedback(wrong_pid)
///     .correction("green is actually my favorite color")
///     .await?;
/// # Ok(())
/// # }
/// ```
#[must_use = "feedback(..) returns a builder that must be awaited"]
pub struct FeedbackBuilder<'a> {
    client: &'a Client,
    pid: String,
    correction: Option<String>,
}

impl<'a> FeedbackBuilder<'a> {
    pub(super) fn new(client: &'a Client, pid: String) -> Self {
        Self {
            client,
            pid,
            correction: None,
        }
    }

    /// Sets the correction text the user supplied.
    ///
    /// Woven into the re-extraction prompt so the model fixes its reasoning
    /// rather than re-deriving the same wrong fact. Optional: without it the
    /// source is re-extracted blind, which is rarely useful — supply it.
    pub fn correction(mut self, correction: impl Into<String>) -> Self {
        self.correction = Some(correction.into());
        self
    }
}

impl<'a> IntoFuture for FeedbackBuilder<'a> {
    type Output = Result<(), ClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(execute(self))
    }
}

async fn execute(builder: FeedbackBuilder<'_>) -> Result<(), ClientError> {
    let FeedbackBuilder {
        client,
        pid,
        correction,
    } = builder;

    let inner = client.inner.clone();

    // Resolve the wrong fact and validate it is a correctable extraction: a
    // semantic row with an episodic source. The reprocess job is anchored on
    // that source, not on the semantic pid.
    let target = inner.store.recall(&pid).await?;
    if target.kind != MemoryKind::Semantic {
        return Err(ClientError::NotCorrectable {
            pid,
            reason: "feedback corrects a wrong extraction; the target must be a semantic memory (edit the episodic source instead)".to_string(),
        });
    }
    let source_pid = target.source_pid.ok_or_else(|| ClientError::NotCorrectable {
        pid: pid.clone(),
        reason: "semantic memory has no episodic source to reprocess".to_string(),
    })?;

    let mut payload = serde_json::json!({ "reason": RetirementReason::Rejected.as_ref() });
    if let Some(correction) = correction {
        payload["feedback"] = serde_json::Value::String(correction);
    }

    inner.jobs.enqueue(JobKind::Reprocess, source_pid, payload).await?;

    Ok(())
}
