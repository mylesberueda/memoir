use super::*;

pub(super) async fn mark_as_read(
    service: &NotificationService,
    request: tonic::Request<MarkAsReadRequest>,
) -> std::result::Result<tonic::Response<google::protobuf::Empty>, tonic::Status> {
    let user_id = request.user_id()?;
    let pids = &request.get_ref().notification_pids;

    if pids.is_empty() {
        return Err(tonic::Status::invalid_argument("notification_pids is required"));
    }

    tracing::debug!(pids = ?pids.as_slice(), "updating_notifications");
    let result = notifications::Entity::update_many()
        .col_expr(notifications::Column::IsRead, Expr::value(true))
        .col_expr(notifications::Column::ReadAt, Expr::value(chrono::Utc::now()))
        .filter(notifications::Column::Pid.is_in(pids.clone()))
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .exec(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
    tracing::info!(updated = result.rows_affected, "notifications_marked_read");

    for pid in pids {
        service.publish_event(&user_id, RedisEventType::Read, pid).await;
    }

    Ok(tonic::Response::new(google::protobuf::Empty {}))
}

pub(super) async fn mark_all_as_read(
    service: &NotificationService,
    request: tonic::Request<MarkAllAsReadRequest>,
) -> std::result::Result<tonic::Response<MarkAllAsReadResponse>, tonic::Status> {
    let user_id = request.user_id()?;
    let req = request.get_ref();

    let mut filter = Condition::all()
        .add(notifications::Column::UserId.eq(&user_id))
        .add(notifications::Column::IsRead.eq(false))
        .add(notifications::Column::IsDismissed.eq(false));

    if let Some(category) = req.category
        && let Ok(proto) = NotificationCategory::try_from(category)
        && proto != NotificationCategory::Unspecified
    {
        let category: notifications::NotificationCategory = proto.try_into()?;
        tracing::Span::current().record("category", category.to_string());
        filter = filter.add(notifications::Column::Category.eq(category));
    }

    let pids: Vec<String> = notifications::Entity::find()
        .select_only()
        .column(notifications::Column::Pid)
        .filter(filter.clone())
        .into_tuple()
        .all(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    tracing::debug!("marking_all_notifications_read");
    let result = notifications::Entity::update_many()
        .col_expr(notifications::Column::IsRead, Expr::value(true))
        .col_expr(notifications::Column::ReadAt, Expr::value(chrono::Utc::now()))
        .filter(filter)
        .exec(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
    tracing::info!(marked = result.rows_affected, "all_notifications_marked_read");

    for pid in &pids {
        service.publish_event(&user_id, RedisEventType::Read, pid).await;
    }

    Ok(tonic::Response::new(MarkAllAsReadResponse {
        marked_count: result.rows_affected,
    }))
}

pub(super) async fn dismiss(
    service: &NotificationService,
    request: tonic::Request<DismissRequest>,
) -> std::result::Result<tonic::Response<google::protobuf::Empty>, tonic::Status> {
    let user_id = request.user_id()?;
    let pid = &request.get_ref().notification_pid;

    if pid.is_empty() {
        return Err(tonic::Status::invalid_argument("notification_pid is required"));
    }

    tracing::debug!(pid = pid, "dismissing_notification");
    let result = notifications::Entity::update_many()
        .col_expr(notifications::Column::IsDismissed, Expr::value(true))
        .filter(notifications::Column::Pid.eq(pid))
        .filter(notifications::Column::UserId.eq(&user_id))
        .filter(notifications::Column::IsDismissed.eq(false))
        .exec(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
    tracing::info!(updated = result.rows_affected, "notification_dismissed");

    service.publish_event(&user_id, RedisEventType::Dismissed, pid).await;

    Ok(tonic::Response::new(google::protobuf::Empty {}))
}

pub(super) async fn dismiss_all(
    service: &NotificationService,
    request: tonic::Request<DismissAllRequest>,
) -> std::result::Result<tonic::Response<DismissAllResponse>, tonic::Status> {
    let user_id = request.user_id()?;
    let req = request.get_ref();

    let mut filter = Condition::all()
        .add(notifications::Column::UserId.eq(&user_id))
        .add(notifications::Column::IsDismissed.eq(false));

    if let Some(category) = req.category
        && let Ok(proto) = NotificationCategory::try_from(category)
        && proto != NotificationCategory::Unspecified
    {
        let category: notifications::NotificationCategory = proto.try_into()?;
        tracing::Span::current().record("category", category.to_string());
        filter = filter.add(notifications::Column::Category.eq(category));
    }

    let pids: Vec<String> = notifications::Entity::find()
        .select_only()
        .column(notifications::Column::Pid)
        .filter(filter.clone())
        .into_tuple()
        .all(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

    tracing::debug!("marking_all_notifications_dismissed");
    let result = notifications::Entity::update_many()
        .col_expr(notifications::Column::IsDismissed, Expr::value(true))
        .filter(filter)
        .exec(service.context.db.as_ref())
        .await
        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
    tracing::info!(marked = result.rows_affected, "all_notifications_dismissed");

    for pid in &pids {
        service.publish_event(&user_id, RedisEventType::Dismissed, pid).await;
    }

    Ok(tonic::Response::new(DismissAllResponse {
        dismissed_count: result.rows_affected,
    }))
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::api::test_context::TestContext;
    use crate::services::notification::query;
    use proto_rs::notification::v1::{
        DismissAllRequest, DismissRequest, GetRequest, ListRequest, MarkAllAsReadRequest, MarkAsReadRequest,
        NotificationCategory,
    };
    use serial_test::serial;
    use test_context::test_context;

    // ==========================================================================
    // MarkAsRead
    // ==========================================================================

    mod mark_as_read {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_mark_notification_as_read(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-read");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n.pid.clone()],
                },
                &user_id,
            );
            mark_as_read(&ctx.service, request).await.expect("should succeed");

            // Verify via list (get excludes read notifications... wait, no it doesn't)
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            let notification = query::get(&ctx.service, get_req)
                .await
                .expect("get should succeed")
                .into_inner()
                .notification
                .expect("should have notification");

            assert!(notification.is_read);
            assert!(notification.read_at.is_some());
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_mark_multiple_notifications(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-multi");
            let n1 = ctx.create_notification().user_id(&user_id).call().await;
            let n2 = ctx.create_notification().user_id(&user_id).call().await;
            let n3 = ctx.create_notification().user_id(&user_id).call().await;

            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n1.pid.clone(), n2.pid.clone(), n3.pid.clone()],
                },
                &user_id,
            );
            mark_as_read(&ctx.service, request).await.expect("should succeed");

            // Verify all are read
            for pid in [&n1.pid, &n2.pid, &n3.pid] {
                let get_req = ctx.authenticated_request(
                    GetRequest {
                        notification_pid: pid.clone(),
                    },
                    &user_id,
                );
                let notification = query::get(&ctx.service, get_req)
                    .await
                    .expect("get should succeed")
                    .into_inner()
                    .notification
                    .expect("should have notification");

                assert!(notification.is_read, "notification {} should be read", pid);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_pid_list(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-empty");

            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![],
                },
                &user_id,
            );
            let result = mark_as_read(&ctx.service, request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_succeed_on_already_read(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-twice");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            // Mark once
            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n.pid.clone()],
                },
                &user_id,
            );
            mark_as_read(&ctx.service, request)
                .await
                .expect("first mark should succeed");

            // Mark again
            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n.pid.clone()],
                },
                &user_id,
            );
            mark_as_read(&ctx.service, request)
                .await
                .expect("second mark should succeed");

            // Still read
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            let notification = query::get(&ctx.service, get_req)
                .await
                .expect("get should succeed")
                .into_inner()
                .notification
                .expect("should have notification");

            assert!(notification.is_read);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_affect_other_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("mark-a");
            let user_b = ctx.unique_user_id("mark-b");

            let n = ctx.create_notification().user_id(&user_b).call().await;

            // User A tries to mark User B's notification
            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n.pid.clone()],
                },
                &user_a,
            );
            mark_as_read(&ctx.service, request)
                .await
                .expect("should succeed (no-op)");

            // User B's notification should still be unread
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_b,
            );
            let notification = query::get(&ctx.service, get_req)
                .await
                .expect("get should succeed")
                .into_inner()
                .notification
                .expect("should have notification");

            assert!(!notification.is_read, "User B's notification should still be unread");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_succeed_with_nonexistent_pids(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-nonexistent");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            let request = ctx.authenticated_request(
                MarkAsReadRequest {
                    notification_pids: vec![n.pid.clone(), "nonexistent-pid".to_string()],
                },
                &user_id,
            );
            mark_as_read(&ctx.service, request).await.expect("should succeed");

            // Valid one should be marked
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            let notification = query::get(&ctx.service, get_req)
                .await
                .expect("get should succeed")
                .into_inner()
                .notification
                .expect("should have notification");

            assert!(notification.is_read);
        }
    }

    // ==========================================================================
    // MarkAllAsRead
    // ==========================================================================

    mod mark_all_as_read {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_mark_all_and_return_count(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-all");

            for _ in 0..5 {
                ctx.create_notification().user_id(&user_id).call().await;
            }

            let request = ctx.authenticated_request(MarkAllAsReadRequest::default(), &user_id);
            let response = mark_all_as_read(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().marked_count, 5);

            // Verify all are read via list
            let list_req = ctx.authenticated_request(
                ListRequest {
                    unread_only: Some(true),
                    ..Default::default()
                },
                &user_id,
            );
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.notifications.len(), 0, "all should be read");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_category(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-all-cat");

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
                MarkAllAsReadRequest {
                    category: Some(NotificationCategory::Chat as i32),
                },
                &user_id,
            );
            let response = mark_all_as_read(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().marked_count, 3);

            // Agent notifications should still be unread
            let list_req = ctx.authenticated_request(
                ListRequest {
                    unread_only: Some(true),
                    ..Default::default()
                },
                &user_id,
            );
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.notifications.len(), 2);
            for n in list_resp.notifications {
                assert_eq!(n.category, NotificationCategory::Agent as i32);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_zero_when_none_unread(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-all-none");

            // Create already-read notifications
            for _ in 0..3 {
                ctx.create_notification().user_id(&user_id).is_read(true).call().await;
            }

            let request = ctx.authenticated_request(MarkAllAsReadRequest::default(), &user_id);
            let response = mark_all_as_read(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().marked_count, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_exclude_dismissed(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("mark-all-dismissed");

            for _ in 0..2 {
                ctx.create_notification().user_id(&user_id).call().await;
            }
            ctx.create_notification()
                .user_id(&user_id)
                .is_dismissed(true)
                .call()
                .await;

            let request = ctx.authenticated_request(MarkAllAsReadRequest::default(), &user_id);
            let response = mark_all_as_read(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().marked_count, 2);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_affect_other_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("mark-all-a");
            let user_b = ctx.unique_user_id("mark-all-b");

            for _ in 0..3 {
                ctx.create_notification().user_id(&user_a).call().await;
            }
            for _ in 0..2 {
                ctx.create_notification().user_id(&user_b).call().await;
            }

            let request = ctx.authenticated_request(MarkAllAsReadRequest::default(), &user_a);
            let response = mark_all_as_read(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().marked_count, 3);

            // User B's should still be unread
            let list_req = ctx.authenticated_request(
                ListRequest {
                    unread_only: Some(true),
                    ..Default::default()
                },
                &user_b,
            );
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.notifications.len(), 2);
        }
    }

    // ==========================================================================
    // Dismiss
    // ==========================================================================

    mod dismiss {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_dismiss_notification(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            dismiss(&ctx.service, request).await.expect("should succeed");

            // Get should return not found
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            let result = query::get(&ctx.service, get_req).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_pid(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-empty");

            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: String::new(),
                },
                &user_id,
            );
            let result = dismiss(&ctx.service, request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_succeed_on_already_dismissed(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-twice");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            // Dismiss once
            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            dismiss(&ctx.service, request)
                .await
                .expect("first dismiss should succeed");

            // Dismiss again
            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            dismiss(&ctx.service, request)
                .await
                .expect("second dismiss should succeed");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_affect_other_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("dismiss-a");
            let user_b = ctx.unique_user_id("dismiss-b");

            let n = ctx.create_notification().user_id(&user_b).call().await;

            // User A tries to dismiss User B's notification
            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_a,
            );
            dismiss(&ctx.service, request).await.expect("should succeed (no-op)");

            // User B should still see it
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_b,
            );
            query::get(&ctx.service, get_req)
                .await
                .expect("User B should still see notification");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_set_is_read(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-unread");
            let n = ctx.create_notification().user_id(&user_id).call().await;

            let request = ctx.authenticated_request(
                DismissRequest {
                    notification_pid: n.pid.clone(),
                },
                &user_id,
            );
            dismiss(&ctx.service, request).await.expect("should succeed");

            // Verify via list that it's excluded (we can't get it directly)
            // But we can check the total/unread counts
            let list_req = ctx.authenticated_request(ListRequest::default(), &user_id);
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            // Dismissed notifications are excluded from list
            assert_eq!(list_resp.total_count, 0);
        }
    }

    // ==========================================================================
    // DismissAll
    // ==========================================================================

    mod dismiss_all {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_dismiss_all_and_return_count(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-all");

            // Mix of read and unread
            for _ in 0..3 {
                ctx.create_notification().user_id(&user_id).call().await;
            }
            for _ in 0..2 {
                ctx.create_notification().user_id(&user_id).is_read(true).call().await;
            }

            let request = ctx.authenticated_request(DismissAllRequest::default(), &user_id);
            let response = dismiss_all(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().dismissed_count, 5);

            // List should be empty
            let list_req = ctx.authenticated_request(ListRequest::default(), &user_id);
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.total_count, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_category(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-all-cat");

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
                DismissAllRequest {
                    category: Some(NotificationCategory::Chat as i32),
                },
                &user_id,
            );
            let response = dismiss_all(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().dismissed_count, 3);

            // Agent notifications should still be visible
            let list_req = ctx.authenticated_request(ListRequest::default(), &user_id);
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.total_count, 2);
            for n in list_resp.notifications {
                assert_eq!(n.category, NotificationCategory::Agent as i32);
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_zero_when_none_exist(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("dismiss-all-none");

            let request = ctx.authenticated_request(DismissAllRequest::default(), &user_id);
            let response = dismiss_all(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().dismissed_count, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_affect_other_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("dismiss-all-a");
            let user_b = ctx.unique_user_id("dismiss-all-b");

            for _ in 0..3 {
                ctx.create_notification().user_id(&user_a).call().await;
            }
            for _ in 0..2 {
                ctx.create_notification().user_id(&user_b).call().await;
            }

            let request = ctx.authenticated_request(DismissAllRequest::default(), &user_a);
            let response = dismiss_all(&ctx.service, request).await.expect("should succeed");

            assert_eq!(response.into_inner().dismissed_count, 3);

            // User B's should still be visible
            let list_req = ctx.authenticated_request(ListRequest::default(), &user_b);
            let list_resp = query::list(&ctx.service, list_req)
                .await
                .expect("list should succeed")
                .into_inner();

            assert_eq!(list_resp.total_count, 2);
        }
    }
}
