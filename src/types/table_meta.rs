use crate::protocol;
use std::convert::From;
use super::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TableMeta {
    pub name: Name,
    pub schema: Vec<PkeyColumnSchema>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PkeyColumnSchema {
    pub name: Name,
    pub type_: PkeyValueType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PkeyValueType {
    Int(PkeyIntTypeOption),
    Str,
    Blob,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PkeyIntTypeOption {
    pub auto_increment: bool,
}

impl From<protocol::TableMeta> for TableMeta {
    fn from(x: protocol::TableMeta) -> Self {
        TableMeta{
            name: x.table_name.into(),
            schema: x.primary_key.iter()
                .map(|x| {
                    PkeyColumnSchema::from(x.clone())
                })
                .collect(),
        }
    }
}

impl From<TableMeta> for protocol::TableMeta {
    fn from(x: TableMeta) -> protocol::TableMeta {
        protocol::TableMeta{
            table_name: x.name.into(),
            primary_key: x.schema.iter()
                .map(|x| {
                    protocol::PrimaryKeySchema::from(x.clone())
                })
                .collect(),
        }
    }
}

impl From<PkeyColumnSchema> for protocol::PrimaryKeySchema {
    fn from(x: PkeyColumnSchema) -> Self {
        let mut res = protocol::PrimaryKeySchema{
            name: x.name.into(),
            type_pb: protocol::PrimaryKeyType::INTEGER,
            option: None,
        };
        match x.type_ {
            PkeyValueType::Int(opts) => {
                res.type_pb = protocol::PrimaryKeyType::INTEGER;
                if opts.auto_increment {
                    res.option = Some(protocol::PrimaryKeyOption::AUTO_INCREMENT);
                }
            }
            PkeyValueType::Str => {
                res.type_pb = protocol::PrimaryKeyType::STRING;
            }
            PkeyValueType::Blob => {
                res.type_pb = protocol::PrimaryKeyType::BINARY;
            }
        };
        res
    }
}

impl From<protocol::PrimaryKeySchema> for PkeyColumnSchema {
    fn from(x: protocol::PrimaryKeySchema) -> Self {
        let mut res = PkeyColumnSchema{
            name: x.name.into(),
            type_: PkeyValueType::Blob,
        };
        match x.type_pb {
            protocol::PrimaryKeyType::INTEGER => {
                let mut opts = PkeyIntTypeOption{
                    auto_increment: false,
                };
                if let Some(o) = x.option {
                    if o == protocol::PrimaryKeyOption::AUTO_INCREMENT {
                        opts.auto_increment = true;
                    }
                }
                res.type_ = PkeyValueType::Int(opts);
            }
            protocol::PrimaryKeyType::STRING => {
                res.type_ = PkeyValueType::Str;
            }
            protocol::PrimaryKeyType::BINARY => {
                res.type_ = PkeyValueType::Blob;
            }
        };
        res
    }
}


#[cfg(test)]
impl Arbitrary for TableMeta {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self{
            name: Name::arbitrary(g),
            schema: Vec::<PkeyColumnSchema>::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let name = self.name.clone();
        let schema = self.schema.clone();
        let change_name = self.name.shrink()
            .map(move |x| {
                TableMeta{
                    name: x,
                    schema: schema.clone(),
                }
            });
        let change_schema = self.schema.shrink()
            .map(move |x| {
                TableMeta{
                    name: name.clone(),
                    schema: x,
                }
            });
        Box::new(change_name.chain(change_schema))
    }
}

#[cfg(test)]
impl Arbitrary for PkeyColumnSchema {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self{
            name: Name::arbitrary(g),
            type_: PkeyValueType::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let type_ = self.type_.clone();
        let change_name = self.name.shrink()
            .map(move |x| {
                PkeyColumnSchema{
                    name: x,
                    type_: type_.clone(),
                }
            });
        Box::new(change_name)
    }
}

#[cfg(test)]
impl Arbitrary for PkeyValueType {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let p = u8::arbitrary(g);
        match p % 3 {
            0 => PkeyValueType::Int(PkeyIntTypeOption::arbitrary(g)),
            1 => PkeyValueType::Str,
            2 => PkeyValueType::Blob,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
impl Arbitrary for PkeyIntTypeOption {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self{
            auto_increment: bool::arbitrary(g),
        }
    }
}

#[cfg(test)]
mod ut {
    use crate::protocol;
    use super::*;

    #[quickcheck]
    fn tablemeta_serde_is_identity_0(oracle: TableMeta) -> bool {
        println!("oracle: {:?}", oracle);
        let middle = protocol::TableMeta::from(oracle.clone());
        println!("middle: {:?}", middle);
        let trial = TableMeta::from(middle);
        println!("trial {:?}", trial);
        oracle == trial
    }
}
