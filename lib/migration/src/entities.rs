use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    UserId,
    Phone,
    Name,
    Locale,
}

#[derive(DeriveIden)]
pub enum Verifications {
    Table,
    VerificationId,
    Phone,
    Code,
    RequestId,
}
