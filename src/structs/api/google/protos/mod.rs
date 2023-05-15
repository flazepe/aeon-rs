pub mod api {
    include!("google.api.rs");
}

pub mod r#type {
    include!("google.r#type.rs");
}

pub mod assistant {
    pub mod embedded {
        pub mod v1alpha2 {
            include!("google.assistant.embedded.v1alpha2.rs");
        }
    }
}
