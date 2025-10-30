use sea_orm::{
    ColumnTrait as _, ConnectionTrait, DbErr, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _, QuerySelect, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod contact;
pub mod source;
pub mod user;

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

pub async fn find_reverse_contacts_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<contact::Model>, AppError> {
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("".into()))?;

    let contacts = contact::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            contact::Entity::belongs_to(user::Entity)
                .to(user::Column::Phone)
                .from(contact::Column::Phone)
                .into(),
        )
        .filter(contact::Column::Phone.eq(user.phone))
        .all(db)
        .await?;

    Ok(contacts)
}

pub async fn find_contacts_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<(contact::Model, Option<user::Model>)>, AppError> {
    let contacts = contact::Entity::find()
        .select_also(user::Entity)
        .join(
            sea_orm::JoinType::InnerJoin,
            contact::Entity::belongs_to(user::Entity)
                .to(user::Column::Phone)
                .from(contact::Column::Phone)
                .into(),
        )
        .filter(contact::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(contacts)
}

pub async fn find_sources_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<source::Model>, AppError> {
    let sources = source::Entity::find()
        .filter(source::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(sources)
}

pub async fn get_source_by_source_user_id_and_user_id<T: ConnectionTrait>(
    db: &T,
    source_user_id: Uuid,
    user_id: Uuid,
) -> Result<Option<source::Model>, AppError> {
    let source = source::Entity::find()
        .filter(source::Column::SourceUserId.eq(source_user_id))
        .filter(source::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(source)
}
