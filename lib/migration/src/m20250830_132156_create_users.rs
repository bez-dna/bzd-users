use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Users::Table)
                    .col(uuid(Users::UserId).primary_key())
                    .col(text(Users::Phone))
                    .col(text_null(Users::Name))
                    .col(text_null(Users::Locale))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_phone_udx")
                    .unique()
                    .table(Users::Table)
                    .col(Users::Phone)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
