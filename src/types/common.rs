#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SettableValue<V>{
    Value(V),
    Default,
    NoChange,
}

