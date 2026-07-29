use bzd_lib::error::Error;
use coset::{CborSerializable as _, CoseKey, cbor::Value};
use url::Url;

use crate::app::error::AppError;

// const FLAG_UP: u8 = 1 << 0;
// const FLAG_UV: u8 = 1 << 2;
// const FLAG_AT: u8 = 1 << 6;
// const FLAG_ED: u8 = 1 << 7;

#[derive(Debug)]
pub struct AuthenticatorData {
    // pub rp_id_hash: [u8; 32],
    // pub flags: u8,
    pub sign_count: i32,
    pub _aaguid: [u8; 16],
    pub credential_id: Vec<u8>,
    pub cose_public_key: CoseKey,
}

impl AuthenticatorData {
    pub fn new(data: &Value) -> Result<Self, Error> {
        let data = data.as_bytes().ok_or(AppError::Internal)?;

        // let mut rp_id_hash = [0u8; 32];
        // rp_id_hash.copy_from_slice(&data[0..32]);
        // let flags = data[32];
        let sign_count = i32::from_be_bytes([data[33], data[34], data[35], data[36]]);

        let mut offset = 37;
        let _aaguid: [u8; 16] = data[offset..offset + 16].try_into()?;
        offset += 16;

        let cred_id_len: usize = u16::from_be_bytes([data[offset], data[offset + 1]]).into();
        offset += 2;

        let credential_id = data[offset..offset + cred_id_len].to_vec();
        offset += cred_id_len;

        let cose_public_key = CoseKey::from_slice(&data[offset..])?;

        Ok(Self {
            // rp_id_hash,
            // flags,
            sign_count,
            _aaguid,
            credential_id,
            cose_public_key,
        })
    }
}

pub struct Origin {
    pub rp_id: String,
}

impl Origin {
    pub fn new(origin: &str) -> Result<Self, Error> {
        let url = Url::parse(origin)?;
        let rp_id = url.host().ok_or(AppError::Internal)?.to_string();

        Ok(Self { rp_id })
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use bzd_lib::error::Error;
    use coset::CborSerializable;

    use crate::app::auth::passkey::{AttestationObject, authenticator_data::AuthenticatorData};

    #[test]
    fn test_parse_auth_data() -> Result<(), Error> {
        let mut buf = Vec::new();
        buf.extend(std::iter::repeat_n(0xAB, 32));
        buf.push(0x05 | 1 << 6);
        buf.extend_from_slice(&42u32.to_be_bytes());
        buf.extend(std::iter::repeat_n(0xCD, 16));
        buf.extend_from_slice(&3u16.to_be_bytes());
        buf.extend_from_slice(&[1, 2, 3]);
        buf.extend_from_slice(&[0xa1, 0x01, 0x02]);

        let ad = AuthenticatorData::new(&buf.into())?;

        assert_eq!(ad.sign_count, 42);
        assert_eq!(ad._aaguid, [0xCD; 16]);
        assert_eq!(ad.credential_id, &[1, 2, 3]);
        assert_eq!(ad.cose_public_key.to_vec()?, &[0xa1, 0x01, 0x02]);
        Ok(())
    }

    #[test]
    fn test_att_object_from_string() -> Result<(), Error> {
        let data = String::from(
            "o2NmbXRkbm9uZWdhdHRTdG10oGhhdXRoRGF0YViYJgNZV95xeI0O2+g02BDJ/bcG+y9OdhSGGn9ZtWcrlIBdAAAAAPv8MAcVTk7MjAtuAgVX170AFM6EmugtEzHSI0LQhtklE1dkGKRypQECAyYgASFYIL4eHTzU9Da66h1eCIDNK4JQHmDcSCzlFL5IBaZ5QUvsIlggto3hXDQifBUEQddU8CQKeBzyT3nzKqEbFpCdf3OnO4I=",
        );

        let attestation: AttestationObject = serde_cbor_2::from_slice(&STANDARD.decode(data)?)?;

        assert!(AuthenticatorData::new(&attestation.auth_data).is_ok());
        Ok(())
    }
}
