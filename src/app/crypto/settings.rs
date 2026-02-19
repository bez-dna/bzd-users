use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CryptoSettings {
    pub key: String,
}
