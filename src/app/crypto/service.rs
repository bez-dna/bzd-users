use aes_gcm::{
    Aes256Gcm, Key, KeyInit as _, Nonce,
    aead::{Aead, consts::U12},
};

use crate::app::{crypto::settings::CryptoSettings, error::AppError};

pub struct CryptoServiceImpl {
    cipher: Aes256Gcm,
    nonce: Nonce<U12>,
}

impl CryptoServiceImpl {
    pub fn new(settings: &CryptoSettings) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(settings.key.as_bytes()).clone();
        let cipher = Aes256Gcm::new(&key);

        let nonce: Nonce<U12> = Nonce::from_slice(&settings.nonce.as_bytes()).to_owned();

        Self { cipher, nonce }
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait CryptoService: Send + Sync {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError>;

    fn decrypt(&self, text: &Vec<u8>) -> Result<String, AppError>;
}

impl CryptoService for CryptoServiceImpl {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError> {
        Ok(self.cipher.encrypt(&self.nonce, text.as_bytes())?)
    }

    fn decrypt(&self, text: &Vec<u8>) -> Result<String, AppError> {
        Ok(String::from_utf8(
            self.cipher.decrypt(&self.nonce, text.as_ref())?,
        )?)
    }
}
