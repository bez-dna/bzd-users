use std::sync::Arc;

use tokio::fs;

use crate::app::{
    auth::{
        encoder::{Encoder, EncoderImpl},
        settings::AuthSettings,
        verification::VerificationClient,
    },
    crypto::state::CryptoState,
    db::DbState,
    error::AppError,
};

#[derive(Clone)]
pub struct AuthState {
    pub verification_client: Arc<VerificationClient>,
    pub db: DbState,
    pub crypto: CryptoState,
    pub settings: AuthSettings,
    pub encoder: Arc<dyn Encoder>,
}

impl AuthState {
    pub async fn new(
        settings: &AuthSettings,
        db: DbState,
        crypto: CryptoState,
    ) -> Result<Self, AppError> {
        let verification_client = Arc::new(VerificationClient::new(settings.verification.clone()));

        let settings = settings.clone();

        let private_key = fs::read_to_string(&settings.private_key_file)
            .await?
            .into_bytes();
        let encoder = EncoderImpl::new(&private_key)?;
        let encoder = Arc::new(encoder);

        Ok(Self {
            settings,
            verification_client,
            db,
            crypto,
            encoder,
        })
    }
}
