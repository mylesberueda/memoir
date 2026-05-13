use crate::AppContext;
use axum::{Router, routing::get};
use std::sync::Arc;
mod v1;

/// HTTP service that provides REST API endpoints.
#[derive(Clone)]
pub(crate) struct HttpService {
    context: Arc<AppContext>,
}

impl HttpService {
    pub(crate) fn new(context: Arc<AppContext>) -> Self {
        Self { context }
    }

    /// Build the HTTP router with all versioned routes.
    pub(crate) async fn router(&self) -> Router {
        Router::new().route("/health", get(|| async { "ok" })).nest(
            "/v1",
            v1::router(self.context.db.clone(), self.context.redis.clone()).await,
        )
    }
}

#[cfg(all(test, feature = "integration"))]
mod tests {
    use super::*;
    use crate::test_utils::TestContext;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use test_context::test_context;
    use tower::ServiceExt;

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_pong_on_ping(ctx: &mut TestContext) {
        let service = HttpService::new(ctx.context.clone());
        let router = service.router().await;

        let response = router
            .oneshot(Request::get("/v1/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"pong");
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_reject_stripe_webhook_without_signature(ctx: &mut TestContext) {
        let service = HttpService::new(ctx.context.clone());
        let router = service.router().await;

        // Webhook without Stripe-Signature header should return 400
        let response = router
            .oneshot(Request::post("/v1/webhooks/stripe").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test_context(TestContext)]
    #[tokio::test]
    async fn should_return_404_on_unknown_route(ctx: &mut TestContext) {
        let service = HttpService::new(ctx.context.clone());
        let router = service.router().await;

        let response = router
            .oneshot(Request::get("/v1/unknown").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
