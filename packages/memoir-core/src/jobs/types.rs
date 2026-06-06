//! Domain types for the write-behind queue.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// Kind of work a `memory_jobs` row represents.
///
/// The set is closed by the CHECK constraint on the `memory_jobs.kind` column
/// (see migration `m20000000_000002_create_memory_jobs`); extending it
/// requires a follow-up migration.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::Display, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum JobKind {
    /// Embed a memory's content and upsert its vector. Used for both
    /// freshly-written episodic rows and freshly-extracted semantic rows.
    Embed,

    /// Run LLM extraction against an episodic memory.
    Extract,

    /// Run NLI categorization against a semantic memory (epic 0011).
    ///
    /// Enqueued per semantic row from the extract handler when a classifier
    /// is configured. Writes the `category` column; never touches confidence.
    Categorize,

    /// Re-derive semantic rows from a corrected episodic source (epic 0011).
    ///
    /// The correction engine: retires the source's derived semantic rows and
    /// re-runs extraction over the (corrected) source. Driven by feedback
    /// (reason `rejected`) and episodic edits (reason `stale`); the reason and
    /// any correction text ride the job payload.
    Reprocess,

    /// Derive relational triples from an episodic source (epic 0012).
    ///
    /// The graph derivation, parallel to [`JobKind::Extract`]: both fan out
    /// from the same episodic write. Enqueued only when the `knowledge-graph`
    /// feature is built; the variant itself is always present so a job row a
    /// graph-enabled build wrote still deserializes in a vector-only build.
    #[strum(serialize = "relational_extract")]
    #[serde(rename = "relational_extract")]
    RelationalExtract,
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
        assert_eq!(JobKind::Categorize.as_ref(), "categorize");
        assert_eq!(JobKind::Reprocess.as_ref(), "reprocess");
        assert_eq!(JobKind::RelationalExtract.as_ref(), "relational_extract");
    }

    #[test]
    fn should_display_job_kind_matching_as_ref() {
        assert_eq!(JobKind::Embed.to_string(), "embed");
        assert_eq!(JobKind::Extract.to_string(), "extract");
        assert_eq!(JobKind::Categorize.to_string(), "categorize");
        assert_eq!(JobKind::Reprocess.to_string(), "reprocess");
        assert_eq!(JobKind::RelationalExtract.to_string(), "relational_extract");
    }

    #[test]
    fn should_serialize_job_kind_as_lowercase_string() {
        assert_eq!(serde_json::to_string(&JobKind::Embed).unwrap(), "\"embed\"");
        assert_eq!(serde_json::to_string(&JobKind::Extract).unwrap(), "\"extract\"");
        assert_eq!(serde_json::to_string(&JobKind::Categorize).unwrap(), "\"categorize\"");
        assert_eq!(serde_json::to_string(&JobKind::Reprocess).unwrap(), "\"reprocess\"");
        assert_eq!(
            serde_json::to_string(&JobKind::RelationalExtract).unwrap(),
            "\"relational_extract\""
        );
    }

    #[test]
    fn should_deserialize_job_kind_from_lowercase_string() {
        assert_eq!(serde_json::from_str::<JobKind>("\"embed\"").unwrap(), JobKind::Embed);
        assert_eq!(
            serde_json::from_str::<JobKind>("\"extract\"").unwrap(),
            JobKind::Extract
        );
        assert_eq!(
            serde_json::from_str::<JobKind>("\"categorize\"").unwrap(),
            JobKind::Categorize
        );
        assert_eq!(
            serde_json::from_str::<JobKind>("\"reprocess\"").unwrap(),
            JobKind::Reprocess
        );
        assert_eq!(
            serde_json::from_str::<JobKind>("\"relational_extract\"").unwrap(),
            JobKind::RelationalExtract
        );
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
