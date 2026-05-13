use super::{FetchedModel, ModelDiscovery, ProviderError};
use proto_rs::rig::v1::{ModelCapabilities, ModelMetadata};
use serde::Deserialize;

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

// ─────────────────────────────────────────────────────────────────────────────
// OpenAI API Response Types
// ─────────────────────────────────────────────────────────────────────────────

/// Response from OpenAI's /v1/models endpoint.
#[derive(Debug, Deserialize)]
struct ListModelsResponse {
    data: Vec<OpenAiModel>,
}

/// A model from the OpenAI API.
///
/// Note: OpenAI's API is frustratingly minimal - it only returns id, object,
/// created, and owned_by. No capability information whatsoever.
///
/// Reference: https://platform.openai.com/docs/api-reference/models/list
#[derive(Debug, Deserialize)]
struct OpenAiModel {
    /// Model identifier (e.g., "gpt-4o", "o1-preview")
    id: String,
    /// Organization that owns the model
    owned_by: Option<String>,
}

/// Model family classification for capability inference.
///
/// Since OpenAI doesn't expose capabilities in their API, we must infer them
/// from model IDs. This enum represents known model families with distinct
/// capability profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelFamily {
    /// o1, o3 series - reasoning models with thinking capability
    Reasoning,
    /// gpt-4o series - multimodal with vision
    Gpt4o,
    /// gpt-4 series (non-o) - text-only, function calling
    Gpt4,
    /// gpt-3.5-turbo series
    Gpt35Turbo,
    /// Other/unknown models
    Other,
}

impl ModelFamily {
    /// Infers the model family from the model ID.
    fn from_model_id(model_id: &str) -> Self {
        // Order matters: more specific patterns first
        if model_id.starts_with("o1") || model_id.starts_with("o3") {
            Self::Reasoning
        } else if model_id.starts_with("gpt-4o") {
            Self::Gpt4o
        } else if model_id.starts_with("gpt-4") {
            Self::Gpt4
        } else if model_id.starts_with("gpt-3.5-turbo") {
            Self::Gpt35Turbo
        } else {
            Self::Other
        }
    }

    /// Returns human-readable family name for metadata.
    fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Reasoning => Some("reasoning"),
            Self::Gpt4o => Some("gpt-4o"),
            Self::Gpt4 => Some("gpt-4"),
            Self::Gpt35Turbo => Some("gpt-3.5-turbo"),
            Self::Other => None,
        }
    }
}

impl From<ModelFamily> for ModelCapabilities {
    fn from(family: ModelFamily) -> Self {
        match family {
            // Reasoning models (o1, o3): thinking enabled, but limited features
            // - No streaming support
            // - No system prompt support (use developer messages instead)
            // - No function calling
            ModelFamily::Reasoning => Self {
                vision: false,
                function_calling: false,
                json_mode: true,
                streaming: false,
                system_prompt: false,
                multi_turn: true,
                thinking: true,
            },
            // GPT-4o: Full-featured multimodal
            ModelFamily::Gpt4o => Self {
                vision: true,
                function_calling: true,
                json_mode: true,
                streaming: true,
                system_prompt: true,
                multi_turn: true,
                thinking: false,
            },
            // GPT-4: Text-only but full-featured
            ModelFamily::Gpt4 => Self {
                vision: false,
                function_calling: true,
                json_mode: true,
                streaming: true,
                system_prompt: true,
                multi_turn: true,
                thinking: false,
            },
            // GPT-3.5-turbo: Older but capable
            ModelFamily::Gpt35Turbo => Self {
                vision: false,
                function_calling: true,
                json_mode: true,
                streaming: true,
                system_prompt: true,
                multi_turn: true,
                thinking: false,
            },
            // Unknown: Conservative defaults
            ModelFamily::Other => Self {
                vision: false,
                function_calling: false,
                json_mode: false,
                streaming: true,
                system_prompt: true,
                multi_turn: true,
                thinking: false,
            },
        }
    }
}

impl From<OpenAiModel> for FetchedModel {
    fn from(model: OpenAiModel) -> Self {
        let family = ModelFamily::from_model_id(&model.id);
        let capabilities = ModelCapabilities::from(family);

        Self {
            model_id: model.id.clone(),
            name: model.id,
            context_window: None, // OpenAI doesn't return this in list
            capabilities,
            metadata: ModelMetadata {
                owned_by: model.owned_by,
                family: family.as_str().map(String::from),
                ..Default::default()
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ModelDiscovery Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct Openai;

impl ModelDiscovery for Openai {
    async fn fetch_models(base_url: Option<&str>, api_key: Option<&str>) -> Result<Vec<FetchedModel>, ProviderError> {
        let base = base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/');
        let url = format!("{}/v1/models", base);

        let client = reqwest::Client::new();
        let mut request = client.get(&url);

        if let Some(key) = api_key {
            request = request.bearer_auth(key);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(format!(
                "OpenAI API returned status {}",
                response.status()
            )));
        }

        let body: ListModelsResponse = response.json().await?;

        let fetched = body.data.into_iter().map(FetchedModel::from).collect();

        Ok(fetched)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_classify_reasoning_models() {
        assert_eq!(ModelFamily::from_model_id("o1-preview"), ModelFamily::Reasoning);
        assert_eq!(ModelFamily::from_model_id("o1-mini"), ModelFamily::Reasoning);
        assert_eq!(ModelFamily::from_model_id("o3-mini"), ModelFamily::Reasoning);
    }

    #[test]
    fn should_classify_gpt4o_models() {
        assert_eq!(ModelFamily::from_model_id("gpt-4o"), ModelFamily::Gpt4o);
        assert_eq!(ModelFamily::from_model_id("gpt-4o-mini"), ModelFamily::Gpt4o);
        assert_eq!(ModelFamily::from_model_id("gpt-4o-2024-08-06"), ModelFamily::Gpt4o);
    }

    #[test]
    fn should_classify_gpt4_models() {
        assert_eq!(ModelFamily::from_model_id("gpt-4"), ModelFamily::Gpt4);
        assert_eq!(ModelFamily::from_model_id("gpt-4-turbo"), ModelFamily::Gpt4);
        assert_eq!(ModelFamily::from_model_id("gpt-4-0125-preview"), ModelFamily::Gpt4);
    }

    #[test]
    fn should_classify_gpt35_models() {
        assert_eq!(ModelFamily::from_model_id("gpt-3.5-turbo"), ModelFamily::Gpt35Turbo);
        assert_eq!(
            ModelFamily::from_model_id("gpt-3.5-turbo-0125"),
            ModelFamily::Gpt35Turbo
        );
    }

    #[test]
    fn should_classify_unknown_models() {
        assert_eq!(ModelFamily::from_model_id("davinci-002"), ModelFamily::Other);
        assert_eq!(ModelFamily::from_model_id("text-embedding-3-small"), ModelFamily::Other);
    }

    #[test]
    fn reasoning_models_should_have_thinking_capability() {
        let caps = ModelCapabilities::from(ModelFamily::Reasoning);
        assert!(caps.thinking, "reasoning models should support thinking");
        assert!(!caps.streaming, "reasoning models should not support streaming");
        assert!(
            !caps.system_prompt,
            "reasoning models should not support system prompts"
        );
    }

    #[test]
    fn gpt4o_should_have_vision_capability() {
        let caps = ModelCapabilities::from(ModelFamily::Gpt4o);
        assert!(caps.vision, "gpt-4o should support vision");
        assert!(caps.function_calling, "gpt-4o should support function calling");
        assert!(!caps.thinking, "gpt-4o should not support thinking");
    }
}
