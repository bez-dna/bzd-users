use std::num::TryFromIntError;

use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(_) | AppError::VerificationCode => {
                Self::invalid_argument(error.to_string())
            }
            AppError::NotFound => Self::not_found(error.to_string()),
            _ => Self::internal(error.to_string()),
        }
    }
}

/*
Кажется нужно разделить этот enum на два, или сделать новый тип ошибки под ошибки, которые могут
возникнуть на старте.

Идея в том, что ошибки которые возникают в процессе работы приложки нужно переводить в gRPC Status с пэйлоадом который,
способен обработать фронт, а те что на старте нужно просто детализировать со стектрейсом, пока все в Other (сори)
*/

#[derive(Error, Debug)]
pub enum AppError {
    #[error("VALIDATION")]
    Validation(#[from] validator::ValidationErrors),
    #[error("DB")]
    Db(#[from] sea_orm::DbErr),
    #[error("UUID")]
    Uuid(#[from] uuid::Error),
    #[error("NOT_FOUND")]
    NotFound,
    #[error("VERIFICATION_SEND")]
    VerificationSend,
    #[error("VERIFICATION_CODE")]
    VerificationCode,
    #[error("OTHER")]
    Other,
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        Self::Other
    }
}

impl From<TryFromIntError> for AppError {
    fn from(_: TryFromIntError) -> Self {
        Self::Other
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Self::Other
    }
}
impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        Self::Other
    }
}
