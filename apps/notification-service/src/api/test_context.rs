#![cfg(all(test, feature = "integration"))]

use crate::models::{notification_preferences, notifications};
use crate::{AppContext, NotificationService};
use migration::MigratorTrait as _;
use platform_rs::middleware::auth::User;
use proto_rs::notification::v1::{NotificationCategory, NotificationPriority, OriginService};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;
use test_context::AsyncTestContext;

/// Test context for notification-service integration tests.
///
/// Provides database and Redis connections, along with helper methods
/// for creating authenticated requests.
pub struct TestContext {
    pub service: NotificationService,
    context: Arc<AppContext>,
    created_user_ids: Vec<String>,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let context = crate::AppContext::new()
            .await
            .expect("failed to initialize app context");

        migration::Migrator::up(context.db.as_ref(), None)
            .await
            .expect("Failed to run migrations on test database");

        Self {
            service: NotificationService::new(context.clone()),
            context,
            created_user_ids: Vec::new(),
        }
    }

    async fn teardown(self) {
        for user_id in &self.created_user_ids {
            // Clean up notifications first (no FK dependency, but good practice)
            let _ = notifications::Entity::delete_many()
                .filter(notifications::Column::UserId.eq(user_id))
                .exec(self.context.db.as_ref())
                .await;

            let _ = notification_preferences::Entity::delete_many()
                .filter(notification_preferences::Column::UserId.eq(user_id))
                .exec(self.context.db.as_ref())
                .await;
        }
    }
}

#[bon::bon]
impl TestContext {
    /// Generates a unique user ID and tracks it for cleanup.
    pub fn unique_user_id(&mut self, suffix: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let user_id = format!("test-user-{}-{}", suffix, timestamp);
        self.created_user_ids.push(user_id.clone());
        user_id
    }

    /// Creates a tonic::Request with mock User injected.
    ///
    /// This bypasses JWT validation since we call handlers directly.
    pub fn authenticated_request<T>(&self, inner: T, user_id: &str) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        let user = User {
            id: user_id.to_string(),
            email: Some(format!("{}@test.com", user_id)),
            name: Some(format!("Test User {}", user_id)),
            org_roles: std::collections::HashMap::new(),
            email_verified: Some(true),
            metadata: std::collections::HashMap::new(),
        };
        request.extensions_mut().insert(user);
        request
    }

    /// Creates a notification directly in the database.
    #[builder]
    pub async fn create_notification(
        &self,
        user_id: &str,
        #[builder(default = "org_test".to_string())] org_pid: String,
        #[builder(default = "Test Notification".to_string())] title: String,
        #[builder(default = "Test description".to_string())] description: String,
        #[builder(default = NotificationCategory::System)] category: NotificationCategory,
        #[builder(default = NotificationPriority::Normal)] priority: NotificationPriority,
        #[builder(default = OriginService::Api)] origin_service: OriginService,
        #[builder(default = false)] is_read: bool,
        #[builder(default = false)] is_dismissed: bool,
    ) -> notifications::Model {
        let active = notifications::ActiveModel {
            user_id: Set(user_id.to_string()),
            org_pid: Set(org_pid),
            title: Set(title),
            description: Set(description),
            category: Set(category.try_into().expect("valid category")),
            priority: Set(priority.into()),
            origin_service: Set(origin_service.try_into().expect("valid origin_service")),
            is_read: Set(is_read),
            is_dismissed: Set(is_dismissed),
            ..Default::default()
        };

        active
            .insert(self.context.db.as_ref())
            .await
            .expect("Failed to create notification")
    }
}
