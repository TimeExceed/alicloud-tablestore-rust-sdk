use chrono::*;

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

pub trait Response {
    fn set_server_timestamp(&mut self, tm: Option<DateTime<Utc>>) -> ();
    fn get_server_timestamp(&self) -> &Option<DateTime<Utc>>;
    fn set_request_id(&mut self, req_id: Option<String>) -> ();
    fn get_request_id(&self) -> &Option<String>;
}

#[derive(Debug, Clone)]
struct BaseResponse {
    pub server_timestamp: Option<DateTime<Utc>>,
    pub req_id: Option<String>,
}

impl Default for BaseResponse {
    fn default() -> BaseResponse {
        BaseResponse{
            server_timestamp: None,
            req_id: None,
        }
    }
}

mod common;
pub use self::common::*;
mod table_meta;
pub use self::table_meta::*;
mod table_options;
pub use self::table_options::*;
mod list_table;
pub use self::list_table::*;
