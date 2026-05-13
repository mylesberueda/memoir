use proto_rs::chat::v1::*;

pub(crate) struct ModerationService {}

impl ModerationService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl moderation_service_server::ModerationService for ModerationService {
    async fn report_message(
        &self,
        _request: tonic::Request<ReportMessageRequest>,
    ) -> std::result::Result<tonic::Response<ReportMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn report_user(
        &self,
        _request: tonic::Request<ReportUserRequest>,
    ) -> std::result::Result<tonic::Response<ReportUserResponse>, tonic::Status> {
        todo!()
    }

    async fn get_reports(
        &self,
        _request: tonic::Request<GetReportsRequest>,
    ) -> std::result::Result<tonic::Response<GetReportsResponse>, tonic::Status> {
        todo!()
    }

    async fn resolve_report(
        &self,
        _request: tonic::Request<ResolveReportRequest>,
    ) -> std::result::Result<tonic::Response<ResolveReportResponse>, tonic::Status> {
        todo!()
    }

    async fn mute_user(
        &self,
        _request: tonic::Request<MuteUserRequest>,
    ) -> std::result::Result<tonic::Response<MuteUserResponse>, tonic::Status> {
        todo!()
    }

    async fn unmute_user(
        &self,
        _request: tonic::Request<UnmuteUserRequest>,
    ) -> std::result::Result<tonic::Response<UnmuteUserResponse>, tonic::Status> {
        todo!()
    }

    async fn ban_user(
        &self,
        _request: tonic::Request<BanUserRequest>,
    ) -> std::result::Result<tonic::Response<BanUserResponse>, tonic::Status> {
        todo!()
    }

    async fn unban_user(
        &self,
        _request: tonic::Request<UnbanUserRequest>,
    ) -> std::result::Result<tonic::Response<UnbanUserResponse>, tonic::Status> {
        todo!()
    }

    async fn get_banned_users(
        &self,
        _request: tonic::Request<GetBannedUsersRequest>,
    ) -> std::result::Result<tonic::Response<GetBannedUsersResponse>, tonic::Status> {
        todo!()
    }

    async fn get_muted_users(
        &self,
        _request: tonic::Request<GetMutedUsersRequest>,
    ) -> std::result::Result<tonic::Response<GetMutedUsersResponse>, tonic::Status> {
        todo!()
    }

    async fn get_moderation_log(
        &self,
        _request: tonic::Request<GetModerationLogRequest>,
    ) -> std::result::Result<tonic::Response<GetModerationLogResponse>, tonic::Status> {
        todo!()
    }

    async fn get_auto_mod_settings(
        &self,
        _request: tonic::Request<GetAutoModSettingsRequest>,
    ) -> std::result::Result<tonic::Response<GetAutoModSettingsResponse>, tonic::Status> {
        todo!()
    }

    async fn update_auto_mod_settings(
        &self,
        _request: tonic::Request<UpdateAutoModSettingsRequest>,
    ) -> std::result::Result<tonic::Response<UpdateAutoModSettingsResponse>, tonic::Status> {
        todo!()
    }
}
