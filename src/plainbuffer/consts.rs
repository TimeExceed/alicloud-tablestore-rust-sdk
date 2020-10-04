use crate::{Error, ErrorCode};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VariantType {
    Integer = 0x0,
    Double = 0x1,
    Boolean = 0x2,
    String = 0x3,
    Null = 0x6,
    Blob = 0x7,
    InfMin = 0x9,
    InfMax = 0xa,
    AutoIncrement = 0xb,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tag {
    None = 0,
    RowKey = 0x1,
    RowData = 0x2,
    Cell = 0x3,
    CellName = 0x4,
    CellValue = 0x5,
    CellType = 0x6,
    CellTimestamp = 0x7,
    RowDeleteMarker = 0x8,
    RowChecksum = 0x9,
    CellChecksum = 0x0A,
}

impl Default for VariantType {
    fn default() -> Self {
        VariantType::Null
    }
}

impl TryFrom<u8> for VariantType {
    type Error = crate::Error;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0x0 => Ok(VariantType::Integer),
            0x1 => Ok(VariantType::Double),
            0x2 => Ok(VariantType::Boolean),
            0x3 => Ok(VariantType::String),
            0x6 => Ok(VariantType::Null),
            0x7 => Ok(VariantType::Blob),
            0x9 => Ok(VariantType::InfMin),
            0xa => Ok(VariantType::InfMax),
            0xb => Ok(VariantType::AutoIncrement),
            _ => Err(Error{
                code: ErrorCode::CorruptedResponse,
                message: "".to_string(),
            })
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::None
    }
}

impl TryFrom<u8> for Tag {
    type Error = crate::Error;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
             0 => Ok(Tag::None),
             0x1 => Ok(Tag::RowKey),
             0x2 => Ok(Tag::RowData),
             0x3 => Ok(Tag::Cell),
             0x4 => Ok(Tag::CellName),
             0x5 => Ok(Tag::CellValue),
             0x6 => Ok(Tag::CellType),
             0x7 => Ok(Tag::CellTimestamp),
             0x8 => Ok(Tag::RowDeleteMarker),
             0x9 => Ok(Tag::RowChecksum),
             0x0A => Ok(Tag::CellChecksum),
             _ => Err(Error{
                code: ErrorCode::CorruptedResponse,
                message: "".to_string(),
             })
        }
    }
}

pub const HEADER: u32 = 0x75;
