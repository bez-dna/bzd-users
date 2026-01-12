use bzd_users_api::auth::{
    CompleteRequest, CompleteResponse, JoinRequest, JoinResponse, auth_service_server::AuthService,
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

    async fn complete(
        &self,
        req: Request<CompleteRequest>,
    ) -> Result<Response<CompleteResponse>, Status> {
        let res = complete::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

mod join {
    use bzd_users_api::auth::{JoinRequest, JoinResponse, join_response::Verification};
    use validator::Validate as _;

    use crate::app::{
        auth::{service, state::AuthState},
        error::AppError,
    };

    pub async fn handler(
        AuthState {
            db,
            crypto,
            verification_client,
            ..
        }: &AuthState,
        req: JoinRequest,
    ) -> Result<JoinResponse, AppError> {
        let res = service::join(&db.conn, verification_client, crypto, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<JoinRequest> for service::join::Request {
        type Error = AppError;

        fn try_from(req: JoinRequest) -> Result<Self, Self::Error> {
            let data = Self {
                phone_number: req.phone_number(),
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<service::join::Response> for JoinResponse {
        fn from(res: service::join::Response) -> Self {
            Self {
                verification: Some(Verification {
                    verification_id: Some(res.verification.verification_id.into()),
                }),

                is_new: Some(res.user.is_none()),
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
                    phone_number: Some(111),
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    phone_number: Some(-7_900_000_0000),
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    phone_number: Some(8_100_000_0000),
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    phone_number: Some(6_900_000_0000),
                })
                .is_err()
            );

            assert!(
                TryInto::<service::join::Request>::try_into(JoinRequest {
                    phone_number: Some(7_900_000_0000),
                })
                .is_ok()
            );
        }
    }
}

mod complete {
    use bzd_users_api::auth::{CompleteRequest, CompleteResponse};
    use uuid::Uuid;
    use validator::Validate;

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
        let res = service::complete(
            &db.conn,
            encoder.as_ref(),
            req.try_into()?,
            settings.verification.debug,
        )
        .await?;

        Ok(res.into())
    }

    impl TryFrom<CompleteRequest> for service::complete::Request {
        type Error = AppError;

        fn try_from(req: CompleteRequest) -> Result<Self, Self::Error> {
            let data = Self {
                verification_id: Uuid::parse_str(req.verification_id())?,
                code: req.code().into(),
                name: req.name,
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<service::complete::Response> for CompleteResponse {
        fn from(res: service::complete::Response) -> Self {
            Self { jwt: Some(res.jwt) }
        }
    }

    #[cfg(test)]
    mod tests {
        use bzd_users_api::auth::CompleteRequest;
        use uuid::Uuid;

        use crate::app::{auth::service, error::AppError};

        #[test]
        fn convert_grpc_request_2_service() -> Result<(), AppError> {
            let req = TryInto::<service::complete::Request>::try_into(CompleteRequest {
                verification_id: Some(Uuid::now_v7().into()),
                code: Some("1234".into()),
                name: Some("NAME".into()),
            });

            assert!(req.is_ok());
            assert_eq!(req?.code, "1234");

            let req = TryInto::<service::complete::Request>::try_into(CompleteRequest {
                verification_id: Some(Uuid::now_v7().into()),
                code: Some("1234".into()),
                name: Some("".into()),
            });

            assert!(!req.is_ok());

            Ok(())
        }
    }
}
