pub(crate) use super::_entity::users::*;
use proto_rs::api::v1::{User, UserSettings};
use sea_orm::ActiveModelBehavior;

impl From<Model> for User {
    fn from(user: Model) -> Self {
        // Convert JsonValue to UserSettings via serde deserialization
        let settings: Option<UserSettings> = serde_json::from_value(user.settings).ok();

        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            settings,
            created_at: user.created_at.and_utc().to_rfc3339(),
            updated_at: user.updated_at.and_utc().to_rfc3339(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
