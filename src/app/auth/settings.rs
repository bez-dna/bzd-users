use serde::Deserialize;

use crate::app::auth::verification::VerificationSettings;

#[derive(Deserialize, Clone)]
pub struct AuthSettings {
    pub verification: VerificationSettings,
    pub private_key_file: String,
}
