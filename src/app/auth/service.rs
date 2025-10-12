use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rand::Rng as _;
use sea_orm::DbConn;

use crate::app::{
    auth::{Claims, repo, state::AuthState},
    error::AppError,
    state::CryptoState,
};

pub async fn join(
    db: &DbConn,
    AuthState {
        verification_client,
        ..
    }: &AuthState,
    crypto: &CryptoState,
    req: join::Request,
) -> Result<join::Response, AppError> {
    /*
    Не учитывается гонка, ну точнее будет pg ошибка на уникальном индексе и просто отлуп с ошибкой
    Нужно будет этот кейс в будущем поправить, но пока не крит, маловероятный кейс когда один
    нормальный юзер будет долбить ручку с N-устройств в одну секунду времени.
    */

    let cipher_phone_number = crypto.encrypt(&req.phone_number.to_string())?;

    let verification =
        match repo::find_verification_by_phone(db, cipher_phone_number.clone()).await? {
            /*
            request_id не передается в send, а это значит что на каждый вызов будет улетать новый запрос в тг,
            это может привести к тому, что можно всадить весь бюджет отправки кодов или задолбать жертву.

            план такой: нужно будет при наличии verification проверять его created_at и если ограничить отправку кодов,
            например, раз в минуту или любой другой механизм.

            ну и на стороне тг есть защита от флуда, так что прям супер быстро не провернуть эту атаку
            */
            Some(verification) => {
                verification_client
                    .send(None, req.phone_number, verification.code.clone())
                    .await?;

                verification
            }
            None => {
                let code: i32 = rand::rng().random_range(1000..9999);

                let verification_response = verification_client
                    .send(None, req.phone_number, code.to_string())
                    .await?;

                repo::create_verification(
                    db,
                    repo::verification::Model::new(
                        cipher_phone_number,
                        code,
                        verification_response.request_id,
                    ),
                )
                .await?
            }
        };

    let user = repo::find_user_by_phone(db, verification.phone.clone()).await?;

    Ok(join::Response { verification, user })
}

pub mod join {
    use serde::Serialize;
    use validator::Validate;

    use crate::app::auth::repo::{user, verification};

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone_number: i64,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub verification: verification::Model,
        pub user: Option<user::Model>,
    }
}

pub async fn complete(
    db: &DbConn,
    state: &AuthState,
    req: complete::Request,
) -> Result<complete::Response, AppError> {
    let verification = repo::find_verification(db, req.verification_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if verification.code != req.code {
        return Err(AppError::VerificationCode);
    }

    let user = match repo::find_user_by_phone(db, verification.phone.clone()).await? {
        Some(user) => user,
        None => {
            if let Some(name) = req.name {
                repo::create_user(db, repo::user::Model::new(verification.phone.clone(), name))
                    .await?
            } else {
                return Err(AppError::CompleteName);
            }
        }
    };

    let claims = Claims {
        sub: user.user_id,
        exp: (Utc::now() + Duration::days(300)).timestamp().try_into()?,
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(&state.private_key)?,
    )?;

    repo::delete_verification(db, verification).await?;

    Ok(complete::Response { jwt })
}

pub mod complete {
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Validate)]
    pub struct Request {
        pub verification_id: Uuid,
        pub code: String,
        #[validate(length(min = 3))]
        pub name: Option<String>,
    }

    pub struct Response {
        pub jwt: String,
    }
}
