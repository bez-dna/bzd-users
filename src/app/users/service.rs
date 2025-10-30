use sea_orm::DbConn;

use crate::app::{
    error::AppError,
    state::CryptoState,
    users::{UserDecryptedPhone, repo},
};

pub async fn get_user(
    db: &DbConn,
    crypto: &CryptoState,
    req: get_user::Request,
) -> Result<get_user::Response, AppError> {
    let user = UserDecryptedPhone::new(
        repo::get_user_by_id(db, req.user_id)
            .await?
            .ok_or(AppError::NotFound)?,
        crypto,
    )?;

    Ok(get_user::Response { user })
}

pub mod get_user {
    use uuid::Uuid;

    use crate::app::users::UserDecryptedPhone;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub user: UserDecryptedPhone,
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

    use crate::app::users::UserDecryptedPhone;

    pub struct Request {
        pub user_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub users: Vec<UserDecryptedPhone>,
    }
}
