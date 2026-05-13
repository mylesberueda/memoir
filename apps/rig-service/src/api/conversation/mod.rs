pub(crate) mod proto;

use crate::models::conversations;
use proto_rs::rig::v1;

pub(crate) struct Conversation {
    pub(crate) model: conversations::Model,
    pub(crate) agent_pid: String,
    pub(crate) messages: Vec<v1::Message>,
}
