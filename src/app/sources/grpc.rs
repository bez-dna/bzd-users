use bzd_users_api::{
    CreateSourceRequest, CreateSourceResponse, GetSourcesRequest, GetSourcesResponse,
    sources_service_server::SourcesService,
};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, sources::service, state::AppState};

pub struct GrpcSourcesService {
    pub state: AppState,
}

impl GrpcSourcesService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl SourcesService for GrpcSourcesService {
    async fn get_sources(
        &self,
        req: Request<GetSourcesRequest>,
    ) -> Result<Response<GetSourcesResponse>, Status> {
        let res = get_sources(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn create_source(
        &self,
        req: Request<CreateSourceRequest>,
    ) -> Result<Response<CreateSourceResponse>, Status> {
        let res = create_source(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn get_sources(
    AppState { db, .. }: &AppState,
    req: GetSourcesRequest,
) -> Result<GetSourcesResponse, AppError> {
    let res = service::get_sources(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_sources {
    use bzd_users_api::{GetSourcesRequest, GetSourcesResponse, get_sources_response};

    use crate::app::{
        error::AppError,
        sources::{
            repo,
            service::{
                self,
                get_sources::{ContactWithUser, Response},
            },
        },
    };

    impl TryFrom<GetSourcesRequest> for service::get_sources::Request {
        type Error = AppError;

        fn try_from(req: GetSourcesRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for GetSourcesResponse {
        fn from(res: Response) -> Self {
            Self {
                contacts: res.contacts_with_user.into_iter().map(Into::into).collect(),
                sources: res.sources.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<ContactWithUser> for get_sources_response::Contact {
        fn from(ContactWithUser { contact, user }: ContactWithUser) -> Self {
            Self {
                contact_id: Some(contact.contact_id.into()),
                contact_user_id: Some(user.user_id.into()),
            }
        }
    }

    impl From<repo::source::Model> for get_sources_response::Source {
        fn from(source: repo::source::Model) -> Self {
            Self {
                source_id: Some(source.source_id.into()),
                source_user_id: Some(source.source_user_id.into()),
            }
        }
    }
}

async fn create_source(
    AppState { db, .. }: &AppState,
    req: CreateSourceRequest,
) -> Result<CreateSourceResponse, AppError> {
    let res = service::create_source(db, req.try_into()?).await?;

    Ok(res.into())
}

mod create_source {
    use bzd_users_api::{CreateSourceRequest, CreateSourceResponse};
    use uuid::Uuid;

    use crate::app::{error::AppError, sources::service};

    impl TryFrom<CreateSourceRequest> for service::create_source::Request {
        type Error = AppError;

        fn try_from(req: CreateSourceRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: Uuid::parse_str(req.user_id())?,
                source_user_id: Uuid::parse_str(req.source_user_id())?,
            })
        }
    }

    impl From<service::create_source::Response> for CreateSourceResponse {
        fn from(res: service::create_source::Response) -> Self {
            Self {
                source_id: Some(res.source.source_id.into()),
            }
        }
    }
}
