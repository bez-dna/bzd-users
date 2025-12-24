use std::sync::Arc;

use crate::app::{
    contacts::{
        repo::ContactsRepoImpl,
        service::{ContactsService, ContactsServiceImpl},
    },
    state::AppState,
};

pub struct ContactsState {
    pub service: Arc<dyn ContactsService>,
}

impl ContactsState {
    pub fn new(state: &AppState) -> Self {
        let repo = ContactsRepoImpl::new(state.db.conn.clone());
        let repo = Arc::new(repo);

        let service = ContactsServiceImpl::new(repo, state.crypto.service.clone());
        let service = Arc::new(service);

        Self { service }
    }
}
