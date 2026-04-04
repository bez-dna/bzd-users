use rand::RngExt as _;
use sea_orm::DbConn;

use crate::app::{
    auth::{
        encoder::{Claims, Encoder},
        repo::{self, UserModel, VerificationModel},
    },
    crypto::state::CryptoState,
    error::AppError,
};

pub async fn join(
    db: &DbConn,
    crypto: &CryptoState,
    req: join::Request,
) -> Result<join::Response, AppError> {
    let cipher_phone = crypto.encryptor.encrypt(&req.phone.to_string())?;
    let verification = repo::get_verification_by_phone(db, cipher_phone.clone()).await?;

    Ok(join::Response { verification })
}

pub mod join {
    use serde::Serialize;
    use validator::Validate;

    use crate::app::auth::repo::VerificationModel;

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone: i64,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub verification: VerificationModel,
    }
}

pub async fn complete(
    db: &DbConn,
    encoder: &dyn Encoder,
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
                repo::create_user(db, UserModel::new(verification.phone.clone(), name)).await?
            } else {
                return Err(AppError::CompleteName);
            }
        }
    };

    let claims = Claims::new(user.user_id)?;
    let jwt = encoder.encode(&claims)?;

    // TODO: нужно будет вернуть удаление когда будет нормальный флоу
    // repo::delete_verification(db, verification).await?;

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
        use sea_orm::{DatabaseBackend, MockDatabase};
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

            let res = service::complete(&db, &encoder, req).await;

            assert!(res.is_err());
            assert!(matches!(res, Err(AppError::VerificationCode)));

            Ok(())
        }

        mod stub {
            use crate::app::auth::repo::VerificationModel;

            pub fn verification(code: i32) -> VerificationModel {
                VerificationModel::new(vec![], code)
            }
        }
    }
}

pub async fn create_verification(
    db: &DbConn,
    crypto: &CryptoState,
    req: create_verification::Request,
) -> Result<create_verification::Response, AppError> {
    let cipher_phone = crypto.encryptor.encrypt(&req.phone.to_string())?;
    let code = rand::rng().random_range(1000..9999);

    let verification =
        repo::create_verification(db, VerificationModel::new(cipher_phone, code)).await?;

    Ok(create_verification::Response { verification })
}

pub mod create_verification {
    use serde::Serialize;
    use validator::Validate;

    use crate::app::auth::repo::VerificationModel;

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone: i64,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub verification: VerificationModel,
    }
}
