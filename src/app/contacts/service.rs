use sea_orm::DbConn;

use crate::app::{
    contacts::repo::{self, ContactModel},
    crypto::state::CryptoState,
    error::AppError,
};

pub async fn create_contacts(
    db: &DbConn,
    crypto: &CryptoState,
    req: create_contacts::Request,
) -> Result<create_contacts::Response, AppError> {
    for it in req.contacts {
        repo::create_contact(
            db,
            ContactModel::new(
                req.user_id,
                crypto.encryptor.encrypt(&it.phone.to_string())?,
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

    #[cfg(test)]
    mod tests {
        use std::sync::Arc;

        use bzd_lib::error::Error;
        use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
        use uuid::Uuid;

        use crate::app::{
            contacts::service::{
                self,
                create_contacts::{Contact, Request},
            },
            crypto::{encryptor::MockEncryptor, state::CryptoState},
        };

        #[tokio::test]
        async fn successfully_create_contacts() -> Result<(), Error> {
            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_exec_results([
                    MockExecResult {
                        last_insert_id: 0,
                        rows_affected: 1,
                    },
                    MockExecResult {
                        last_insert_id: 0,
                        rows_affected: 1,
                    },
                ])
                .into_connection();

            let mut encryptor = MockEncryptor::new();
            encryptor
                .expect_encrypt()
                .times(2)
                .returning(|_| Ok(vec![1, 2, 3]));

            let crypto = CryptoState {
                encryptor: Arc::new(encryptor),
            };

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

            let res = service::create_contacts(&db, &crypto, req).await;

            assert!(res.is_ok());

            Ok(())
        }
    }
}
