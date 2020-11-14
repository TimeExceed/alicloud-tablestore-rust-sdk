use bytes::Bytes;
use crate::Error;
use crate::plainbuffer::PbufSerde;
use crate::protocol as pb;
use std::convert::TryFrom;
use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GetRangeRequest {
    pub table_name: Name,
    pub inclusive_start: ExtendedRowKey,
    pub exclusive_end: ExtendedRowKey,
}

#[derive(Debug, Clone)]
pub struct GetRangeResponse {
    pub base: super::BaseResponse,
    pub rows: Vec<Row>,
    pub next_row_key: Option<ExtendedRowKey>,
}

impl From<GetRangeRequest> for pb::GetRangeRequest {
    fn from(x: GetRangeRequest) -> Self {
        let direction = if x.inclusive_start <= x.exclusive_end {
            pb::Direction::FORWARD
        } else {
            pb::Direction::BACKWARD
        };
        Self{
            table_name: x.table_name.into(),
            direction,
            columns_to_get: vec![],
            time_range: None,
            max_versions: Some(1),
            limit: None,
            inclusive_start_primary_key: x.inclusive_start.to_pbuf(),
            exclusive_end_primary_key: x.exclusive_end.to_pbuf(),
            cache_blocks: true,
            filter: None,
            start_column: None,
            end_column: None,
            token: None,
        }
    }
}

impl From<GetRangeRequest> for Bytes {
    fn from(x: GetRangeRequest) -> Bytes {
        serialize_request::<GetRangeRequest, pb::GetRangeRequest>(x)
    }
}


impl From<pb::GetRangeResponse> for GetRangeResponse {
    fn from(x: pb::GetRangeResponse) -> Self {
        let buf = Bytes::from(x.rows);
        let rows = Vec::<Row>::from_pbuf(buf).unwrap();
        let next_row_key = if let Some(nrk) = x.next_start_primary_key {
            let buf = Bytes::from(nrk);
            let res = ExtendedRowKey::from_pbuf(buf).unwrap();
            Some(res)
        } else {
            None
        };
        Self{
            base: super::BaseResponse::default(),
            rows,
            next_row_key,
        }
    }
}

impl TryFrom<Vec<u8>> for GetRangeResponse {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        super::new_response::<Self, pb::GetRangeResponse>(&v)
    }
}

impl super::Request for GetRangeRequest {
    fn action(&self) -> Action {
        Action::GetRange
    }

    fn path(&self) -> String {
        self.action().to_string()
    }
}

impl super::Response for GetRangeResponse {
    fn base_mut_ref(&mut self) -> &mut BaseResponse {
        &mut self.base
    }
}
