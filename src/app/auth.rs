use bzd_users_api::auth::auth_service_server::AuthServiceServer;
use serde::Serialize;
use uuid::Uuid;

use crate::app::state::AppState;
use grpc::GrpcAuthService;

mod grpc;
mod repo;
mod service;
pub mod settings;
pub mod state;
mod verification;

pub fn service(state: &AppState) -> AuthServiceServer<GrpcAuthService> {
    AuthServiceServer::new(GrpcAuthService::new(state.auth.clone()))
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub type PrivateKey = Vec<u8>;
