use bytes::{Bytes, BytesMut};
use crate::plainbuffer as pbuf;
use crate::types::*;
use pbuf::{Serde, SerdeWithCrc8, PbufSerde};

#[quickcheck]
fn serde_pkeyvalue(oracle: RowKeyValue) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = RowKeyValue::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_ext_pkeyvalue(oracle: ExtendedRowKeyValue) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    println!("{:?}", buf);
    let mut trial_chksum = 0u8;
    let trial = ExtendedRowKeyValue
        ::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_pkeycolumn(oracle: RowKeyColumn) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = RowKeyColumn::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_ext_pkeycolumn(oracle: ExtendedRowKeyColumn) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = ExtendedRowKeyColumn::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_attrvalue(oracle: AttrValue) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = AttrValue::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_attr(oracle: Attribute) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = Attribute::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_rowkey(oracle: RowKey) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    println!("{:?}", buf);
    let mut trial_chksum = 0u8;
    let trial = RowKey::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_ext_rowkey(oracle: ExtendedRowKey) {
    let mut oracle_chksum = 0u8;
    let mut buf = BytesMut::new();
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    println!("{:?}", buf);
    let mut trial_chksum = 0u8;
    let trial = ExtendedRowKey::deserialize_crc8(&mut buf, &mut trial_chksum)
        .unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

#[quickcheck]
fn serde_row(oracle: Row) {
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    println!("{:?}", buf);
    let trial = Row::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn pbufserde_row(oracle: Row) {
    let buf = Bytes::from(oracle.to_pbuf());
    println!("{:?}", buf);
    let trial = Row::from_pbuf(buf).unwrap();
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn pbufserde_row_vec(oracle: Vec<Row>) {
    let buf = Bytes::from(oracle.to_pbuf());
    println!("{:?}", buf);
    let trial = Vec::<Row>::from_pbuf(buf).unwrap();
    assert_eq!(oracle, trial);
}

