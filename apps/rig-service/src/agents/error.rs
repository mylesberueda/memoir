use crate::api::message::Message;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AgentError {
    #[error("agent build error")]
    BuildError,

    #[error("missing {0}")]
    MissingRequired(String),

    #[expect(
        dead_code,
        reason = "Reserved for future transport-level disconnect handling in the new agent runtime"
    )]
    #[error("client disconnected")]
    ClientDisconnected,

    #[error("stream error: {0}")]
    StreamError(String),

    #[error("stream ended without final response")]
    StreamEndedWithoutFinalResponse,

    /// Inference was cancelled by user request. Contains the partial message built so far.
    #[error("inference cancelled")]
    Cancelled(Message),

    #[error("completion error: {0}")]
    CompletionError(String),

    #[error("inference timed out after {0}s")]
    Timeout(u32),

    /// Inference was idle (no stream activity) for too long. Contains the partial message built so far.
    #[error("inference idle timed out after {0}s of inactivity")]
    IdleTimeout(u32, Message),
}
