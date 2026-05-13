use super::*;
use common_rs::crypto::Redactable;
use proto_rs::rig::v1;

impl From<Provider> for v1::Provider {
    fn from(provider_model: Provider) -> Self {
        let source = if provider_model.model.is_system() {
            v1::ProviderSource::System
        } else {
            v1::ProviderSource::User
        };

        let credentials = provider_model
            .credentials
            .map(|value| value.redacted())
            .unwrap_or_default();

        Self {
            identifier: Some(v1::provider::Identifier::Pid(provider_model.model.pid)),
            name: provider_model.model.name,
            provider_type: provider_model.model.provider_type,
            source: source.into(),
            user_id: provider_model.model.created_by,
            config: None,
            credentials,
            endpoint_url: provider_model.model.base_url.unwrap_or_default(),
            is_active: provider_model.model.is_active,
            created_at: provider_model.model.created_at.and_utc().to_rfc3339(),
            updated_at: provider_model.model.updated_at.and_utc().to_rfc3339(),
        }
    }
}
