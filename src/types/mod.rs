use bytes::Bytes;
use crate::Error;
use std::convert::TryFrom;

pub(crate) trait ApiTrait {
    type Request: Into<Bytes>;
    type Response: TryFrom<Vec<u8>, Error=Error>;

    fn path(&self) -> &'static str;
}

mod list_table;
pub use self::list_table::*;
