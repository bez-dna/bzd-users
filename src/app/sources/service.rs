use sea_orm::DbConn;

use crate::app::{error::AppError, sources::repo};

pub async fn get_sources(
    _db: &DbConn,
    _req: get_sources::Request,
) -> Result<get_sources::Response, AppError> {
    Ok(get_sources::Response {})
}

pub mod get_sources {
    pub struct Request {}

    pub struct Response {}
}

pub async fn create_source(
    db: &DbConn,
    req: create_source::Request,
) -> Result<create_source::Response, AppError> {
    let source_user = repo::get_user_by_id(db, req.source_user_id).await?;
    let source = repo::create_source(
        db,
        repo::source::Model::new(req.user_id, source_user.user_id),
    )
    .await?;

    Ok(create_source::Response { source })
}

pub mod create_source {
    use uuid::Uuid;

    use crate::app::sources::repo;

    pub struct Request {
        pub user_id: Uuid,
        pub source_user_id: Uuid,
    }

    pub struct Response {
        pub source: repo::source::Model,
    }
}
