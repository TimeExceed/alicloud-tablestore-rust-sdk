
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SettableValue<V>{
    Value(V),
    Default,
    NoChange,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

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
