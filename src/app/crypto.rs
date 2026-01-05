pub mod encryptor;
pub mod settings;
pub mod state;

pub type Cipher = aes_gcm::Aes256Gcm;
pub type Key = aes_gcm::Key<Cipher>;
pub type Nonce = aes_gcm::Nonce<aes_gcm::aead::consts::U12>;
