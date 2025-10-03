use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Sources;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Sources::Table)
                    .col(uuid(Sources::SourceId).primary_key())
                    .col(uuid(Sources::UserId))
                    .col(uuid(Sources::SourceUserId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("sources_user_id_source_user_id_udx")
                    .unique()
                    .table(Sources::Table)
                    .col(Sources::UserId)
                    .col(Sources::SourceUserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("sources_user_id_idx")
                    .table(Sources::Table)
                    .col(Sources::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("sources_source_user_id_idx")
                    .table(Sources::Table)
                    .col(Sources::SourceUserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sources::Table).to_owned())
            .await
    }
}
