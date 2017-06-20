use super::BRecord;
use super::super::ParseError;

/// IGC file record type representing a single line
#[derive(Debug)]
pub enum Record {
    /// FR manufacturer and FR serial no.
    A,

    /// Fix
    B(BRecord),

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

    /// Empty line
    Empty,
}

impl Record {
    pub fn parse(bytes: &[u8]) -> Result<Record, ParseError> {
        if bytes.is_empty() {
            return Ok(Record::Empty);
        }

        match bytes[0] {
            b'A' => Ok(Record::A),
            b'B' => BRecord::parse(bytes).map(Record::B),
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
}