mod context;

pub use context::*;

use super::BoxFuture;
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct OrganizationPid(pub String);

#[derive(Debug, Clone, Default)]
pub struct OrganizationLayer;

impl OrganizationLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for OrganizationLayer {
    type Service = OrganizationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OrganizationMiddleware { inner }
    }
}

#[derive(Debug, Clone)]
pub struct OrganizationMiddleware<S> {
    inner: S,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for OrganizationMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Default + Send + 'static,
    ResBody: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: http::Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move {
            if let Some(org_pid) = req
                .headers()
                .get("x-organization-id")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
            {
                req.extensions_mut().insert(OrganizationPid(org_pid));
            }

            inner.call(req).await
        })
    }
}
