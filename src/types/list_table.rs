use bytes::Bytes;
use chrono::*;
use crate::Error;
use crate::protocol;
use quick_protobuf::{MessageRead, BytesReader, message::MessageWrite};
use std::convert::TryFrom;

pub const LIST_TABLE: &str = "/ListTable";

#[derive(Debug, Clone, Default)]
pub struct ListTableRequest {}
#[derive(Debug, Clone)]
pub struct ListTableResponse {
    base: super::BaseResponse,
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
            base: super::BaseResponse::default(),
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

impl super::Response for ListTableResponse {
    fn set_server_timestamp(&mut self, tm: Option<DateTime<Utc>>) -> () {
        self.base.server_timestamp = tm;
    }

    fn get_server_timestamp(&self) -> &Option<DateTime<Utc>> {
        &self.base.server_timestamp
    }

    fn set_request_id(&mut self, req_id: Option<String>) -> () {
        self.base.req_id = req_id;
    }

    fn get_request_id(&self) -> &Option<String> {
        &self.base.req_id
    }
}
