extern crate encoding;

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

pub use reader::Reader;
pub use error::*;
pub use records::*;
pub use parsers::time::Time;
