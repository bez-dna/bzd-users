use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        // TODO: нужно добавить принт ошбики в debug уровень

        match error {
            AppError::Validation(_) => Self::invalid_argument(error.to_string()),
            AppError::NotFound => Self::not_found(error.to_string()),
            AppError::Forbidden => Self::permission_denied(error.to_string()),
            AppError::Internal | AppError::Unreachable => Self::internal(error.to_string()),
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
    // Ok
    #[error("VALIDATION")]
    Validation(#[from] validator::ValidationErrors),
    #[error("NOT_FOUND")]
    NotFound,
    #[error("FORBIDDEN")]
    Forbidden,

    // Ok
    #[error("INTERNAL")]
    Internal,
    #[error("UNREACHABLE")]
    Unreachable,
}

// TODO: надо разобраться с этим поглууубже
macro_rules! impl_from_other {
    ($($err:ty),+ $(,)?) => {
        $(
            impl From<$err> for AppError {
                fn from(_: $err) -> Self {
                    Self::Internal
                }
            }
        )+
    };
}

impl_from_other!(
    bzd_lib::error::Error,
    uuid::Error,
    serde_json::Error,
    sea_orm::DbErr,
    jsonwebtoken::errors::Error,
    coset::CoseError,
);
