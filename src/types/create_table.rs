use bytes::Bytes;
use crate::Error;
use crate::protocol as pb;
use std::convert::TryFrom;
use super::*;

#[derive(Debug, Clone)]
pub struct CreateTableRequest {
    pub table_meta: TableMeta,
    pub options: TableOptions,
}

impl CreateTableRequest {
    pub fn new(table_meta: TableMeta) -> Self {
        Self{
            table_meta,
            options: TableOptions::default_for_create(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTableResponse {
    pub base: super::BaseResponse,
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
            base: super::BaseResponse::default()
        }
    }
}

impl From<CreateTableRequest> for Bytes {
    fn from(x: CreateTableRequest) -> Bytes {
        serialize_request::<CreateTableRequest, pb::CreateTableRequest>(x)
    }
}

impl TryFrom<Vec<u8>> for CreateTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        new_response::<Self, pb::CreateTableResponse>(&v)
    }
}

impl super::Request for CreateTableRequest {
    fn action(&self) -> Action {
        Action::CreateTable
    }

    fn path(&self) -> String {
        self.action().to_string()
    }
}

impl super::Response for CreateTableResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
