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

#[derive(DeriveIden)]
pub enum Contacts {
    Table,
    ContactId,
    UserId,
    Phone,
    Name,
    DeviceContactId,
}

#[derive(DeriveIden)]
pub enum Sources {
    Table,
    SourceId,
    UserId,
    SourceUserId,
}
