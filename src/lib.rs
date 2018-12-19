#[macro_use]
extern crate failure;
#[macro_use]
extern crate derivative;

mod cdef;
mod rimers;

pub use self::rimers::{Commit, Context, RimeRs};
