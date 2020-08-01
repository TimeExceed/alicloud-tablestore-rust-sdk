use bytes::Bytes;
use crate::Error;

#[derive(Debug, Clone)]
pub struct Credential {
    pub id: Bytes,
    pub secret: Bytes,
    pub token: Option<Bytes>,
}

impl Credential {
    pub fn new<P, Q>(
        id: P,
        secret: Q,
    ) -> Result<Credential, Error> 
    where
        P: ToString,
        Q: ToString,
    {
        let res = Credential{
            id: Bytes::from(id.to_string()),
            secret: Bytes::from(secret.to_string()),
            token: None,
        };
        Ok(res)
    }
}
