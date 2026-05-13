//! Request extension traits for extracting auth info from requests.
//!
//! Provides a unified `RequestAuthExt` trait that works with both `tonic::Request<T>`
//! and `http::Request<T>`, returning the appropriate error type for each.

use crate::cache::ResolvedPermissions;
use crate::middleware::{
    auth::User,
    organization::{OrgContext, OrganizationPid},
};
use tonic::Status;

/// Extension trait for extracting authentication and organization info from requests.
///
/// Uses a Generic Associated Type (GAT) to allow different return types:
/// - `tonic::Request<T>` returns `Result<U, Status>` (gRPC error on failure)
/// - `http::Request<T>` returns `Option<U>` (None on missing)
pub trait RequestAuthExt {
    type Output<U>;

    /// Extract the unified User from request extensions.
    fn user(&self) -> Self::Output<&User>;

    /// Extract the user ID (shorthand for `user()?.id`).
    fn user_id(&self) -> Self::Output<String>;

    /// Extract the organization PID from the request extensions.
    fn organization_pid(&self) -> Self::Output<String>;

    /// Extract the resolved permissions from the OrgContext extension.
    fn org_permissions(&self) -> Self::Output<ResolvedPermissions>;
}

impl<T> RequestAuthExt for tonic::Request<T> {
    type Output<U> = Result<U, Status>;

    fn user(&self) -> Result<&User, Status> {
        self.extensions()
            .get::<User>()
            .ok_or(Status::unauthenticated("Authentication required"))
    }

    fn user_id(&self) -> Result<String, Status> {
        self.user().map(|u| u.id.clone())
    }

    fn organization_pid(&self) -> Result<String, Status> {
        self.extensions()
            .get::<OrganizationPid>()
            .ok_or(Status::failed_precondition("Organization required"))
            .map(|id| id.0.clone())
    }

    fn org_permissions(&self) -> Result<ResolvedPermissions, Status> {
        self.extensions()
            .get::<OrgContext>()
            .ok_or(Status::failed_precondition("Organization context required"))
            .map(|ctx| ctx.permissions.clone())
    }
}

impl<T> RequestAuthExt for http::Request<T> {
    type Output<U> = Option<U>;

    fn user(&self) -> Option<&User> {
        self.extensions().get::<User>()
    }

    fn user_id(&self) -> Option<String> {
        self.user().map(|u| u.id.clone())
    }

    fn organization_pid(&self) -> Option<String> {
        self.extensions().get::<OrganizationPid>().map(|id| id.0.clone())
    }

    fn org_permissions(&self) -> Option<ResolvedPermissions> {
        self.extensions().get::<OrgContext>().map(|ctx| ctx.permissions.clone())
    }
}
