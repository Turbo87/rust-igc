#[macro_use]
extern crate nom;

extern crate cgmath;
extern crate chrono;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod parsers;
mod error;

pub use error::ParseError;
use parsers::b_record::BRecord;

/// IGC file record type representing a single line
pub enum Record {
    /// FR manufacturer and FR serial no.
    A,

    /// Fix
    B(parsers::b_record::BRecord),

    /// Task/declaration
    C,

    /// Differential GPS
    D,

    /// Event
    E,

    /// Satellite constellation
    F,

    /// Security
    G,

    /// File header
    H,

    /// List of additional data included at end of each B-record
    I,

    /// List of additional data included at end of each K-record
    J,

    /// Frequent data, additional to the B-record
    K,

    /// Logbook/comments
    L,
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
    parse_line_from_bytes(line.as_bytes())
}

fn parse_line_from_bytes(bytes: &[u8]) -> Result<Record, ParseError> {
    match bytes[0] {
        b'A' => Ok(Record::A),
        b'B' => parse_b_record(bytes).map(Record::B),
        b'C' => Ok(Record::C),
        b'D' => Ok(Record::D),
        b'E' => Ok(Record::E),
        b'F' => Ok(Record::F),
        b'G' => Ok(Record::G),
        b'H' => Ok(Record::H),
        b'I' => Ok(Record::I),
        b'J' => Ok(Record::J),
        b'K' => Ok(Record::K),
        b'L' => Ok(Record::L),
        _ => Err(ParseError::UnknownRecordType(bytes[0])),
    }
}

fn parse_b_record(bytes: &[u8]) -> Result<BRecord, ParseError> {
    parsers::b_record::b_record(bytes)
}