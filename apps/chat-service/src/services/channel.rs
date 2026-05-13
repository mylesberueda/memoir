use proto_rs::chat::v1::*;

pub(crate) struct ChannelService {}

impl ChannelService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl channel_service_server::ChannelService for ChannelService {
    async fn get_channels(
        &self,
        _request: tonic::Request<GetChannelsRequest>,
    ) -> std::result::Result<tonic::Response<GetChannelsResponse>, tonic::Status> {
        todo!()
    }

    async fn create_channel(
        &self,
        _request: tonic::Request<CreateChannelRequest>,
    ) -> std::result::Result<tonic::Response<CreateChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn get_channel(
        &self,
        _request: tonic::Request<GetChannelRequest>,
    ) -> std::result::Result<tonic::Response<GetChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn update_channel(
        &self,
        _request: tonic::Request<UpdateChannelRequest>,
    ) -> std::result::Result<tonic::Response<UpdateChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn delete_channel(
        &self,
        _request: tonic::Request<DeleteChannelRequest>,
    ) -> std::result::Result<tonic::Response<DeleteChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn invite_to_channel(
        &self,
        _request: tonic::Request<InviteToChannelRequest>,
    ) -> std::result::Result<tonic::Response<InviteToChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn leave_channel(
        &self,
        _request: tonic::Request<LeaveChannelRequest>,
    ) -> std::result::Result<tonic::Response<LeaveChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn kick_from_channel(
        &self,
        _request: tonic::Request<KickFromChannelRequest>,
    ) -> std::result::Result<tonic::Response<KickFromChannelResponse>, tonic::Status> {
        todo!()
    }

    async fn get_channel_members(
        &self,
        _request: tonic::Request<GetChannelMembersRequest>,
    ) -> std::result::Result<tonic::Response<GetChannelMembersResponse>, tonic::Status> {
        todo!()
    }

    async fn update_member_role(
        &self,
        _request: tonic::Request<UpdateMemberRoleRequest>,
    ) -> std::result::Result<tonic::Response<UpdateMemberRoleResponse>, tonic::Status> {
        todo!()
    }

    async fn mark_as_read(
        &self,
        _request: tonic::Request<MarkAsReadRequest>,
    ) -> std::result::Result<tonic::Response<MarkAsReadResponse>, tonic::Status> {
        todo!()
    }

    async fn get_unread_counts(
        &self,
        _request: tonic::Request<GetUnreadCountsRequest>,
    ) -> std::result::Result<tonic::Response<GetUnreadCountsResponse>, tonic::Status> {
        todo!()
    }

    async fn start_conversation(
        &self,
        _request: tonic::Request<StartConversationRequest>,
    ) -> std::result::Result<tonic::Response<StartConversationResponse>, tonic::Status> {
        todo!()
    }

    async fn get_conversations(
        &self,
        _request: tonic::Request<GetConversationsRequest>,
    ) -> std::result::Result<tonic::Response<GetConversationsResponse>, tonic::Status> {
        todo!()
    }
}
