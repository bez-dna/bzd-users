use serde::Deserialize;

use crate::app::auth::verification::VerificationSettings;

#[derive(Deserialize, Clone)]
pub struct AuthSettings {
    pub verification: VerificationSettings,
}
