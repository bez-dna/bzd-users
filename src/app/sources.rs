use bzd_users_api::sources_service_server::SourcesServiceServer;

use crate::app::{sources::grpc::GrpcSourcesService, state::AppState};

mod grpc;
mod repo;
mod service;

pub fn sources_service(state: AppState) -> SourcesServiceServer<GrpcSourcesService> {
    SourcesServiceServer::new(GrpcSourcesService::new(state))
}
