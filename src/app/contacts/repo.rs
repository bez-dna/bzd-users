use sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel as _, sea_query::OnConflict};

use crate::app::error::AppError;

mod contact;

pub type ContactModel = contact::Model;

pub async fn create_contact<T: ConnectionTrait>(
    db: &T,
    model: ContactModel,
) -> Result<(), AppError> {
    contact::Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::columns([contact::Column::UserId, contact::Column::Phone])
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;

    Ok(())
}

mod create_contact {
    #[cfg(test)]
    mod tests {
        use bzd_lib::error::Error;
        use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
        use uuid::Uuid;

        use crate::app::contacts::repo::{self, ContactModel};

        #[tokio::test]
        async fn successfully_create_contact() -> Result<(), Error> {
            // Синтетический тест чтобы провериить мок от sea_orm
            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_exec_results([MockExecResult {
                    last_insert_id: 0,
                    rows_affected: 1,
                }])
                .into_connection();

            let res = repo::create_contact(
                &db,
                ContactModel::new(Uuid::now_v7(), vec![], "NAME".into(), "DC_ID".into()),
            )
            .await;

            assert!(res.is_ok());

            Ok(())
        }
    }
}
