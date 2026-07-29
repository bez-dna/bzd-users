use bzd_lib::error::Error;

use crate::app::{
    auth::state::AuthState, db::DbState, settings::AppSettings, users::state::UsersState,
};

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthState,
    pub users: UsersState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let db = DbState::new(&settings.db).await?;

        let auth = AuthState::new(&settings.auth, db.clone()).await?;
        let users = UsersState { db: db.clone() };

        Ok(Self { auth, users })
    }
}
