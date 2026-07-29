pub use sea_orm_migration::prelude::*;

mod entities;

mod m20250830_132156_create_users;
mod m20260712_130933_create_user_challenges;
mod m20260723_152747_create_user_credentials;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250830_132156_create_users::Migration),
            Box::new(m20260712_130933_create_user_challenges::Migration),
            Box::new(m20260723_152747_create_user_credentials::Migration),
        ]
    }
}
