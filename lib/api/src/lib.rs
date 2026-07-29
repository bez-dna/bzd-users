pub mod auth {
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");

    tonic::include_proto!("bzd.users.auth");
}

pub mod users {
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("users_descriptor");

    tonic::include_proto!("bzd.users.users");
}
