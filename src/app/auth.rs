use bzd_users_api::auth::auth_service_server::AuthServiceServer;

use crate::app::state::AppState;
use grpc::GrpcAuthService;

mod encoder;
mod grpc;
mod passkey;
mod repo;
mod service;
pub mod settings;
pub mod state;

pub fn service(state: &AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state.auth.clone()))
}

pub type PrivateKey = Vec<u8>;
