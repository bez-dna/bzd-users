use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("VERIFICATION_SEND")]
    VerificationSend,
    #[error(transparent)]
    VerificationClient(#[from] reqwest::Error),
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
}
