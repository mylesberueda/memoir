#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error("Terminal I/O error: {0}")]
    Io(#[from] std::io::Error),
}
