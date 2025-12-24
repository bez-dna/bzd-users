use bzd_lib::error::Error;

use crate::app::{
    auth::state::AuthState, crypto::state::CryptoState, db::DbState, settings::AppSettings,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: DbState,
    pub auth: AuthState,
    pub crypto: CryptoState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let auth = AuthState::new(&settings.auth).await?;
        let db = DbState::new(&settings.db).await?;
        let crypto = CryptoState::new(&settings.crypto)?;

        Ok(Self {
            settings,
            auth,
            db,
            crypto,
        })
    }
}

// #[derive(Clone)]
// pub struct CryptoState {
//     cipher: Aes256Gcm,
//     nonce: Nonce<U12>,
// }

// impl CryptoState {
//     pub async fn new(settings: &CryptoSettings) -> Result<Self, Error> {
//         let key = Key::<Aes256Gcm>::from_slice(settings.key.as_bytes()).clone();
//         let cipher = Aes256Gcm::new(&key);

//         let nonce: Nonce<U12> = Nonce::from_slice(&settings.nonce.as_bytes()).to_owned();

//         Ok(Self { cipher, nonce })
//     }

//     pub fn encrypt(&self, text: &String) -> Result<Vec<u8>, AppError> {
//         Ok(self.cipher.encrypt(&self.nonce, text.as_bytes())?)
//     }

//     pub fn decrypt(&self, text: &Vec<u8>) -> Result<String, AppError> {
//         Ok(String::from_utf8(
//             self.cipher.decrypt(&self.nonce, text.as_ref())?,
//         )?)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use bzd_lib::error::Error;

//     use crate::app::{settings::CryptoSettings, state::CryptoState};

//     #[tokio::test]
//     async fn encrypt_and_decrypt() -> Result<(), Error> {
//         let key = "5881aaa1f5bd0d16de70de19bf59714c".into();
//         let nonce = "bdf861dd474d".into();

//         let settings = CryptoSettings { key, nonce };
//         let state = CryptoState::new(&settings).await?;

//         let text = String::from("TEXT_2_ENCRYPT");
//         let cipher_text = state.encrypt(&text)?;

//         assert_eq!(text, state.decrypt(&cipher_text)?);

//         Ok(())
//     }
// }
