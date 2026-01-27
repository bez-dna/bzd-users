use crate::app::db::DbState;

#[derive(Clone)]
pub struct UsersState {
    pub db: DbState,
}
