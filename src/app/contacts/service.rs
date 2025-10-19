use sea_orm::DbConn;

use crate::app::{contacts::repo, error::AppError, state::CryptoState};

pub async fn create_contacts(
    db: &DbConn,
    crypto: &CryptoState,
    req: create_contacts::Request,
) -> Result<create_contacts::Response, AppError> {
    for it in req.contacts {
        repo::create_contact(
            db,
            repo::contact::Model::new(
                req.user_id,
                crypto.encrypt(&it.phone.to_string())?,
                it.name,
                it.device_contact_id,
            ),
        )
        .await?;
    }

    Ok(create_contacts::Response {})
}

pub mod create_contacts {
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Validate, Debug)]
    pub struct Request {
        pub user_id: Uuid,
        #[validate(nested)]
        pub contacts: Vec<Contact>,
    }

    #[derive(Validate, Debug)]
    pub struct Contact {
        pub name: String,
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone: i64,
        pub device_contact_id: String,
    }

    pub struct Response {}
}
