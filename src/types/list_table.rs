use bytes::Bytes;
use crate::Error;
use crate::protocol as pb;
use std::convert::TryFrom;
use super::*;

const LIST_TABLE: &str = "/ListTable";

#[derive(Debug, Clone, Default)]
pub struct ListTableRequest {}

#[derive(Debug, Clone)]
pub struct ListTableResponse {
    pub base: super::BaseResponse,
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
        serialize_request::<ListTableRequest, pb::ListTableRequest>(x)
    }
}

impl TryFrom<Vec<u8>> for ListTableResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        super::new_response::<Self, pb::ListTableResponse>(&v)
    }
}

impl super::Request for ListTableRequest {
    fn path(&self) -> &'static str {
        LIST_TABLE
    }
}

impl super::Response for ListTableResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
