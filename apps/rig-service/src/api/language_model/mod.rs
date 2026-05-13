pub(crate) mod proto;

use crate::models::language_models;

pub(crate) struct LanguageModel {
    pub(crate) model: language_models::Model,
    pub(crate) provider_pid: String,
    pub(crate) provider_type: String,
    pub(crate) provider_name: String,
    pub(crate) provider_created_by: Option<String>,
    pub(crate) provider_organization_pid: Option<String>,
}

impl LanguageModel {
    pub(crate) fn is_accessible_in_user_context(&self, user_id: &str) -> bool {
        self.provider_created_by.is_none()
            || (self.provider_organization_pid.is_none() && self.provider_created_by.as_deref() == Some(user_id))
    }

    pub(crate) fn is_accessible_in_org_context(&self, org_pid: &str) -> bool {
        self.provider_created_by.is_none() || self.provider_organization_pid.as_deref() == Some(org_pid)
    }
}
