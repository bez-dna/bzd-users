use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::UserChallenges;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(UserChallenges::Table)
                    .col(uuid(UserChallenges::UserChallengeId).primary_key())
                    .col(binary(UserChallenges::Challenge))
                    .col(uuid(UserChallenges::UserId))
                    .col(text(UserChallenges::Login))
                    .col(text(UserChallenges::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("user_challenges_challenge_udx")
                    .unique()
                    .table(UserChallenges::Table)
                    .col(UserChallenges::Challenge)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserChallenges::Table).to_owned())
            .await
    }
}
