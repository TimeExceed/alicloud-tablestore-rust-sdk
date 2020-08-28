use bytes::Bytes;
use chrono::*;
use crate::Error;
use crate::protocol as pb;
use quick_protobuf::{MessageRead, BytesReader, message::MessageWrite};
use std::convert::TryFrom;
use super::*;

pub const CREATE_TABLE: &str = "/CreateTable";

#[derive(Debug, Clone)]
pub struct CreateTableRequest {
    pub table_meta: TableMeta,
    pub options: TableOptions,
}

#[derive(Debug, Clone)]
pub struct CreateTableResponse {
    base: super::BaseResponse,
}

impl From<CreateTableRequest> for pb::CreateTableRequest {
    fn from(x: CreateTableRequest) -> pb::CreateTableRequest {
        let table_meta = x.table_meta.into();
        let (opts, cu) = x.options.into();
        pb::CreateTableRequest{
            table_meta,
            reserved_throughput: pb::ReservedThroughput{
                capacity_unit: cu,
            },
            table_options: Some(opts),
            partitions: vec![],
        }
    }
}

impl From<pb::CreateTableResponse> for CreateTableResponse {
    fn from(_: pb::CreateTableResponse) -> CreateTableResponse {
        CreateTableResponse{
            base: super::BaseResponse::default() // FIXME:
        }
    }
}

impl From<CreateTableRequest> for Bytes {
    fn from(x: CreateTableRequest) -> Bytes {
        let req = pb::CreateTableRequest::from(x);
        let len = req.get_size();
        let mut body = Vec::new();
        body.resize(len, 0u8);
        let writer = quick_protobuf::writer::BytesWriter::new(&mut body);
        let mut writer = quick_protobuf::writer::Writer::new(writer);
        req.write_message(&mut writer).unwrap();
        Bytes::from(body)
    }
}

impl TryFrom<Vec<u8>> for CreateTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::from_bytes(&v);
        let resp = pb::CreateTableResponse::from_reader(&mut reader, &v)?;
        Ok(resp.into())
    }
}

impl super::Response for CreateTableResponse {
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
