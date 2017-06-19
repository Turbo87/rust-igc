extern crate cgmath;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate approx;

mod records;
mod coordinate;
mod error;
mod time;

pub use error::ParseError;
pub use records::*;
pub use coordinate::Point;

use std::io::BufRead;

/// Parse a single line of an IGC flight log file
///
/// *Currently only supports B records*
///
/// # Examples
///
/// ```
/// # extern crate cgmath;
/// # extern crate chrono;
/// # extern crate igc;
/// #
/// # use igc::{parse_line, Record, Point};
/// # use cgmath::Deg;
/// # use chrono::NaiveTime;
/// #
/// # fn main() {
/// let record = parse_line("B1414065016925N00953112EA021640228700309").unwrap();
/// match record {
///     Record::B(record) => {
///         assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
///         assert_eq!(record.location, Point::new(Deg(9.8852), Deg(50.28208333333333)));
///         assert_eq!(record.valid, true);
///         assert_eq!(record.pressure_altitude, Some(2164));
///         assert_eq!(record.gnss_altitude, Some(2287));
///         assert_eq!(String::from_utf8(record.extra).unwrap(), "00309");
///     },
///     _ => panic!("Unknown record")
/// }
/// # }
/// ```
pub fn parse_line(line: &str) -> Result<Record, ParseError> {
    Record::parse(line.as_bytes())
}

pub fn parse<R: BufRead>(reader: R) -> RecordsIter<R> {
    RecordsIter { reader: reader }
}
