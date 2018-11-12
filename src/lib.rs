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
extern crate galvanic_assert;

#[cfg(test)]
#[macro_use]
extern crate proptest;

mod records;
mod parsers;
mod error;
mod reader;
pub mod new_api;
pub mod utils;

pub use reader::Reader;
pub use error::*;
pub use records::*;
pub use parsers::time::Time;
