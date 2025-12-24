use std::sync::Arc;

use bzd_lib::error::Error;

use crate::app::crypto::{
    service::{CryptoService, CryptoServiceImpl},
    settings::CryptoSettings,
};

#[derive(Clone)]
pub struct CryptoState {
    pub service: Arc<dyn CryptoService>,
}

impl CryptoState {
    pub fn new(settings: &CryptoSettings) -> Result<Self, Error> {
        let service = CryptoServiceImpl::new(settings);
        let service = Arc::new(service);

        Ok(Self { service })
    }
}
