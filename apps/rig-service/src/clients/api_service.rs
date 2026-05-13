use proto_rs::api::v1::{GetUsersRequest, UserSummary, user_service_client::UserServiceClient};
use std::collections::HashMap;
use tonic::transport::Channel;

type InnerClient = UserServiceClient<Channel>;

pub(crate) struct ApiServiceClient {
    client: InnerClient,
}

impl ApiServiceClient {
    pub(crate) fn new(url: &str) -> Result<Self, tonic::transport::Error> {
        let channel = Channel::from_shared(url.to_string())
            .map_err(|e| panic!("Invalid API_SERVICE_URL: {}", e))?
            .connect_lazy();

        Ok(Self {
            client: UserServiceClient::new(channel),
        })
    }

    fn authenticated_request<T>(&self, inner: T, token: &str, org_pid: &str) -> tonic::Request<T> {
        let mut request = tonic::Request::new(inner);
        let bearer = format!("Bearer {}", token);
        request
            .metadata_mut()
            .insert("authorization", bearer.parse().expect("bearer token is valid ASCII"));
        request
            .metadata_mut()
            .insert("x-organization-id", org_pid.parse().expect("org_pid is valid ASCII"));
        request
    }

    /// Batch fetch user summaries by IDs, scoped to the caller's org.
    /// Returns a HashMap keyed by user_id for easy lookup.
    pub(crate) async fn get_users(
        &self,
        token: &str,
        org_pid: &str,
        user_ids: Vec<String>,
    ) -> Result<HashMap<String, UserSummary>, tonic::Status> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let mut client = self.client.clone();
        let request = self.authenticated_request(GetUsersRequest { user_ids }, token, org_pid);

        let response = client.get_users(request).await?.into_inner();

        Ok(response.users.into_iter().map(|u| (u.id.clone(), u)).collect())
    }
}
