use super::*;
use proto_rs::rig::v1;

impl From<LanguageModel> for v1::Model {
    fn from(language_model: LanguageModel) -> Self {
        let capabilities: Option<v1::ModelCapabilities> = language_model.model.capabilities_proto();
        let metadata: Option<v1::ModelMetadata> = serde_json::from_value(language_model.model.metadata.clone()).ok();

        Self {
            identifier: Some(v1::model::Identifier::Pid(language_model.model.pid)),
            model_id: language_model.model.model_id,
            name: language_model.model.name,
            provider_pid: language_model.provider_pid,
            provider_type: language_model.provider_type,
            provider_name: language_model.provider_name,
            context_length: language_model.model.context_window,
            capabilities,
            metadata,
            is_active: language_model.model.is_active,
            deprecation_message: language_model.model.deprecation_message,
            last_fetched_at: language_model.model.last_fetched_at.map(|dt| dt.and_utc().to_rfc3339()),
            created_at: language_model.model.created_at.and_utc().to_rfc3339(),
            updated_at: language_model.model.updated_at.and_utc().to_rfc3339(),
        }
    }
}
