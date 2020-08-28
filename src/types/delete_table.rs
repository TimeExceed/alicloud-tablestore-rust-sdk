use bytes::Bytes;
use chrono::*;
use crate::Error;
use crate::protocol as pb;
use quick_protobuf::{MessageRead, BytesReader, message::MessageWrite};
use std::convert::TryFrom;

pub const DELETE_TABLE: &str = "/DeleteTable";

#[derive(Debug, Clone, Default)]
pub struct DeleteTableRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteTableResponse {
    base: super::BaseResponse,
}

impl From<DeleteTableRequest> for pb::DeleteTableRequest {
    fn from(x: DeleteTableRequest) -> pb::DeleteTableRequest {
        pb::DeleteTableRequest{
            table_name: x.name,
        }
    }
}

impl From<pb::DeleteTableResponse> for DeleteTableResponse {
    fn from(_x: pb::DeleteTableResponse) -> DeleteTableResponse {
        DeleteTableResponse{
            base: super::BaseResponse::default(),
        }
    }
}

impl From<DeleteTableRequest> for Bytes {
    fn from(x: DeleteTableRequest) -> Bytes {
        let req: pb::DeleteTableRequest = x.into();
        let len = req.get_size();
        let mut body = Vec::new();
        body.resize(len, 0u8);
        let writer = quick_protobuf::writer::BytesWriter::new(&mut body);
        let mut writer = quick_protobuf::writer::Writer::new(writer);
        req.write_message(&mut writer).unwrap();
        Bytes::from(body)
    }
}

impl TryFrom<Vec<u8>> for DeleteTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::from_bytes(&v);
        let resp = pb::DeleteTableResponse::from_reader(&mut reader, &v)?;
        Ok(resp.into())
    }
}

impl super::Response for DeleteTableResponse {
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
