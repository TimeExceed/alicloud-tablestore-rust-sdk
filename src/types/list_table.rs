use bytes::Bytes;
use crate::Error;
use crate::protocol as pb;
use std::convert::TryFrom;
use super::*;

#[derive(Debug, Clone, Default)]
pub struct ListTableRequest {}

#[derive(Debug, Clone)]
pub struct ListTableResponse {
    pub base: super::BaseResponse,
    pub tables: Vec<Name>,
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
            tables: x.table_names
                .into_iter()
                .map(|x| {
                    x.into()
                })
                .collect(),
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
    fn action(&self) -> Action {
        Action::ListTable
    }

    fn path(&self) -> String {
        self.action().to_string()
    }
}

impl super::Response for ListTableResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
