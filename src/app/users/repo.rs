use sea_orm::{ColumnTrait as _, ConnectionTrait, EntityTrait, QueryFilter as _, QuerySelect as _};

use uuid::Uuid;

use crate::app::error::AppError;

mod contact;
mod user;

pub type UserModel = user::Model;

// pub struct UsersRepoImpl {
//     // pub db: Arc<DbConn>,
// }

// impl UsersRepoImpl {
//     pub fn new() -> Self {
//         Self {}
//     }
//     // pub fn new(db: Arc<DbConn>) -> Self {
//     //     Self { db }
//     // }
// }

// #[async_trait]
// #[cfg_attr(test, mockall::automock)]
// pub trait UsersRepo {
//     async fn get_user_by_id(
//         &self,
//         db: &dyn ConnectionTrait,
//         //         user_id: Uuid,
//     ) -> Result<Option<user::Model>, AppError>;
// }

// #[async_trait]
// impl UsersRepo for UsersRepoImpl {

// }

// async fn get_users_by_ids(
//     &self,
//     db: dyn ConnectionTrait,
//     user_ids: Vec<Uuid>,
// ) -> Result<Vec<user::Model>, AppError>;

// async fn get_users_by_user_id(
//     &self,
//     db: dyn ConnectionTrait,
//     user_id: Uuid,
// ) -> Result<Vec<user::Model>, AppError>;
// }

//     // async fn get_users_by_ids(
//     //     &self,
//     //     db: dyn ConnectionTrait,
//     //     user_ids: Vec<Uuid>,
//     // ) -> Result<Vec<user::Model>, AppError> {
//     //     let topics = user::Entity::find()
//     //         .filter(user::Column::UserId.is_in(user_ids))
//     //         .all(db)
//     //         .await?;

//     //     Ok(topics)
//     // }

// }

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

// .ok_or(AppError::NotFound)

pub async fn get_users_by_ids<T: ConnectionTrait>(
    db: &T,
    user_ids: Vec<Uuid>,
) -> Result<Vec<UserModel>, AppError> {
    let topics = user::Entity::find()
        .filter(user::Column::UserId.is_in(user_ids))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_users_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<UserModel>, AppError> {
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
