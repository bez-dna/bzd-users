use std::sync::Arc;

use crate::app::crypto::{
    encryptor::{Encryptor, EncryptorImpl},
    settings::CryptoSettings,
};

#[derive(Clone)]
pub struct CryptoState {
    pub encryptor: Arc<dyn Encryptor>,
}

impl CryptoState {
    pub fn new(settings: &CryptoSettings) -> Self {
        let encryptor = EncryptorImpl::new(settings);
        let encryptor = Arc::new(encryptor);

        Self { encryptor }
    }
}
