//! Domain types for the write-behind queue.

use chrono::{DateTime, FixedOffset};

/// Kind of work a `memory_jobs` row represents.
///
/// The set is closed by the CHECK constraint on the `memory_jobs.kind` column
/// (see migration `m20000000_000002_create_memory_jobs`); extending it
/// requires a follow-up migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobKind {
    /// Embed a memory's content and upsert its vector. Used for both
    /// freshly-written episodic rows and freshly-extracted semantic rows.
    Embed,

    /// Run LLM extraction against an episodic memory.
    Extract,
}

impl JobKind {
    /// Returns the canonical lowercase string used in storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Embed => "embed",
            Self::Extract => "extract",
        }
    }
}

impl std::fmt::Display for JobKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Lifecycle state of a `memory_jobs` row.
///
/// Mirrors the CHECK constraint on the `memory_jobs.state` column. The
/// `done` state is not represented — completion deletes the row rather than
/// transitioning it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl JobState {
    /// Returns the canonical lowercase string used in storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Claimed => "claimed",
            Self::Failed => "failed",
        }
    }
}

impl std::fmt::Display for JobState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_job_kind_as_lowercase_string() {
        assert_eq!(JobKind::Embed.as_str(), "embed");
        assert_eq!(JobKind::Extract.as_str(), "extract");
    }

    #[test]
    fn should_display_job_kind_matching_as_str() {
        assert_eq!(JobKind::Embed.to_string(), "embed");
        assert_eq!(JobKind::Extract.to_string(), "extract");
    }

    #[test]
    fn should_render_job_state_as_lowercase_string() {
        assert_eq!(JobState::Pending.as_str(), "pending");
        assert_eq!(JobState::Claimed.as_str(), "claimed");
        assert_eq!(JobState::Failed.as_str(), "failed");
    }

    #[test]
    fn should_display_job_state_matching_as_str() {
        assert_eq!(JobState::Pending.to_string(), "pending");
    }
}
