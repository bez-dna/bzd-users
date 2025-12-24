use std::sync::Arc;

use sea_orm::{DbConn, EntityTrait, IntoActiveModel as _, sea_query::OnConflict};
use tonic::async_trait;

use crate::app::error::AppError;

pub mod contact;

pub struct ContactsRepoImpl {
    pub db: Arc<DbConn>,
}

impl ContactsRepoImpl {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait ContactsRepo: Send + Sync {
    async fn create_contact(&self, model: contact::Model) -> Result<(), AppError>;
}

#[async_trait]
impl ContactsRepo for ContactsRepoImpl {
    async fn create_contact(&self, model: contact::Model) -> Result<(), AppError> {
        contact::Entity::insert(model.into_active_model())
            .on_conflict(
                OnConflict::columns([contact::Column::UserId, contact::Column::Phone])
                    .do_nothing()
                    .to_owned(),
            )
            .do_nothing()
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }
}

mod create_contact {
    #[cfg(test)]
    mod tests {
        use std::sync::Arc;

        use bzd_lib::error::Error;
        use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
        use uuid::Uuid;

        use crate::app::contacts::repo::{ContactsRepo, ContactsRepoImpl, contact};

        #[tokio::test]
        async fn successfully_create_contact() -> Result<(), Error> {
            // Синтетический тест чтобы провериить мок от sea_orm
            let db = MockDatabase::new(DatabaseBackend::Postgres);
            let db = db.append_exec_results([MockExecResult {
                last_insert_id: 0,
                rows_affected: 1,
            }]);

            let db = db.into_connection();
            let repo = ContactsRepoImpl::new(Arc::new(db));

            let res = repo
                .create_contact(contact::Model::new(
                    Uuid::now_v7(),
                    vec![],
                    "NAME".into(),
                    "DC_ID".into(),
                ))
                .await;

            assert!(res.is_ok());

            Ok(())
        }
    }
}
