use super::*;

pub(super) async fn get(
    service: &NotificationService,
    request: tonic::Request<GetPreferencesRequest>,
) -> std::result::Result<tonic::Response<GetPreferencesResponse>, tonic::Status> {
    let user_id = request.user_id()?;

    tracing::debug!("fetching_preferences");
    let preferences = notification_preferences::Entity::find()
        .filter(notification_preferences::Column::UserId.eq(&user_id))
        .one(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    let proto = match preferences {
        Some(model) => model.into(),
        None => {
            // If no preferences exist, we'll just send some defaults back.
            tracing::debug!("no_preferences_found_returning_defaults");
            NotificationPreferences {
                push_enabled: true,
                email_enabled: true,
                sound_enabled: true,
                category_preferences: vec![],
            }
        }
    };
    tracing::info!("preferences_retrieved");

    Ok(tonic::Response::new(GetPreferencesResponse {
        preferences: Some(proto),
    }))
}

pub(super) async fn update(
    service: &NotificationService,
    request: tonic::Request<UpdatePreferencesRequest>,
) -> std::result::Result<tonic::Response<UpdatePreferencesResponse>, tonic::Status> {
    let user_id = request.user_id()?;
    let req = request.get_ref();

    let prefs = req
        .preferences
        .as_ref()
        .ok_or(tonic::Status::invalid_argument("preferences is required"))?;

    let category_preferences: Option<serde_json::Value> = if prefs.category_preferences.is_empty() {
        None
    } else {
        Some(
            serde_json::to_value(&prefs.category_preferences)
                .map_err(|e| tonic::Status::internal(format!("Failed to serialize: {e}")))?,
        )
    };

    let existing = notification_preferences::Entity::find()
        .filter(notification_preferences::Column::UserId.eq(&user_id))
        .one(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    tracing::debug!("updating_preferences");
    let updated = if let Some(model) = existing {
        let mut active: notification_preferences::ActiveModel = model.into();
        active.push_enabled = Set(prefs.push_enabled);
        active.email_enabled = Set(prefs.email_enabled);
        active.sound_enabled = Set(prefs.sound_enabled);
        active.category_preferences = Set(category_preferences);
        active.updated_at = Set(chrono::Utc::now().into());

        active
            .update(service.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
    } else {
        let active = notification_preferences::ActiveModel {
            user_id: Set(user_id.clone()),
            push_enabled: Set(prefs.push_enabled),
            email_enabled: Set(prefs.email_enabled),
            sound_enabled: Set(prefs.sound_enabled),
            category_preferences: Set(category_preferences),
            ..Default::default()
        };

        active
            .insert(service.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
    };
    tracing::info!("preferences_updated");

    Ok(tonic::Response::new(UpdatePreferencesResponse {
        preferences: Some(updated.into()),
    }))
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::api::test_context::TestContext;
    use proto_rs::notification::v1::{
        CategoryPreference, GetPreferencesRequest, NotificationCategory, NotificationPreferences,
        UpdatePreferencesRequest,
    };
    use serial_test::serial;
    use test_context::test_context;

    // ==========================================================================
    // GetPreferences
    // ==========================================================================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_defaults_when_no_preferences_exist(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("no-prefs");
        let request = ctx.authenticated_request(GetPreferencesRequest {}, &user_id);

        let response = get(&ctx.service, request).await.expect("get should succeed");
        let prefs = response.into_inner().preferences.expect("preferences should exist");

        assert!(prefs.push_enabled, "push_enabled should default to true");
        assert!(prefs.email_enabled, "email_enabled should default to true");
        assert!(prefs.sound_enabled, "sound_enabled should default to true");
        assert!(
            prefs.category_preferences.is_empty(),
            "category_preferences should be empty"
        );
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_saved_preferences(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("saved-prefs");

        // Save preferences first
        let update_req = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: false,
                    email_enabled: true,
                    sound_enabled: false,
                    category_preferences: vec![],
                }),
            },
            &user_id,
        );
        update(&ctx.service, update_req).await.expect("update should succeed");

        // Get them back
        let get_req = ctx.authenticated_request(GetPreferencesRequest {}, &user_id);
        let response = get(&ctx.service, get_req).await.expect("get should succeed");
        let prefs = response.into_inner().preferences.expect("preferences should exist");

        assert!(!prefs.push_enabled);
        assert!(prefs.email_enabled);
        assert!(!prefs.sound_enabled);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_leak_other_users_preferences(ctx: &mut TestContext) {
        let user_a = ctx.unique_user_id("user-a");
        let user_b = ctx.unique_user_id("user-b");

        // User A disables everything
        let update_req = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: false,
                    email_enabled: false,
                    sound_enabled: false,
                    category_preferences: vec![],
                }),
            },
            &user_a,
        );
        update(&ctx.service, update_req).await.expect("update should succeed");

        // User B should get defaults, not User A's settings
        let get_req = ctx.authenticated_request(GetPreferencesRequest {}, &user_b);
        let response = get(&ctx.service, get_req).await.expect("get should succeed");
        let prefs = response.into_inner().preferences.expect("preferences should exist");

        assert!(prefs.push_enabled, "User B should get default push_enabled=true");
        assert!(prefs.email_enabled, "User B should get default email_enabled=true");
        assert!(prefs.sound_enabled, "User B should get default sound_enabled=true");
    }

    // ==========================================================================
    // UpdatePreferences
    // ==========================================================================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_create_preferences_on_first_update(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("first-update");

        let request = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: false,
                    email_enabled: true,
                    sound_enabled: false,
                    category_preferences: vec![],
                }),
            },
            &user_id,
        );

        let response = update(&ctx.service, request).await.expect("update should succeed");
        let prefs = response.into_inner().preferences.expect("preferences should exist");

        assert!(!prefs.push_enabled);
        assert!(prefs.email_enabled);
        assert!(!prefs.sound_enabled);

        // Verify persisted
        let get_req = ctx.authenticated_request(GetPreferencesRequest {}, &user_id);
        let stored = get(&ctx.service, get_req)
            .await
            .expect("get should succeed")
            .into_inner()
            .preferences
            .expect("should have preferences");

        assert!(!stored.push_enabled);
        assert!(stored.email_enabled);
        assert!(!stored.sound_enabled);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_update_existing_preferences(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("update-existing");

        // First update: all enabled
        let req1 = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: true,
                    email_enabled: true,
                    sound_enabled: true,
                    category_preferences: vec![],
                }),
            },
            &user_id,
        );
        update(&ctx.service, req1).await.expect("first update should succeed");

        // Second update: all disabled
        let req2 = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: false,
                    email_enabled: false,
                    sound_enabled: false,
                    category_preferences: vec![],
                }),
            },
            &user_id,
        );
        let response = update(&ctx.service, req2).await.expect("second update should succeed");
        let prefs = response.into_inner().preferences.expect("preferences should exist");

        assert!(!prefs.push_enabled);
        assert!(!prefs.email_enabled);
        assert!(!prefs.sound_enabled);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_persist_category_preferences(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("category-prefs");

        let request = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: true,
                    email_enabled: true,
                    sound_enabled: true,
                    category_preferences: vec![
                        CategoryPreference {
                            category: NotificationCategory::Chat as i32,
                            enabled: false,
                            push: false,
                            email: true,
                            min_priority: 0,
                        },
                        CategoryPreference {
                            category: NotificationCategory::Agent as i32,
                            enabled: true,
                            push: true,
                            email: false,
                            min_priority: 0,
                        },
                    ],
                }),
            },
            &user_id,
        );
        update(&ctx.service, request).await.expect("update should succeed");

        // Verify persisted
        let get_req = ctx.authenticated_request(GetPreferencesRequest {}, &user_id);
        let prefs = get(&ctx.service, get_req)
            .await
            .expect("get should succeed")
            .into_inner()
            .preferences
            .expect("preferences should exist");

        assert_eq!(prefs.category_preferences.len(), 2);

        let chat = prefs
            .category_preferences
            .iter()
            .find(|p| p.category == NotificationCategory::Chat as i32)
            .expect("should have chat preference");
        assert!(!chat.enabled);
        assert!(!chat.push);
        assert!(chat.email);

        let agent = prefs
            .category_preferences
            .iter()
            .find(|p| p.category == NotificationCategory::Agent as i32)
            .expect("should have agent preference");
        assert!(agent.enabled);
        assert!(agent.push);
        assert!(!agent.email);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_reject_missing_preferences(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("missing-prefs");

        let request = ctx.authenticated_request(UpdatePreferencesRequest { preferences: None }, &user_id);

        let result = update(&ctx.service, request).await;
        assert!(result.is_err());

        let status = result.unwrap_err();
        assert_eq!(status.code(), tonic::Code::InvalidArgument);
        assert!(status.message().contains("preferences is required"));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_affect_other_users(ctx: &mut TestContext) {
        let user_a = ctx.unique_user_id("update-a");
        let user_b = ctx.unique_user_id("update-b");

        // User B saves preferences first
        let req_b = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: true,
                    email_enabled: true,
                    sound_enabled: true,
                    category_preferences: vec![],
                }),
            },
            &user_b,
        );
        update(&ctx.service, req_b).await.expect("User B update should succeed");

        // User A saves different preferences
        let req_a = ctx.authenticated_request(
            UpdatePreferencesRequest {
                preferences: Some(NotificationPreferences {
                    push_enabled: false,
                    email_enabled: false,
                    sound_enabled: false,
                    category_preferences: vec![],
                }),
            },
            &user_a,
        );
        update(&ctx.service, req_a).await.expect("User A update should succeed");

        // User B's preferences should be unchanged
        let get_req = ctx.authenticated_request(GetPreferencesRequest {}, &user_b);
        let prefs = get(&ctx.service, get_req)
            .await
            .expect("get should succeed")
            .into_inner()
            .preferences
            .expect("preferences should exist");

        assert!(prefs.push_enabled, "User B's push_enabled should still be true");
        assert!(prefs.email_enabled, "User B's email_enabled should still be true");
        assert!(prefs.sound_enabled, "User B's sound_enabled should still be true");
    }
}
