pub(crate) mod api_service;
mod gemini;
pub(crate) mod notification;
mod ollama;
mod openai;
pub(crate) mod qdrant;
pub(crate) mod storage;

pub(crate) use api_service::ApiServiceClient;
pub(crate) use notification::NotificationClient;
pub(crate) use qdrant::QdrantClient;
pub(crate) use storage::StorageClient;

use crate::models::providers::ProviderKind;
use proto_rs::rig::v1::{ModelCapabilities, ModelMetadata};

/// A model fetched from a provider API.
#[derive(Debug, Clone)]
pub(crate) struct FetchedModel {
    pub model_id: String,
    pub name: String,
    pub context_window: Option<i32>,
    pub capabilities: ModelCapabilities,
    pub metadata: ModelMetadata,
}

/// Error from provider operations.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ProviderError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// Trait for provider-specific operations.
pub(crate) trait ModelDiscovery {
    /// Fetch available models from the provider.
    fn fetch_models(
        base_url: Option<&str>,
        api_key: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<FetchedModel>, ProviderError>> + Send;
}

/// Fetch models for a provider by kind.
pub(crate) async fn fetch_models(
    kind: ProviderKind,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<Vec<FetchedModel>, ProviderError> {
    match kind {
        ProviderKind::Ollama => ollama::Ollama::fetch_models(base_url, api_key).await,
        ProviderKind::Openai => openai::Openai::fetch_models(base_url, api_key).await,
        ProviderKind::Gemini => gemini::Gemini::fetch_models(base_url, api_key).await,
    }
}
