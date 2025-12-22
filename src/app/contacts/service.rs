use std::sync::Arc;

use tonic::async_trait;

use crate::app::{
    contacts::repo::{self, ContactsRepo},
    error::AppError,
    state::CryptoState,
};

pub struct ContactsServiceImpl<R: ContactsRepo> {
    repo: Arc<R>,
    crypto: Arc<CryptoState>,
}

impl<R: ContactsRepo> ContactsServiceImpl<R> {
    pub fn new(repo: Arc<R>, crypto: Arc<CryptoState>) -> Self {
        Self { repo, crypto }
    }
}

#[async_trait]
pub trait ContactsService: Send + Sync {
    async fn create_contacts(
        &self,
        req: create_contacts::Request,
    ) -> Result<create_contacts::Response, AppError>;
}

#[async_trait]
impl<R: ContactsRepo> ContactsService for ContactsServiceImpl<R> {
    async fn create_contacts(
        &self,
        req: create_contacts::Request,
    ) -> Result<create_contacts::Response, AppError> {
        for it in req.contacts {
            self.repo
                .create_contact(repo::contact::Model::new(
                    req.user_id,
                    self.crypto.encrypt(&it.phone.to_string())?,
                    it.name,
                    it.device_contact_id,
                ))
                .await?;
        }

        Ok(create_contacts::Response {})
    }
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
