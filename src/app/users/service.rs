use sea_orm::DbConn;

use crate::app::{error::AppError, users::repo};

pub async fn get_user(db: &DbConn, req: get_user::Request) -> Result<get_user::Response, AppError> {
    let user = repo::find_user_by_id(db, req.user_id).await?;

    Ok(get_user::Response { user })
}

pub mod get_user {
    use uuid::Uuid;

    use crate::app::users::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub user: Option<repo::user::Model>,
    }
}
