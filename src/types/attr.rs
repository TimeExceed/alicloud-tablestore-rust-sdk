use bytes::Bytes;
use super::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen, empty_shrinker};

#[derive(Debug, Clone)]
pub enum AttrValue {
    Str(String),
    Int(i64),
    Blob(Bytes),
    Bool(bool),
    Float(f64),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    pub name: Name,
    pub value: AttrValue,
    pub timestamp: AttrTimestamp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttrTimestamp {
    ServerAttach,
    ClientAttach(DateTime),
}

impl PartialEq for AttrValue {
    fn eq(&self, other: &AttrValue) -> bool {
        match self {
            AttrValue::Str(x) => match other {
                AttrValue::Str(y) => x == y,
                _ => false,
            }
            AttrValue::Int(x) => match other {
                AttrValue::Int(y) => x == y,
                _ => false,
            }
            AttrValue::Blob(x) => match other {
                AttrValue::Blob(y) => x == y,
                _ => false,
            }
            AttrValue::Bool(x) => match other {
                AttrValue::Bool(y) => x == y,
                _ => false,
            }
            AttrValue::Float(x) => match other {
                AttrValue::Float(y) => x.to_be_bytes() == y.to_be_bytes(),
                _ => false,
            }
        }
    }
}

impl Eq for AttrValue {}

impl From<RowKeyValue> for AttrValue {
    fn from(x: RowKeyValue) -> Self {
        match x {
            RowKeyValue::Blob(x) => AttrValue::Blob(x),
            RowKeyValue::Int(x) => AttrValue::Int(x),
            RowKeyValue::Str(x) => AttrValue::Str(x),
        }
    }
}

#[cfg(test)]
impl Arbitrary for AttrValue {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        match g.next_u32() % 5 {
            0 => AttrValue::Int(i64::arbitrary(g)),
            1 => AttrValue::Str(String::arbitrary(g)),
            2 => AttrValue::Blob(Bytes::from(Vec::<u8>::arbitrary(g))),
            3 => AttrValue::Bool(bool::arbitrary(g)),
            4 => AttrValue::Float(f64::arbitrary(g)),
            _ => unimplemented!()
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            AttrValue::Int(x) => {
                let res = x.shrink()
                    .map(|x| {
                        AttrValue::Int(x)
                    });
                Box::new(res)
            }
            AttrValue::Str(x) => {
                let res = x.shrink()
                    .map(|x| {
                        AttrValue::Str(x)
                    });
                Box::new(res)
            }
            AttrValue::Blob(x) => {
                let res = x.to_vec().shrink()
                    .map(|x| {
                        AttrValue::Blob(Bytes::from(x))
                    });
                Box::new(res)
            }
            _ => empty_shrinker(),
        }
    }
}

impl From<Option<DateTime>> for AttrTimestamp {
    fn from(x: Option<DateTime>) -> Self {
        match x {
            Some(x) => AttrTimestamp::ClientAttach(x),
            None => AttrTimestamp::ServerAttach,
        }
    }
}

#[cfg(test)]
impl Arbitrary for Attribute {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let has_tm = bool::arbitrary(g);
        let timestamp = if has_tm {
            AttrTimestamp::ClientAttach(DateTime::now())
        } else {
            AttrTimestamp::ServerAttach
        };
        Attribute{
            name: Name::arbitrary(g),
            value: AttrValue::arbitrary(g),
            timestamp,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let me = self.clone();
        let name_iter = self.name.shrink()
            .map(move |x| {
                let mut me = me.clone();
                me.name = x.clone();
                me
            });
        let me = self.clone();
        let value_iter = self.value.shrink()
            .map(move |x| {
                let mut me = me.clone();
                me.value = x.clone();
                me
            });
        Box::new(name_iter.chain(value_iter))
    }
}
