use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "contacts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub contact_id: Uuid,
    pub user_id: Uuid,
    pub phone: Vec<u8>,
    pub name: String,
    pub device_contact_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(user_id: Uuid, phone: Vec<u8>, name: String, device_contact_id: String) -> Self {
        let now = Utc::now().naive_utc();
        let contact_id = Uuid::now_v7();

        Self {
            contact_id,
            user_id,
            phone,
            name,
            device_contact_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
