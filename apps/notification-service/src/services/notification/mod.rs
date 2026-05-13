mod actions;
mod preferences;
mod query;

use crate::{
    AppContext, REDIS_PROJECT_PREFIX,
    models::{notification_preferences, notifications},
};
use chrono::DateTime;
use fred::prelude::{EventInterface, PubsubInterface as _};
use platform_rs::ext::RequestAuthExt;
use proto_rs::{google, notification::v1::*};
use sea_orm::{
    ActiveModelTrait as _, ActiveValue::Set, ColumnTrait as _, Condition, EntityTrait as _, ExprTrait as _,
    FromQueryResult, PaginatorTrait as _, QueryFilter as _, QueryOrder, QuerySelect as _, prelude::Expr,
};
use std::{collections::HashMap, sync::Arc};
use tokio_stream::wrappers::ReceiverStream;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum RedisEventType {
    New,
    Updated,
    Read,
    Dismissed,
    StreamUpdate,
}

#[derive(Debug)]
pub(crate) struct NotificationService {
    context: Arc<AppContext>,
}

impl NotificationService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }

    async fn publish_event(&self, user_id: &str, event_type: RedisEventType, pid: &str) {
        let channel = Self::stream_key(user_id);
        let payload = serde_json::json!({ "type": event_type, "pid": pid }).to_string();

        if let Err(e) = self.context.redis.publish::<i64, _, _>(&channel, &payload).await {
            tracing::warn!(channel, error = %e, "redis_publish_failed");
        } else {
            tracing::debug!(channel, event_type = ?event_type, pid, "redis_event_published");
        }
    }

    async fn publish_stream_update(&self, user_id: &str, pid: &str, update: &StreamUpdate) {
        let channel = Self::stream_key(user_id);

        let payload = serde_json::json!({
          "type": RedisEventType::StreamUpdate,
          "pid": pid,
          "title": update.title,
          "description": update.description,
          "icon_url": update.icon_url,
          "priority": update.priority,
        })
        .to_string();

        if let Err(e) = self.context.redis.publish::<i64, _, _>(&channel, &payload).await {
            tracing::warn!(channel, error = %e, "redis_stream_update_publish_failed");
        } else {
            tracing::debug!(channel, pid, "redis_stream_update_published");
        }
    }

    fn stream_key(user_id: &str) -> String {
        format!("{REDIS_PROJECT_PREFIX}:user:{user_id}")
    }
}

#[tonic::async_trait]
impl notification_service_server::NotificationService for NotificationService {
    #[instrument(skip(self, request), fields(user_id))]
    async fn push(
        &self,
        request: tonic::Request<PushRequest>,
    ) -> std::result::Result<tonic::Response<PushResponse>, tonic::Status> {
        let req = request.get_ref();

        if req.user_id.is_empty() {
            return Err(tonic::Status::invalid_argument("user_id is required"));
        }

        tracing::Span::current().record("user_id", &req.user_id);

        if req.title.is_empty() {
            return Err(tonic::Status::invalid_argument("title is required"));
        }

        if req.description.is_empty() {
            return Err(tonic::Status::invalid_argument("description is required"));
        }

        if let Some(ref key) = req.idempotency_key {
            tracing::debug!(key, "idempotency_key_check");
            if let Some(existing) = notifications::Entity::find()
                .filter(notifications::Column::IdempotencyKey.eq(key))
                .filter(notifications::Column::UserId.eq(&req.user_id))
                .one(self.context.db.as_ref())
                .await
                .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
            {
                tracing::debug!(id = existing.id, "Deduplicated notification via idempotency key");
                return Ok(tonic::Response::new(PushResponse {
                    id: existing.id,
                    notification_pid: existing.pid,
                    deduplicated: true,
                }));
            }
        }

        let actions: Option<serde_json::Value> = if req.actions.is_empty() {
            None
        } else {
            Some(
                serde_json::to_value(&req.actions)
                    .map_err(|e| tonic::Status::internal(format!("Failed to serialize actions: {e}")))?,
            )
        };

        let origin_entity_type = req
            .origin_entity_type
            .map(|v| {
                OriginEntityType::try_from(v)
                    .map_err(|_| tonic::Status::invalid_argument("invalid origin_entity_type"))?
                    .try_into()
            })
            .transpose()?;

        let timestamp = req
            .expires_at
            .as_deref()
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok().map(|t| t.to_utc().into()));

        tracing::debug!("creating_notification");
        let notification = notifications::ActiveModel {
            user_id: Set(req.user_id.clone()),
            org_pid: Set(req.org_pid.clone()),
            title: Set(req.title.clone()),
            description: Set(req.description.clone()),
            icon_url: Set(req.icon_url.clone()),
            category: Set(req.category().try_into()?),
            priority: Set(req.priority().into()),
            actions: Set(actions),
            origin_service: Set(req.origin_service().try_into()?),
            origin_entity_type: Set(origin_entity_type),
            origin_entity_pid: Set(req.origin_entity_pid.clone()),
            expires_at: Set(timestamp),
            idempotency_key: Set(req.idempotency_key.clone()),
            ..Default::default()
        };

        let notification = notification
            .insert(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

        tracing::info!(id = notification.id, pid = notification.pid, "notification_created");

        self.publish_event(&req.user_id, RedisEventType::New, &notification.pid)
            .await;

        Ok(tonic::Response::new(PushResponse {
            id: notification.id,
            notification_pid: notification.pid,
            deduplicated: false,
        }))
    }

    #[instrument(skip(self, request))]
    async fn push_batch(
        &self,
        request: tonic::Request<PushBatchRequest>,
    ) -> std::result::Result<tonic::Response<PushBatchResponse>, tonic::Status> {
        let req = request.get_ref();

        if req.notifications.is_empty() {
            return Err(tonic::Status::invalid_argument("notifications cannot be empty"));
        }

        for (i, n) in req.notifications.iter().enumerate() {
            if n.user_id.is_empty() {
                return Err(tonic::Status::invalid_argument(format!(
                    "notifications[{i}]: user_id is required"
                )));
            }

            if n.title.is_empty() {
                return Err(tonic::Status::invalid_argument(format!(
                    "notifications[{i}]: title is required"
                )));
            }

            if n.description.is_empty() {
                return Err(tonic::Status::invalid_argument(format!(
                    "notifications[{i}]: description is required"
                )));
            }
        }

        let mut keys: Vec<(String, String)> = Vec::new();
        for n in &req.notifications {
            if let Some(ref key) = n.idempotency_key {
                keys.push((n.user_id.clone(), key.clone()))
            }
        }

        let existing_pids: HashMap<(String, String), String> = if !keys.is_empty() {
            let mut condition = Condition::any();
            for (user_id, key) in &keys {
                condition = condition.add(
                    Condition::all()
                        .add(notifications::Column::UserId.eq(user_id))
                        .add(notifications::Column::IdempotencyKey.eq(key)),
                );
            }

            notifications::Entity::find()
                .select_only()
                .column(notifications::Column::UserId)
                .column(notifications::Column::IdempotencyKey)
                .column(notifications::Column::Pid)
                .filter(condition)
                .into_tuple()
                .all(self.context.db.as_ref())
                .await
                .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
                .into_iter()
                .map(|(user_id, key, pid)| ((user_id, key), pid))
                .collect()
        } else {
            HashMap::new()
        };

        let mut to_insert: Vec<notifications::ActiveModel> = Vec::new();
        let mut result_pids: Vec<String> = Vec::new();
        let mut deduplicated_count: u64 = 0;

        for n in &req.notifications {
            if let Some(ref key) = n.idempotency_key
                && let Some(existing_pid) = existing_pids.get(&(n.user_id.clone(), key.clone()))
            {
                result_pids.push(existing_pid.clone());
                deduplicated_count += 1;
                continue;
            }

            let actions: Option<serde_json::Value> = if n.actions.is_empty() {
                None
            } else {
                Some(
                    serde_json::to_value(&n.actions)
                        .map_err(|e| tonic::Status::internal(format!("Failed to serialize actions: {e}")))?,
                )
            };

            let origin_entity_type = n
                .origin_entity_type
                .map(|v| {
                    OriginEntityType::try_from(v)
                        .map_err(|_| tonic::Status::invalid_argument("invalid origin_entity_type"))?
                        .try_into()
                })
                .transpose()?;

            let expires_at = n
                .expires_at
                .as_deref()
                .and_then(|t| DateTime::parse_from_rfc3339(t).ok().map(|t| t.to_utc().into()));

            let pid = nanoid::nanoid!();
            result_pids.push(pid.clone());

            to_insert.push(notifications::ActiveModel {
                pid: Set(pid),
                user_id: Set(n.user_id.clone()),
                org_pid: Set(n.org_pid.clone()),
                title: Set(n.title.clone()),
                description: Set(n.description.clone()),
                icon_url: Set(n.icon_url.clone()),
                category: Set(n.category().try_into()?),
                priority: Set(n.priority().into()),
                actions: Set(actions),
                origin_service: Set(n.origin_service().try_into()?),
                origin_entity_type: Set(origin_entity_type),
                origin_entity_pid: Set(n.origin_entity_pid.clone()),
                expires_at: Set(expires_at),
                idempotency_key: Set(n.idempotency_key.clone()),
                ..Default::default()
            });
        }

        let created_count = to_insert.len() as u64;

        if !to_insert.is_empty() {
            notifications::Entity::insert_many(to_insert)
                .exec(self.context.db.as_ref())
                .await
                .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
        }

        tracing::info!(
            created = created_count,
            deduplicated = deduplicated_count,
            "batch_push_complete"
        );

        for (i, n) in req.notifications.iter().enumerate() {
            let deduplicated = n
                .idempotency_key
                .as_ref()
                .is_some_and(|k| existing_pids.contains_key(&(n.user_id.clone(), k.clone())));

            if !deduplicated {
                self.publish_event(&n.user_id, RedisEventType::New, &result_pids[i])
                    .await;
            }
        }

        Ok(tonic::Response::new(PushBatchResponse {
            created_count,
            deduplicated_count,
            notification_pids: result_pids,
        }))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn update(
        &self,
        request: tonic::Request<UpdateRequest>,
    ) -> std::result::Result<tonic::Response<UpdateResponse>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.get_ref();

        tracing::debug!(id = req.id, "fetching_notification_for_update");
        let notification = notifications::Entity::find_by_id(req.id)
            .filter(notifications::Column::UserId.eq(&user_id))
            .filter(notifications::Column::IsDismissed.eq(false))
            .one(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?
            .ok_or(tonic::Status::not_found("Notification not found"))?;

        let pid = notification.pid.clone();

        let mut active: notifications::ActiveModel = notification.into();

        if let Some(ref title) = req.title {
            active.title = Set(title.clone());
        }

        if let Some(ref description) = req.description {
            active.description = Set(description.clone());
        }

        if let Some(ref icon_url) = req.icon_url {
            active.icon_url = Set(Some(icon_url.clone()));
        }

        if let Some(priority) = req.priority
            && let Ok(proto) = NotificationPriority::try_from(priority)
        {
            active.priority = Set(proto.into());
        }

        if !req.actions.is_empty() {
            active.actions =
                Set(Some(serde_json::to_value(&req.actions).map_err(|e| {
                    tonic::Status::internal(format!("Failed to serialize actions: {e}"))
                })?));
        }

        let updated = active
            .update(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
        tracing::info!(id = updated.id, pid = %pid, "notification_updated");

        self.publish_event(&user_id, RedisEventType::Updated, &pid).await;

        Ok(tonic::Response::new(UpdateResponse {
            notification: Some(updated.into()),
        }))
    }

    #[instrument(skip(self, request), fields(user_id, pid))]
    async fn push_stream(
        &self,
        request: tonic::Request<tonic::Streaming<PushStreamMessage>>,
    ) -> std::result::Result<tonic::Response<PushStreamResponse>, tonic::Status> {
        let mut stream = request.into_inner();
        let first_msg = stream
            .message()
            .await
            .map_err(|e| tonic::Status::internal(format!("Stream error: {e}")))?
            .ok_or(tonic::Status::invalid_argument("Stream closed without init message"))?;

        let init_req = match first_msg.message {
            Some(push_stream_message::Message::Init(req)) => req,
            Some(_) => return Err(tonic::Status::invalid_argument("First message must be init")),
            None => return Err(tonic::Status::invalid_argument("Empty message")),
        };

        if init_req.user_id.is_empty() {
            return Err(tonic::Status::invalid_argument("user_id is required"));
        }

        if init_req.title.is_empty() {
            return Err(tonic::Status::invalid_argument("title is required"));
        }

        if init_req.description.is_empty() {
            return Err(tonic::Status::invalid_argument("description is required"));
        }

        let user_id = init_req.user_id.clone();
        tracing::Span::current().record("user_id", &user_id);

        let actions: Option<serde_json::Value> = if init_req.actions.is_empty() {
            None
        } else {
            Some(
                serde_json::to_value(&init_req.actions)
                    .map_err(|e| tonic::Status::internal(format!("Failed to serialize actions: {e}")))?,
            )
        };

        let origin_entity_type = init_req
            .origin_entity_type
            .map(|v| {
                OriginEntityType::try_from(v)
                    .map_err(|_| tonic::Status::invalid_argument("invalid origin_entity_type"))?
                    .try_into()
            })
            .transpose()?;

        let timestamp = init_req
            .expires_at
            .as_deref()
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok().map(|t| t.to_utc().into()));

        let notification = notifications::ActiveModel {
            user_id: Set(init_req.user_id.clone()),
            org_pid: Set(init_req.org_pid.clone()),
            title: Set(init_req.title.clone()),
            description: Set(init_req.description.clone()),
            icon_url: Set(init_req.icon_url.clone()),
            category: Set(init_req.category().try_into()?),
            priority: Set(init_req.priority().into()),
            actions: Set(actions),
            origin_service: Set(init_req.origin_service().try_into()?),
            origin_entity_type: Set(origin_entity_type),
            origin_entity_pid: Set(init_req.origin_entity_pid.clone()),
            expires_at: Set(timestamp),
            idempotency_key: Set(init_req.idempotency_key.clone()),
            ..Default::default()
        };

        tracing::debug!("creating_notification");
        let notification = notification
            .insert(self.context.db.as_ref())
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
        tracing::Span::current().record("pid", &notification.pid);
        tracing::debug!(id = notification.id, "notification_created");

        self.publish_event(&user_id, RedisEventType::New, &notification.pid)
            .await;

        let mut last_update: Option<StreamUpdate> = None;
        let mut completed = false;

        while let Some(msg) = stream
            .message()
            .await
            .map_err(|e| tonic::Status::internal(format!("Stream error: {e}")))?
        {
            match msg.message {
                Some(push_stream_message::Message::Init(_)) => {
                    tracing::warn!(id = &notification.id, "duplicate_init_ignored");
                }
                Some(push_stream_message::Message::Update(update)) => {
                    // Ephemeral update - fan out via Redis, no db write
                    last_update = Some(update.clone());
                    self.publish_stream_update(&user_id, &notification.pid, &update).await;
                    tracing::debug!(id = &notification.id, "stream_update_published");
                }
                Some(push_stream_message::Message::Complete(complete)) => {
                    let mut active: notifications::ActiveModel = notification.clone().into();
                    active.title = Set(complete.title.clone());
                    active.description = Set(complete.description.clone());

                    if !complete.actions.is_empty() {
                        active.actions =
                            Set(Some(serde_json::to_value(&complete.actions).map_err(|e| {
                                tonic::Status::internal(format!("Failed to serialize actions: {e}"))
                            })?));
                    }

                    active
                        .update(self.context.db.as_ref())
                        .await
                        .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

                    tracing::info!(id = notification.id, "stream_complete");
                    self.publish_event(&user_id, RedisEventType::Updated, &notification.pid)
                        .await;
                    completed = true;
                    break;
                }
                None => continue,
            }
        }

        if !completed && let Some(update) = last_update {
            let mut active: notifications::ActiveModel = notification.clone().into();

            if let Some(ref title) = update.title {
                active.title = Set(title.clone());
            }

            if let Some(ref description) = update.description {
                active.description = Set(description.clone());
            }

            if let Some(ref icon_url) = update.icon_url {
                active.icon_url = Set(Some(icon_url.clone()))
            }

            if let Some(priority) = update.priority
                && let Ok(proto) = NotificationPriority::try_from(priority)
            {
                active.priority = Set(proto.into());
            }

            if !update.actions.is_empty() {
                active.actions =
                    Set(Some(serde_json::to_value(&update.actions).map_err(|e| {
                        tonic::Status::internal(format!("Failed to serialize actions: {e}"))
                    })?));
            }

            active
                .update(self.context.db.as_ref())
                .await
                .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;

            tracing::warn!(
                id = notification.id,
                "stream_closed_without_complete_persisted_last_state"
            );
            self.publish_event(&user_id, RedisEventType::Updated, &notification.pid)
                .await;
        }

        Ok(tonic::Response::new(PushStreamResponse {
            id: notification.id,
            notification_pid: notification.pid,
        }))
    }

    #[doc = " Server streaming response type for the Subscribe method."]
    type SubscribeStream = ReceiverStream<Result<NotificationEvent, tonic::Status>>;

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn subscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> std::result::Result<tonic::Response<Self::SubscribeStream>, tonic::Status> {
        let user_id = request.user_id()?;
        let req = request.get_ref();

        let category_filter: Vec<notifications::NotificationCategory> = req
            .categories
            .iter()
            .filter_map(|&c| {
                NotificationCategory::try_from(c)
                    .ok()
                    .and_then(|c| notifications::NotificationCategory::try_from(c).ok())
            })
            .collect();

        let priority_filter: Option<notifications::NotificationPriority> = req
            .min_priority
            .and_then(|p| NotificationPriority::try_from(p).ok())
            .filter(|&p| p != NotificationPriority::Unspecified && p != NotificationPriority::Low)
            .map(Into::into);

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<NotificationEvent, tonic::Status>>(32);

        let channel = NotificationService::stream_key(&user_id);
        self.context
            .redis_subscriber
            .subscribe(&channel)
            .await
            .map_err(|e| tonic::Status::internal(format!("Redis error: {e}")))?;

        tracing::info!(channel = %channel, "subscribed_to_notifications");
        let db = self.context.db.clone();
        let mut message_rx = self.context.redis_subscriber.message_rx();
        let subscriber = self.context.redis_subscriber.clone();
        let expected_channel = channel.clone();

        #[derive(serde::Deserialize)]
        struct RedisEvent {
            #[serde(rename = "type")]
            event_type: RedisEventType,
            pid: String,
            #[serde(default)]
            title: Option<String>,
            #[serde(default)]
            description: Option<String>,
            #[serde(default)]
            icon_url: Option<String>,
            #[serde(default)]
            priority: Option<i32>,
        }

        tokio::spawn(async move {
            while let Ok(message) = message_rx.recv().await {
                if message.channel != expected_channel {
                    continue;
                }

                let payload: RedisEvent = match serde_json::from_str(&message.value.as_str().unwrap_or_default()) {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::warn!(error = %e, "invalid_redis_message");
                        continue;
                    }
                };

                if payload.pid.is_empty() {
                    continue;
                }

                let event = match payload.event_type {
                    RedisEventType::New | RedisEventType::Updated => {
                        let notification = match notifications::Entity::find()
                            .filter(notifications::Column::UserId.eq(&user_id))
                            .filter(notifications::Column::Pid.eq(&payload.pid))
                            .filter(notifications::Column::IsDismissed.eq(false))
                            .one(db.as_ref())
                            .await
                        {
                            Ok(Some(n)) => n,
                            Ok(None) => continue, // Already exists or doesn't exist
                            Err(e) => {
                                tracing::warn!(error = %e, pid = payload.pid, "db_lookup_failed");
                                continue;
                            }
                        };

                        if !category_filter.is_empty() && !category_filter.contains(&notification.category) {
                            continue;
                        }

                        if priority_filter.as_ref().is_some_and(|p| &notification.priority < p) {
                            continue;
                        }

                        let proto: Notification = notification.into();

                        if payload.event_type == RedisEventType::New {
                            NotificationEvent {
                                event: Some(notification_event::Event::Notification(proto)),
                            }
                        } else {
                            NotificationEvent {
                                event: Some(notification_event::Event::Updated(NotificationUpdated {
                                    notification: Some(proto),
                                })),
                            }
                        }
                    }
                    RedisEventType::StreamUpdate => NotificationEvent {
                        event: Some(notification_event::Event::Progress(NotificationProgress {
                            notification_pid: payload.pid,
                            title: payload.title,
                            description: payload.description,
                            icon_url: payload.icon_url,
                            priority: payload.priority,
                        })),
                    },
                    RedisEventType::Read => NotificationEvent {
                        event: Some(notification_event::Event::Read(NotificationRead {
                            notification_pid: payload.pid.to_string(),
                            read_at: chrono::Utc::now().to_rfc3339(),
                        })),
                    },
                    RedisEventType::Dismissed => NotificationEvent {
                        event: Some(notification_event::Event::Dismissed(NotificationDismissed {
                            notification_pid: payload.pid.to_string(),
                        })),
                    },
                };

                if tx.send(Ok(event)).await.is_err() {
                    tracing::debug!("client_disconnected");
                    break;
                }
            }

            if let Err(e) = subscriber.unsubscribe(&expected_channel).await {
                tracing::warn!(error = %e, channel = %expected_channel, "unsubscribe_failed");
            }

            tracing::info!(channel = %expected_channel, "subscription_ended");
        });

        Ok(tonic::Response::new(ReceiverStream::new(rx)))
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn list(
        &self,
        request: tonic::Request<ListRequest>,
    ) -> std::result::Result<tonic::Response<ListResponse>, tonic::Status> {
        query::list(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> std::result::Result<tonic::Response<GetResponse>, tonic::Status> {
        query::get(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn get_unread_count(
        &self,
        request: tonic::Request<GetUnreadCountRequest>,
    ) -> std::result::Result<tonic::Response<GetUnreadCountResponse>, tonic::Status> {
        query::get_unread_count(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn mark_as_read(
        &self,
        request: tonic::Request<MarkAsReadRequest>,
    ) -> std::result::Result<tonic::Response<google::protobuf::Empty>, tonic::Status> {
        actions::mark_as_read(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, category))]
    async fn mark_all_as_read(
        &self,
        request: tonic::Request<MarkAllAsReadRequest>,
    ) -> std::result::Result<tonic::Response<MarkAllAsReadResponse>, tonic::Status> {
        actions::mark_all_as_read(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn dismiss(
        &self,
        request: tonic::Request<DismissRequest>,
    ) -> std::result::Result<tonic::Response<google::protobuf::Empty>, tonic::Status> {
        actions::dismiss(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?, category))]
    async fn dismiss_all(
        &self,
        request: tonic::Request<DismissAllRequest>,
    ) -> std::result::Result<tonic::Response<DismissAllResponse>, tonic::Status> {
        actions::dismiss_all(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn get_preferences(
        &self,
        request: tonic::Request<GetPreferencesRequest>,
    ) -> std::result::Result<tonic::Response<GetPreferencesResponse>, tonic::Status> {
        preferences::get(self, request).await
    }

    #[instrument(skip(self, request), fields(user_id = %request.user_id()?))]
    async fn update_preferences(
        &self,
        request: tonic::Request<UpdatePreferencesRequest>,
    ) -> std::result::Result<tonic::Response<UpdatePreferencesResponse>, tonic::Status> {
        preferences::update(self, request).await
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use crate::api::test_context::TestContext;
    use proto_rs::notification::v1::{
        GetRequest, NotificationCategory, NotificationPriority, OriginService, PushBatchRequest, PushRequest,
        SubscribeRequest, UpdateRequest, notification_event, notification_service_server::NotificationService as _,
    };
    use serial_test::serial;
    use test_context::test_context;
    use tokio_stream::StreamExt;

    // ==========================================================================
    // Push
    // ==========================================================================

    mod push {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_notification_and_return_id_pid(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-create");

            let request = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Test Title".to_string(),
                description: "Test Description".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });

            let response = ctx.service.push(request).await.expect("push should succeed");
            let resp = response.into_inner();

            assert!(resp.id > 0);
            assert!(!resp.notification_pid.is_empty());
            assert!(!resp.deduplicated);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_missing_user_id(ctx: &mut TestContext) {
            let request = tonic::Request::new(PushRequest {
                user_id: String::new(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });

            let result = ctx.service.push(request).await;

            assert!(result.is_err());
            let status = result.unwrap_err();
            assert_eq!(status.code(), tonic::Code::InvalidArgument);
            assert!(status.message().contains("user_id"));
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_missing_title(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-no-title");

            let request = tonic::Request::new(PushRequest {
                user_id,
                title: String::new(),
                description: "Test".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });

            let result = ctx.service.push(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_missing_description(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-no-desc");

            let request = tonic::Request::new(PushRequest {
                user_id,
                title: "Test".to_string(),
                description: String::new(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });

            let result = ctx.service.push(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deduplicate_on_same_idempotency_key(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-dedup");
            let key = format!("test-key-{}", nanoid::nanoid!());

            // First push
            let request = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "First".to_string(),
                description: "First".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                idempotency_key: Some(key.clone()),
                ..Default::default()
            });
            let first = ctx.service.push(request).await.expect("first push should succeed");
            let first_resp = first.into_inner();
            assert!(!first_resp.deduplicated);

            // Second push with same key
            let request = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Second".to_string(),
                description: "Second".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                idempotency_key: Some(key),
                ..Default::default()
            });
            let second = ctx.service.push(request).await.expect("second push should succeed");
            let second_resp = second.into_inner();

            assert!(second_resp.deduplicated);
            assert_eq!(second_resp.notification_pid, first_resp.notification_pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_deduplicate_across_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("push-dedup-a");
            let user_b = ctx.unique_user_id("push-dedup-b");
            let key = format!("shared-key-{}", nanoid::nanoid!());

            // User A
            let request = tonic::Request::new(PushRequest {
                user_id: user_a.clone(),
                title: "User A".to_string(),
                description: "User A".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                idempotency_key: Some(key.clone()),
                ..Default::default()
            });
            let resp_a = ctx.service.push(request).await.unwrap().into_inner();

            // User B with same key
            let request = tonic::Request::new(PushRequest {
                user_id: user_b.clone(),
                title: "User B".to_string(),
                description: "User B".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                idempotency_key: Some(key),
                ..Default::default()
            });
            let resp_b = ctx.service.push(request).await.unwrap().into_inner();

            assert!(!resp_b.deduplicated);
            assert_ne!(resp_a.notification_pid, resp_b.notification_pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_default_priority_to_normal(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-priority");

            let request = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                // priority not set
                ..Default::default()
            });
            let resp = ctx.service.push(request).await.unwrap().into_inner();

            // Verify via get
            let get_req = ctx.authenticated_request(
                GetRequest {
                    notification_pid: resp.notification_pid,
                },
                &user_id,
            );
            let notification = crate::services::notification::query::get(&ctx.service, get_req)
                .await
                .unwrap()
                .into_inner()
                .notification
                .unwrap();

            assert_eq!(notification.priority, NotificationPriority::Normal as i32);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_unspecified_category(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("push-no-cat");

            let request = tonic::Request::new(PushRequest {
                user_id,
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: NotificationCategory::Unspecified as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });

            let result = ctx.service.push(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }
    }

    // ==========================================================================
    // PushBatch
    // ==========================================================================

    mod push_batch {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_create_batch_and_return_ordered_pids(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("batch-create");

            let request = tonic::Request::new(PushBatchRequest {
                notifications: vec![
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "First".to_string(),
                        description: "First".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "Second".to_string(),
                        description: "Second".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "Third".to_string(),
                        description: "Third".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                ],
            });

            let resp = ctx.service.push_batch(request).await.unwrap().into_inner();

            assert_eq!(resp.created_count, 3);
            assert_eq!(resp.deduplicated_count, 0);
            assert_eq!(resp.notification_pids.len(), 3);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_empty_batch(ctx: &mut TestContext) {
            let request = tonic::Request::new(PushBatchRequest { notifications: vec![] });

            let result = ctx.service.push_batch(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_reject_batch_with_missing_required_field(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("batch-invalid");

            let request = tonic::Request::new(PushBatchRequest {
                notifications: vec![
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "Valid".to_string(),
                        description: "Valid".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: String::new(), // Missing title
                        description: "Invalid".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                ],
            });

            let result = ctx.service.push_batch(request).await;

            assert!(result.is_err());
            let status = result.unwrap_err();
            assert_eq!(status.code(), tonic::Code::InvalidArgument);
            assert!(status.message().contains("[1]"), "should mention index");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_deduplicate_against_existing_notifications(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("batch-dedup");
            let key = format!("batch-key-{}", nanoid::nanoid!());

            // First create one via push
            let request = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Existing".to_string(),
                description: "Existing".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                idempotency_key: Some(key.clone()),
                ..Default::default()
            });
            let existing = ctx.service.push(request).await.unwrap().into_inner();

            // Then batch with same key
            let request = tonic::Request::new(PushBatchRequest {
                notifications: vec![PushRequest {
                    user_id: user_id.clone(),
                    title: "Duplicate".to_string(),
                    description: "Duplicate".to_string(),
                    category: NotificationCategory::System as i32,
                    origin_service: OriginService::Api as i32,
                    idempotency_key: Some(key),
                    ..Default::default()
                }],
            });

            let resp = ctx.service.push_batch(request).await.unwrap().into_inner();

            assert_eq!(resp.created_count, 0);
            assert_eq!(resp.deduplicated_count, 1);
            assert_eq!(resp.notification_pids[0], existing.notification_pid);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_deduplicate_same_key_different_users(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("batch-a");
            let user_b = ctx.unique_user_id("batch-b");
            let key = format!("shared-batch-{}", nanoid::nanoid!());

            let request = tonic::Request::new(PushBatchRequest {
                notifications: vec![
                    PushRequest {
                        user_id: user_a.clone(),
                        title: "User A".to_string(),
                        description: "User A".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        idempotency_key: Some(key.clone()),
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_b.clone(),
                        title: "User B".to_string(),
                        description: "User B".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        idempotency_key: Some(key),
                        ..Default::default()
                    },
                ],
            });

            let resp = ctx.service.push_batch(request).await.unwrap().into_inner();

            assert_eq!(resp.created_count, 2);
            assert_eq!(resp.deduplicated_count, 0);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_accurate_counts(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("batch-counts");
            let key1 = format!("key1-{}", nanoid::nanoid!());
            let key2 = format!("key2-{}", nanoid::nanoid!());

            // Pre-create 2 notifications
            for key in [&key1, &key2] {
                let request = tonic::Request::new(PushRequest {
                    user_id: user_id.clone(),
                    title: "Existing".to_string(),
                    description: "Existing".to_string(),
                    category: NotificationCategory::System as i32,
                    origin_service: OriginService::Api as i32,
                    idempotency_key: Some(key.clone()),
                    ..Default::default()
                });
                ctx.service.push(request).await.unwrap();
            }

            // Batch: 2 duplicates + 3 new
            let request = tonic::Request::new(PushBatchRequest {
                notifications: vec![
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "Dup1".to_string(),
                        description: "Dup1".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        idempotency_key: Some(key1),
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "Dup2".to_string(),
                        description: "Dup2".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        idempotency_key: Some(key2),
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "New1".to_string(),
                        description: "New1".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "New2".to_string(),
                        description: "New2".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                    PushRequest {
                        user_id: user_id.clone(),
                        title: "New3".to_string(),
                        description: "New3".to_string(),
                        category: NotificationCategory::System as i32,
                        origin_service: OriginService::Api as i32,
                        ..Default::default()
                    },
                ],
            });

            let resp = ctx.service.push_batch(request).await.unwrap().into_inner();

            assert_eq!(resp.created_count, 3);
            assert_eq!(resp.deduplicated_count, 2);
            assert_eq!(resp.notification_pids.len(), 5);
        }
    }

    // ==========================================================================
    // Update
    // ==========================================================================

    mod update {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_and_return_modified_notification(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("update-basic");

            // Create notification
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Original".to_string(),
                description: "Original".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let created = ctx.service.push(push_req).await.unwrap().into_inner();

            // Update it
            let update_req = ctx.authenticated_request(
                UpdateRequest {
                    id: created.id,
                    title: Some("Updated".to_string()),
                    ..Default::default()
                },
                &user_id,
            );
            let updated = ctx.service.update(update_req).await.unwrap().into_inner();
            let notification = updated.notification.expect("should have notification");

            assert_eq!(notification.title, "Updated");
            assert_eq!(notification.description, "Original"); // unchanged
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_return_not_found_for_invalid_id(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("update-invalid");

            let request = ctx.authenticated_request(
                UpdateRequest {
                    id: 999999,
                    title: Some("Test".to_string()),
                    ..Default::default()
                },
                &user_id,
            );

            let result = ctx.service.update(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_update_other_users_notifications(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("update-a");
            let user_b = ctx.unique_user_id("update-b");

            // User A creates notification
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_a.clone(),
                title: "User A's".to_string(),
                description: "User A's".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let created = ctx.service.push(push_req).await.unwrap().into_inner();

            // User B tries to update it
            let update_req = ctx.authenticated_request(
                UpdateRequest {
                    id: created.id,
                    title: Some("Hacked".to_string()),
                    ..Default::default()
                },
                &user_b,
            );

            let result = ctx.service.update(update_req).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_update_dismissed_notifications(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("update-dismissed");

            // Create and dismiss
            let n = ctx
                .create_notification()
                .user_id(&user_id)
                .is_dismissed(true)
                .call()
                .await;

            let request = ctx.authenticated_request(
                UpdateRequest {
                    id: n.id,
                    title: Some("Updated".to_string()),
                    ..Default::default()
                },
                &user_id,
            );

            let result = ctx.service.update(request).await;

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), tonic::Code::NotFound);
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_support_partial_updates(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("update-partial");

            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Original Title".to_string(),
                description: "Original Description".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let created = ctx.service.push(push_req).await.unwrap().into_inner();

            // Update only title
            let update_req = ctx.authenticated_request(
                UpdateRequest {
                    id: created.id,
                    title: Some("New Title".to_string()),
                    description: None,
                    ..Default::default()
                },
                &user_id,
            );
            let updated = ctx.service.update(update_req).await.unwrap().into_inner();
            let notification = updated.notification.unwrap();

            assert_eq!(notification.title, "New Title");
            assert_eq!(notification.description, "Original Description");
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_update_all_mutable_fields(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("update-all");

            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Original".to_string(),
                description: "Original".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                priority: NotificationPriority::Low as i32,
                ..Default::default()
            });
            let created = ctx.service.push(push_req).await.unwrap().into_inner();

            let update_req = ctx.authenticated_request(
                UpdateRequest {
                    id: created.id,
                    title: Some("New Title".to_string()),
                    description: Some("New Description".to_string()),
                    icon_url: Some("https://example.com/icon.png".to_string()),
                    priority: Some(NotificationPriority::Urgent as i32),
                    ..Default::default()
                },
                &user_id,
            );
            let updated = ctx.service.update(update_req).await.unwrap().into_inner();
            let notification = updated.notification.unwrap();

            assert_eq!(notification.title, "New Title");
            assert_eq!(notification.description, "New Description");
            assert_eq!(notification.icon_url, Some("https://example.com/icon.png".to_string()));
            assert_eq!(notification.priority, NotificationPriority::Urgent as i32);
        }
    }

    // ==========================================================================
    // Subscribe
    // ==========================================================================

    mod subscribe {
        use super::*;

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_receive_new_notification_event(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-new");

            let subscribe_req = ctx.authenticated_request(SubscribeRequest::default(), &user_id);
            let mut stream = ctx
                .service
                .subscribe(subscribe_req)
                .await
                .expect("subscribe should succeed")
                .into_inner();

            // Give Redis subscription time to establish
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push notification
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Test Title".to_string(),
                description: "Test Description".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let push_resp = ctx.service.push(push_req).await.unwrap().into_inner();

            // Receive event
            let event = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next())
                .await
                .expect("should receive event within timeout")
                .expect("stream should have event")
                .expect("event should be ok");

            match event.event {
                Some(notification_event::Event::Notification(n)) => {
                    assert_eq!(n.pid, push_resp.notification_pid);
                    assert_eq!(n.title, "Test Title");
                }
                other => panic!("expected Notification event, got {:?}", other),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_receive_updated_event(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-updated");

            let subscribe_req = ctx.authenticated_request(SubscribeRequest::default(), &user_id);
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push then update
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Original".to_string(),
                description: "Original".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let push_resp = ctx.service.push(push_req).await.unwrap().into_inner();

            // Consume the new notification event
            let _ = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next()).await;

            // Update
            let update_req = ctx.authenticated_request(
                UpdateRequest {
                    id: push_resp.id,
                    title: Some("Updated".to_string()),
                    ..Default::default()
                },
                &user_id,
            );
            ctx.service.update(update_req).await.unwrap();

            // Receive updated event
            let event = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next())
                .await
                .expect("should receive event")
                .expect("stream should have event")
                .expect("event should be ok");

            match event.event {
                Some(notification_event::Event::Updated(u)) => {
                    let n = u.notification.unwrap();
                    assert_eq!(n.pid, push_resp.notification_pid);
                    assert_eq!(n.title, "Updated");
                }
                other => panic!("expected Updated event, got {:?}", other),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_receive_read_event(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-read");

            let subscribe_req = ctx.authenticated_request(SubscribeRequest::default(), &user_id);
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let push_resp = ctx.service.push(push_req).await.unwrap().into_inner();

            // Consume new event
            let _ = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next()).await;

            // Mark as read
            let read_req = ctx.authenticated_request(
                proto_rs::notification::v1::MarkAsReadRequest {
                    notification_pids: vec![push_resp.notification_pid.clone()],
                },
                &user_id,
            );
            ctx.service.mark_as_read(read_req).await.unwrap();

            // Receive read event
            let event = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next())
                .await
                .expect("should receive event")
                .expect("stream should have event")
                .expect("event should be ok");

            match event.event {
                Some(notification_event::Event::Read(r)) => {
                    assert_eq!(r.notification_pid, push_resp.notification_pid);
                    assert!(!r.read_at.is_empty());
                }
                other => panic!("expected Read event, got {:?}", other),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_receive_dismissed_event(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-dismissed");

            let subscribe_req = ctx.authenticated_request(SubscribeRequest::default(), &user_id);
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            let push_resp = ctx.service.push(push_req).await.unwrap().into_inner();

            // Consume new event
            let _ = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next()).await;

            // Dismiss
            let dismiss_req = ctx.authenticated_request(
                proto_rs::notification::v1::DismissRequest {
                    notification_pid: push_resp.notification_pid.clone(),
                },
                &user_id,
            );
            ctx.service.dismiss(dismiss_req).await.unwrap();

            // Receive dismissed event
            let event = tokio::time::timeout(tokio::time::Duration::from_secs(2), stream.next())
                .await
                .expect("should receive event")
                .expect("stream should have event")
                .expect("event should be ok");

            match event.event {
                Some(notification_event::Event::Dismissed(d)) => {
                    assert_eq!(d.notification_pid, push_resp.notification_pid);
                }
                other => panic!("expected Dismissed event, got {:?}", other),
            }
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_category(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-cat-filter");

            // Subscribe only to Chat category
            let subscribe_req = ctx.authenticated_request(
                SubscribeRequest {
                    categories: vec![NotificationCategory::Chat as i32],
                    ..Default::default()
                },
                &user_id,
            );
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push Agent notification (should be filtered out)
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Agent".to_string(),
                description: "Agent".to_string(),
                category: NotificationCategory::Agent as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            ctx.service.push(push_req).await.unwrap();

            // Should timeout - no event received
            let result = tokio::time::timeout(tokio::time::Duration::from_millis(200), stream.next()).await;

            assert!(
                result.is_err(),
                "should not receive Agent notification when filtering for Chat"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_filter_by_min_priority(ctx: &mut TestContext) {
            let user_id = ctx.unique_user_id("sub-priority-filter");

            // Subscribe only to High priority and above
            let subscribe_req = ctx.authenticated_request(
                SubscribeRequest {
                    min_priority: Some(NotificationPriority::High as i32),
                    ..Default::default()
                },
                &user_id,
            );
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push Low priority notification (should be filtered out)
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_id.clone(),
                title: "Low".to_string(),
                description: "Low".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                priority: NotificationPriority::Low as i32,
                ..Default::default()
            });
            ctx.service.push(push_req).await.unwrap();

            // Should timeout - no event received
            let result = tokio::time::timeout(tokio::time::Duration::from_millis(200), stream.next()).await;

            assert!(
                result.is_err(),
                "should not receive Low priority when filtering for High+"
            );
        }

        #[test_context(TestContext)]
        #[tokio::test]
        #[serial]
        async fn should_not_receive_other_users_events(ctx: &mut TestContext) {
            let user_a = ctx.unique_user_id("sub-a");
            let user_b = ctx.unique_user_id("sub-b");

            // User A subscribes
            let subscribe_req = ctx.authenticated_request(SubscribeRequest::default(), &user_a);
            let mut stream = ctx.service.subscribe(subscribe_req).await.unwrap().into_inner();

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // Push to User B
            let push_req = tonic::Request::new(PushRequest {
                user_id: user_b.clone(),
                title: "For B".to_string(),
                description: "For B".to_string(),
                category: NotificationCategory::System as i32,
                origin_service: OriginService::Api as i32,
                ..Default::default()
            });
            ctx.service.push(push_req).await.unwrap();

            // User A should not receive it
            let result = tokio::time::timeout(tokio::time::Duration::from_millis(200), stream.next()).await;

            assert!(result.is_err(), "User A should not receive User B's notifications");
        }
    }
}
