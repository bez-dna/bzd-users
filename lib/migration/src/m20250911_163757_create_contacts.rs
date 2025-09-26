use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Contacts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Contacts::Table)
                    .col(uuid(Contacts::ContactId).primary_key())
                    .col(uuid(Contacts::UserId))
                    .col(binary(Contacts::Phone))
                    .col(text(Contacts::Name))
                    .col(text(Contacts::DeviceContactId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("contacts_user_id_phone_udx")
                    .unique()
                    .table(Contacts::Table)
                    .col(Contacts::UserId)
                    .col(Contacts::Phone)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contacts::Table).to_owned())
            .await
    }
}
