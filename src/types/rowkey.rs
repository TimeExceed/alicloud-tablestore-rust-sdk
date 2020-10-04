use bytes::Bytes;
use crate::{Error, ErrorCode};
use std::convert::TryFrom;
use super::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen, empty_shrinker};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RowKey(pub Vec<RowKeyColumn>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtendedRowKey(pub Vec<ExtendedRowKeyColumn>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RowKeyColumn {
    pub name: Name,
    pub value: RowKeyValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtendedRowKeyColumn {
    pub name: Name,
    pub value: ExtendedRowKeyValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RowKeyValue {
    Int(i64),
    Str(String),
    Blob(Bytes),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtendedRowKeyValue {
    Int(i64),
    Str(String),
    Blob(Bytes),
    InfMin,
    InfMax,
    AutoIncr,
}

impl RowKey {
    pub fn new(keys: Vec<RowKeyColumn>) -> Self {
        Self(keys)
    }

    pub fn iter(&self) -> impl Iterator<Item=&RowKeyColumn> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=RowKeyColumn> {
        self.0.into_iter()
    }
}

impl ExtendedRowKey {
    pub fn new(keys: Vec<ExtendedRowKeyColumn>) -> Self {
        Self(keys)
    }

    pub fn iter(&self) -> impl Iterator<Item=&ExtendedRowKeyColumn> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=ExtendedRowKeyColumn> {
        self.0.into_iter()
    }
}

impl From<RowKey> for ExtendedRowKey {
    fn from(rk: RowKey) -> Self {
        let ext_rk = rk.into_iter()
            .map(|x| {
                ExtendedRowKeyColumn::from(x)
            })
            .collect();
        ExtendedRowKey::new(ext_rk)
    }
}

impl TryFrom<ExtendedRowKey> for RowKey {
    type Error = Error;

    fn try_from(ext_rk: ExtendedRowKey) -> Result<RowKey, Error> {
        let mut xs = vec![];
        let r = ext_rk.into_iter()
            .try_for_each(|x| {
                match RowKeyColumn::try_from(x) {
                    Ok(x) => {
                        xs.push(x);
                        Ok(())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            });
        match r {
            Ok(_) => Ok(RowKey::new(xs)),
            Err(e) => Err(e),
        }
    }
}

impl From<RowKeyColumn> for ExtendedRowKeyColumn {
    fn from(x: RowKeyColumn) -> Self {
        ExtendedRowKeyColumn{
            name: x.name,
            value: ExtendedRowKeyValue::from(x.value),
        }
    }
}

impl TryFrom<ExtendedRowKeyColumn> for RowKeyColumn {
    type Error = Error;

    fn try_from(x: ExtendedRowKeyColumn) -> Result<Self, Error> {
        Ok(RowKeyColumn{
            name: x.name,
            value: RowKeyValue::try_from(x.value)?,
        })
    }
}

impl From<RowKeyValue> for ExtendedRowKeyValue {
    fn from(x: RowKeyValue) -> Self {
        match x {
            RowKeyValue::Int(x) => ExtendedRowKeyValue::Int(x),
            RowKeyValue::Str(x) => ExtendedRowKeyValue::Str(x),
            RowKeyValue::Blob(x) => ExtendedRowKeyValue::Blob(x),
        }
    }
}

impl TryFrom<ExtendedRowKeyValue> for RowKeyValue {
    type Error = Error;

    fn try_from(value: ExtendedRowKeyValue) -> Result<Self, Self::Error> {
        let msg = "Cannot convert InfMin/InfMax/AutoIncr to PrimaryKeyValue";
        match value {
            ExtendedRowKeyValue::Int(x) => Ok(RowKeyValue::Int(x)),
            ExtendedRowKeyValue::Str(x) => Ok(RowKeyValue::Str(x)),
            ExtendedRowKeyValue::Blob(x) => Ok(RowKeyValue::Blob(x)),
            _ => Err(Error{
                code: ErrorCode::ClientUnknown,
                message: msg.to_string(),
            })
        }
    }
}

#[cfg(test)]
impl Arbitrary for RowKeyColumn {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        RowKeyColumn{
            name: Name::arbitrary(g),
            value: RowKeyValue::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let name = self.name.clone();
        let value = self.value.clone();
        let name_shrinker = self.name
            .shrink()
            .map(move |x| {
                RowKeyColumn{
                    name: x,
                    value: value.clone(),
                }
            });
        let value_shrinker = self.value
            .shrink()
            .map(move |x| {
                RowKeyColumn{
                    name: name.clone(),
                    value: x,
                }
            });
        Box::new(name_shrinker.chain(value_shrinker))
    }
}

#[cfg(test)]
impl Arbitrary for ExtendedRowKeyColumn {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        ExtendedRowKeyColumn{
            name: Name::arbitrary(g),
            value: ExtendedRowKeyValue::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let name = self.name.clone();
        let value = self.value.clone();
        let name_shrinker = self.name
            .shrink()
            .map(move |x| {
                ExtendedRowKeyColumn{
                    name: x,
                    value: value.clone(),
                }
            });
        let value_shrinker = self.value
            .shrink()
            .map(move |x| {
                ExtendedRowKeyColumn{
                    name: name.clone(),
                    value: x,
                }
            });
        Box::new(name_shrinker.chain(value_shrinker))
    }
}

#[cfg(test)]
impl Arbitrary for RowKeyValue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        loop {
            let res = ExtendedRowKeyValue::arbitrary(g);
            match RowKeyValue::try_from(res) {
                Ok(x) => {
                    return x;
                }
                Err(_) => {}
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let res = ExtendedRowKeyValue::from(self.clone())
            .shrink()
            .map(|x| {
                RowKeyValue::try_from(x.clone()).unwrap()
            });
        Box::new(res)
    }
}

#[cfg(test)]
impl Arbitrary for ExtendedRowKeyValue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        match g.next_u32() % 6 {
            0 => ExtendedRowKeyValue::Int(i64::arbitrary(g)),
            1 => ExtendedRowKeyValue::Str(String::arbitrary(g)),
            2 => ExtendedRowKeyValue::Blob(Bytes::from(Vec::<u8>::arbitrary(g))),
            3 => ExtendedRowKeyValue::InfMin,
            4 => ExtendedRowKeyValue::InfMax,
            5 => ExtendedRowKeyValue::AutoIncr,
            _ => unimplemented!()
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            ExtendedRowKeyValue::Int(x) => {
                let res = x.shrink()
                    .map(|x| {
                        ExtendedRowKeyValue::Int(x)
                    });
                Box::new(res)
            }
            ExtendedRowKeyValue::Str(x) => {
                let res = x.shrink()
                    .map(|x| {
                        ExtendedRowKeyValue::Str(x)
                    });
                Box::new(res)
            }
            ExtendedRowKeyValue::Blob(x) => {
                let res = x.to_vec().shrink()
                    .map(|x| {
                        ExtendedRowKeyValue::Blob(Bytes::from(x))
                    });
                Box::new(res)
            }
            _ => empty_shrinker(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for RowKey {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        loop {
            let ext = ExtendedRowKey::arbitrary(g);
            if let Ok(res) = RowKey::try_from(ext) {
                return res;
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let ext = ExtendedRowKey::from(self.clone());
        let res = ext.shrink()
            .map(|x| {
                RowKey::try_from(x.clone()).unwrap()
            });
        Box::new(res)
    }
}

#[cfg(test)]
impl Arbitrary for ExtendedRowKey {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut keys = vec![];
        loop {
            let go_on = bool::arbitrary(g);
            if !go_on {
                break;
            }
            let col = ExtendedRowKeyColumn::arbitrary(g);
            keys.push(col);
        }
        ExtendedRowKey::new(keys)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let higher = self.0.len();
        let mut lower = higher / 2;
        let xs_fn = move || {
            if lower >= higher {
                return None;
            }
            let mid = (lower + higher) / 2;
            lower = mid + 1;
            return Some(mid);
        };
        let for_cut_tail = self.clone();
        let xs = std::iter::from_fn(xs_fn)
            .map(move |x| {
                let orig: &[ExtendedRowKeyColumn] = for_cut_tail.0.as_slice();
                let mut res: Vec<ExtendedRowKeyColumn> = vec![];
                res.extend_from_slice(&orig[0..x]);
                ExtendedRowKey::new(res)
            });
        let mut res: Box<dyn Iterator<Item = Self>> = Box::new(xs);
        for i in 0..self.0.len() {
            let me = self.0.clone();
            let ys = self.0[i].shrink()
                .map(move |x| {
                    let mut me_too = me.clone();
                    me_too[i] = x.clone();
                    ExtendedRowKey::new(me_too)
                });
            res = Box::new(res.chain(ys))
        }
        res
    }
}
