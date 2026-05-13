use super::*;

pub(super) async fn list(
    service: &NotificationService,
    request: tonic::Request<ListRequest>,
) -> std::result::Result<tonic::Response<ListResponse>, tonic::Status> {
    let user_id = request.user_id()?;
    let req = request.get_ref();

    const DEFAULT_LIMIT: u64 = 20;
    const MAX_LIMIT: u64 = 100;

    let limit = if req.limit == 0 {
        DEFAULT_LIMIT
    } else {
        req.limit.clamp(1, MAX_LIMIT)
    };

    let mut query = notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false));

    if req.unread_only.unwrap_or_default() {
        query = query.filter(notifications::Column::IsRead.eq(false));
    }

    if !req.categories.is_empty() {
        let categories: Vec<notifications::NotificationCategory> = req
            .categories
            .iter()
            .filter_map(|&c| {
                NotificationCategory::try_from(c)
                    .ok()
                    .and_then(|c| notifications::NotificationCategory::try_from(c).ok())
            })
            .collect();

        if !categories.is_empty() {
            query = query.filter(notifications::Column::Category.is_in(categories));
        }
    }

    if let Some(min_priority) = req.min_priority
        && let Ok(proto_priority) = NotificationPriority::try_from(min_priority)
        && proto_priority != NotificationPriority::Unspecified
        && proto_priority != NotificationPriority::Low
    {
        let db_priority: notifications::NotificationPriority = proto_priority.into();
        query = query.filter(notifications::Column::Priority.gte(db_priority));
    }

    if let Some(ref cursor) = req.cursor
        && let Some((ts_str, id_str)) = cursor.split_once('_')
        && let (Ok(ts_micros), Ok(cursor_id)) = (ts_str.parse::<i64>(), id_str.parse::<i64>())
        && let Some(dt) = DateTime::from_timestamp_micros(ts_micros)
    {
        query = query.filter(
            notifications::Column::CreatedAt
                .lt(dt)
                .or(notifications::Column::CreatedAt
                    .eq(dt)
                    .and(notifications::Column::Id.lt(cursor_id))),
        );
    }

    // We still need to query the whole table to get totals, these counts are fine.
    let total_count = notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .count(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    let unread_count = notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .filter(notifications::Column::IsRead.eq(false))
        .count(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    tracing::debug!("fetching_notifications");
    let notifications = query
        .order_by_desc(notifications::Column::CreatedAt)
        .order_by_desc(notifications::Column::Id) // Tie-breaker
        .limit(Some(limit + 1)) // Need the next one to use as the cursor
        .all(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    // SAFETY(Myles): `limit` is clamped to (1, 100) - there's no way it can
    // overflow when cast here. Literally any modern usize can handle 100.
    let has_more = notifications.len() > limit as usize;
    let notifications: Vec<_> = notifications.into_iter().take(limit as usize).collect();
    let next_cursor = if has_more {
        notifications
            .last()
            // We use micros here, to match Postgres' time precision
            .map(|n| format!("{}_{}", n.created_at.timestamp_micros(), n.id))
    } else {
        None
    };

    tracing::info!(
        count = notifications.len(),
        total = total_count,
        unread = unread_count,
        "notifications_listed"
    );

    Ok(tonic::Response::new(ListResponse {
        notifications: notifications.into_iter().map(|n| n.into()).collect(),
        total_count,
        unread_count,
        next_cursor,
    }))
}

pub(super) async fn get(
    service: &NotificationService,
    request: tonic::Request<GetRequest>,
) -> std::result::Result<tonic::Response<GetResponse>, tonic::Status> {
    let user_id = request.user_id()?;
    let pid = &request.get_ref().notification_pid;

    if pid.is_empty() {
        return Err(tonic::Status::invalid_argument("notification_pid is required"));
    }

    tracing::debug!(pid, "fetching_notification");
    let notification = notifications::Entity::find()
        .filter(notifications::Column::Pid.eq(pid))
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .one(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
        .ok_or(tonic::Status::not_found("Notification not found"))?;
    tracing::info!(pid, "notification_retrieved");

    Ok(tonic::Response::new(GetResponse {
        notification: Some(notification.into()),
    }))
}

pub(super) async fn get_unread_count(
    service: &NotificationService,
    request: tonic::Request<GetUnreadCountRequest>,
) -> std::result::Result<tonic::Response<GetUnreadCountResponse>, tonic::Status> {
    let user_id = request.user_id()?;

    #[derive(Debug, FromQueryResult)]
    struct CategoryCount {
        category: notifications::NotificationCategory,
        count: i64,
    }

    let category_counts: Vec<CategoryCount> = notifications::Entity::find()
        .select_only()
        .column(notifications::Column::Category)
        .column_as(notifications::Column::Id.count(), "count")
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .filter(notifications::Column::IsRead.eq(false))
        .group_by(notifications::Column::Category)
        .into_model::<CategoryCount>()
        .all(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    let by_category: HashMap<String, u64> = category_counts
        .into_iter()
        .map(|cc| {
            let proto: NotificationCategory = cc.category.into();
            (proto.as_str_name().to_string(), cc.count as u64) // TODO(_): Possibly bad cast?
        })
        .collect();

    let total = by_category.values().sum();

    tracing::info!(total, categories = ?by_category.keys().collect::<Vec<_>>(), "unread_count_retrieved");

    Ok(tonic::Response::new(GetUnreadCountResponse { total, by_category }))
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::api::test_context::TestContext;
    use proto_rs::notification::v1::{
        GetRequest, GetUnreadCountRequest, ListRequest, NotificationCategory, NotificationPriority,
    };
    use serial_test::serial;
    use test_context::test_context;

    // ==========================================================================
    // Get
    // ==========================================================================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_notification_with_all_fields(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("get-fields");
        let n = ctx
            .create_notification()
            .user_id(&user_id)
            .title("Custom Title".to_string())
            .description("Custom Description".to_string())
            .category(NotificationCategory::Chat)
            .priority(NotificationPriority::High)
            .call()
            .await;

        let request = ctx.authenticated_request(
            GetRequest {
                notification_pid: n.pid.clone(),
            },
            &user_id,
        );
        let response = get(&ctx.service, request).await.expect("get should succeed");
        let notification = response.into_inner().notification.expect("should have notification");

        assert_eq!(notification.pid, n.pid);
        assert_eq!(notification.title, "Custom Title");
        assert_eq!(notification.description, "Custom Description");
        assert_eq!(notification.category, NotificationCategory::Chat as i32);
        assert_eq!(notification.priority, NotificationPriority::High as i32);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_not_found_for_invalid_pid(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("get-invalid");

        let request = ctx.authenticated_request(
            GetRequest {
                notification_pid: "nonexistent-pid".to_string(),
            },
            &user_id,
        );
        let result = get(&ctx.service, request).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_leak_other_users_notifications(ctx: &mut TestContext) {
        let user_a = ctx.unique_user_id("get-leak-a");
        let user_b = ctx.unique_user_id("get-leak-b");

        let n = ctx.create_notification().user_id(&user_a).call().await;

        // User B tries to access User A's notification
        let request = ctx.authenticated_request(
            GetRequest {
                notification_pid: n.pid.clone(),
            },
            &user_b,
        );
        let result = get(&ctx.service, request).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_return_dismissed_notifications(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("get-dismissed");
        let n = ctx
            .create_notification()
            .user_id(&user_id)
            .is_dismissed(true)
            .call()
            .await;

        let request = ctx.authenticated_request(
            GetRequest {
                notification_pid: n.pid.clone(),
            },
            &user_id,
        );
        let result = get(&ctx.service, request).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
    }

    // ==========================================================================
    // List
    // ==========================================================================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_empty_list_with_zero_counts(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-empty");

        let request = ctx.authenticated_request(ListRequest::default(), &user_id);
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert!(resp.notifications.is_empty());
        assert_eq!(resp.total_count, 0);
        assert_eq!(resp.unread_count, 0);
        assert!(resp.next_cursor.is_none());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_accurate_total_and_unread_counts(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-counts");

        // Create 3 unread and 2 read
        for _ in 0..3 {
            ctx.create_notification().user_id(&user_id).call().await;
        }
        for _ in 0..2 {
            ctx.create_notification().user_id(&user_id).is_read(true).call().await;
        }

        let request = ctx.authenticated_request(ListRequest::default(), &user_id);
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.total_count, 5);
        assert_eq!(resp.unread_count, 3);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_paginate_with_next_cursor(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-paginate");

        for i in 0..25 {
            ctx.create_notification()
                .user_id(&user_id)
                .title(format!("Notification {}", i))
                .call()
                .await;
        }

        let request = ctx.authenticated_request(
            ListRequest {
                limit: 10,
                ..Default::default()
            },
            &user_id,
        );
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications.len(), 10);
        assert!(resp.next_cursor.is_some());
        assert_eq!(resp.total_count, 25);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_continue_from_cursor_without_duplicates(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-cursor");

        for i in 0..15 {
            ctx.create_notification()
                .user_id(&user_id)
                .title(format!("Notification {}", i))
                .call()
                .await;
        }

        // First page
        let request = ctx.authenticated_request(
            ListRequest {
                limit: 10,
                ..Default::default()
            },
            &user_id,
        );
        let first = list(&ctx.service, request).await.unwrap().into_inner();
        let cursor = first.next_cursor.expect("should have cursor");
        let first_pids: Vec<_> = first.notifications.iter().map(|n| n.pid.clone()).collect();

        // Second page
        let request = ctx.authenticated_request(
            ListRequest {
                limit: 10,
                cursor: Some(cursor),
                ..Default::default()
            },
            &user_id,
        );
        let second = list(&ctx.service, request).await.unwrap().into_inner();
        let second_pids: Vec<_> = second.notifications.iter().map(|n| n.pid.clone()).collect();

        // No overlap
        for pid in &second_pids {
            assert!(!first_pids.contains(pid), "Duplicate notification in pagination");
        }

        assert_eq!(second.notifications.len(), 5);
        assert!(second.next_cursor.is_none());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_filter_by_unread_only(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-unread");

        for _ in 0..5 {
            ctx.create_notification().user_id(&user_id).is_read(true).call().await;
        }
        for _ in 0..3 {
            ctx.create_notification().user_id(&user_id).call().await;
        }

        let request = ctx.authenticated_request(
            ListRequest {
                unread_only: Some(true),
                ..Default::default()
            },
            &user_id,
        );
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications.len(), 3);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_filter_by_category(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-category");

        for _ in 0..3 {
            ctx.create_notification()
                .user_id(&user_id)
                .category(NotificationCategory::Chat)
                .call()
                .await;
        }
        for _ in 0..2 {
            ctx.create_notification()
                .user_id(&user_id)
                .category(NotificationCategory::Agent)
                .call()
                .await;
        }

        let request = ctx.authenticated_request(
            ListRequest {
                categories: vec![NotificationCategory::Chat as i32],
                ..Default::default()
            },
            &user_id,
        );
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications.len(), 3);
        for n in resp.notifications {
            assert_eq!(n.category, NotificationCategory::Chat as i32);
        }
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_filter_by_min_priority(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-priority");

        ctx.create_notification()
            .user_id(&user_id)
            .priority(NotificationPriority::Low)
            .call()
            .await;
        ctx.create_notification()
            .user_id(&user_id)
            .priority(NotificationPriority::Normal)
            .call()
            .await;
        ctx.create_notification()
            .user_id(&user_id)
            .priority(NotificationPriority::High)
            .call()
            .await;
        ctx.create_notification()
            .user_id(&user_id)
            .priority(NotificationPriority::Urgent)
            .call()
            .await;

        let request = ctx.authenticated_request(
            ListRequest {
                min_priority: Some(NotificationPriority::High as i32),
                ..Default::default()
            },
            &user_id,
        );
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications.len(), 2);
        for n in resp.notifications {
            assert!(n.priority >= NotificationPriority::High as i32);
        }
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_order_newest_first(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-order");

        let n1 = ctx
            .create_notification()
            .user_id(&user_id)
            .title("First".to_string())
            .call()
            .await;
        let n2 = ctx
            .create_notification()
            .user_id(&user_id)
            .title("Second".to_string())
            .call()
            .await;
        let n3 = ctx
            .create_notification()
            .user_id(&user_id)
            .title("Third".to_string())
            .call()
            .await;

        let request = ctx.authenticated_request(ListRequest::default(), &user_id);
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications[0].pid, n3.pid);
        assert_eq!(resp.notifications[1].pid, n2.pid);
        assert_eq!(resp.notifications[2].pid, n1.pid);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_exclude_dismissed_from_list(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("list-dismissed");

        ctx.create_notification().user_id(&user_id).call().await;
        ctx.create_notification()
            .user_id(&user_id)
            .is_dismissed(true)
            .call()
            .await;

        let request = ctx.authenticated_request(ListRequest::default(), &user_id);
        let response = list(&ctx.service, request).await.expect("list should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.notifications.len(), 1);
        assert_eq!(resp.total_count, 1);
    }

    // ==========================================================================
    // GetUnreadCount
    // ==========================================================================

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_zero_when_no_notifications(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("count-zero");

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_id);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.total, 0);
        assert!(resp.by_category.is_empty());
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_return_total_unread_count(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("count-total");

        for _ in 0..5 {
            ctx.create_notification().user_id(&user_id).call().await;
        }

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_id);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");

        assert_eq!(response.into_inner().total, 5);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_group_by_category(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("count-category");

        for _ in 0..2 {
            ctx.create_notification()
                .user_id(&user_id)
                .category(NotificationCategory::Chat)
                .call()
                .await;
        }
        for _ in 0..3 {
            ctx.create_notification()
                .user_id(&user_id)
                .category(NotificationCategory::Agent)
                .call()
                .await;
        }

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_id);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");
        let resp = response.into_inner();

        assert_eq!(resp.total, 5);
        assert_eq!(resp.by_category.get("NOTIFICATION_CATEGORY_CHAT"), Some(&2));
        assert_eq!(resp.by_category.get("NOTIFICATION_CATEGORY_AGENT"), Some(&3));
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_exclude_read_notifications(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("count-exclude-read");

        for _ in 0..3 {
            ctx.create_notification().user_id(&user_id).is_read(true).call().await;
        }
        for _ in 0..2 {
            ctx.create_notification().user_id(&user_id).call().await;
        }

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_id);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");

        assert_eq!(response.into_inner().total, 2);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_exclude_dismissed_notifications(ctx: &mut TestContext) {
        let user_id = ctx.unique_user_id("count-exclude-dismissed");

        for _ in 0..2 {
            ctx.create_notification()
                .user_id(&user_id)
                .is_dismissed(true)
                .call()
                .await;
        }
        for _ in 0..2 {
            ctx.create_notification().user_id(&user_id).call().await;
        }

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_id);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");

        assert_eq!(response.into_inner().total, 2);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    #[serial]
    async fn should_not_leak_other_users_counts(ctx: &mut TestContext) {
        let user_a = ctx.unique_user_id("count-leak-a");
        let user_b = ctx.unique_user_id("count-leak-b");

        for _ in 0..5 {
            ctx.create_notification().user_id(&user_a).call().await;
        }
        for _ in 0..3 {
            ctx.create_notification().user_id(&user_b).call().await;
        }

        let request = ctx.authenticated_request(GetUnreadCountRequest {}, &user_a);
        let response = get_unread_count(&ctx.service, request).await.expect("should succeed");

        assert_eq!(response.into_inner().total, 5);
    }
}
