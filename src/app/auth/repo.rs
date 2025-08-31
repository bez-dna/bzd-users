use sea_orm::{
    ActiveModelTrait, ColumnTrait as _, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _,
};

use crate::app::auth::error::AuthError;

pub mod verification;

pub async fn create_verification<T: ConnectionTrait>(
    db: &T,
    model: verification::Model,
) -> Result<verification::Model, AuthError> {
    let message = model.into_active_model().insert(db).await?;

    Ok(message)
}

pub async fn find_verification_by_phone<T: ConnectionTrait>(
    db: &T,
    phone: String,
) -> Result<Option<verification::Model>, AuthError> {
    let verification = verification::Entity::find()
        .filter(verification::Column::Phone.eq(phone))
        .one(db)
        .await?;

    Ok(verification)
}
