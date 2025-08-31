use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Verifications;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Verifications::Table)
                    .col(uuid(Verifications::VerificationId).primary_key())
                    .col(text(Verifications::Phone))
                    .col(integer(Verifications::Code))
                    .col(text(Verifications::RequestId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("verifications_phone_udx")
                    .unique()
                    .table(Verifications::Table)
                    .col(Verifications::Phone)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("verifications_request_id_udx")
                    .unique()
                    .table(Verifications::Table)
                    .col(Verifications::RequestId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Verifications::Table).to_owned())
            .await
    }
}
