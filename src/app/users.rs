use bzd_users_api::users_service_server::UsersServiceServer;

use crate::app::state::AppState;
use grpc::GrpcUsersService;

mod grpc;
mod repo;
mod service;

pub fn users_service(state: AppState) -> UsersServiceServer<GrpcUsersService> {
    UsersServiceServer::new(GrpcUsersService::new(state))
}
