use bytes::{Bytes, BytesMut};
use crate::plainbuffer as pbuf;
use crate::types::*;
use pbuf::{Serde, SerdeWithCrc8};

#[quickcheck]
fn serde_u8(oracle: u8) {
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    let trial = u8::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn serde_u32(oracle: u32) {
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    let trial = u32::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn serde_u64(oracle: u64) {
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    let trial = u64::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn serde_blob(oracle: Vec<u8>) {
    let oracle = Bytes::from(oracle);
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    let trial = Bytes::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn serde_str(oracle: String) {
    let mut buf = BytesMut::new();
    oracle.serialize(&mut buf);
    let mut buf = Bytes::from(buf);
    let trial = String::deserialize(&mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
}

#[quickcheck]
fn serde_name(oracle: Name) {
    let mut buf = BytesMut::new();
    let mut oracle_chksum = 0u8;
    oracle.serialize_crc8(&mut buf, &mut oracle_chksum);
    let mut buf = Bytes::from(buf);
    let mut trial_chksum = 0u8;
    let trial = Name::deserialize_crc8(&mut buf, &mut trial_chksum).unwrap();
    assert!(buf.is_empty());
    assert_eq!(oracle, trial);
    assert_eq!(oracle_chksum, trial_chksum);
}

