use crate::app::{crypto::state::CryptoState, db::DbState};

#[derive(Clone)]
pub struct UsersState {
    pub db: DbState,
    pub crypto: CryptoState,
}
