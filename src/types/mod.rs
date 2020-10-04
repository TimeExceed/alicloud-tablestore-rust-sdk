use bytes::Bytes;
use crate::Error;
use quick_protobuf::{MessageRead, BytesReader, message::MessageWrite};

pub trait Request {
    fn action(&self) -> Action;
    fn path(&self) -> String;
}

pub(crate) trait Response {
    fn base_mut_ref(&mut self) -> &mut BaseResponse;

    fn reset_base(
        &mut self,
        server_tm: Option<chrono::DateTime<chrono::Utc>>,
        req_id: Option<String>,
    ) {
        self.base_mut_ref().server_timestamp = server_tm;
        self.base_mut_ref().req_id = req_id;
    }
}

pub(in crate::types) fn serialize_request<Req, PbReq>(
    x: Req,
) -> Bytes
where
    Req: std::marker::Sized,
    PbReq: From<Req> + MessageWrite
{
    let req = PbReq::from(x);
    let len = req.get_size();
    let mut body = Vec::new();
    body.resize(len, 0u8);
    let writer = quick_protobuf::writer::BytesWriter::new(&mut body);
    let mut writer = quick_protobuf::writer::Writer::new(writer);
    req.write_message(&mut writer).unwrap();
    Bytes::from(body)
}

pub(in crate::types) fn new_response<'a, Resp, PbResp>(
    b: &'a [u8],
) -> Result<Resp, Error>
where
    PbResp: MessageRead<'a>,
    Resp: From<PbResp> + std::marker::Sized,
{
    let mut reader = BytesReader::from_bytes(b);
    let resp = PbResp::from_reader(&mut reader, b)?;
    Ok(resp.into())
}

#[derive(Debug, Clone)]
pub struct BaseResponse {
    pub server_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub req_id: Option<String>,
}

impl Default for BaseResponse {
    fn default() -> BaseResponse {
        BaseResponse{
            server_timestamp: None,
            req_id: None,
        }
    }
}

mod common;
pub use self::common::*;
mod action;
pub use self::action::*;
mod name;
pub use self::name::*;
mod table_meta;
pub use self::table_meta::*;
mod table_options;
pub use self::table_options::*;
mod condition;
pub use self::condition::*;
mod in_return;
pub use self::in_return::*;
mod rowkey;
pub use self::rowkey::*;
mod attr;
pub use self::attr::*;
mod row;
pub use self::row::*;

mod list_table;
pub use self::list_table::*;
mod create_table;
pub use self::create_table::*;
mod delete_table;
pub use self::delete_table::*;
mod put_row;
pub use self::put_row::*;
mod get_range;
pub use self::get_range::*;
