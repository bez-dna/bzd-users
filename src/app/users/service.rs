use sea_orm::DbConn;

use crate::app::{
    error::AppError,
    state::CryptoState,
    users::{repo, service::get_users::UserDecryptedPhone},
};

pub async fn get_user(db: &DbConn, req: get_user::Request) -> Result<get_user::Response, AppError> {
    let user = repo::get_user_by_id(db, req.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(get_user::Response { user })
}

pub mod get_user {
    use uuid::Uuid;

    use crate::app::users::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub user: repo::user::Model,
    }
}

pub async fn get_users(
    db: &DbConn,
    crypto: &CryptoState,
    req: get_users::Request,
) -> Result<get_users::Response, AppError> {
    let users = repo::get_users_by_user_ids(db, req.user_ids)
        .await?
        .into_iter()
        .map(|user| UserDecryptedPhone::new(user, crypto))
        .collect::<Result<Vec<UserDecryptedPhone>, _>>()?;

    Ok(get_users::Response { users })
}

pub mod get_users {
    use uuid::Uuid;

    use crate::app::{error::AppError, state::CryptoState, users::repo};

    pub struct Request {
        pub user_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub users: Vec<UserDecryptedPhone>,
    }

    pub struct UserDecryptedPhone {
        pub user_id: Uuid,
        pub phone: String,
        pub name: String,
    }

    impl UserDecryptedPhone {
        pub fn new(user: repo::user::Model, crypto: &CryptoState) -> Result<Self, AppError> {
            Ok(Self {
                user_id: user.user_id,
                phone: crypto.decrypt(&user.phone)?,
                name: user.name,
            })
        }
    }

    // impl From<repo::user::Model> for UserDecryptedPhone {
    //     fn from(user: repo::user::Model) -> Self {
    //         Self {
    //             user_id: user.user_id,
    //             phone: "QQQ".into(),
    //             name: user.name,
    //         }
    //     }
    // }
}
