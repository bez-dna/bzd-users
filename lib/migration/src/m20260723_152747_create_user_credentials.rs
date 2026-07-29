use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::UserCredentials;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserCredentials::Table)
                    .col(uuid(UserCredentials::UserCredentialId).primary_key())
                    .col(uuid(UserCredentials::UserId))
                    .col(binary(UserCredentials::CredentialId))
                    .col(binary(UserCredentials::PublicKey))
                    .col(integer(UserCredentials::SignCount))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_credentials_credential_id_udx")
                    .unique()
                    .table(UserCredentials::Table)
                    .col(UserCredentials::CredentialId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("users_credentials_public_key_udx")
                    .unique()
                    .table(UserCredentials::Table)
                    .col(UserCredentials::PublicKey)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserCredentials::Table).to_owned())
            .await
    }
}
