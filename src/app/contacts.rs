use bzd_users_api::contacts_service_server::ContactsServiceServer;

use crate::app::{contacts::grpc::GrpcContactsService, state::AppState};

mod grpc;
mod repo;
mod service;

pub fn contacts_service(state: AppState) -> ContactsServiceServer<GrpcContactsService> {
    ContactsServiceServer::new(GrpcContactsService::new(state))
}
