use bytes::Bytes;
use chrono::*;
use crate::Error;
use crate::protocol as pb;
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

impl From<ListTableRequest> for pb::ListTableRequest {
    fn from(_: ListTableRequest) -> pb::ListTableRequest {
        pb::ListTableRequest{}
    }
}

impl From<pb::ListTableResponse> for ListTableResponse {
    fn from(x: pb::ListTableResponse) -> ListTableResponse {
        ListTableResponse{
            base: super::BaseResponse::default(),
            tables: x.table_names,
        }
    }
}

impl From<ListTableRequest> for Bytes {
    fn from(x: ListTableRequest) -> Bytes {
        let req: pb::ListTableRequest = x.into();
        let len = req.get_size();
        let mut body = Vec::new();
        body.resize(len, 0u8);
        let writer = quick_protobuf::writer::BytesWriter::new(&mut body);
        let mut writer = quick_protobuf::writer::Writer::new(writer);
        req.write_message(&mut writer).unwrap();
        Bytes::from(body)
    }
}

impl TryFrom<Vec<u8>> for ListTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::from_bytes(&v);
        let resp = pb::ListTableResponse::from_reader(&mut reader, &v)?;
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
