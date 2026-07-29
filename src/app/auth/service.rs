use coset::{CborSerializable, CoseKey};
use sea_orm::{DbConn, TransactionTrait as _};

use crate::app::{
    auth::{
        encoder::{Claims, Encoder},
        passkey::{
            ClientData, ClientDataType, PublicKeyCredentialCreationOptions,
            PublicKeyCredentialRequestOptions,
            authenticator_data::{AuthenticatorData, Origin},
        },
        repo::{self, UserChallengeModel, UserCredentialModel, UserModel},
        settings::AuthSettings,
    },
    error::AppError,
};

pub async fn join(
    db: &DbConn,
    settings: &AuthSettings,
    req: join::Request,
) -> Result<join::Response, AppError> {
    let res = match repo::find_user_by_login_with_user_credentials(db, &req.login).await? {
        Some((user, user_credentials)) => {
            let public_key: PublicKeyCredentialRequestOptions = (user_credentials, settings).into();

            repo::create_user_challenge(
                db,
                UserChallengeModel::new(
                    public_key.challenge.clone(),
                    user.user_id,
                    user.login,
                    user.name,
                ),
            )
            .await?;

            public_key.into()
        }
        None => {
            let public_key: PublicKeyCredentialCreationOptions = (req, settings).into();

            repo::create_user_challenge(
                db,
                UserChallengeModel::new(
                    public_key.challenge.clone(),
                    public_key.user.id,
                    public_key.user.name.clone(),
                    public_key.user.display_name.clone(),
                ),
            )
            .await?;

            public_key.into()
        }
    };

    Ok(res)
}

pub mod join {
    use rand::Rng as _;
    use regex::regex;
    use serde::Serialize;
    use std::vec;
    use uuid::Uuid;
    use validator::{Validate, ValidationError};

    use crate::app::auth::{
        passkey::{
            PublicKeyCredentialCreationOptions, PublicKeyCredentialDescriptor,
            PublicKeyCredentialRequestOptions, PublicKeyCredentialRpEntity,
            PublicKeyCredentialUserEntity,
        },
        repo::UserCredentialModel,
        settings::AuthSettings,
    };

    #[derive(Validate)]
    pub struct Request {
        #[validate(length(min = 5), custom(function = "validate_login"))]
        pub login: String,
        pub name: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Response {
        Creation(PublicKeyCredentialCreationOptions),
        Request(PublicKeyCredentialRequestOptions),
    }

    impl From<(Vec<UserCredentialModel>, &AuthSettings)> for PublicKeyCredentialRequestOptions {
        fn from((user_credentials, settings): (Vec<UserCredentialModel>, &AuthSettings)) -> Self {
            let mut challenge = vec![0u8; 128];
            rand::rng().fill_bytes(&mut challenge);

            Self {
                challenge,
                rp_id: Some(settings.rp.id.clone()),
                allow_credentials: user_credentials
                    .into_iter()
                    .map(|it| PublicKeyCredentialDescriptor {
                        id: it.credential_id,
                    })
                    .collect(),
            }
        }
    }

    impl From<(Request, &AuthSettings)> for PublicKeyCredentialCreationOptions {
        fn from((req, settings): (Request, &AuthSettings)) -> Self {
            // TODO: нужно убрать хардкод
            let user_id = Uuid::now_v7();
            let mut challenge = vec![0u8; 128];
            rand::rng().fill_bytes(&mut challenge);

            Self {
                rp: PublicKeyCredentialRpEntity {
                    id: settings.rp.id.clone(),
                    name: settings.rp.name.clone(),
                },
                challenge,
                user: PublicKeyCredentialUserEntity {
                    id: user_id,
                    name: req.login,
                    display_name: req.name,
                },
            }
        }
    }

    impl From<PublicKeyCredentialCreationOptions> for Response {
        fn from(public_key: PublicKeyCredentialCreationOptions) -> Self {
            Self::Creation(public_key)
        }
    }

    impl From<PublicKeyCredentialRequestOptions> for Response {
        fn from(public_key: PublicKeyCredentialRequestOptions) -> Self {
            Self::Request(public_key)
        }
    }

    fn validate_login(login: &str) -> Result<(), ValidationError> {
        if regex!(r"^[a-z0-9]+$").is_match(login) {
            Ok(())
        } else {
            Err(ValidationError::new("Login must be valid"))
        }
    }

    #[cfg(test)]
    mod tests {
        use bzd_lib::error::Error;

        use crate::app::auth::service::join::validate_login;

        #[test]
        fn test_validate_login() -> Result<(), Error> {
            assert!(validate_login("login").is_ok());
            assert!(validate_login("brg").is_ok());

            assert!(validate_login("log-in").is_err());
            assert!(validate_login("log in").is_err());
            assert!(validate_login("log/in").is_err());
            assert!(validate_login("log/in!").is_err());
            assert!(validate_login(" login").is_err());
            assert!(validate_login("login ").is_err());
            assert!(validate_login("LOGIN").is_err());

            Ok(())
        }
    }
}

pub async fn login(
    db: &DbConn,
    encoder: &dyn Encoder,
    settings: &AuthSettings,
    req: login::Request,
) -> Result<login::Response, AppError> {
    let client_data = ClientData::new(&req.client_data)?;

    if client_data.tp != ClientDataType::Get {
        return Err(AppError::Internal);
    }

    if Origin::new(&client_data.origin)?.rp_id != settings.rp.id {
        return Err(AppError::Internal);
    }

    // TODO: добавить проверку RP ID
    // TODO: добавить проверку флага User Present
    // TODO: добавить проверку sign_count

    let txn = db.begin().await?;

    // TODO: Добавить TTL для user_challenge ~5 мин
    let user_challenge = repo::get_user_challenge_with_lock(&txn, &client_data.challenge).await?;
    let user_credential = repo::get_user_credential(&txn, &req.credential_id).await?;
    let cose_public_key = CoseKey::from_slice(&user_credential.public_key)?;

    if req.credential_id != user_credential.credential_id {
        return Err(AppError::Internal);
    }

    login::verify(&req, &cose_public_key)?;

    let user = repo::get_user_by_id(&txn, user_credential.user_id).await?;

    repo::delete_user_challenge(&txn, user_challenge).await?;

    txn.commit().await?;

    let claims = Claims::new(user.user_id)?;
    let jwt = encoder.encode(&claims)?;

    Ok(login::Response { jwt })
}

pub mod login {
    use bzd_lib::error::Error;
    use coset::CoseKey;
    use ecdsa::signature::Verifier;
    use ecdsa::{VerifyingKey, der::Signature};
    use p256::NistP256;
    use p256::pkcs8::der::Decode as _;
    use serde::Deserialize;
    use serde_with::base64::Base64;
    use serde_with::serde_as;
    use sha2::{Digest as _, Sha256};

    #[serde_as]
    #[derive(Deserialize, Debug)]
    pub struct Request {
        #[serde_as(as = "Base64")]
        pub credential_id: Vec<u8>,
        #[serde_as(as = "Base64")]
        pub client_data: Vec<u8>,
        #[serde_as(as = "Base64")]
        pub authenticator_data: Vec<u8>,
        #[serde_as(as = "Base64")]
        pub signature: Vec<u8>,
    }

    pub struct Response {
        pub jwt: String,
    }

    pub fn verify(req: &Request, public_key: &CoseKey) -> Result<(), Error> {
        let vk: VerifyingKey<NistP256> =
            VerifyingKey::from_sec1_bytes(&public_key.to_sec1_octet_string()?)?;
        let signature: Signature<NistP256> = Signature::from_der(&req.signature)?;
        let client_data_hash = Sha256::digest(&req.client_data).to_vec();

        let mut message: Vec<u8> = req.authenticator_data.clone();
        message.extend(&client_data_hash);

        vk.verify(&message, &signature)?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use base64::{Engine, engine::general_purpose::STANDARD};
        use bzd_lib::error::Error;
        use coset::{CborSerializable, CoseKey};

        use crate::app::auth::service::login::{Request, verify};

        #[test]
        fn test_cose_public_validate() -> Result<(), Error> {
            let cose_public_key = CoseKey::from_slice(&STANDARD.decode("pQECAyYgASFYIKT/8N5PxvKjzOkD9j9T9M19303D+mFKuNWVPyapUffVIlggQkQ4BeBMiV+3+tR7K4YlUnFpw2l1Sa33TkosH00JSBc=")?)?;

            let req = Request {
                credential_id: STANDARD.decode("DwYWIodblU1icJRKXgHNUs3YTu8=")?,
                client_data: STANDARD.decode("eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoiY3hMLUlvRnQ1MEF0M1FJbElCVVhKYUtjSE5lQ3pkc1ZqYjdBUm9QLW51dzFjMHVoclBMZUJoWUNJeWgyYndib1FaNG84bnMwdTBJd2J5ckpEM2dWVENPZnBuXzlWLU1QRnViM2xzYkxra01BS05mV1ZBVUNickRsYWJZLUVhRkIxZGlERWJvNFQ1dUNLLW9VS3ZtWjNPcmkxQmtKbzNZMlpfVXd5bmhIR2FRIiwib3JpZ2luIjoiaHR0cHM6Ly9ycC5iZXpkbmEuYXBwIn0=")?,
                authenticator_data: STANDARD.decode("JgNZV95xeI0O2+g02BDJ/bcG+y9OdhSGGn9ZtWcrlIAdAAAAAA==")?,
                signature: STANDARD.decode("MEQCIH6UFC2iafCvMNQwjTXx22Rpq0rt3jxSsrXj8urG5r66AiADGDFoV1CHFXQeH7cCC+iF1QzrKuHSUPhGVAOpacyPAA==")?,
            };

            assert!(verify(&req, &cose_public_key).is_ok());

            Ok(())
        }
    }
}

pub async fn complete(
    db: &DbConn,
    encoder: &dyn Encoder,
    settings: &AuthSettings,
    req: complete::Request,
) -> Result<complete::Response, AppError> {
    let ad = AuthenticatorData::new(&req.attestation_object.auth_data)?;
    let client_data = ClientData::new(&req.client_data)?;

    if req.credential_id != ad.credential_id {
        return Err(AppError::Internal);
    }

    if client_data.tp != ClientDataType::Create {
        return Err(AppError::Internal);
    }

    if Origin::new(&client_data.origin)?.rp_id != settings.rp.id {
        return Err(AppError::Internal);
    }

    let txn = db.begin().await?;

    // TODO: Добавить TTL для user_challenge ~5 мин
    let user_challenge = repo::get_user_challenge_with_lock(&txn, &client_data.challenge).await?;

    let user = repo::create_user(
        &txn,
        UserModel::new(
            user_challenge.user_id,
            user_challenge.login.clone(),
            user_challenge.name.clone(),
        ),
    )
    .await?;

    repo::create_user_credential(
        &txn,
        UserCredentialModel::new(
            user.user_id,
            req.credential_id,
            ad.cose_public_key.to_vec()?,
            ad.sign_count,
        ),
    )
    .await?;

    repo::delete_user_challenge(&txn, user_challenge).await?;

    txn.commit().await?;

    let claims = Claims::new(user.user_id)?;
    let jwt = encoder.encode(&claims)?;

    Ok(complete::Response { jwt })
}

pub mod complete {
    use serde::Deserialize;
    use serde_with::base64::Base64;
    use serde_with::serde_as;

    use crate::app::auth::passkey::AttestationObject;

    #[serde_as]
    #[derive(Deserialize, Debug)]
    pub struct Request {
        #[serde_as(as = "Base64")]
        pub credential_id: Vec<u8>,
        #[serde_as(as = "Base64")]
        pub client_data: Vec<u8>,
        #[serde_as(as = "Base64")]
        pub attestation_object: AttestationObject,
    }

    pub struct Response {
        pub jwt: String,
    }
}
