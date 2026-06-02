//! Domain types for the write-behind queue.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Kind of work a `memory_jobs` row represents.
///
/// The set is closed by the CHECK constraint on the `memory_jobs.kind` column
/// (see migration `m20000000_000002_create_memory_jobs`); extending it
/// requires a follow-up migration.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    strum::AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum JobKind {
    /// Embed a memory's content and upsert its vector. Used for both
    /// freshly-written episodic rows and freshly-extracted semantic rows.
    Embed,

    /// Run LLM extraction against an episodic memory.
    Extract,
}

/// Lifecycle state of a `memory_jobs` row.
///
/// Mirrors the CHECK constraint on the `memory_jobs.state` column. The
/// `done` state is not represented — completion deletes the row rather than
/// transitioning it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum JobState {
    /// Created, awaiting claim.
    Pending,

    /// Currently leased to a worker. Recovered by lease expiry if the worker
    /// dies before completing.
    Claimed,

    /// Terminal-but-replayable failure. The admin surface (ticket 0008) lets
    /// operators retry or discard.
    Failed,
}

/// A row of the `memory_jobs` table, hydrated for handler consumption.
#[derive(Debug, Clone)]
pub struct Job {
    pub id: i64,
    pub source_pid: String,
    pub kind: JobKind,
    pub state: JobState,
    pub payload: serde_json::Value,
    pub attempts: i32,
    pub failure_reason: Option<String>,
    pub claimed_at: Option<DateTime<FixedOffset>>,
    pub claimed_by: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

/// A failed-state `memory_jobs` row exposed to admin callers.
///
/// Deliberately excludes the original `payload` and any content from the
/// referenced memory: operators triaging a failure see the metadata they
/// need (kind, source pid, attempt count, failure reason, last update) but
/// not user content, which avoids leaking PII into operator dashboards.
/// Operators who need the underlying memory's content can resolve it
/// separately via [`crate::client::Client::recall`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedJob {
    pub id: i64,
    pub source_pid: String,
    pub kind: JobKind,
    pub attempts: i32,
    pub failure_reason: Option<String>,
    pub updated_at: DateTime<FixedOffset>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_job_kind_as_lowercase_string() {
        assert_eq!(JobKind::Embed.as_ref(), "embed");
        assert_eq!(JobKind::Extract.as_ref(), "extract");
    }

    #[test]
    fn should_display_job_kind_matching_as_ref() {
        assert_eq!(JobKind::Embed.to_string(), "embed");
        assert_eq!(JobKind::Extract.to_string(), "extract");
    }

    #[test]
    fn should_serialize_job_kind_as_lowercase_string() {
        assert_eq!(serde_json::to_string(&JobKind::Embed).unwrap(), "\"embed\"");
        assert_eq!(serde_json::to_string(&JobKind::Extract).unwrap(), "\"extract\"");
    }

    #[test]
    fn should_deserialize_job_kind_from_lowercase_string() {
        assert_eq!(serde_json::from_str::<JobKind>("\"embed\"").unwrap(), JobKind::Embed);
        assert_eq!(serde_json::from_str::<JobKind>("\"extract\"").unwrap(), JobKind::Extract);
        assert!(serde_json::from_str::<JobKind>("\"nonsense\"").is_err());
    }

    #[test]
    fn should_render_job_state_as_lowercase_string() {
        assert_eq!(JobState::Pending.as_ref(), "pending");
        assert_eq!(JobState::Claimed.as_ref(), "claimed");
        assert_eq!(JobState::Failed.as_ref(), "failed");
    }

    #[test]
    fn should_display_job_state_matching_as_ref() {
        assert_eq!(JobState::Pending.to_string(), "pending");
    }
}
