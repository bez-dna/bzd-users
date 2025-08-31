use bzd_users_api::auth_service_server::AuthServiceServer;

use crate::app::state::AppState;
use grpc::GrpcAuthService;

pub mod error;
mod grpc;
mod repo;
mod service;
pub mod settings;
pub mod state;
mod verification;

pub fn auth_service(state: AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state))
}
