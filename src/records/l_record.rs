use std::str::{FromStr, from_utf8_unchecked};

use regex::bytes::Regex;

use ::{Error, Result};
use ::utils::text::as_text;

#[derive(Debug, PartialEq, Eq, EnumString)]
pub enum LRecordSource {
    /// Pilot input
    PLT,
    /// After flight pilot input
    PFC,
    /// Official observer input
    OOI,
    /// Other input (e.g. manufacturer)
    #[strum(default="true")]
    Other(String),
}

impl LRecordSource {
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> LRecordSource {
        debug_assert!(bytes.is_ascii());
        let code = from_utf8_unchecked(bytes);
        LRecordSource::from_str(code).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LRecord {
    pub source: LRecordSource,
    pub text: String,
}

impl LRecord {
    pub fn parse(line: &[u8]) -> Result<LRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^L
                ([A-Z0-9]{3})     # source
                (.+)              # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let source = unsafe { LRecordSource::from_bytes_unchecked(&cap[1]) };
        let text = as_text(&cap[2]).ok_or_else(|| Error::invalid_record(line))?;

        Ok(LRecord { source, text })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let record = LRecord::parse(b"LPLTThis flight was my second 1000km attempt").unwrap();
        assert_eq!(record.source, LRecordSource::PLT);
        assert_eq!(record.text, "This flight was my second 1000km attempt");

        let record = LRecord::parse(b"LOOIfrom Eagle Field").unwrap();
        assert_eq!(record.source, LRecordSource::OOI);
        assert_eq!(record.text, "from Eagle Field");

        assert!(LRecord::parse(b"LOOI").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            LRecord::parse(s.as_bytes());
        }
    }
}
