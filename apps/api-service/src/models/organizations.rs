pub(crate) use super::_entity::organizations::*;
use proto_rs::api::v1::{Organization, OrganizationSettings};
use sea_orm::ActiveModelBehavior;

impl Model {
    pub(crate) fn to_proto(&self, member_count: u64) -> Organization {
        // Convert JsonValue to OrganizationSettings via serde deserialization
        let settings: Option<OrganizationSettings> = serde_json::from_value(self.settings.clone()).ok();

        Organization {
            pid: self.pid.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            settings,
            created_at: self.created_at.and_utc().to_rfc3339(),
            updated_at: self.updated_at.and_utc().to_rfc3339(),
            member_count,
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
