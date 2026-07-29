use bzd_lib::error::Error;
use coset::cbor::Value;
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;
use uuid::Uuid;

pub mod authenticator_data;

#[serde_as]
#[derive(Serialize)]
pub struct PublicKeyCredentialRequestOptions {
    #[serde_as(as = "Base64")]
    pub challenge: Vec<u8>,
    pub rp_id: Option<String>,
    pub allow_credentials: Vec<PublicKeyCredentialDescriptor>,
}

#[serde_as]
#[derive(Serialize)]
pub struct PublicKeyCredentialDescriptor {
    #[serde_as(as = "Base64")]
    pub id: Vec<u8>,
}

#[serde_as]
#[derive(Serialize)]
pub struct PublicKeyCredentialCreationOptions {
    #[serde_as(as = "Base64")]
    pub challenge: Vec<u8>,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
}

#[derive(Serialize)]
pub struct PublicKeyCredentialRpEntity {
    pub id: String,
    pub name: String,
}

#[serde_as]
#[derive(Serialize)]
pub struct PublicKeyCredentialUserEntity {
    #[serde_as(as = "Base64")]
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
}

#[serde_as]
#[derive(Deserialize, Debug, Serialize)]
pub struct ClientData {
    #[serde(rename = "type")]
    pub tp: ClientDataType,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub challenge: Vec<u8>,
    pub origin: String,
}

impl ClientData {
    pub fn new(data: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(data)?)
    }
}

#[serde_as]
#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub enum ClientDataType {
    #[serde(rename = "webauthn.create")]
    Create,

    #[serde(rename = "webauthn.get")]
    Get,
}

#[serde_as]
#[derive(Deserialize, Debug, Serialize)]
pub struct AttestationObject {
    #[serde(rename = "authData")]
    pub auth_data: Value,
}

impl TryFrom<Vec<u8>> for AttestationObject {
    type Error = Error;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_cbor_2::from_slice(&val)?)
    }
}
