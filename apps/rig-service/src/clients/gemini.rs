use super::{FetchedModel, ModelDiscovery, ProviderError};
use proto_rs::rig::v1::{ModelCapabilities, ModelMetadata};
use serde::Deserialize;

const DEFAULT_BASE_URL: &str = "https://generativelanguage.googleapis.com";

// ─────────────────────────────────────────────────────────────────────────────
// Gemini API Response Types
// ─────────────────────────────────────────────────────────────────────────────

/// Response from Gemini's list models endpoint.
#[derive(Debug, Deserialize)]
struct ListModelsResponse {
    models: Vec<GeminiModel>,
}

/// A model from the Gemini API.
///
/// Reference: https://ai.google.dev/api/models
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiModel {
    /// Model resource name (e.g., "models/gemini-1.5-flash")
    name: String,
    /// Human-readable display name
    display_name: Option<String>,
    /// Supported generation methods (e.g., ["generateContent", "streamGenerateContent"])
    #[serde(default)]
    supported_generation_methods: Vec<String>,
    /// Maximum input tokens
    input_token_limit: Option<i64>,
    /// Maximum output tokens
    output_token_limit: Option<i64>,
    /// Whether the model supports extended thinking/reasoning
    #[serde(default)]
    thinking: bool,
}

impl GeminiModel {
    /// Returns the model ID without the "models/" prefix.
    fn model_id(&self) -> &str {
        self.name.strip_prefix("models/").unwrap_or(&self.name)
    }

    /// Returns true if this is a generative model (not an embedding model).
    fn is_generative(&self) -> bool {
        self.supported_generation_methods.iter().any(|m| m == "generateContent")
    }

    /// Infers the model family from the model ID.
    fn family(&self) -> Option<String> {
        let model_id = self.model_id();
        if model_id.contains("gemini-2.5") {
            Some("gemini-2.5".to_string())
        } else if model_id.contains("gemini-2.0") {
            Some("gemini-2.0".to_string())
        } else if model_id.contains("gemini-1.5") {
            Some("gemini-1.5".to_string())
        } else if model_id.contains("gemini-1.0") {
            Some("gemini-1.0".to_string())
        } else if model_id.starts_with("gemini") {
            Some("gemini".to_string())
        } else {
            None
        }
    }
}

impl From<GeminiModel> for ModelCapabilities {
    fn from(model: GeminiModel) -> Self {
        let supports_generate = model
            .supported_generation_methods
            .contains(&"generateContent".to_string());
        let supports_streaming = model
            .supported_generation_methods
            .contains(&"streamGenerateContent".to_string());

        Self {
            // All generative Gemini models support multimodal input
            vision: supports_generate,
            // Function calling available on generative models
            function_calling: supports_generate,
            // JSON mode available on generative models
            json_mode: supports_generate,
            // Streaming from API response
            streaming: supports_streaming,
            // All Gemini models support system prompts
            system_prompt: true,
            // All Gemini models support multi-turn
            multi_turn: true,
            // Thinking directly from API response
            thinking: model.thinking,
        }
    }
}

impl From<GeminiModel> for FetchedModel {
    fn from(model: GeminiModel) -> Self {
        let model_id = model.model_id().to_string();
        let name = model.display_name.clone().unwrap_or_else(|| model_id.clone());
        let context_window = model.input_token_limit.map(|v| v as i32);
        let max_output_tokens = model.output_token_limit;
        let family = model.family();
        let capabilities = ModelCapabilities::from(model);

        Self {
            model_id,
            name,
            context_window,
            capabilities,
            metadata: ModelMetadata {
                family,
                max_output_tokens,
                ..Default::default()
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ModelDiscovery Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct Gemini;

impl ModelDiscovery for Gemini {
    async fn fetch_models(base_url: Option<&str>, api_key: Option<&str>) -> Result<Vec<FetchedModel>, ProviderError> {
        let base = base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/');
        let api_key =
            api_key.ok_or_else(|| ProviderError::InvalidResponse("Gemini requires an API key".to_string()))?;

        let url = format!("{}/v1beta/models?key={}", base, api_key);
        let response = reqwest::get(&url).await?;

        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(format!(
                "Gemini API returned status {}",
                response.status()
            )));
        }

        let body: ListModelsResponse = response.json().await?;

        let fetched = body
            .models
            .into_iter()
            .filter(|m| m.is_generative())
            .map(FetchedModel::from)
            .collect();

        Ok(fetched)
    }
}
