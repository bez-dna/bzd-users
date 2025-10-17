use sea_orm::{ColumnTrait as _, ConnectionTrait, EntityTrait, QueryFilter as _};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod user;

pub async fn get_user_by_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Option<user::Model>, AppError> {
    let user = user::Entity::find_by_id(user_id).one(db).await?;

    Ok(user)
}

pub async fn get_users_by_user_ids<T: ConnectionTrait>(
    db: &T,
    user_ids: Vec<Uuid>,
) -> Result<Vec<user::Model>, AppError> {
    let topics = user::Entity::find()
        .filter(user::Column::UserId.is_in(user_ids))
        .all(db)
        .await?;

    Ok(topics)
}
