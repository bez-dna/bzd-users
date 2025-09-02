use std::sync::Arc;

use bzd_lib::error::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use crate::app::{auth::state::AuthState, settings::AppSettings};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: Arc<DbConn>,
    pub auth: AuthState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let auth = AuthState::new(&settings.auth).await?;

        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        Ok(Self { settings, auth, db })
    }
}
