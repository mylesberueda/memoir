pub(crate) use super::_entity::language_models::*;
use crate::{api::language_model::LanguageModel, models::providers};
use proto_rs::rig::v1::ModelCapabilities;
use sea_orm::{ActiveModelBehavior, ColumnTrait as _, ConnectionTrait, EntityTrait as _, QueryFilter as _};

impl Model {
    /// Parse capabilities JSONB into proto ModelCapabilities.
    pub(crate) fn capabilities_proto(&self) -> Option<ModelCapabilities> {
        serde_json::from_value(self.capabilities.clone()).ok()
    }

    /// Check if model is deprecated.
    pub(crate) fn is_deprecated(&self) -> bool {
        self.deprecation_message.is_some()
    }
}

impl ModelEx {
    /// Parse capabilities JSONB into proto ModelCapabilities.
    pub(crate) fn capabilities_proto(&self) -> Option<ModelCapabilities> {
        serde_json::from_value(self.capabilities.clone()).ok()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ModelError {
    #[error(transparent)]
    Query(#[from] sea_orm::DbErr),
    #[error("model not found")]
    NotFound,
    #[error("model missing provider")]
    MissingProvider,
}

impl Entity {
    pub(crate) async fn find_language_model_by_pid<C>(db: &C, pid: &str) -> Result<LanguageModel, ModelError>
    where
        C: ConnectionTrait,
    {
        let (model, provider) = Self::find()
            .filter(Column::Pid.eq(pid))
            .find_also_related(providers::Entity)
            .one(db)
            .await?
            .ok_or(ModelError::NotFound)?;

        let provider = provider.ok_or(ModelError::MissingProvider)?;

        Ok(Self::assemble_language_model(model, provider))
    }

    pub(crate) fn assemble_language_model(
        model: super::_entity::language_models::Model,
        provider: providers::Model,
    ) -> LanguageModel {
        LanguageModel {
            model,
            provider_pid: provider.pid,
            provider_type: provider.provider_type,
            provider_name: provider.name,
            provider_created_by: provider.created_by,
            provider_organization_pid: provider.organization_pid,
        }
    }
}
