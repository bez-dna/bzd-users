use bzd_users_api::users::{
    GetUserRequest, GetUserResponse, GetUserUsersRequest, GetUserUsersResponse, GetUsersRequest,
    GetUsersResponse, users_service_server::UsersService,
};
use tonic::{Request, Response, Status};

use crate::app::users::state::UsersState;

pub struct GrpcUsersService {
    pub state: UsersState,
}

impl GrpcUsersService {
    pub fn new(state: UsersState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl UsersService for GrpcUsersService {
    async fn get_user(
        &self,
        req: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let res = get_user::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_users(
        &self,
        req: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        let res = get_users::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_users(
        &self,
        req: Request<GetUserUsersRequest>,
    ) -> Result<Response<GetUserUsersResponse>, Status> {
        let res = get_user_users::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

mod get_user {
    use bzd_users_api::users::{GetUserRequest, GetUserResponse, get_user_response::User};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        users::{
            service::{self, get_user::Response},
            state::UsersState,
        },
    };

    pub async fn handler(
        UsersState { db, crypto, .. }: &UsersState,
        req: GetUserRequest,
    ) -> Result<GetUserResponse, AppError> {
        let res = service::get_user(&db.conn, crypto, req.try_into()?).await?;

        Ok(res.into())
    }

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

mod get_users {
    use bzd_users_api::users::{GetUsersRequest, GetUsersResponse, get_users_response};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        users::{
            service::{
                self,
                get_users::{Response, User},
            },
            state::UsersState,
        },
    };

    pub async fn handler(
        UsersState { db, crypto, .. }: &UsersState,
        req: GetUsersRequest,
    ) -> Result<GetUsersResponse, AppError> {
        let res = service::get_users(&db.conn, crypto, req.try_into()?).await?;

        Ok(res.into())
    }

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

    impl From<User> for get_users_response::User {
        fn from(user: User) -> Self {
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

mod get_user_users {
    use bzd_users_api::users::{GetUserUsersRequest, GetUserUsersResponse};

    use crate::app::{
        error::AppError,
        users::{
            service::{
                self,
                get_user_users::{Request, Response},
            },
            state::UsersState,
        },
    };

    pub async fn handler(
        UsersState { db, .. }: &UsersState,
        req: GetUserUsersRequest,
    ) -> Result<GetUserUsersResponse, AppError> {
        let res = service::get_user_users(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

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
