pub(crate) use super::_entity::providers::*;
use crate::{api::crypto::get_crypto, api::provider::Provider, models::secrets};
use sea_orm::{ActiveModelBehavior, ColumnTrait as _, ConnectionTrait, EntityTrait as _, QueryFilter as _};

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum ProviderKind {
    Ollama,
    Openai,
    Gemini,
}

impl Model {
    pub(crate) fn kind(&self) -> ProviderKind {
        self.provider_type.parse().unwrap_or_else(|_| {
            tracing::warn!(provider_id = self.id, provider_type = %self.provider_type, "unknown provider type, defaulting to Openai");
            ProviderKind::Openai
        })
    }

    /// System providers have no creator - they're available to everyone
    pub(crate) fn is_system(&self) -> bool {
        self.created_by.is_none()
    }

    /// Check if provider is accessible in personal context (no organization)
    pub(crate) fn is_accessible_in_user_context(&self, user_id: &str) -> bool {
        self.is_system() || (self.organization_pid.is_none() && self.created_by.as_deref() == Some(user_id))
    }

    /// Check if provider is accessible in organization context
    pub(crate) fn is_accessible_in_org_context(&self, org_pid: &str) -> bool {
        self.is_system() || self.organization_pid.as_deref() == Some(org_pid)
    }
}

impl ModelEx {
    pub(crate) fn kind(&self) -> ProviderKind {
        self.provider_type.parse().unwrap_or_else(|_| {
            tracing::warn!(provider_id = self.id, provider_type = %self.provider_type, "unknown provider type, defaulting to Openai");
            ProviderKind::Openai
        })
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ProviderError {
    #[error(transparent)]
    Query(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Secret(#[from] common_rs::crypto::CryptoError),
    #[error("provider not found")]
    NotFound,
}

impl Entity {
    pub(crate) async fn find_provider_by_pid<C>(db: &C, pid: &str) -> Result<Provider, ProviderError>
    where
        C: ConnectionTrait,
    {
        let (provider, secret) = Self::find()
            .filter(Column::Pid.eq(pid))
            .find_also_related(secrets::Entity)
            .one(db)
            .await?
            .ok_or(ProviderError::NotFound)?;

        let credentials = secret.map(|secret| secret.decrypt()).transpose()?;

        Ok(Self::assemble_provider(provider, credentials))
    }

    pub(crate) fn assemble_provider(model: Model, credentials: Option<common_rs::crypto::Secret>) -> Provider {
        Provider { model, credentials }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::DateTime;

    fn make_model(provider_type: &str) -> Model {
        Model {
            id: 1,
            pid: "provider_test".to_string(),
            organization_pid: Some("org_test".to_string()),
            name: "Test Provider".to_string(),
            provider_type: provider_type.to_string(),
            base_url: None,
            is_active: true,
            is_deprecated: false,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
            created_by: None,
        }
    }

    mod kind {
        use super::*;

        #[test]
        fn should_parse_ollama_kind() {
            let model = make_model("ollama");

            let result = model.kind();

            assert_eq!(result, ProviderKind::Ollama);
        }

        #[test]
        fn should_parse_openai_kind() {
            let model = make_model("openai");

            let result = model.kind();

            assert_eq!(result, ProviderKind::Openai);
        }

        #[test]
        fn should_default_to_openai_for_unknown_kind() {
            let model = make_model("unknown-provider");

            let result = model.kind();

            assert_eq!(result, ProviderKind::Openai);
        }

        #[test]
        fn should_default_to_openai_for_empty_kind() {
            let model = make_model("");

            let result = model.kind();

            assert_eq!(result, ProviderKind::Openai);
        }
    }

    fn make_system_provider() -> Model {
        Model {
            created_by: None,
            organization_pid: None,
            ..make_model("openai")
        }
    }

    fn make_org_provider(org_pid: &str) -> Model {
        Model {
            created_by: Some("user_test".to_string()),
            organization_pid: Some(org_pid.to_string()),
            ..make_model("openai")
        }
    }

    mod access_control {
        use super::*;

        #[test]
        fn should_allow_system_provider_from_any_org() {
            let provider = make_system_provider();
            assert!(provider.is_accessible_in_org_context("any_org"));
        }

        #[test]
        fn should_allow_org_provider_from_matching_org() {
            let provider = make_org_provider("org_1");
            assert!(provider.is_accessible_in_org_context("org_1"));
        }

        #[test]
        fn should_deny_org_provider_from_different_org() {
            let provider = make_org_provider("org_1");
            assert!(!provider.is_accessible_in_org_context("org_2"));
        }

        #[test]
        fn should_identify_system_provider() {
            let provider = make_system_provider();
            assert!(provider.is_system());
        }

        #[test]
        fn should_identify_non_system_provider() {
            let provider = make_org_provider("org_1");
            assert!(!provider.is_system());
        }
    }
}
