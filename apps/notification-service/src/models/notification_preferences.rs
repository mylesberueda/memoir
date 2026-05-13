pub(crate) use super::_entity::notification_preferences::*;
use proto_rs::notification::v1::{
    CategoryPreference as ProtoCategoryPreference, NotificationPreferences as ProtoNotificationPreferences,
};

impl From<Model> for ProtoNotificationPreferences {
    fn from(model: Model) -> Self {
        let category_preferences: Vec<ProtoCategoryPreference> = match model
            .category_preferences
            .and_then(|v| serde_json::from_value::<Vec<ProtoCategoryPreference>>(v).ok())
        {
            Some(v) => v,
            None => {
                tracing::error!(id = model.id, "corrupt_notification_preferences");
                vec![]
            }
        };

        Self {
            push_enabled: model.push_enabled,
            email_enabled: model.email_enabled,
            sound_enabled: model.sound_enabled,
            category_preferences,
        }
    }
}
