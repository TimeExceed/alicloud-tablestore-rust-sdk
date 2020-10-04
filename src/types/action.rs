use std::string::ToString;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    CreateTable,
    DeleteTable,
    ListTable,
    PutRow,
    GetRange,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::CreateTable => "/CreateTable".to_string(),
            Action::DeleteTable => "/DeleteTable".to_string(),
            Action::ListTable => "/ListTable".to_string(),
            Action::PutRow => "/PutRow".to_string(),
            Action::GetRange => "/GetRange".to_string(),
        }
    }
}
