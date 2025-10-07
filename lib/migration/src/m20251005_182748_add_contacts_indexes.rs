use sea_orm_migration::prelude::*;

use crate::entities::Contacts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("contacts_user_id_idx")
                    .table(Contacts::Table)
                    .col(Contacts::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("contacts_phone_idx")
                    .table(Contacts::Table)
                    .col(Contacts::Phone)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("contacts_user_id_idx")
                    .table(Contacts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("contacts_phone_idx")
                    .table(Contacts::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
