pub use sea_orm_migration::prelude::*;

mod entities;

mod m20250830_132156_create_users;
mod m20250831_070628_create_verifications;
mod m20250911_163757_create_contacts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250830_132156_create_users::Migration),
            Box::new(m20250831_070628_create_verifications::Migration),
            Box::new(m20250911_163757_create_contacts::Migration),
        ]
    }
}
