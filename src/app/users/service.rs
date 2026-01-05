use sea_orm::DbConn;

use crate::app::{
    crypto::state::CryptoState,
    error::AppError,
    users::{repo, service::get_user::User},
};

pub async fn get_user(
    db: &DbConn,
    crypto: &CryptoState,
    req: get_user::Request,
) -> Result<get_user::Response, AppError> {
    let user = repo::get_user_by_id(db, req.user_id).await?;
    let user: User = (user, crypto).try_into()?;

    Ok(get_user::Response { user })
}

pub mod get_user {
    use uuid::Uuid;

    use crate::app::{crypto::state::CryptoState, error::AppError, users::repo::UserModel};

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub user: User,
    }

    pub struct User {
        pub user_id: Uuid,
        pub phone: String,
        pub name: String,
        pub abbr: String,
        pub color: String,
    }

    impl TryFrom<(UserModel, &CryptoState)> for User {
        type Error = AppError;

        fn try_from((user, crypto): (UserModel, &CryptoState)) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: user.user_id,
                phone: crypto.encryptor.decrypt(&user.phone)?,
                name: user.name.clone(),
                abbr: user.abbr(),
                color: user.color(),
            })
        }
    }
}

pub async fn get_users(
    db: &DbConn,
    crypto: &CryptoState,
    req: get_users::Request,
) -> Result<get_users::Response, AppError> {
    let users = repo::get_users_by_ids(db, req.user_ids)
        .await?
        .into_iter()
        .map(|user| (user, crypto).try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(get_users::Response { users })
}

pub mod get_users {
    use uuid::Uuid;

    use crate::app::{crypto::state::CryptoState, error::AppError, users::repo::UserModel};

    pub struct Request {
        pub user_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub users: Vec<User>,
    }

    pub struct User {
        pub user_id: Uuid,
        pub phone: String,
        pub name: String,
        pub abbr: String,
        pub color: String,
    }

    impl TryFrom<(UserModel, &CryptoState)> for User {
        type Error = AppError;

        fn try_from((user, crypto): (UserModel, &CryptoState)) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: user.user_id,
                phone: crypto.encryptor.decrypt(&user.phone)?,
                name: user.name.clone(),
                abbr: user.abbr(),
                color: user.color(),
            })
        }
    }
}

pub async fn get_user_users(
    db: &DbConn,
    req: get_user_users::Request,
) -> Result<get_user_users::Response, AppError> {
    let users = repo::get_users_by_user_id(db, req.user_id).await?;

    Ok(get_user_users::Response { users })
}

pub mod get_user_users {
    use uuid::Uuid;

    use crate::app::users::repo::UserModel;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub users: Vec<UserModel>,
    }
}
