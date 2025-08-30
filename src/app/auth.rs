use bzd_users_api::auth_service_server::AuthServiceServer;

use crate::app::state::AppState;
use grpc::GrpcAuthService;

mod grpc;
mod service;

pub fn auth_service(state: AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state))
}
