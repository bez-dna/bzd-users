use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

use crate::app::{auth::PrivateKey, error::AppError};

#[derive(Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Claims {
    // надо убрать отсюда Result, чёт бесползеный какой-то
    pub fn new(user_id: Uuid) -> Result<Self, AppError> {
        Ok(Self {
            sub: user_id,
            exp: (Utc::now() + Duration::days(300)).timestamp().try_into()?,
        })
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait Encoder: Send + Sync {
    fn encode(&self, claims: &Claims) -> Result<String, AppError>;
}

pub struct EncoderImpl {
    header: Header,
    key: EncodingKey,
}

impl EncoderImpl {
    pub fn new(private_key: &PrivateKey) -> Result<Self, AppError> {
        Ok(Self {
            header: Header::new(jsonwebtoken::Algorithm::RS256),
            key: EncodingKey::from_rsa_pem(private_key)?,
        })
    }
}

impl Encoder for EncoderImpl {
    fn encode(&self, claims: &Claims) -> Result<String, AppError> {
        let jwt = jsonwebtoken::encode(&self.header, &claims, &self.key)?;

        Ok(jwt)
    }
}
