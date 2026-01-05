pub const AUTH_FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");

tonic::include_proto!("bzd.users.auth");

pub mod contacts {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("contacts_descriptor");

    tonic::include_proto!("bzd.users.contacts");
}

pub mod users {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("users_descriptor");

    tonic::include_proto!("bzd.users.users");
}
