use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::{error::AppError, sources::repo};

pub async fn get_sources(
    db: &DbConn,
    req: get_sources::Request,
) -> Result<get_sources::Response, AppError> {
    let reverse_contact_ids: Vec<Uuid> = repo::find_reverse_contacts_by_user_id(db, req.user_id)
        .await?
        .iter()
        .map(|it| it.user_id)
        .collect();

    let contacts = repo::find_contacts_by_user_id(db, req.user_id)
        .await?
        .into_iter()
        .filter_map(|(contact, user)| match user {
            Some(user) => Some((contact, user)),
            None => None,
        })
        .filter(|(_, user)| reverse_contact_ids.contains(&user.user_id))
        .collect();

    Ok(get_sources::Response { contacts })
}

pub mod get_sources {
    use uuid::Uuid;

    use crate::app::sources::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub contacts: Vec<(repo::contact::Model, repo::user::Model)>,
    }

    // pub struct ContactWithUser {
    //     pub contact: repo::contact::Model,
    //     pub user: repo::user::Model,
    // }
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
