pub mod google {
    pub mod api {
        include!("firestore_rpc/google.api.rs");
    }
    pub mod firestore {
        pub mod v1 {
            include!("firestore_rpc/google.firestore.v1.rs");
        }
    }
    pub mod protobuf {
        include!("firestore_rpc/google.protobuf.rs");
    }
    pub mod r#type {
        include!("firestore_rpc/google.r#type.rs");
    }
    pub mod rpc {
        include!("firestore_rpc/google.rpc.rs");
    }
}
