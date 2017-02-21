#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate nom;

extern crate chrono;
extern crate geo;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod records;
mod parsers;
