use bytes::Bytes;
use crate::Error;
use crate::protocol as pb;
use crate::plainbuffer::PbufSerde;
use std::convert::{TryFrom};
use super::*;

#[derive(Debug, Clone)]
pub struct PutRowRequest {
    pub table_name: Name,
    pub row: Row,
    pub condition: Condition,
    pub in_return: InReturn,
}

impl PutRowRequest {
    pub fn new<T: ToString>(table_name: T, row: Row) -> Result<Self, Error> {
        Ok(Self{
            table_name: Name::new(table_name),
            row,
            condition: Condition{
                row_exist: RowExistenceExpectation::Ignore,
            },
            in_return: InReturn::Nothing,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PutRowResponse {
    pub base: super::BaseResponse,
}

impl From<PutRowRequest> for pb::PutRowRequest {
    fn from(x: PutRowRequest) -> pb::PutRowRequest {
        pb::PutRowRequest{
            table_name: x.table_name.into(),
            row: x.row.to_pbuf(),
            condition: x.condition.into(),
            return_content: Some(x.in_return.into()),
        }
    }
}

impl From<pb::PutRowResponse> for PutRowResponse {
    fn from(_: pb::PutRowResponse) -> PutRowResponse {
        PutRowResponse{
            base: super::BaseResponse::default()
        }
    }
}

impl From<PutRowRequest> for Bytes {
    fn from(x: PutRowRequest) -> Bytes {
        serialize_request::<PutRowRequest, pb::PutRowRequest>(x)
    }
}

impl TryFrom<Vec<u8>> for PutRowResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        new_response::<Self, pb::PutRowResponse>(&v)
    }
}

impl super::Request for PutRowRequest {
    fn action(&self) -> Action {
        Action::PutRow
    }

    fn path(&self) -> String {
        self.action().to_string()
    }
}

impl super::Response for PutRowResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
