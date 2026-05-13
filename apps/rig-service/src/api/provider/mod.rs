pub(crate) mod proto;

use crate::models::providers;
use common_rs::crypto::Secret;

pub(crate) struct Provider {
    pub(crate) model: providers::Model,
    pub(crate) credentials: Option<Secret>,
}
