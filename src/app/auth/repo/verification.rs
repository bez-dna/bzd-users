use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "verifications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub verification_id: Uuid,
    pub phone: Vec<u8>,
    pub code: String,
    pub request_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(phone: Vec<u8>, code: i32, request_id: String) -> Self {
        let now = Utc::now().naive_utc();
        let verification_id = Uuid::now_v7();

        Self {
            verification_id,
            phone,
            code: code.to_string(),
            request_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
