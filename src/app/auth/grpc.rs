use bzd_users_api::auth::{
    CompleteRequest, CompleteResponse, JoinRequest, JoinResponse, LoginRequest, LoginResponse,
    auth_service_server::AuthService,
};
use tonic::{Request, Response, Status};

use crate::app::auth::state::AuthState;

pub struct GrpcAuthService {
    pub state: AuthState,
}

impl GrpcAuthService {
    pub fn new(state: AuthState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn join(&self, req: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let res = join::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn login(&self, req: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let res = login::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn complete(
        &self,
        req: Request<CompleteRequest>,
    ) -> Result<Response<CompleteResponse>, Status> {
        let res = complete::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

mod join {
    use bzd_users_api::auth::{JoinRequest, JoinResponse};
    use serde_json::json;
    use validator::Validate as _;

    use crate::app::{
        auth::{service, state::AuthState},
        error::AppError,
    };

    pub async fn handler(
        AuthState { db, settings, .. }: &AuthState,
        req: JoinRequest,
    ) -> Result<JoinResponse, AppError> {
        let res = service::join(&db.conn, &settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<JoinRequest> for service::join::Request {
        type Error = AppError;

        fn try_from(req: JoinRequest) -> Result<Self, Self::Error> {
            let data = Self {
                login: req.login().to_lowercase().into(),
                name: req.login().into(),
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<service::join::Response> for JoinResponse {
        fn from(res: service::join::Response) -> Self {
            Self {
                response: Some(json!(res).to_string()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use bzd_users_api::auth::JoinRequest;

        use crate::app::auth::service;

        #[test]
        fn convert_grpc_request_2_service() {
            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    login: Some("log".into())
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    login: Some("log-in".into())
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    login: Some("Login".into())
                })
                .is_ok()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    login: Some("login".into())
                })
                .is_ok()
            );
        }
    }
}

mod login {
    use bzd_users_api::auth::{LoginRequest, LoginResponse};

    use crate::app::{
        auth::{service, state::AuthState},
        error::AppError,
    };

    pub async fn handler(
        AuthState {
            db,
            encoder,
            settings,
            ..
        }: &AuthState,
        req: LoginRequest,
    ) -> Result<LoginResponse, AppError> {
        let res = service::login(&db.conn, encoder.as_ref(), settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<LoginRequest> for service::login::Request {
        type Error = AppError;

        fn try_from(req: LoginRequest) -> Result<Self, Self::Error> {
            let data = serde_json::from_str(req.request())?;

            Ok(data)
        }
    }

    impl From<service::login::Response> for LoginResponse {
        fn from(res: service::login::Response) -> Self {
            Self { jwt: Some(res.jwt) }
        }
    }
}

mod complete {
    use bzd_users_api::auth::{CompleteRequest, CompleteResponse};

    use crate::app::{
        auth::{service, state::AuthState},
        error::AppError,
    };

    pub async fn handler(
        AuthState {
            db,
            encoder,
            settings,
            ..
        }: &AuthState,
        req: CompleteRequest,
    ) -> Result<CompleteResponse, AppError> {
        let res = service::complete(&db.conn, encoder.as_ref(), settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<CompleteRequest> for service::complete::Request {
        type Error = AppError;

        fn try_from(req: CompleteRequest) -> Result<Self, Self::Error> {
            let data = serde_json::from_str(req.request())?;

            Ok(data)
        }
    }

    impl From<service::complete::Response> for CompleteResponse {
        fn from(res: service::complete::Response) -> Self {
            Self { jwt: Some(res.jwt) }
        }
    }
}
