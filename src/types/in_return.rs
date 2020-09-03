use crate::protocol as pb;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InReturn {
    Nothing,
    RowKey,
}

impl From<InReturn> for pb::ReturnType {
    fn from(x: InReturn) -> Self {
        match x {
            InReturn::Nothing => pb::ReturnType::RT_NONE,
            InReturn::RowKey => pb::ReturnType::RT_PK,
        }
    }
}

impl From<pb::ReturnType> for InReturn {
    fn from(x: pb::ReturnType) -> Self {
        match x {
            pb::ReturnType::RT_NONE => InReturn::Nothing,
            pb::ReturnType::RT_PK => InReturn::Nothing,
        }
    }
}

impl From<InReturn> for pb::ReturnContent {
    fn from(x: InReturn) -> Self {
        Self{
            return_type: Some(x.into())
        }
    }
}
