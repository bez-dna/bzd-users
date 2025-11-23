use sea_orm::{ColumnTrait as _, ConnectionTrait, EntityTrait, QueryFilter as _, QuerySelect as _};
use uuid::Uuid;

use crate::app::error::AppError;

mod contact;
pub mod user;

pub async fn get_user_by_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Option<user::Model>, AppError> {
    let user = user::Entity::find_by_id(user_id).one(db).await?;

    Ok(user)
}

pub async fn get_users_by_ids<T: ConnectionTrait>(
    db: &T,
    user_ids: Vec<Uuid>,
) -> Result<Vec<user::Model>, AppError> {
    let topics = user::Entity::find()
        .filter(user::Column::UserId.is_in(user_ids))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_users_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<user::Model>, AppError> {
    let users = user::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            user::Entity::belongs_to(contact::Entity)
                .from(user::Column::Phone)
                .to(contact::Column::Phone)
                .into(),
        )
        .filter(contact::Column::UserId.eq(user_id))
        .filter(user::Column::UserId.ne(user_id))
        .all(db)
        .await?;

    Ok(users)
}
