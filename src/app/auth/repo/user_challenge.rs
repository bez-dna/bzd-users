use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "user_challenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_challenge_id: Uuid,
    pub challenge: Vec<u8>,
    pub user_id: Uuid,
    pub login: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(challenge: Vec<u8>, user_id: Uuid, login: String, name: String) -> Self {
        let user_challenge_id = Uuid::now_v7();
        let now = Utc::now().naive_utc();

        Self {
            user_challenge_id,
            challenge,
            user_id,
            login,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
