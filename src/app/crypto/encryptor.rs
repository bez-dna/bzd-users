use bzd_lib::error::Error;
use hmac::Mac;

use crate::app::{
    crypto::{HmacSha256, settings::CryptoSettings},
    error::AppError,
};

pub struct EncryptorImpl {
    mac: HmacSha256,
}

impl EncryptorImpl {
    pub fn new(settings: &CryptoSettings) -> Result<Self, Error> {
        let mac = HmacSha256::new_from_slice(settings.key.as_bytes())?;

        Ok(Self { mac })
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait Encryptor: Send + Sync {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError>;
}

impl Encryptor for EncryptorImpl {
    fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError> {
        let mut mac = self.mac.clone();
        mac.update(text.as_bytes());

        Ok(mac.finalize().into_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use bzd_lib::error::Error;
    use hex_literal::hex;

    use crate::app::crypto::{
        encryptor::{Encryptor, EncryptorImpl},
        settings::CryptoSettings,
    };

    #[test]
    fn encrypt_and_decrypt() -> Result<(), Error> {
        let key = "5881aaa1f5bd0d16de70de19bf59714c".into();

        let settings = CryptoSettings { key };
        let encryptor = EncryptorImpl::new(&settings)?;

        let text = String::from("TEXT_2_ENCRYPT");
        let hash = encryptor.encrypt(&text)?;

        assert_eq!(
            hex!("787e964807e3169afb2f0c842f54ffacc02e0fbbbc72782eab2cbf9ef47b5007").to_vec(),
            hash
        );

        Ok(())
    }
}
