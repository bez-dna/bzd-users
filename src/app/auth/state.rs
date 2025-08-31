use std::sync::Arc;

use crate::app::auth::{settings::AuthSettings, verification::VerificationClient};

#[derive(Clone)]
pub struct AuthState {
    pub verification_client: Arc<VerificationClient>,
}

impl AuthState {
    pub fn new(settings: &AuthSettings) -> Self {
        let verification_client = Arc::new(VerificationClient::new(settings.verification.clone()));

        Self {
            verification_client,
        }
    }
}
