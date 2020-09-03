use crate::protocol as pb;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RowExistenceExpectation {
    Ignore,
    ExpectExist,
    ExpectNotExist,
}

impl From<pb::RowExistenceExpectation> for RowExistenceExpectation {
    fn from(x: pb::RowExistenceExpectation) -> Self {
        match x {
            pb::RowExistenceExpectation::IGNORE => RowExistenceExpectation::Ignore,
            pb::RowExistenceExpectation::EXPECT_EXIST => RowExistenceExpectation::ExpectExist,
            pb::RowExistenceExpectation::EXPECT_NOT_EXIST => RowExistenceExpectation::ExpectNotExist,
        }
    }
}

impl From<RowExistenceExpectation> for pb::RowExistenceExpectation {
    fn from(x: RowExistenceExpectation) -> Self {
        match x {
            RowExistenceExpectation::Ignore => pb::RowExistenceExpectation::IGNORE,
            RowExistenceExpectation::ExpectExist => pb::RowExistenceExpectation::EXPECT_EXIST,
            RowExistenceExpectation::ExpectNotExist => pb::RowExistenceExpectation::EXPECT_NOT_EXIST,
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Condition {
    pub row_exist: RowExistenceExpectation,
}

impl Condition {
    pub fn new(row_exist: RowExistenceExpectation) -> Self {
        Self{
            row_exist,
        }
    }
}

impl From<Condition> for pb::Condition {
    fn from(x: Condition) -> Self {
        Self{
            row_existence: x.row_exist.into(),
            column_condition: None,
        }
    }
}

impl From<pb::Condition> for Condition {
    fn from(x: pb::Condition) -> Self {
        if x.column_condition.is_some() {
            warn!("Column conditions are not supported yet.");
        }
        Self{
            row_exist: x.row_existence.into(),
        }
    }
}
