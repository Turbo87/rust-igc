extern crate encoding;

#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate strum;
#[macro_use] extern crate strum_macros;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
#[macro_use]
extern crate proptest;

mod records;
mod error;
pub mod utils;

pub use self::records::*;
pub use self::error::{Error, Result};
pub use self::utils::time::{Time, Date};
