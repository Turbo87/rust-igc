use std::str::from_utf8_unchecked;
use regex::bytes::Regex;

use ::utils::datetime::Time;
use ::{Error, Result};


#[derive(Debug, PartialEq, Eq)]
pub struct FRecord {
    pub time: Time,
    pub satellite_ids: Vec<String>,
}

impl FRecord {
    pub fn parse(line: &[u8]) -> Result<FRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^F
                (\d{6})               # UTC time
                ((?:[A-Z0-9]{2})*)    # satellite IDs
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let time = Time::parse_unchecked(&cap[1]);
        let satellite_ids: Vec<_> = cap[2]
            .chunks(2)
            .map(|bytes| unsafe { from_utf8_unchecked(bytes) }.into())
            .collect();

        Ok(FRecord { time, satellite_ids })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let record = FRecord::parse(b"F1234560102ABC342").unwrap();
        assert_eq!(record.time, Time::from_hms(12, 34, 56));
        assert_eq!(record.satellite_ids, vec!["01", "02", "AB", "C3", "42"]);

        let record = FRecord::parse(b"F123456").unwrap();
        assert_eq!(record.time, Time::from_hms(12, 34, 56));
        assert_eq!(record.satellite_ids, Vec::<String>::new());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            FRecord::parse(s.as_bytes());
        }
    }
}
