use bytes::Bytes;
use crate::Error;
use crate::protocol as pb;
use std::convert::TryFrom;
use super::*;

const DELETE_TABLE: &str = "/DeleteTable";

#[derive(Debug, Clone, Default)]
pub struct DeleteTableRequest {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteTableResponse {
    pub base: super::BaseResponse,
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
        serialize_request::<DeleteTableRequest, pb::DeleteTableRequest>(x)
    }
}

impl TryFrom<Vec<u8>> for DeleteTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        super::new_response::<Self, pb::DeleteTableResponse>(&v)
    }
}

impl super::Request for DeleteTableRequest {
    fn path(&self) -> &'static str {
        DELETE_TABLE
    }
}

impl super::Response for DeleteTableResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
