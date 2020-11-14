use super::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Row {
    pub row_key: RowKey,
    pub attrs: Vec<Attribute>,
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pkeys: Vec<_> = self.row_key.iter()
            .map(|x| {
                match &x.value {
                    RowKeyValue::Blob(x) => format!(r#"b"{:?}""#, x),
                    RowKeyValue::Int(x) => format!("{}", x),
                    RowKeyValue::Str(x) => format!("{}", x),
                }
            })
            .collect();
        let pkey = &pkeys.join("|");
        if self.attrs.is_empty() {
            return f.write_str(pkey);
        }
        let attrs: Vec<_> = self.attrs.iter()
            .map(|x| {
                let v = match &x.value {
                    AttrValue::Blob(x) => format!(r#"b"{:?}""#, x),
                    AttrValue::Int(x) => format!("{}", x),
                    AttrValue::Str(x) => format!("{}", x),
                    AttrValue::Bool(x) => if *x {"true".to_string()} else {"false".to_string()},
                    AttrValue::Float(x) => format!("{:?}", x),
                };
                format!("{}:{}", x.name, v)
            })
            .collect();
        let attrs = &attrs.join("|");
        let res = format!("{}=>{{{}}}", pkey, attrs);
        f.write_str(&res)
    }
}

#[cfg(test)]
impl Arbitrary for Row {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Row{
            row_key: RowKey::arbitrary(g),
            attrs: Vec::<Attribute>::arbitrary(g),
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let clone_attrs = self.attrs.clone();
        let res = self.row_key
            .shrink()
            .map(move |x| {
                Row{
                    row_key: x.clone(),
                    attrs: clone_attrs.clone(),
                }
            });
        let row_key = self.row_key.clone();
        let xs = self.attrs.shrink()
            .map(move |x| {
                Row{
                    row_key: row_key.clone(),
                    attrs: x.clone(),
                }
            });
        Box::new(res.chain(xs))
    }
}
