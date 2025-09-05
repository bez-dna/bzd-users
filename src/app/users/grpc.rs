use bzd_users_api::{GetUserRequest, GetUserResponse, users_service_server::UsersService};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, state::AppState, users::service};

pub struct GrpcUsersService {
    pub state: AppState,
}

impl GrpcUsersService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl UsersService for GrpcUsersService {
    async fn get_user(
        &self,
        req: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let res = get_user(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn get_user(
    AppState { db, .. }: &AppState,
    req: GetUserRequest,
) -> Result<GetUserResponse, AppError> {
    let res = service::get_user(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_user {
    use bzd_users_api::{GetUserRequest, GetUserResponse, get_user_response::User};
    use uuid::Uuid;

    use crate::app::{error::AppError, users::service};

    impl TryFrom<GetUserRequest> for service::get_user::Request {
        type Error = AppError;

        fn try_from(req: GetUserRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: Uuid::parse_str(req.user_id())?,
            })
        }
    }

    impl From<service::get_user::Response> for GetUserResponse {
        fn from(res: service::get_user::Response) -> Self {
            Self {
                user: match res.user {
                    Some(user) => Some(User {
                        user_id: Some(user.user_id.into()),
                        name: user.name.into(),
                    }),
                    None => None,
                },
            }
        }
    }
}
