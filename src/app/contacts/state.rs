use std::sync::Arc;

use sea_orm::DbConn;

use crate::app::{
    contacts::{
        repo::ContactsRepoDb,
        service::{ContactsService, ContactsServiceImpl},
    },
    state::CryptoState,
};

pub struct ContactsState {
    pub service: Arc<dyn ContactsService>,
}

impl ContactsState {
    pub fn new(db: &DbConn, crypto: &CryptoState) -> Self {
        let repo = Arc::new(ContactsRepoDb {
            db: Arc::new(db.clone()),
        });
        let crypto = Arc::new(crypto.clone());

        let service = Arc::new(ContactsServiceImpl::new(repo, crypto));

        Self { service }
    }
}
