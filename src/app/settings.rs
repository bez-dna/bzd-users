use bzd_lib::settings::DBSettings;
use bzd_lib::settings::Settings;

use bzd_lib::settings::HttpSettings;
use serde::Deserialize;

use crate::app::auth::settings::AuthSettings;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub http: HttpSettings,
    pub auth: AuthSettings,
    pub db: DBSettings,
}

impl Settings<AppSettings> for AppSettings {}
