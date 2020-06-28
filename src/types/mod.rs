#[derive(Debug, Copy, Clone)]
pub(crate) struct Api {
    pub path: &'static str,
}

impl Api {
    pub(crate) fn new(path: &'static str) -> Api {
        Api{
            path,
        }
    }
}

mod list_table;
pub use self::list_table::*;
