use platform_rs::ext::RequestAuthExt;
use proto_rs::chat::v1::*;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone)]
pub(crate) struct ChatService {}

impl ChatService {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl chat_service_server::ChatService for ChatService {
    #[doc = " Server streaming response type for the Open method."]
    type OpenStream = ReceiverStream<Result<ServerEvent, tonic::Status>>;

    async fn open(
        &self,
        request: tonic::Request<tonic::Streaming<ClientEvent>>,
    ) -> std::result::Result<tonic::Response<Self::OpenStream>, tonic::Status> {
        let _user_id = request.user_id()?;
        todo!()
    }

    async fn send_message(
        &self,
        _request: tonic::Request<SendMessageRequest>,
    ) -> std::result::Result<tonic::Response<SendMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn get_message_history(
        &self,
        _request: tonic::Request<GetMessageHistoryRequest>,
    ) -> std::result::Result<tonic::Response<GetMessageHistoryResponse>, tonic::Status> {
        todo!()
    }

    async fn get_message(
        &self,
        _request: tonic::Request<GetMessageRequest>,
    ) -> std::result::Result<tonic::Response<GetMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn edit_message(
        &self,
        _request: tonic::Request<EditMessageRequest>,
    ) -> std::result::Result<tonic::Response<EditMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn delete_message(
        &self,
        _request: tonic::Request<DeleteMessageRequest>,
    ) -> std::result::Result<tonic::Response<DeleteMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn react_to_message(
        &self,
        _request: tonic::Request<ReactToMessageRequest>,
    ) -> std::result::Result<tonic::Response<ReactToMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_reaction(
        &self,
        _request: tonic::Request<RemoveReactionRequest>,
    ) -> std::result::Result<tonic::Response<RemoveReactionResponse>, tonic::Status> {
        todo!()
    }

    async fn pin_message(
        &self,
        _request: tonic::Request<PinMessageRequest>,
    ) -> std::result::Result<tonic::Response<PinMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn unpin_message(
        &self,
        _request: tonic::Request<UnpinMessageRequest>,
    ) -> std::result::Result<tonic::Response<UnpinMessageResponse>, tonic::Status> {
        todo!()
    }

    async fn get_pinned_messages(
        &self,
        _request: tonic::Request<GetPinnedMessagesRequest>,
    ) -> std::result::Result<tonic::Response<GetPinnedMessagesResponse>, tonic::Status> {
        todo!()
    }

    async fn get_thread(
        &self,
        _request: tonic::Request<GetThreadRequest>,
    ) -> std::result::Result<tonic::Response<GetThreadResponse>, tonic::Status> {
        todo!()
    }

    async fn search_messages(
        &self,
        _request: tonic::Request<SearchMessagesRequest>,
    ) -> std::result::Result<tonic::Response<SearchMessagesResponse>, tonic::Status> {
        todo!()
    }
}
