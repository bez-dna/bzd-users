use bzd_users_api::{
    GetUserRequest, GetUserResponse, GetUserUsersRequest, GetUserUsersResponse, GetUsersRequest,
    GetUsersResponse, users_service_server::UsersService,
};
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

    async fn get_users(
        &self,
        req: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        let res = get_users(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_users(
        &self,
        req: Request<GetUserUsersRequest>,
    ) -> Result<Response<GetUserUsersResponse>, Status> {
        let res = get_user_users(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn get_user(
    AppState { db, crypto, .. }: &AppState,
    req: GetUserRequest,
) -> Result<GetUserResponse, AppError> {
    let res = service::get_user(db, crypto, req.try_into()?).await?;

    Ok(res.into())
}

mod get_user {
    use bzd_users_api::{GetUserRequest, GetUserResponse, get_user_response::User};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        users::service::{self, get_user::Response},
    };

    impl TryFrom<GetUserRequest> for service::get_user::Request {
        type Error = AppError;

        fn try_from(req: GetUserRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: Uuid::parse_str(req.user_id())?,
            })
        }
    }

    impl From<Response> for GetUserResponse {
        fn from(Response { user }: Response) -> Self {
            Self {
                user: Some(User {
                    user_id: Some(user.user_id.into()),
                    name: user.name.into(),
                    phone: user.phone.into(),
                    abbr: user.abbr.into(),
                    color: user.color.into(),
                }),
            }
        }
    }
}

async fn get_users(
    AppState { db, crypto, .. }: &AppState,
    req: GetUsersRequest,
) -> Result<GetUsersResponse, AppError> {
    let res = service::get_users(db, crypto, req.try_into()?).await?;

    Ok(res.into())
}

mod get_users {
    use bzd_users_api::{GetUsersRequest, GetUsersResponse, get_users_response};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        users::{
            UserDecryptedPhone,
            service::{self, get_users::Response},
        },
    };

    impl TryFrom<GetUsersRequest> for service::get_users::Request {
        type Error = AppError;

        fn try_from(req: GetUsersRequest) -> Result<Self, Self::Error> {
            let user_ids = req
                .user_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self { user_ids })
        }
    }

    impl From<Response> for GetUsersResponse {
        fn from(res: Response) -> Self {
            Self {
                users: res.users.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<UserDecryptedPhone> for get_users_response::User {
        fn from(user: UserDecryptedPhone) -> Self {
            Self {
                user_id: Some(user.user_id.into()),
                phone: user.phone.into(),
                name: user.name.into(),
                abbr: user.abbr.into(),
                color: user.color.into(),
            }
        }
    }
}

async fn get_user_users(
    AppState { db, .. }: &AppState,
    req: GetUserUsersRequest,
) -> Result<GetUserUsersResponse, AppError> {
    let res = service::get_user_users(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_user_users {
    use bzd_users_api::{GetUserUsersRequest, GetUserUsersResponse};

    use crate::app::{
        error::AppError,
        users::service::get_user_users::{Request, Response},
    };

    impl TryFrom<GetUserUsersRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserUsersRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for GetUserUsersResponse {
        fn from(res: Response) -> Self {
            Self {
                user_ids: res.users.iter().map(|it| it.user_id.into()).collect(),
            }
        }
    }
}
