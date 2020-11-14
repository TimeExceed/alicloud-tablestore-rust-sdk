use bytes::Bytes;
use crate::Error;

#[cfg(test)]
mod tests;

mod crc;
pub(in crate::plainbuffer) use crc::*;
mod consts;
pub(in crate::plainbuffer) use consts::*;
mod serde;
pub(in crate::plainbuffer) use serde::*;

pub(crate) trait PbufSerde: Sized {
    fn to_pbuf(&self) -> Vec<u8>;
    fn from_pbuf(buf: Bytes) -> Result<Self, Error>;
}

impl PbufSerde for crate::types::Row {
    fn to_pbuf(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        consts::HEADER.serialize(&mut buf);
        self.serialize(&mut buf);
        buf
    }

    fn from_pbuf(mut buf: Bytes) -> Result<Self, Error> {
        let header = u32::deserialize(&mut buf)?;
        if header != consts::HEADER {
            return serde::issue_error();
        }
        crate::types::Row::deserialize(&mut buf)
    }
}

impl PbufSerde for Vec<crate::types::Row> {
    fn to_pbuf(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        consts::HEADER.serialize(&mut buf);
        self.iter()
            .for_each(|x| {
                x.serialize(&mut buf);
            });
        buf
    }

    fn from_pbuf(mut buf: Bytes) -> Result<Self, Error> {
        let header = u32::deserialize(&mut buf)?;
        if header != consts::HEADER {
            return serde::issue_error();
        }
        let mut res = vec![];
        while !buf.is_empty() {
            let row = crate::types::Row::deserialize(&mut buf)?;
            res.push(row);
        }
        Ok(res)
    }
}

impl PbufSerde for crate::types::ExtendedRowKey {
    fn to_pbuf(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        let mut checksum = 0u8;
        consts::HEADER.serialize(&mut buf);
        self.serialize_crc8(&mut buf, &mut checksum);
        crc8_u8(&mut checksum, 0); // placeholder for missing row-delete marker
        Tag::RowChecksum.serialize(&mut buf);
        checksum.serialize(&mut buf);
        buf
    }

    fn from_pbuf(mut buf: Bytes) -> Result<Self, Error> {
        let header = u32::deserialize(&mut buf)?;
        if header != consts::HEADER {
            return serde::issue_error();
        }
        let mut checksum = 0u8;
        let res = crate::types::ExtendedRowKey::deserialize_crc8(&mut buf, &mut checksum)?;
        Ok(res)
    }
}
