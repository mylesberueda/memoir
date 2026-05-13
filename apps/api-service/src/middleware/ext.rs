#![allow(unused)] // TODO(_): Leave until fully implemented

use super::{OrganizationContext, UserContext};
use platform_rs::ext::RequestAuthExt;
use tonic::Status;

/// Service-specific extension trait that adds `organization_context` and `user_context`
/// on top of the common `RequestAuthExt` methods.
pub(crate) trait RequestExt: RequestAuthExt {
    fn organization_context(&self) -> Self::Output<&OrganizationContext>;
    fn user_context(&self) -> Self::Output<&UserContext>;
}

impl<T> RequestExt for tonic::Request<T> {
    fn organization_context(&self) -> <Self as RequestAuthExt>::Output<&OrganizationContext> {
        self.extensions()
            .get::<OrganizationContext>()
            .ok_or(Status::failed_precondition("Organization required"))
    }

    fn user_context(&self) -> <Self as RequestAuthExt>::Output<&UserContext> {
        self.extensions()
            .get::<UserContext>()
            .ok_or(Status::failed_precondition("User required"))
    }
}

impl<T> RequestExt for http::Request<T> {
    fn organization_context(&self) -> Option<&OrganizationContext> {
        self.extensions().get::<OrganizationContext>()
    }

    fn user_context(&self) -> <Self as RequestAuthExt>::Output<&UserContext> {
        self.extensions().get::<UserContext>()
    }
}
