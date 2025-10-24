use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::{error::AppError, sources::repo};

pub async fn get_sources(
    db: &DbConn,
    req: get_sources::Request,
) -> Result<get_sources::Response, AppError> {
    let sources = repo::find_sources_by_user_id(db, req.user_id).await?;

    let source_user_ids: Vec<Uuid> = sources.iter().map(|it| it.source_user_id).collect();

    let reverse_contact_ids: Vec<Uuid> = repo::find_reverse_contacts_by_user_id(db, req.user_id)
        .await?
        .iter()
        .map(|it| it.user_id)
        .collect();

    let contacts_with_user = repo::find_contacts_by_user_id(db, req.user_id)
        .await?
        .into_iter()
        .filter_map(|(contact, user)| {
            user.filter(|it| reverse_contact_ids.contains(&it.user_id))
                .map(|user| get_sources::ContactWithUser { contact, user })
        })
        .filter(|it| !source_user_ids.contains(&it.user.user_id))
        .collect();

    Ok(get_sources::Response {
        sources,
        contacts_with_user,
    })
}

pub mod get_sources {
    use uuid::Uuid;

    use crate::app::sources::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub contacts_with_user: Vec<ContactWithUser>,
        pub sources: Vec<repo::source::Model>,
    }

    pub struct ContactWithUser {
        pub contact: repo::contact::Model,
        pub user: repo::user::Model,
    }
}

pub async fn get_source(
    db: &DbConn,
    req: get_source::Request,
) -> Result<get_source::Response, AppError> {
    let source = repo::get_source_by_id(db, req.source_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if source.user_id != req.user_id {
        return Err(AppError::NotFound);
    }

    Ok(source.into())
}

pub mod get_source {
    use uuid::Uuid;

    use crate::app::sources::repo;

    pub struct Request {
        pub user_id: Uuid,
        pub source_id: Uuid,
    }

    pub struct Response {
        pub source: repo::source::Model,
    }

    impl From<repo::source::Model> for Response {
        fn from(source: repo::source::Model) -> Self {
            Self { source }
        }
    }
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
