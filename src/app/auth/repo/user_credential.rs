use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "user_credentials")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_credential_id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(
        user_id: Uuid,
        credential_id: Vec<u8>,
        public_key: Vec<u8>,
        sign_count: i32,
    ) -> Self {
        let user_credential_id = Uuid::now_v7();
        let now = Utc::now().naive_utc();

        Self {
            user_credential_id,
            user_id,
            credential_id,
            public_key,
            sign_count,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
