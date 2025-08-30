use bzd_users_api::{JoinRequest, JoinResponse, auth_service_server::AuthService};
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
}

async fn join(
    AppState { settings, .. }: &AppState,
    request: JoinRequest,
) -> Result<JoinResponse, AppError> {
    let response = service::join(request.try_into()?).await?;

    Ok(response.into())
}

mod join {
    use bzd_users_api::{JoinRequest, JoinResponse};
    use serde_json::json;
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
            JoinResponse {
                response: Some(json!(res).to_string()),
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
