use std::sync::Arc;

use bzd_users_api::{CreateContactsRequest, CreateContactsResponse, contacts_service_server};
use tonic::{Request, Response, Status, async_trait};

use crate::app::{contacts::state::ContactsState, state::AppState};

pub struct GrpcContactsService {
    pub state: Arc<ContactsState>,
}

impl GrpcContactsService {
    pub fn new(state: &AppState) -> Self {
        let state = Arc::new(ContactsState::new(state));

        Self { state }
    }
}

#[async_trait]
impl contacts_service_server::ContactsService for GrpcContactsService {
    async fn create_contacts(
        &self,
        req: Request<CreateContactsRequest>,
    ) -> Result<Response<CreateContactsResponse>, Status> {
        let res = self
            .state
            .service
            .create_contacts(req.into_inner().try_into()?)
            .await?;

        Ok(Response::new(res.into()))
    }
}

mod create_contacts {
    use bzd_users_api::{CreateContactsRequest, CreateContactsResponse, create_contacts_request};
    use uuid::Uuid;
    use validator::Validate as _;

    use crate::app::{
        contacts::service::create_contacts::{Contact, Request, Response},
        error::AppError,
    };

    impl TryFrom<CreateContactsRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateContactsRequest) -> Result<Self, Self::Error> {
            let contacts = req
                .contacts
                .clone()
                .into_iter()
                .map(|it| it.try_into())
                .collect::<Result<Vec<Contact>, AppError>>()?
                .into_iter()
                .filter(|it| it.phone > 7_000_000_0000 && it.phone < 7_999_999_9999)
                .collect();

            let data = Self {
                user_id: Uuid::parse_str(req.user_id())?,
                contacts,
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl TryFrom<create_contacts_request::Contact> for Contact {
        type Error = AppError;

        fn try_from(contact: create_contacts_request::Contact) -> Result<Self, Self::Error> {
            let phone_number = contact.phone_number().trim();

            let phone_number = if phone_number.starts_with("8") {
                phone_number.replacen("8", "7", 1)
            } else {
                phone_number.into()
            };

            Ok(Self {
                name: contact.name().into(),
                phone: phone_number
                    .chars()
                    .filter(|it| it.is_ascii_digit())
                    .collect::<String>()
                    .parse()?,
                device_contact_id: contact.device_contact_id().into(),
            })
        }
    }

    impl From<Response> for CreateContactsResponse {
        fn from(_: Response) -> Self {
            Self::default()
        }
    }

    #[cfg(test)]
    mod tests {
        use bzd_users_api::{CreateContactsRequest, create_contacts_request::Contact};
        use uuid::Uuid;

        use crate::app::{
            contacts::service::create_contacts::{self, Request},
            error::AppError,
        };

        #[test]
        fn convert_grpc_request_2_service() -> Result<(), AppError> {
            let req: Request = CreateContactsRequest {
                user_id: Some(Uuid::now_v7().into()),
                contacts: [
                    Contact {
                        phone_number: Some("(555) 564-8583".into()),
                        name: Some("Kate Bell".into()),
                        device_contact_id: Some("177C371E-701D-42F8-A03B-C61CA31627F6".into()),
                    },
                    Contact {
                        phone_number: Some("8 (909) 111-2222".into()),
                        name: Some("Kate Bell".into()),
                        device_contact_id: Some("177C371E-701D-42F8-A03B-C61CA31627F6".into()),
                    },
                    Contact {
                        phone_number: Some("888-555-5512".into()),
                        name: None,
                        device_contact_id: Some("410FE041-5C4E-48DA-B4DE-04C15EA3DBAC".into()),
                    },
                    Contact {
                        phone_number: Some("+7 (999) 111-22-33".into()),
                        name: Some("".into()),
                        device_contact_id: Some("E94CD15C-7964-4A9B-8AC4-10D7CFB791FD".into()),
                    },
                ]
                .to_vec(),
            }
            .try_into()?;

            assert_eq!(2, req.contacts.len());

            Ok(())
        }

        #[test]
        fn convert_phone_number() -> Result<(), AppError> {
            let contact: create_contacts::Contact = Contact {
                phone_number: Some("7 (999) 777 11-22".into()),
                name: None,
                device_contact_id: None,
            }
            .try_into()?;

            assert_eq!(79997771122, contact.phone);

            let contact: create_contacts::Contact = Contact {
                phone_number: Some("8 (999) 777 11-22".into()),
                name: None,
                device_contact_id: None,
            }
            .try_into()?;

            assert_eq!(79997771122, contact.phone);

            let contact: create_contacts::Contact = Contact {
                phone_number: Some(" 8 (999) 777 11-22 ".into()),
                name: None,
                device_contact_id: None,
            }
            .try_into()?;

            assert_eq!(79997771122, contact.phone);

            Ok(())
        }
    }
}
