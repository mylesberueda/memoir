pub(crate) use super::_entity::{
    notifications::*,
    sea_orm_active_enums::{NotificationCategory, NotificationPriority, OriginEntityType, OriginService},
};
use proto_rs::notification::v1::{
    Notification as ProtoNotification, NotificationCategory as ProtoNotificationCategory,
    NotificationPriority as ProtoNotificationPriority, OriginEntityType as ProtoOriginEntityType,
    OriginService as ProtoOriginService,
};

impl From<Model> for ProtoNotification {
    fn from(model: Model) -> ProtoNotification {
        let category: ProtoNotificationCategory = model.category.into();
        let priority: ProtoNotificationPriority = model.priority.into();
        let origin_service: ProtoOriginService = model.origin_service.into();

        Self {
            pid: model.pid,
            user_id: model.user_id,
            org_pid: model.org_pid,
            title: model.title,
            description: model.description,
            icon_url: model.icon_url,
            category: category.into(),
            priority: priority.into(),
            actions: model
                .actions
                .and_then(|v| match serde_json::from_value(v) {
                    Ok(a) => Some(a),
                    Err(e) => {
                        tracing::error!(error = %e, "corrupt_json_actions");
                        None
                    }
                })
                .unwrap_or_default(),
            origin_service: origin_service.into(),
            origin_entity_type: model.origin_entity_type.map(|t| {
                let proto: ProtoOriginEntityType = t.into();
                proto.into()
            }),
            origin_entity_pid: model.origin_entity_pid,
            is_read: model.is_read,
            read_at: model.read_at.map(|t| t.to_rfc3339()),
            is_dismissed: model.is_dismissed,
            expires_at: model.expires_at.map(|t| t.to_rfc3339()),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl From<NotificationCategory> for ProtoNotificationCategory {
    fn from(category: NotificationCategory) -> ProtoNotificationCategory {
        match category {
            NotificationCategory::Chat => ProtoNotificationCategory::Chat,
            NotificationCategory::Agent => ProtoNotificationCategory::Agent,
            NotificationCategory::System => ProtoNotificationCategory::System,
            NotificationCategory::Moderation => ProtoNotificationCategory::Moderation,
            NotificationCategory::Billing => ProtoNotificationCategory::Billing,
            NotificationCategory::Social => ProtoNotificationCategory::Social,
        }
    }
}

impl From<NotificationPriority> for ProtoNotificationPriority {
    fn from(priority: NotificationPriority) -> ProtoNotificationPriority {
        match priority {
            NotificationPriority::Low => ProtoNotificationPriority::Low,
            NotificationPriority::Normal => ProtoNotificationPriority::Normal,
            NotificationPriority::High => ProtoNotificationPriority::High,
            NotificationPriority::Urgent => ProtoNotificationPriority::Urgent,
        }
    }
}

impl From<OriginService> for ProtoOriginService {
    fn from(service: OriginService) -> Self {
        match service {
            OriginService::Api => ProtoOriginService::Api,
            OriginService::Chat => ProtoOriginService::Chat,
            OriginService::Rig => ProtoOriginService::Rig,
            OriginService::Agent => ProtoOriginService::Agent,
            OriginService::Notification => ProtoOriginService::Notification,
        }
    }
}

impl From<OriginEntityType> for ProtoOriginEntityType {
    fn from(entity_type: OriginEntityType) -> Self {
        match entity_type {
            OriginEntityType::Channel => ProtoOriginEntityType::Channel,
            OriginEntityType::Message => ProtoOriginEntityType::Message,
            OriginEntityType::Agent => ProtoOriginEntityType::Agent,
            OriginEntityType::Run => ProtoOriginEntityType::Run,
            OriginEntityType::Workflow => ProtoOriginEntityType::Workflow,
            OriginEntityType::User => ProtoOriginEntityType::User,
        }
    }
}

impl TryFrom<ProtoNotificationCategory> for NotificationCategory {
    type Error = tonic::Status;

    fn try_from(proto: ProtoNotificationCategory) -> Result<Self, Self::Error> {
        match proto {
            ProtoNotificationCategory::Unspecified => Err(tonic::Status::invalid_argument("category is required")),
            ProtoNotificationCategory::Chat => Ok(NotificationCategory::Chat),
            ProtoNotificationCategory::Agent => Ok(NotificationCategory::Agent),
            ProtoNotificationCategory::System => Ok(NotificationCategory::System),
            ProtoNotificationCategory::Moderation => Ok(NotificationCategory::Moderation),
            ProtoNotificationCategory::Billing => Ok(NotificationCategory::Billing),
            ProtoNotificationCategory::Social => Ok(NotificationCategory::Social),
        }
    }
}

impl From<ProtoNotificationPriority> for NotificationPriority {
    fn from(proto: ProtoNotificationPriority) -> Self {
        match proto {
            ProtoNotificationPriority::Unspecified => NotificationPriority::Normal,
            ProtoNotificationPriority::Low => NotificationPriority::Low,
            ProtoNotificationPriority::Normal => NotificationPriority::Normal,
            ProtoNotificationPriority::High => NotificationPriority::High,
            ProtoNotificationPriority::Urgent => NotificationPriority::Urgent,
        }
    }
}

impl TryFrom<ProtoOriginService> for OriginService {
    type Error = tonic::Status;

    fn try_from(proto: ProtoOriginService) -> Result<Self, Self::Error> {
        match proto {
            ProtoOriginService::Unspecified => Err(tonic::Status::invalid_argument("origin_service is required")),
            ProtoOriginService::Api => Ok(OriginService::Api),
            ProtoOriginService::Chat => Ok(OriginService::Chat),
            ProtoOriginService::Rig => Ok(OriginService::Rig),
            ProtoOriginService::Agent => Ok(OriginService::Agent),
            ProtoOriginService::Notification => Ok(OriginService::Notification),
        }
    }
}

impl TryFrom<ProtoOriginEntityType> for OriginEntityType {
    type Error = tonic::Status;

    fn try_from(proto: ProtoOriginEntityType) -> Result<Self, Self::Error> {
        match proto {
            ProtoOriginEntityType::Unspecified => Err(tonic::Status::invalid_argument(
                "origin_entity_type cannot be unspecified if provided",
            )),
            ProtoOriginEntityType::Channel => Ok(OriginEntityType::Channel),
            ProtoOriginEntityType::Message => Ok(OriginEntityType::Message),
            ProtoOriginEntityType::Agent => Ok(OriginEntityType::Agent),
            ProtoOriginEntityType::Run => Ok(OriginEntityType::Run),
            ProtoOriginEntityType::Workflow => Ok(OriginEntityType::Workflow),
            ProtoOriginEntityType::User => Ok(OriginEntityType::User),
        }
    }
}
