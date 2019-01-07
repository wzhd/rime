#[macro_use]
extern crate failure;

mod cdef;
mod rimers;
#[cfg(feature = "network")]
pub mod server;

pub use self::rimers::{Commit, Context, RimeRs};
use serde_derive::{Deserialize, Serialize};
use std::os::raw::c_int;

/// Usually needed after each key press
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub commit: Option<Commit>,
    pub context: Context,
}

/// Keyboard input
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPress {
    pub key_code: c_int,
    pub mask: c_int,
}
