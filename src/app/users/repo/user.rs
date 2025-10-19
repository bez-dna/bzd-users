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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn abbr(&self) -> String {
        let parts: Vec<&str> = self
            .name
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        match parts.len() {
            0 => "".into(),
            1 => parts[0].chars().take(2).collect(),
            _ => {
                let first = parts[0].chars().next();
                let second = parts[1].chars().next();
                match (first, second) {
                    (Some(a), Some(b)) => format!("{}{}", a, b),
                    (Some(a), None) => a.to_string(),
                    _ => "".into(),
                }
            }
        }
        .to_uppercase()
    }

    pub fn color(&self) -> String {
        random_color::RandomColor::new()
            .alpha(0.3)
            .seed(self.user_id.to_string())
            .to_rgba_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::app::users::repo::user::Model;

    #[test]
    fn check_abbr_trait() {
        let mut user = Model {
            user_id: Uuid::now_v7(),
            phone: vec![],
            name: "".into(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        assert_eq!("", user.abbr());

        user.name = "Andrei Br".into();
        assert_eq!("AB", user.abbr());

        user.name = "Иван Белец".into();
        assert_eq!("ИБ", user.abbr());

        user.name = "Джагернаут".into();
        assert_eq!("ДЖ", user.abbr());

        user.name = "Д".into();
        assert_eq!("Д", user.abbr());
    }
}
