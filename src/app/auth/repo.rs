use sea_orm::{
    ActiveModelTrait, ColumnTrait as _, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    ModelTrait as _, QueryFilter as _, QuerySelect as _,
};
use uuid::Uuid;

use crate::app::error::AppError;

mod user;
mod user_challenge;
mod user_credential;

pub type UserModel = user::Model;
pub type UserChallengeModel = user_challenge::Model;
pub type UserCredentialModel = user_credential::Model;

pub async fn get_user_by_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<UserModel, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user)
}

pub async fn create_user<T: ConnectionTrait>(
    db: &T,
    model: UserModel,
) -> Result<UserModel, AppError> {
    let user = model.into_active_model().insert(db).await?;

    Ok(user)
}
pub async fn find_user_by_login_with_user_credentials<T: ConnectionTrait>(
    db: &T,
    login: &String,
) -> Result<Option<(UserModel, Vec<UserCredentialModel>)>, AppError> {
    match user::Entity::find()
        .filter(user::Column::Login.eq(login))
        .one(db)
        .await?
    {
        Some(user) => {
            let user_credentials = user.find_related(user_credential::Entity).all(db).await?;

            Ok(Some((user, user_credentials)))
        }
        None => Ok(None),
    }
}

pub async fn create_user_challenge<T: ConnectionTrait>(
    db: &T,
    model: UserChallengeModel,
) -> Result<UserChallengeModel, AppError> {
    let user_challenge = model.into_active_model().insert(db).await?;

    Ok(user_challenge)
}

pub async fn get_user_challenge_with_lock<T: ConnectionTrait>(
    db: &T,
    challenge: &[u8],
) -> Result<UserChallengeModel, AppError> {
    let user_challenge = user_challenge::Entity::find()
        .filter(user_challenge::Column::Challenge.eq(challenge))
        .lock_exclusive()
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user_challenge)
}

pub async fn delete_user_challenge<T: ConnectionTrait>(
    db: &T,
    model: UserChallengeModel,
) -> Result<(), AppError> {
    model.delete(db).await?;

    Ok(())
}

pub async fn create_user_credential<T: ConnectionTrait>(
    db: &T,
    model: UserCredentialModel,
) -> Result<UserCredentialModel, AppError> {
    let user_credential = model.into_active_model().insert(db).await?;

    Ok(user_credential)
}

pub async fn get_user_credential<T: ConnectionTrait>(
    db: &T,
    credential_id: &[u8],
) -> Result<UserCredentialModel, AppError> {
    let user_credential = user_credential::Entity::find()
        .filter(user_credential::Column::CredentialId.eq(credential_id))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user_credential)
}
