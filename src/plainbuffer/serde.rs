use bytes::{Bytes, Buf, BufMut};
use crate::{Error, ErrorCode};
use crate::types::*;
use std::convert::TryFrom;

pub(crate) trait Serde {
    fn serialize(&self, out: &mut dyn BufMut);
    fn deserialize(inp: &mut dyn Buf) -> Result<Self, Error> where Self: Sized;
}

pub(crate) trait SerdeWithCrc8 {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8);
    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> where Self: Sized;
}

pub fn issue_error<T>() -> Result<T, Error> {
    Err(Error{
        code: ErrorCode::CorruptedResponse,
        message: "Fail to parse protobuf in response".to_string(),
    })
}

impl Serde for u8 {
    fn serialize(&self, out: &mut dyn BufMut) {
        out.put_u8(*self);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<u8, Error> {
        if !inp.has_remaining() {
            return issue_error();
        }
        Ok(inp.get_u8())
    }
}

impl Serde for u32 {
    fn serialize(&self, out: &mut dyn BufMut) {
        out.put_u32_le(*self);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<u32, Error> {
        if inp.remaining() < 4 {
            return issue_error();
        }
        Ok(inp.get_u32_le())
    }
}

impl Serde for u64 {
    fn serialize(&self, out: &mut dyn BufMut) {
        out.put_u64_le(*self);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<u64, Error> {
        if inp.remaining() < 8 {
            return issue_error();
        }
        Ok(inp.get_u64_le())
    }
}

impl SerdeWithCrc8 for u64 {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        self.serialize(out);
        self.to_le_bytes().iter()
            .for_each(|x| {
                super::crc8_u8(checksum, *x);
            });
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<u64, Error> {
        let res = u64::deserialize(inp)?;
        res.to_le_bytes().iter()
            .for_each(|x| {
                super::crc8_u8(checksum, *x);
            });
        Ok(res)
    }
}

impl Serde for i64 {
    fn serialize(&self, out: &mut dyn BufMut) {
        (*self as u64).serialize(out)
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<i64, Error> {
        let x = u64::deserialize(inp)?;
        Ok(x as i64)
    }
}

impl SerdeWithCrc8 for i64 {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        (*self as u64).serialize_crc8(out, checksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<i64, Error> {
        let res = u64::deserialize_crc8(inp, checksum)?;
        Ok(res as i64)
    }
}

impl<'a> Serde for &'a [u8] {
    fn serialize(&self, out: &mut dyn BufMut) {
        (self.len() as u32).serialize(out);
        out.put_slice(self.as_ref());
    }

    fn deserialize(_inp: &mut dyn Buf) -> Result<&'a [u8], Error> {
        unimplemented!()
    }
}

impl<'a> SerdeWithCrc8 for &'a [u8] {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        self.serialize(out);
        super::crc8_u32(checksum, self.len() as u32);
        super::crc8_blob(checksum, self);
    }

    fn deserialize_crc8(
        _inp: &mut dyn Buf,
        _checksum: &mut u8,
    ) -> Result<&'a [u8], Error> {
        unimplemented!()
    }
}

impl Serde for Vec<u8> {
    fn serialize(&self, _out: &mut dyn BufMut) {
        unimplemented!()
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<Vec<u8>, Error> {
        let len = u32::deserialize(inp)?;
        let len = len as usize;
        if inp.remaining() < len {
            return issue_error();
        }
        let mut res: Vec<u8> = vec![];
        res.resize(len, 0u8);
        inp.copy_to_slice(&mut res);
        Ok(res)
    }
}

impl SerdeWithCrc8 for Vec<u8> {
    fn serialize_crc8(&self, _out: &mut dyn BufMut, _checksum: &mut u8) {
        unimplemented!()
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Vec<u8>, Error> {
        let res = Vec::<u8>::deserialize(inp)?;
        match u32::try_from(res.len()) {
            Ok(x) => {
                super::crc8_u32(checksum, x);
            }
            _ => {
                return issue_error();
            }
        }
        super::crc8_blob(checksum, &res);
        Ok(res)
    }
}

impl Serde for Bytes {
    fn serialize(&self, out: &mut dyn BufMut) {
        self.as_ref().serialize(out);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<Bytes, Error> {
        let raw = Vec::<u8>::deserialize(inp)?;
        Ok(Bytes::from(raw))
    }
}

impl SerdeWithCrc8 for Bytes {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        self.as_ref().serialize_crc8(out, checksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Bytes, Error> {
        let raw = Vec::<u8>::deserialize_crc8(inp, checksum)?;
        Ok(Bytes::from(raw))
    }
}

impl Serde for String {
    fn serialize(&self, out: &mut dyn BufMut) {
        self.as_bytes().serialize(out);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<String, Error> {
        let raw = Vec::<u8>::deserialize(inp)?;
        match String::from_utf8(raw) {
            Ok(x) => Ok(x),
            Err(_) => issue_error()
        }
    }
}

impl SerdeWithCrc8 for String {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        self.as_bytes().serialize_crc8(out, checksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<String, Error> {
        let raw = Vec::<u8>::deserialize_crc8(inp, checksum)?;
        match String::from_utf8(raw) {
            Ok(x) => Ok(x),
            Err(_) => issue_error()
        }
    }
}

impl SerdeWithCrc8 for Name {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::CellName.serialize(out);
        let name = <&str>::from(self);
        let name = name.as_bytes();
        name.serialize(out);
        super::crc8_blob(checksum, name);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        if super::Tag::deserialize(inp)? != super::Tag::CellName {
            return issue_error();
        }
        let res = String::deserialize(inp)?;
        super::crc8_blob(checksum, res.as_bytes());
        Ok(res.into())
    }
}

impl SerdeWithCrc8 for RowKeyValue {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        ExtendedRowKeyValue::from(self.clone()).serialize_crc8(out, checksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<RowKeyValue, Error> {
        let res = ExtendedRowKeyValue::deserialize_crc8(inp, checksum)?;
        let res = RowKeyValue::try_from(res)?;
        Ok(res)
    }
}

impl SerdeWithCrc8 for ExtendedRowKeyValue {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::CellValue.serialize(out);
        match self {
            ExtendedRowKeyValue::Int(x) => {
                ((std::mem::size_of::<i64>() + 1) as u32).serialize(out);
                super::VariantType::Integer.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            ExtendedRowKeyValue::Str(x) => {
                ((x.len() + std::mem::size_of::<u32>() + 1) as u32).serialize(out);
                super::VariantType::String.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            ExtendedRowKeyValue::Blob(x) => {
                ((x.len() + std::mem::size_of::<u32>() + 1) as u32).serialize(out);
                super::VariantType::Blob.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            ExtendedRowKeyValue::InfMin => {
                1u32.serialize(out);
                super::VariantType::InfMin.serialize_crc8(out, checksum);
            }
            ExtendedRowKeyValue::InfMax => {
                1u32.serialize(out);
                super::VariantType::InfMax.serialize_crc8(out, checksum);
            }
            ExtendedRowKeyValue::AutoIncr => {
                1u32.serialize(out);
                super::VariantType::AutoIncrement.serialize_crc8(out, checksum);
            }
        }
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<ExtendedRowKeyValue, Error> {
        let expect_cellvalue = super::Tag::deserialize(inp)?;
        if expect_cellvalue != super::Tag::CellValue {
            return issue_error();
        }
        let exp_payload_len = usize::try_from(u32::deserialize(inp)?).unwrap();
        let vt = super::VariantType::deserialize_crc8(inp, checksum)?;
        let mut real_payload_len = 1usize;
        let res = match vt {
            super::VariantType::Integer => {
                let x = i64::deserialize_crc8(inp, checksum)?;
                real_payload_len += std::mem::size_of_val(&x);
                Ok(ExtendedRowKeyValue::Int(x))
            }
            super::VariantType::String => {
                let s = String::deserialize_crc8(inp, checksum)?;
                real_payload_len += s.len();
                real_payload_len += std::mem::size_of::<u32>();
                Ok(ExtendedRowKeyValue::Str(s))
            }
            super::VariantType::Blob => {
                let b = Bytes::deserialize_crc8(inp, checksum)?;
                real_payload_len += b.len();
                real_payload_len += std::mem::size_of::<u32>();
                Ok(ExtendedRowKeyValue::Blob(b))
            }
            super::VariantType::InfMin => Ok(ExtendedRowKeyValue::InfMin),
            super::VariantType::InfMax => Ok(ExtendedRowKeyValue::InfMax),
            super::VariantType::AutoIncrement => {
                Ok(ExtendedRowKeyValue::AutoIncr)
            }
            _ => unimplemented!()
        };
        if exp_payload_len != real_payload_len {
            println!("here: {} {}", exp_payload_len, real_payload_len);
            return issue_error();
        }
        res
    }
}

impl Serde for super::Tag {
    fn serialize(&self, out: &mut dyn BufMut) {
        (*self as u8).serialize(out)
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<super::Tag, Error> {
        if !inp.has_remaining() {
            return issue_error();
        }
        let x = u8::deserialize(inp)?;
        let res = super::Tag::try_from(x)?;
        Ok(res)
    }
}

impl Serde for super::VariantType {
    fn serialize(&self, out: &mut dyn BufMut) {
        (*self as u8).serialize(out)
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<super::VariantType, Error> {
        if !inp.has_remaining() {
            return issue_error();
        }
        let x = u8::deserialize(inp)?;
        let res = super::VariantType::try_from(x)?;
        Ok(res)
    }
}

impl SerdeWithCrc8 for super::VariantType {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        self.serialize(out);
        super::crc8_u8(checksum, *self as u8);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<super::VariantType, Error> {
        let res = super::VariantType::deserialize(inp)?;
        super::crc8_u8(checksum, res as u8);
        Ok(res)
    }
}

impl SerdeWithCrc8 for RowKeyColumn {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        let mut cell_chksum = 0u8;
        super::Tag::Cell.serialize(out);
        self.name.serialize_crc8(out, &mut cell_chksum);
        self.value.serialize_crc8(out, &mut cell_chksum);
        super::Tag::CellChecksum.serialize(out);
        cell_chksum.serialize(out);
        super::crc8_u8(checksum, cell_chksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        let ext = ExtendedRowKeyColumn::deserialize_crc8(inp, checksum)?;
        match RowKeyColumn::try_from(ext) {
            Ok(x) => Ok(x),
            Err(_) => issue_error(),
        }
    }
}

impl SerdeWithCrc8 for ExtendedRowKeyColumn {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        let mut cell_chksum = 0u8;
        super::Tag::Cell.serialize(out);
        self.name.serialize_crc8(out, &mut cell_chksum);
        self.value.serialize_crc8(out, &mut cell_chksum);
        super::Tag::CellChecksum.serialize(out);
        cell_chksum.serialize(out);
        super::crc8_u8(checksum, cell_chksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        if super::Tag::deserialize(inp)? != super::Tag::Cell {
            return issue_error();
        }
        let mut real_chksum = 0u8;
        let name = Name::deserialize_crc8(inp, &mut real_chksum)?;
        println!("name: {:?}", name);
        let value = ExtendedRowKeyValue::deserialize_crc8(inp, &mut real_chksum)?;
        println!("value: {:?}", value);
        deser_check_checksum(inp, real_chksum)?;
        println!("real checksum: {:x}", real_chksum);
        super::crc8_u8(checksum, real_chksum);
        Ok(ExtendedRowKeyColumn{
            name,
            value,
        })
    }
}

impl SerdeWithCrc8 for AttrValue {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::CellValue.serialize(out);
        match self {
            AttrValue::Str(x) => {
                ((x.len() + 1) as u32).serialize(out);
                super::VariantType::String.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            AttrValue::Int(x) => {
                ((std::mem::size_of::<i64>() + 1) as u32).serialize(out);
                super::VariantType::Integer.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            AttrValue::Blob(x) => {
                ((x.len() + 1) as u32).serialize(out);
                super::VariantType::Blob.serialize_crc8(out, checksum);
                x.serialize_crc8(out, checksum);
            }
            AttrValue::Bool(x) => {
                2u32.serialize(out);
                super::VariantType::Boolean.serialize_crc8(out, checksum);
                let x = if *x {
                    1u8
                } else {
                    0u8
                };
                x.serialize(out);
                super::crc8_u8(checksum, x);
            }
            AttrValue::Float(x) => {
                let xs = x.to_le_bytes();
                ((xs.len() + 1) as u32).serialize(out);
                super::VariantType::Double.serialize_crc8(out, checksum);
                out.put_slice(xs.as_ref());
                super::crc8_blob(checksum, &xs);
            }
        }
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        let expect_cellvalue = super::Tag::deserialize(inp)?;
        if expect_cellvalue != super::Tag::CellValue {
            return issue_error();
        }
        let _ = u32::deserialize(inp)?;
        let vt = super::VariantType::deserialize_crc8(inp, checksum)?;
        match vt {
            super::VariantType::Integer => {
                let x = i64::deserialize_crc8(inp, checksum)?;
                Ok(AttrValue::Int(x))
            }
            super::VariantType::String => {
                let s = String::deserialize_crc8(inp, checksum)?;
                Ok(AttrValue::Str(s))
            }
            super::VariantType::Blob => {
                let b = Bytes::deserialize_crc8(inp, checksum)?;
                Ok(AttrValue::Blob(b))
            }
            super::VariantType::Boolean => {
                let x = u8::deserialize(inp)?;
                super::crc8_u8(checksum, x);
                Ok(AttrValue::Bool(x > 0))
            }
            super::VariantType::Double => {
                let x = u64::deserialize(inp)?;
                super::crc8_u64(checksum, x);
                Ok(AttrValue::Float(f64::from_le_bytes(x.to_le_bytes())))
            }
            _ => unimplemented!()
        }
    }
}

impl SerdeWithCrc8 for Attribute {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::Cell.serialize(out);
        let mut cell_chksum = 0u8;
        self.name.serialize_crc8(out, &mut cell_chksum);
        self.value.serialize_crc8(out, &mut cell_chksum);
        if let AttrTimestamp::ClientAttach(tm) = &self.timestamp {
            super::Tag::CellTimestamp.serialize(out);
            let msecs = tm.to_millis();
            msecs.serialize_crc8(out, &mut cell_chksum);
        }
        super::Tag::CellChecksum.serialize(out);
        cell_chksum.serialize(out);
        super::crc8_u8(checksum, cell_chksum);
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        if super::Tag::deserialize(inp)? != super::Tag::Cell {
            return issue_error();
        }
        let mut cell_chksum = 0u8;
        let name = Name::deserialize_crc8(inp, &mut cell_chksum)?;
        let value = AttrValue::deserialize_crc8(inp, &mut cell_chksum)?;
        let tm = if peek_and_expect(inp, super::Tag::CellTimestamp) {
            super::Tag::deserialize(inp)?;
            let msecs = i64::deserialize_crc8(inp, &mut cell_chksum)?;
            Some(DateTime::from_millis(msecs))
        } else {
            None
        };
        deser_check_checksum(inp, cell_chksum)?;
        super::crc8_u8(checksum, cell_chksum);
        Ok(Attribute{
            name,
            value,
            timestamp: AttrTimestamp::from(tm),
        })
    }
}

impl SerdeWithCrc8 for RowKey {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::RowKey.serialize(out);
        self.iter()
            .for_each(|x| {
                x.serialize_crc8(out, checksum);
            });
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        let ext = ExtendedRowKey::deserialize_crc8(inp, checksum)?;
        RowKey::try_from(ext)
    }
}

impl SerdeWithCrc8 for ExtendedRowKey {
    fn serialize_crc8(&self, out: &mut dyn BufMut, checksum: &mut u8) {
        super::Tag::RowKey.serialize(out);
        self.iter()
            .for_each(|x| {
                x.serialize_crc8(out, checksum);
            });
    }

    fn deserialize_crc8(
        inp: &mut dyn Buf,
        checksum: &mut u8,
    ) -> Result<Self, Error> {
        if super::Tag::deserialize(inp)? != super::Tag::RowKey {
            return issue_error();
        }
        let mut res = vec![];
        while peek_and_expect(inp, super::Tag::Cell) {
            let x = ExtendedRowKeyColumn::deserialize_crc8(inp, checksum)?;
            println!("{:?}", x);
            res.push(x);
        }
        Ok(ExtendedRowKey::new(res))
    }
}

impl Serde for Row {
    fn serialize(&self, out: &mut dyn BufMut) {
        let mut checksum = 0u8;
        self.row_key.serialize_crc8(out, &mut checksum);
        if !self.attrs.is_empty() {
            super::Tag::RowData.serialize(out);
            for x in self.attrs.iter() {
                x.serialize_crc8(out, &mut checksum);
            }
        }
        super::crc8_u8(&mut checksum, 0); // placeholder for missing row-delete marker
        super::Tag::RowChecksum.serialize(out);
        checksum.serialize(out);
    }

    fn deserialize(inp: &mut dyn Buf) -> Result<Self, Error> {
        let mut checksum = 0u8;
        let row_key = RowKey::deserialize_crc8(inp, &mut checksum)?;
        let mut attrs = vec![];
        if peek_and_expect(inp, super::Tag::RowData) {
            let _ = super::Tag::deserialize(inp)?;
            loop {
                if !peek_and_expect(inp, super::Tag::Cell) {
                    break;
                }
                let attr = Attribute::deserialize_crc8(inp, &mut checksum)?;
                attrs.push(attr);
            }
        }
        super::crc8_u8(&mut checksum, 0u8); // placeholder for missing row-delete marker
        if peek_and_expect(inp, super::Tag::RowChecksum) {
            let _ = super::Tag::deserialize(inp)?;
            let exp = u8::deserialize(inp)?;
            if checksum != exp {
                return issue_error();
            }
        }
        Ok(Row{
            row_key,
            attrs,
        })
    }
}

fn peek_and_expect(inp: &mut dyn Buf, exp: super::Tag) -> bool {
    if let Ok(tag) = peek_tag(inp) {
        if tag == exp {
            return true;
        }
    }
    return false;
}

fn peek_tag(inp: &mut dyn Buf) -> Result<super::Tag, Error> {
    if !inp.has_remaining() {
        return issue_error();
    }
    let xs = inp.bytes();
    assert!(!xs.is_empty());
    Ok(super::Tag::try_from(xs[0])?)
}

fn deser_check_checksum(
    inp: &mut dyn Buf,
    real_checksum: u8,
) -> Result<(), Error> {
    if super::Tag::deserialize(inp)? != super::Tag::CellChecksum {
        return issue_error();
    }
    let expect_chksum = u8::deserialize(inp)?;
    if real_checksum != expect_chksum {
        return issue_error();
    }
    Ok(())
}
