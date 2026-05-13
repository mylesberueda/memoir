pub mod auth;
pub mod organization;
pub mod ratelimit;
pub mod user_cache;

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
