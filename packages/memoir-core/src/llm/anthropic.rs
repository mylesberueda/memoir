//! Factory for `rig::providers::anthropic::Client`.

use rig_core::providers::anthropic;

use super::LlmError;

/// Builds an Anthropic rig client.
pub(super) fn build_client(api_key: &str) -> Result<anthropic::Client, LlmError> {
    anthropic::Client::builder()
        .api_key(api_key.to_string())
        .build()
        .map_err(|err| LlmError::Connection(err.to_string()))
}
