use std::sync::Arc;

use tokio::fs;

use crate::app::{
    auth::{settings::AuthSettings, verification::VerificationClient},
    error::AppError,
};

#[derive(Clone)]
pub struct AuthState {
    pub verification_client: Arc<VerificationClient>,
    pub private_key: Vec<u8>,
}

impl AuthState {
    pub async fn new(settings: &AuthSettings) -> Result<Self, AppError> {
        let verification_client = Arc::new(VerificationClient::new(settings.verification.clone()));

        let private_key = fs::read_to_string(&settings.private_key_file)
            .await?
            .into_bytes();

        Ok(Self {
            verification_client,
            private_key,
        })
    }
}
