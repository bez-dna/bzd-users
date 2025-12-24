use std::sync::Arc;

use tonic::async_trait;

use crate::app::{
    contacts::repo::{self, ContactsRepo},
    crypto::service::CryptoService,
    error::AppError,
};

pub struct ContactsServiceImpl {
    repo: Arc<dyn ContactsRepo>,
    crypto: Arc<dyn CryptoService>,
}

impl ContactsServiceImpl {
    pub fn new(repo: Arc<dyn ContactsRepo>, crypto: Arc<dyn CryptoService>) -> Self {
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
impl ContactsService for ContactsServiceImpl {
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

    #[cfg(test)]
    mod tests {
        use std::sync::Arc;

        use bzd_lib::error::Error;
        use uuid::Uuid;

        use crate::app::contacts::repo::MockContactsRepo;
        use crate::app::contacts::service::ContactsService;
        use crate::app::contacts::service::ContactsServiceImpl;
        use crate::app::contacts::service::create_contacts::{Contact, Request};
        use crate::app::crypto::service::MockCryptoService;

        #[tokio::test]
        async fn successfully_create_contacts() -> Result<(), Error> {
            let mut repo = MockContactsRepo::new();
            repo.expect_create_contact()
                .times(2)
                .returning(|_| Box::pin(async move { Ok(()) }));

            let mut crypto = MockCryptoService::new();
            crypto.expect_encrypt().times(2).returning(|_| Ok(vec![]));

            let req = Request {
                user_id: Uuid::now_v7(),
                contacts: vec![
                    Contact {
                        name: "NAME_1".into(),
                        phone: 111,
                        device_contact_id: "DC_ID_1".into(),
                    },
                    Contact {
                        name: "NAME_2".into(),
                        phone: 222,
                        device_contact_id: "DC_ID_2".into(),
                    },
                ],
            };

            let service = ContactsServiceImpl::new(Arc::new(repo), Arc::new(crypto));

            let res = service.create_contacts(req).await;

            assert!(res.is_ok());

            Ok(())
        }
    }
}
