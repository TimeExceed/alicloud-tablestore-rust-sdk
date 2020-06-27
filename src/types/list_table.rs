use bytes::Bytes;
use crate::Error;
use crate::protocol;
use quick_protobuf::{MessageRead, BytesReader, message::MessageWrite};
use std::convert::TryFrom;
use super::ApiTrait;

#[derive(Debug, Clone, Copy)]
pub struct ListTable {}

#[derive(Debug, Clone, Default)]
pub struct ListTableRequest {}
#[derive(Debug, Clone)]
pub struct ListTableResponse {
    pub tables: Vec<String>,
}

impl Into<protocol::ListTableRequest> for ListTableRequest {
    fn into(self) -> protocol::ListTableRequest {
        protocol::ListTableRequest{}
    }
}

impl From<crate::protocol::ListTableResponse> for ListTableResponse {
    fn from(x: crate::protocol::ListTableResponse) -> ListTableResponse {
        ListTableResponse{
            tables: x.table_names,
        }
    }
}

impl Into<Bytes> for ListTableRequest {
    fn into(self) -> Bytes {
        let req: protocol::ListTableRequest = self.into();
        let mut body = Vec::with_capacity(req.get_size());
        if req.get_size() > 0 {
            let mut writer = quick_protobuf::writer::Writer::new(&mut body);
            writer.write_message(&req).unwrap();
        }
        Bytes::from(body)
    }
}

impl TryFrom<Vec<u8>> for ListTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::from_bytes(&v);
        let resp = protocol::ListTableResponse::from_reader(&mut reader, &v)?;
        Ok(resp.into())
    }
}

impl ApiTrait for ListTable {
    type Request = ListTableRequest;
    type Response = ListTableResponse;

    fn path(&self) -> &'static str {
        "/ListTable"
    }
}
