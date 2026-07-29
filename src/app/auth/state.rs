use std::sync::Arc;

use bzd_lib::error::Error;
use tokio::fs;

use crate::app::{
    auth::{
        encoder::{Encoder, EncoderImpl},
        settings::AuthSettings,
    },
    db::DbState,
};

#[derive(Clone)]
pub struct AuthState {
    pub settings: AuthSettings,
    pub db: DbState,
    pub encoder: Arc<dyn Encoder>,
}

impl AuthState {
    pub async fn new(settings: &AuthSettings, db: DbState) -> Result<Self, Error> {
        let settings = settings.clone();

        let private_key = fs::read_to_string(&settings.private_key_file)
            .await?
            .into_bytes();
        let encoder = EncoderImpl::new(&private_key)?;
        let encoder = Arc::new(encoder);

        Ok(Self {
            settings,
            db,
            encoder,
        })
    }
}
