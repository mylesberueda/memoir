pub mod rig {
    pub mod v1 {
        include!("rig/v1/rig.v1.rs");
    }
}

pub mod api {
    pub mod v1 {
        include!("api/v1/api.v1.rs");
    }
}

pub mod chat {
    pub mod v1 {
        include!("chat/v1/chat.v1.rs");
    }
}

pub mod notification {
    pub mod v1 {
        include!("notification/v1/notification.v1.rs");
    }
}

pub mod google {
    pub mod protobuf {
        pub use pbjson_types::*;
    }
}
