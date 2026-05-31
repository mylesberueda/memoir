//! Factory for `rig::providers::ollama::Client`.

use rig_core::client::Nothing;
use rig_core::providers::ollama;

use super::LlmError;

/// Builds an Ollama rig client pointed at `base_url`.
///
/// Ollama doesn't use API keys, so the rig builder takes the `Nothing`
/// marker. The daemon itself is not contacted until inference runs; this
/// call only validates that the URL parses.
pub(super) fn build_client(base_url: &str) -> Result<ollama::Client, LlmError> {
    ollama::Client::builder()
        .api_key(Nothing)
        .base_url(base_url)
        .build()
        .map_err(|err| LlmError::Connection(err.to_string()))
}
