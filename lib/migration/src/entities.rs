use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    UserId,
    Login,
    Name,
}

#[derive(DeriveIden)]
pub enum UserChallenges {
    Table,
    UserChallengeId,
    UserId,
    Challenge,
    Login,
    Name,
}

#[derive(DeriveIden)]
pub enum UserCredentials {
    Table,
    UserCredentialId,
    UserId,
    CredentialId,
    PublicKey,
    SignCount,
}
