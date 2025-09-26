use sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel as _, sea_query::OnConflict};

use crate::app::error::AppError;

pub mod contact;

pub async fn create_contact<T: ConnectionTrait>(
    db: &T,
    model: contact::Model,
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
