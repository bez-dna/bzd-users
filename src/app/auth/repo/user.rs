use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub login: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(user_id: Uuid, login: String, name: String) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            user_id,
            login,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_credential::Entity")]
    UserCredential,
}

impl Related<super::user_credential::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserCredential.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
