use rand::Rng as _;
use sea_orm::{DbConn, sqlx::types::chrono::Utc};
use uuid::Uuid;

use crate::app::auth::{error::AuthError, repo, state::AuthState};

pub async fn join(
    db: &DbConn,
    AuthState {
        verification_client,
        ..
    }: &AuthState,
    req: join::Request,
) -> Result<join::Response, AuthError> {
    /*
    Не учитывается гонка, ну точнее будет pg ошибка на уникальном индексе и просто отлуп с ошибкой
    Нужно будет этот кейс в будущем поправить, но пока не крит, маловероятный кейс когда один
    нормальный юзер будет долбить ручку с N-устройств в одну секунду времени.
    */

    let verification =
        match repo::find_verification_by_phone(db, req.phone_number_hash.clone()).await? {
            /*
            request_id не передается в send, а это значит что на каждый вызов будет улетать новый запрос в тг,
            это может привести к тому, что можно всадить весь бюджет отправки кодов или задолбать жертву.

            план такой: нужно будет при наличии verification проверять его created_at и если ограничить отправку кодов,
            например, раз в минуту или любой другой механизм.

            ну и на стороне тг есть защита от флуда, так что прям супер быстро не провернуть эту атаку
            */
            Some(verification) => {
                verification_client
                    .send(None, req.phone_number, verification.code.clone())
                    .await?;

                verification
            }
            None => {
                let code: i32 = rand::rng().random_range(1000..9999);

                let verification_response = verification_client
                    .send(None, req.phone_number, code.to_string())
                    .await?;

                repo::create_verification(
                    db,
                    repo::verification::Model {
                        verification_id: Uuid::now_v7(),
                        phone: req.phone_number_hash,
                        code: code.to_string(),
                        request_id: verification_response.request_id,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    },
                )
                .await?
            }
        };

    Ok(join::Response { verification })
}

pub mod join {
    use serde::Serialize;
    use validator::Validate;

    use crate::app::auth::repo::verification;

    #[derive(Validate)]
    pub struct Request {
        #[validate(range(min = 7_000_000_0000i64, max = 7_999_999_9999i64))]
        pub phone_number: i64,
        pub phone_number_hash: String,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub verification: verification::Model,
    }
}
