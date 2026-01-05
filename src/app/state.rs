use bzd_lib::error::Error;

use crate::app::{
    auth::state::AuthState, contacts::state::ContactsState, crypto::state::CryptoState,
    db::DbState, settings::AppSettings, users::state::UsersState,
};

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthState,
    pub contacts: ContactsState,
    pub users: UsersState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let db = DbState::new(&settings.db).await?;
        let crypto = CryptoState::new(&settings.crypto);

        let auth = AuthState::new(&settings.auth, db.clone(), crypto.clone()).await?;
        let users = UsersState {
            db: db.clone(),
            crypto: crypto.clone(),
        };
        let contacts = ContactsState {
            db: db.clone(),
            crypto: crypto.clone(),
        };

        Ok(Self {
            auth,
            users,
            contacts,
        })
    }
}
