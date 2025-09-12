pub const AUTH_FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");

tonic::include_proto!("bzd.users.auth");

pub const CONTACTS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("contacts_descriptor");

tonic::include_proto!("bzd.users.contacts");

pub const USERS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("users_descriptor");

tonic::include_proto!("bzd.users.users");
