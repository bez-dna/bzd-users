use sea_orm::{
    ActiveModelTrait, ColumnTrait as _, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    ModelTrait as _, QueryFilter as _,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod user;
pub mod verification;

pub async fn create_verification<T: ConnectionTrait>(
    db: &T,
    model: verification::Model,
) -> Result<verification::Model, AppError> {
    let message = model.into_active_model().insert(db).await?;

    Ok(message)
}

pub async fn find_verification_by_phone<T: ConnectionTrait>(
    db: &T,
    phone: String,
) -> Result<Option<verification::Model>, AppError> {
    let verification = verification::Entity::find()
        .filter(verification::Column::Phone.eq(phone))
        .one(db)
        .await?;

    Ok(verification)
}

pub async fn find_verification<T: ConnectionTrait>(
    db: &T,
    verification_id: Uuid,
) -> Result<Option<verification::Model>, AppError> {
    Ok(verification::Entity::find_by_id(verification_id)
        .one(db)
        .await?)
}

pub async fn delete_verification<T: ConnectionTrait>(
    db: &T,
    model: verification::Model,
) -> Result<(), AppError> {
    model.delete(db).await?;

    Ok(())
}

pub async fn create_user<T: ConnectionTrait>(
    db: &T,
    model: user::Model,
) -> Result<user::Model, AppError> {
    let user = model.into_active_model().insert(db).await?;

    Ok(user)
}

pub async fn find_user_by_phone<T: ConnectionTrait>(
    db: &T,
    phone: String,
) -> Result<Option<user::Model>, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Phone.eq(phone))
        .one(db)
        .await?;

    Ok(user)
}
