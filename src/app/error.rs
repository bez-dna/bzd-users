use thiserror::Error;
use tonic::Status;

use crate::app::auth::error::AuthError;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(_) => Self::invalid_argument(error.to_string()),
            _ => Self::internal(error.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Auth(#[from] AuthError),
    // #[error(transparent)]
    // Db(#[from] sea_orm::DbErr),
}
