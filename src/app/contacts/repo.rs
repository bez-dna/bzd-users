use std::sync::Arc;

use sea_orm::{DbConn, EntityTrait, IntoActiveModel as _, sea_query::OnConflict};
use tonic::async_trait;

use crate::app::error::AppError;

pub mod contact;

pub struct ContactsRepoDb {
    pub db: Arc<DbConn>,
}

#[async_trait]
pub trait ContactsRepo: Send + Sync {
    async fn create_contact(&self, model: contact::Model) -> Result<(), AppError>;
}

#[async_trait]
impl ContactsRepo for ContactsRepoDb {
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
