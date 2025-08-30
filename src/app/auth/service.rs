use crate::app::error::AppError;

pub async fn join(req: join::Request) -> Result<join::Response, AppError> {
    Ok(join::Response {})
}

pub mod join {
    use serde::Serialize;
    use validator::Validate;

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone_number: i64,
    }

    #[derive(Serialize)]
    pub struct Response {}
}
