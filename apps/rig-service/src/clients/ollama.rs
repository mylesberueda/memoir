use super::{FetchedModel, ModelDiscovery, ProviderError};
use proto_rs::rig::v1::{ModelCapabilities, ModelMetadata};
use serde::Deserialize;

const DEFAULT_BASE_URL: &str = "http://localhost:11434";

// ─────────────────────────────────────────────────────────────────────────────
// Ollama API Response Types
// ─────────────────────────────────────────────────────────────────────────────

/// Response from Ollama's /api/tags endpoint (list models).
#[derive(Debug, Deserialize)]
struct ListModelsResponse {
    models: Vec<OllamaModelSummary>,
}

/// Summary info from /api/tags - does NOT include capabilities.
#[derive(Debug, Deserialize)]
struct OllamaModelSummary {
    /// Model name (e.g., "llama3:latest")
    name: String,
    /// Size in bytes
    size: Option<i64>,
    /// Model details
    details: Option<OllamaModelDetails>,
}

/// Response from Ollama's /api/show endpoint (model details).
///
/// Reference: https://docs.ollama.com/api-reference/show-model-details
#[derive(Debug, Deserialize)]
struct ShowModelResponse {
    #[allow(dead_code)]
    /// Model details (family, quantization, etc.)
    details: Option<OllamaModelDetails>,
    /// Model capabilities: ["completion", "vision", "tools", "embed"]
    #[serde(default)]
    capabilities: Vec<String>,
}

/// Model details shared between /api/tags and /api/show.
#[derive(Debug, Clone, Deserialize)]
struct OllamaModelDetails {
    /// Model family (e.g., "llama", "qwen2")
    family: Option<String>,
    #[allow(dead_code)]
    /// Parameter size (e.g., "7B", "70B")
    parameter_size: Option<String>,
    /// Quantization level (e.g., "Q4_K_M")
    quantization_level: Option<String>,
}

/// Combined model info after fetching from both endpoints.
struct OllamaModel {
    name: String,
    size: Option<i64>,
    details: Option<OllamaModelDetails>,
    capabilities: Vec<String>,
}

impl OllamaModel {
    /// Creates from summary with capabilities fetched separately.
    fn from_summary_with_capabilities(summary: OllamaModelSummary, capabilities: Vec<String>) -> Self {
        Self {
            name: summary.name,
            size: summary.size,
            details: summary.details,
            capabilities,
        }
    }
}

impl From<&OllamaModel> for ModelCapabilities {
    fn from(model: &OllamaModel) -> Self {
        // Ollama capabilities (v0.6.4+): "completion", "vision", "tools", "thinking", "embedding", "insert", "image"
        let has = |cap: &str| model.capabilities.iter().any(|c| c == cap);

        Self {
            vision: has("vision"),
            function_calling: has("tools"),
            // JSON mode available on all completion models
            json_mode: has("completion"),
            // Streaming available on all completion models
            streaming: has("completion"),
            // System prompt available on all completion models
            system_prompt: has("completion"),
            // Multi-turn available on all completion models
            multi_turn: has("completion"),
            // Thinking capability for reasoning models (qwq, deepseek-r1, etc.)
            thinking: has("thinking"),
        }
    }
}

impl From<OllamaModel> for FetchedModel {
    fn from(model: OllamaModel) -> Self {
        let capabilities = ModelCapabilities::from(&model);

        Self {
            model_id: model.name.clone(),
            name: model.name,
            context_window: None, // Ollama doesn't expose this in API
            capabilities,
            metadata: ModelMetadata {
                size_bytes: model.size,
                quantization: model.details.as_ref().and_then(|d| d.quantization_level.clone()),
                family: model.details.as_ref().and_then(|d| d.family.clone()),
                ..Default::default()
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ModelDiscovery Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct Ollama;

impl ModelDiscovery for Ollama {
    async fn fetch_models(base_url: Option<&str>, _api_key: Option<&str>) -> Result<Vec<FetchedModel>, ProviderError> {
        let base = base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/');
        let client = reqwest::Client::new();

        // Step 1: List all models via /api/tags
        let list_url = format!("{}/api/tags", base);
        tracing::debug!(url = %list_url, "fetching model list from Ollama");

        let response = client.get(&list_url).send().await.map_err(|e| {
            tracing::error!(url = %list_url, error = %e, "Ollama request failed");
            e
        })?;

        if !response.status().is_success() {
            tracing::error!(url = %list_url, status = %response.status(), "Ollama API returned error status");
            return Err(ProviderError::InvalidResponse(format!(
                "Ollama API returned status {}",
                response.status()
            )));
        }

        let list_response: ListModelsResponse = response.json().await.map_err(|e| {
            tracing::error!(url = %list_url, error = %e, "failed to parse Ollama response");
            e
        })?;

        // Step 2: Fetch capabilities for each model via /api/show
        let mut fetched = Vec::with_capacity(list_response.models.len());

        for summary in list_response.models {
            let capabilities = fetch_model_capabilities(&client, base, &summary.name).await;
            let model = OllamaModel::from_summary_with_capabilities(summary, capabilities);
            fetched.push(FetchedModel::from(model));
        }

        tracing::debug!(url = %list_url, model_count = fetched.len(), "fetched models from Ollama");

        Ok(fetched)
    }
}

/// Fetches capabilities for a single model via /api/show.
///
/// Returns empty vec on error (graceful degradation).
async fn fetch_model_capabilities(client: &reqwest::Client, base_url: &str, model_name: &str) -> Vec<String> {
    let show_url = format!("{}/api/show", base_url);

    let response = client
        .post(&show_url)
        .json(&serde_json::json!({ "model": model_name }))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => match resp.json::<ShowModelResponse>().await {
            Ok(show) => show.capabilities,
            Err(e) => {
                tracing::warn!(model = %model_name, error = %e, "failed to parse /api/show response");
                Vec::new()
            }
        },
        Ok(resp) => {
            tracing::warn!(model = %model_name, status = %resp.status(), "failed to fetch model capabilities");
            Vec::new()
        }
        Err(e) => {
            tracing::warn!(model = %model_name, error = %e, "failed to fetch model capabilities");
            Vec::new()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;

    fn ollama_base_url() -> String {
        std::env::var("OLLAMA_BASE_URL").expect("OLLAMA_BASE_URL must be set")
    }

    #[tokio::test]
    async fn should_fetch_models_from_ollama() {
        let base_url = ollama_base_url();
        let models = Ollama::fetch_models(Some(&base_url), None).await.unwrap();

        assert!(!models.is_empty(), "Ollama should have at least one model pulled");
    }

    #[tokio::test]
    async fn should_return_model_id_and_name_for_each_model() {
        let base_url = ollama_base_url();
        let models = Ollama::fetch_models(Some(&base_url), None).await.unwrap();

        for model in &models {
            assert!(!model.model_id.is_empty(), "model_id should not be empty");
            assert!(!model.name.is_empty(), "name should not be empty");
        }
    }

    #[tokio::test]
    async fn should_return_capabilities_from_api() {
        let base_url = ollama_base_url();
        let models = Ollama::fetch_models(Some(&base_url), None).await.unwrap();

        // At least one model should have completion capability
        let has_completion = models.iter().any(|m| m.capabilities.streaming);
        assert!(has_completion, "at least one model should support completion/streaming");
    }

    #[tokio::test]
    async fn should_return_error_for_invalid_base_url() {
        let result = Ollama::fetch_models(Some("http://localhost:99999"), None).await;
        assert!(result.is_err(), "should fail for invalid URL");
    }
}
