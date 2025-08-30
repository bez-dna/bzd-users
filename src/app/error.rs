use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        Self::internal(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}
