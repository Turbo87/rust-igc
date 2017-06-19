#[macro_use]
extern crate nom;

extern crate cgmath;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod parsers;

use nom::IResult;

pub enum Record {
    B(parsers::b_record::BRecord)
}

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
/// # use igc::{parse_line, Record};
/// # use igc::parsers::coordinate::Point;
/// # use cgmath::Deg;
/// # use chrono::NaiveTime;
/// #
/// # fn main() {
/// let record = parse_line("B1414065016925N00953112EA021640228700309\r\n").unwrap();
/// match record {
///     Record::B(record) => {
///         assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
///         assert_eq!(record.location, Point::new(Deg(9.8852), Deg(50.28208333333333)));
///         assert_eq!(record.valid, true);
///         assert_eq!(record.pressure_altitude, Some(2164));
///         assert_eq!(record.gnss_altitude, Some(2287));
///         assert_eq!(record.extra, "00309");
///     },
///     _ => panic!("Unknown record")
/// }
/// # }
/// ```
pub fn parse_line(line: &str) -> Result<Record, ()> {
    match parsers::b_record::b_record(line.as_bytes()) {
        IResult::Done(_, b_record) => Ok(Record::B(b_record)),
        _ => Err(())
    }
}
