use super::*;

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Row {
    pub row_key: RowKey,
    pub attrs: Vec<Attribute>,
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
