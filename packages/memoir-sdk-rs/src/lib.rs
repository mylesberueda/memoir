pub mod example {
    pub mod v1 {
        include!("example/v1/example.v1.rs");
    }
}

pub mod google {
    pub mod protobuf {
        pub use pbjson_types::*;
    }
}
