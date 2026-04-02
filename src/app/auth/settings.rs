use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AuthSettings {
    pub private_key_file: String,
}
