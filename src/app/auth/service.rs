use rand::Rng as _;
use sea_orm::DbConn;

use crate::app::{
    auth::{
        encoder::{Claims, Encoder},
        repo::{self, UserModel, VerificationModel},
        verification::VerificationClient,
    },
    crypto::state::CryptoState,
    error::AppError,
};

pub async fn join(
    db: &DbConn,
    verification_client: &VerificationClient,
    crypto: &CryptoState,
    req: join::Request,
) -> Result<join::Response, AppError> {
    /*
    Не учитывается гонка, ну точнее будет pg ошибка на уникальном индексе и просто отлуп с ошибкой
    Нужно будет этот кейс в будущем поправить, но пока не крит, маловероятный кейс когда один
    нормальный юзер будет долбить ручку с N-устройств в одну секунду времени.
    */

    let cipher_phone_number = crypto.encryptor.encrypt(&req.phone_number.to_string())?;

    let verification = match repo::find_verification_by_phone(db, cipher_phone_number.clone())
        .await?
    {
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
                VerificationModel::new(cipher_phone_number, code, verification_response.request_id),
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

    use crate::app::auth::repo::{UserModel, VerificationModel};

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone_number: i64,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub verification: VerificationModel,
        pub user: Option<UserModel>,
    }
}

pub async fn complete(
    db: &DbConn,
    encoder: &dyn Encoder,
    req: complete::Request,
    debug: Option<bool>,
) -> Result<complete::Response, AppError> {
    let verification = repo::find_verification(db, req.verification_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(debug) = debug
        && !debug
    {
        if verification.code != req.code {
            return Err(AppError::VerificationCode);
        }
    }

    let user = match repo::find_user_by_phone(db, verification.phone.clone()).await? {
        Some(user) => user,
        None => {
            if let Some(name) = req.name {
                repo::create_user(db, UserModel::new(verification.phone.clone(), name)).await?
            } else {
                return Err(AppError::CompleteName);
            }
        }
    };

    let claims = Claims::new(user.user_id)?;
    let jwt = encoder.encode(&claims)?;

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
        #[validate(length(min = 2))]
        pub name: Option<String>,
    }

    pub struct Response {
        pub jwt: String,
    }

    #[cfg(test)]
    mod tests {
        use bzd_lib::error::Error;
        use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
        use uuid::Uuid;

        use crate::app::{
            auth::{
                encoder::MockEncoder,
                service::{self, complete::Request},
            },
            error::AppError,
        };

        #[tokio::test]
        async fn test_complete_without_debug() -> Result<(), Error> {
            let encoder = MockEncoder::new();

            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([[stub::verification(1110)]])
                .into_connection();

            let req = Request {
                verification_id: Uuid::now_v7(),
                code: String::from("1111"),
                name: Some(String::new()),
            };

            let res = service::complete(&db, &encoder, req, Some(false)).await;

            assert!(res.is_err());
            assert!(matches!(res, Err(AppError::VerificationCode)));

            Ok(())
        }

        #[tokio::test]
        async fn test_complete_with_debug() -> Result<(), Error> {
            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([[stub::verification(1110)]])
                .append_query_results([[stub::user()]])
                .append_exec_results([MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                }])
                .into_connection();

            let req = Request {
                verification_id: Uuid::now_v7(),
                code: String::from("1111"),
                name: None,
            };

            let mut encoder = MockEncoder::new();
            encoder
                .expect_encode()
                .times(1)
                .returning(|_| Ok("JWT".into()));

            let res = service::complete(&db, &encoder, req, Some(true)).await;

            assert!(res.is_ok());

            assert_eq!("JWT", res?.jwt);

            Ok(())
        }

        mod stub {
            use crate::app::auth::repo::{UserModel, VerificationModel};

            pub fn user() -> UserModel {
                UserModel::new(vec![], String::new())
            }

            pub fn verification(code: i32) -> VerificationModel {
                VerificationModel::new(vec![], code, String::new())
            }
        }
    }
}
