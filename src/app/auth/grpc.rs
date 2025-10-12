use bzd_users_api::{
    CompleteRequest, CompleteResponse, JoinRequest, JoinResponse, auth_service_server::AuthService,
};
use tonic::{Request, Response, Status};

use crate::app::{auth::service, error::AppError, state::AppState};

pub struct GrpcAuthService {
    pub state: AppState,
}

impl GrpcAuthService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn join(&self, req: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let res = join(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn complete(
        &self,
        req: Request<CompleteRequest>,
    ) -> Result<Response<CompleteResponse>, Status> {
        let res = complete(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn join(
    AppState {
        db, auth, crypto, ..
    }: &AppState,
    req: JoinRequest,
) -> Result<JoinResponse, AppError> {
    let res = service::join(db, auth, crypto, req.try_into()?).await?;

    Ok(res.into())
}

mod join {
    use bzd_users_api::{JoinRequest, JoinResponse, join_response::Verification};
    use validator::Validate as _;

    use crate::app::{auth::service, error::AppError};

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
        use bzd_users_api::JoinRequest;

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

async fn complete(
    AppState { db, auth, .. }: &AppState,
    req: CompleteRequest,
) -> Result<CompleteResponse, AppError> {
    let res = service::complete(db, auth, req.try_into()?).await?;

    Ok(res.into())
}

mod complete {
    use bzd_users_api::{CompleteRequest, CompleteResponse};
    use uuid::Uuid;

    use crate::app::{auth::service, error::AppError};

    impl TryFrom<CompleteRequest> for service::complete::Request {
        type Error = AppError;

        fn try_from(req: CompleteRequest) -> Result<Self, Self::Error> {
            let data = Self {
                verification_id: Uuid::parse_str(req.verification_id())?,
                code: req.code().into(),
            };

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
        use bzd_users_api::CompleteRequest;
        use uuid::Uuid;

        use crate::app::{auth::service, error::AppError};

        #[test]
        fn convert_grpc_request_2_service() -> Result<(), AppError> {
            let req = TryInto::<service::complete::Request>::try_into(CompleteRequest {
                verification_id: Some(Uuid::now_v7().into()),
                code: Some("1234".into()),
            });

            assert!(req.is_ok());
            assert_eq!(req?.code, "1234");

            Ok(())
        }
    }
}
