use aes_gcm::{KeyInit as _, aead::Aead};

use crate::app::{
    crypto::{Cipher, Key, Nonce, settings::CryptoSettings},
    error::AppError,
};

pub struct EncryptorImpl {
    cipher: Cipher,
    nonce: Nonce,
}

impl EncryptorImpl {
    pub fn new(settings: &CryptoSettings) -> Self {
        let key = Key::from_slice(settings.key.as_bytes()).clone();
        let cipher = Cipher::new(&key);

        let nonce = Nonce::from_slice(&settings.nonce.as_bytes()).to_owned();

        Self { cipher, nonce }
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait Encryptor: Send + Sync {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError>;

    fn decrypt(&self, text: &Vec<u8>) -> Result<String, AppError>;
}

impl Encryptor for EncryptorImpl {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError> {
        Ok(self.cipher.encrypt(&self.nonce, text.as_bytes())?)
    }

    fn decrypt(&self, text: &Vec<u8>) -> Result<String, AppError> {
        Ok(String::from_utf8(
            self.cipher.decrypt(&self.nonce, text.as_ref())?,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use bzd_lib::error::Error;

    use crate::app::crypto::{
        encryptor::{Encryptor, EncryptorImpl},
        settings::CryptoSettings,
    };

    #[test]
    fn encrypt_and_decrypt() -> Result<(), Error> {
        let key = "5881aaa1f5bd0d16de70de19bf59714c".into();
        let nonce = "bdf861dd474d".into();

        let settings = CryptoSettings { key, nonce };
        let encryptor = EncryptorImpl::new(&settings);

        let text = String::from("TEXT_2_ENCRYPT");
        let cipher_text = encryptor.encrypt(&text)?;

        assert_eq!(text, encryptor.decrypt(&cipher_text)?);

        Ok(())
    }
}
