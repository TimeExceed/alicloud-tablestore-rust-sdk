#[cfg(test)] use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name(String);

impl Name {
    pub fn new<T: ToString>(name: T) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for Name {
    fn from(name: String) -> Name {
        Name(name)
    }
}

impl From<Name> for String {
    fn from(x: Name) -> String {
        x.0
    }
}

impl<'a> From<&'a Name> for &'a str {
    fn from(x: &'a Name) -> &'a str {
        &x.0
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl Arbitrary for Name {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        const ALPHABET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', '.'];
        const STOPPER: char = '.';
        let mut res = String::new();
        loop {
            let x = random_pick(g, &ALPHABET);
            if x == STOPPER {
                break;
            }
            res.push(x);
        }
        res.into()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let res = self.0.shrink()
            .map(|x| {
                x.into()
            });
        Box::new(res)
    }
}

#[cfg(test)]
fn random_pick<G: Gen, T: Clone>(g: &mut G, xs: &[T]) -> T {
    let i = (g.next_u32() as usize) % xs.len();
    xs[i].clone()
}
