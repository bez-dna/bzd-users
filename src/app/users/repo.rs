use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod user;

pub async fn find_user_by_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Option<user::Model>, AppError> {
    let user = user::Entity::find_by_id(user_id).one(db).await?;

    Ok(user)
}
