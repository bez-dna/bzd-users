use bzd_users_api::users_service_server::UsersServiceServer;
use uuid::Uuid;

use crate::app::{error::AppError, state::AppState};
// use grpc::GrpcUsersService;

// mod grpc;
mod repo;
// mod service;

// pub fn users_service(state: AppState) -> UsersServiceServer<GrpcUsersService> {
//     UsersServiceServer::new(GrpcUsersService::new(state))
// }

// pub struct UserDecryptedPhone {
//     pub user_id: Uuid,
//     pub phone: String,
//     pub name: String,
//     pub abbr: String,
//     pub color: String,
// }

// impl UserDecryptedPhone {
//     pub fn new(user: repo::user::Model) -> Result<Self, AppError> {
//         Ok(Self {
//             user_id: user.user_id,
//             // phone: crypto.decrypt(&user.phone)?,
//             phone: "PHONE".into(),
//             name: user.name.clone(),
//             abbr: user.abbr(),
//             color: user.color(),
//         })
//     }
// }
