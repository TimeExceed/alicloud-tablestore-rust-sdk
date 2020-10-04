use crate::{Error, ErrorCode};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SettableValue<V>{
    Value(V),
    Default,
    NoChange,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl Name {
    pub fn new(name: String) -> Result<Name, Error> {
        if name.is_empty() {
            return Err(Error{
                code: ErrorCode::ClientUnknown,
                message: "Name is required not to be empty.".to_string(),
            })
        }
        Ok(Name(name))
    }
}

impl From<String> for Name {
    fn from(x: String) -> Name {
        Name(x)
    }
}

impl From<Name> for String {
    fn from(x: Name) -> String {
        x.0
    }
}

impl<'a> From<&'a Name> for &'a String {
    fn from(x: &'a Name) -> &'a String {
        &x.0
    }
}

#[cfg(test)]
impl Arbitrary for Name {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        const ALPHABET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', '.'];
        const STOPPER: char = '.';
        loop {
            let mut res = String::new();
            loop {
                let x = random_pick(g, &ALPHABET);
                if x == STOPPER {
                    break;
                }
                res.push(x);
            }
            match Name::new(res) {
                Ok(x) => {
                    return x;
                }
                Err(_) => {}
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(NameShrinker::new(self.clone()))
    }
}

#[cfg(test)]
fn random_pick<G: Gen, T: Clone>(g: &mut G, xs: &[T]) -> T {
    let i = (g.next_u32() as usize) % xs.len();
    xs[i].clone()
}

#[cfg(test)]
struct NameShrinker{
    name: Vec<char>,
    low: usize,
    high: usize,
    mid: Option<usize>,
}

#[cfg(test)]
impl NameShrinker {
    fn new(name: Name) -> Self {
        let name: Vec<char> = name.0.chars().collect();
        let high = name.len();
        Self{
            name,
            low: 0,
            high,
            mid: None,
        }
    }
}

#[cfg(test)]
impl Iterator for NameShrinker {
    type Item = Name;

    fn next(&mut self) -> Option<Name> {
        match self.mid {
            None => {
                if self.low == self.high {
                    return None;
                }
                if self.high == 1 {
                    return None;
                }
                let mid = (self.low + self.high) / 2;
                self.mid = Some(mid);
                let res: String = (&self.name).split_at(mid).0.iter().collect();
                Some(Name(res))
            }
            Some(mid) => {
                self.low = mid + 1;
                if self.low == self.high {
                    return None;
                }
                let mid = (self.low + self.high) / 2;
                self.mid = Some(mid);
                let res: String = (&self.name).split_at(mid).0.iter().collect();
                Some(Name(res))
            }
        }
    }
}

impl DateTime {
    pub fn now() -> DateTime {
        let tm = chrono::Utc::now();
        let millis = tm.timestamp_millis();
        let secs = millis / 1000;
        let subsecs = (millis % 1000 * 1000_000) as u32;
        let tm = chrono::NaiveDateTime::from_timestamp(secs, subsecs);
        DateTime(chrono::DateTime::from_utc(tm, chrono::Utc))
    }

    pub fn to_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }

    pub fn from_millis(millis: i64) -> DateTime {
        let secs = millis / 1000;
        let nsecs = millis % 1000 * 1000_000;
        let nsecs = nsecs as u32;
        let tm = chrono::NaiveDateTime::from_timestamp(secs, nsecs);
        DateTime(chrono::DateTime::from_utc(tm, chrono::Utc))
    }
}
