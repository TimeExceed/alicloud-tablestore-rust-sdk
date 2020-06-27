use crate::Error;

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub address: String,
    pub instance: String,
}

impl Endpoint {
    pub fn new<P, Q>(
        address: P,
        instance: Q,
    ) -> Result<Endpoint, Error> 
    where
        P: ToString,
        Q: ToString,
    {
        let res = Endpoint{
            address: address.to_string(),
            instance: instance.to_string(),
        };
        Ok(res)
    }
}
