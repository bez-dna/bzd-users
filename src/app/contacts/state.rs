use crate::app::{crypto::state::CryptoState, db::DbState};

#[derive(Clone)]
pub struct ContactsState {
    pub db: DbState,
    pub crypto: CryptoState,
}

// impl ContactsState {
//     pub fn new(state: &AppState) -> Self {
//         let repo = ContactsRepoImpl::new(state.db.conn.clone());
//         let repo = Arc::new(repo);

//         let service = ContactsServiceImpl::new(repo, state.crypto.service.clone());
//         let service = Arc::new(service);

//         Self { service }
//     }
// }
