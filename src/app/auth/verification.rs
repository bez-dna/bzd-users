use std::sync::Arc;

use serde::Deserialize;
use tracing::debug;
use uuid::Uuid;

use crate::app::error::AppError;

#[derive(Clone)]
pub struct VerificationClient {
    client: Arc<reqwest::Client>,
    settings: VerificationSettings,
}

impl VerificationClient {
    pub fn new(settings: VerificationSettings) -> Self {
        let client = Arc::new(reqwest::Client::new());

        Self { client, settings }
    }

    pub async fn send(
        &self,
        request_id: Option<String>,
        phone_number: i64,
        code: String,
    ) -> Result<send::Result, AppError> {
        if let Some(debug) = self.settings.debug
            && debug
        {
            debug!("{} - {}", phone_number, code);

            return Ok(send::Result {
                request_id: Uuid::nil().into(),
            });
        }

        let url = [
            self.settings.endpoint.clone(),
            "/sendVerificationMessage".into(),
        ]
        .concat();
        let req = send::Request {
            request_id,
            phone_number: phone_number.to_string(),
            access_token: self.settings.access_token.clone(),
            sender_username: self.settings.sender_username.clone(),
            code,
        };

        let response = self
            .client
            .post(url)
            .json(&req)
            .send()
            .await?
            .json::<send::Response>()
            .await?;

        match response.result {
            Some(result) => Ok(result),
            None => Err(AppError::VerificationSend),
        }
    }
}

mod send {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Request {
        pub request_id: Option<String>,
        pub phone_number: String,
        pub access_token: Option<String>,
        pub sender_username: Option<String>,
        pub code: String,
    }

    /*
    нужно научиться обрабатывать коды ошибок от тг, пока просто все ошибки идут как VerificationSend
    */

    #[derive(Deserialize, Debug)]
    pub struct Response {
        // pub ok: bool,
        pub result: Option<Result>,
        // pub error: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Result {
        pub request_id: String,
    }
}

#[derive(Deserialize, Clone)]
pub struct VerificationSettings {
    pub endpoint: String,
    pub access_token: Option<String>,
    pub sender_username: Option<String>,
    pub debug: Option<bool>,
}
