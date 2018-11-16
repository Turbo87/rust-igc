mod b_record;
mod c_record;
mod h_record;
mod i_record;
mod j_record;
mod k_record;

use ::{Error, Result};
pub use self::b_record::BRecord;
pub use self::c_record::{CRecordDeclaration, CRecordTurnpoint};
pub use self::h_record::HRecord;
pub use self::i_record::IRecord;
pub use self::j_record::JRecord;
pub use self::k_record::KRecord;

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
    I(IRecord),

    /// List of additional data included at end of each K-record
    J,

    /// Frequent data, additional to the B-record
    K,

    /// Logbook/comments
    L,
}

impl Record {
    pub fn parse(bytes: &[u8]) -> Result<Record> {
        if bytes.is_empty() {
            return Err(Error::invalid_record(bytes))
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
            b'I' => IRecord::parse(bytes).map(Record::I),
            b'J' => Ok(Record::J),
            b'K' => Ok(Record::K),
            b'L' => Ok(Record::L),
            _ => Err(Error::invalid_record(bytes)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            Record::parse(s.as_bytes());
        }
    }
}
