use std::sync::Arc;

use bzd_lib::error::Error;

use crate::app::crypto::{
    encryptor::{Encryptor, EncryptorImpl},
    settings::CryptoSettings,
};

#[derive(Clone)]
pub struct CryptoState {
    pub encryptor: Arc<dyn Encryptor>,
}

impl CryptoState {
    pub fn new(settings: &CryptoSettings) -> Result<Self, Error> {
        let encryptor = EncryptorImpl::new(settings)?;
        let encryptor = Arc::new(encryptor);

        Ok(Self { encryptor })
    }
}
