use sea_orm::{
    ColumnTrait as _, ConnectionTrait, DbErr, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod source;
mod user;

pub async fn create_source<T: ConnectionTrait>(
    db: &T,
    model: source::Model,
) -> Result<source::Model, AppError> {
    source::Entity::insert(model.clone().into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;

    let source = source::Entity::find()
        .filter(source::Column::UserId.eq(model.user_id))
        .filter(source::Column::SourceUserId.eq(model.source_user_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("".into()))?;

    Ok(source)
}

pub async fn get_user_by_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<user::Model, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("".into()))?;

    Ok(user)
}
