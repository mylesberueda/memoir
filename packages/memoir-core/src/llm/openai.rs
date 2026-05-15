//! Factory for `rig::providers::openai::Client`.

use rig::providers::openai;

use super::LlmError;

/// Builds an OpenAI rig client.
///
/// When `base_url` is `Some`, the client points at that endpoint instead of
/// the default `https://api.openai.com`. Useful for enterprise proxies and
/// self-hosted OpenAI-compatible servers.
pub(super) fn build_client(api_key: &str, base_url: Option<&str>) -> Result<openai::Client, LlmError> {
    let mut builder = openai::Client::builder().api_key(api_key);
    if let Some(url) = base_url {
        builder = builder.base_url(url);
    }
    builder
        .build()
        .map_err(|err| LlmError::Connection(err.to_string()))
}
