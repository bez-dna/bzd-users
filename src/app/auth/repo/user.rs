use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub phone: Vec<u8>,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(phone: Vec<u8>, name: String) -> Self {
        let now = Utc::now().naive_utc();
        let user_id = Uuid::now_v7();

        Self {
            user_id,
            phone,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
