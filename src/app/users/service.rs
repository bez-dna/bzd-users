use sea_orm::DbConn;

use crate::app::{error::AppError, users::repo};

pub async fn get_user(db: &DbConn, req: get_user::Request) -> Result<get_user::Response, AppError> {
    let user = repo::get_user_by_id(db, req.user_id).await?;

    Ok(get_user::Response { user })
}

pub mod get_user {
    use uuid::Uuid;

    use crate::app::users::repo::UserModel;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub user: UserModel,
    }
}

pub async fn get_users(
    db: &DbConn,
    req: get_users::Request,
) -> Result<get_users::Response, AppError> {
    let users = repo::get_users_by_ids(db, req.user_ids).await?;

    Ok(get_users::Response { users })
}

pub mod get_users {
    use uuid::Uuid;

    use crate::app::users::repo::UserModel;

    pub struct Request {
        pub user_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub users: Vec<UserModel>,
    }
}

pub async fn get_user_users(
    db: &DbConn,
    req: get_user_users::Request,
) -> Result<get_user_users::Response, AppError> {
    // TODO: нужно добавить обратную проверку на нахождение в контактах, иначе это можно использовать как дырку
    // для получения номеров чужих акков
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
