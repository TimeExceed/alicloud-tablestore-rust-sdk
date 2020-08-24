#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod error;
pub use self::error::{Error, ErrorCode};

mod client;
pub use self::client::*;

mod credential;
pub use self::credential::*;

mod endpoint;
pub use self::endpoint::*;

mod types;
pub use self::types::*;

mod protocol;
mod client_impl;

mod client_options;
pub use client_options::*;