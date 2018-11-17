use regex::bytes::Regex;

use ::{Error, Result};


#[derive(Debug, PartialEq, Eq)]
pub struct DRecord {
    pub gps_qualifier: GPSQualifier,
    pub dgps_station_id: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GPSQualifier {
    GPS,
    DGPS,
}

impl GPSQualifier {
    pub fn from_byte(byte: u8) -> Option<GPSQualifier> {
        debug_assert!(byte.is_ascii());
        match byte {
            b'1' => Some(GPSQualifier::GPS),
            b'2' => Some(GPSQualifier::DGPS),
            _ => None,
        }
    }
}

impl DRecord {
    pub fn parse(line: &[u8]) -> Result<DRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^D
                ([12])     # GPS qualifier
                (.{4})?    # DGPS station id
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let gps_qualifier = GPSQualifier::from_byte(cap[1][0]).unwrap();
        let dgps_station_id = cap.get(2).map(|it| it.as_bytes().to_vec());

        Ok(DRecord { gps_qualifier, dgps_station_id })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let record = DRecord::parse(b"D1").unwrap();
        assert_eq!(record.gps_qualifier, GPSQualifier::GPS);
        assert_eq!(record.dgps_station_id, None);

        let record = DRecord::parse(b"D2abc3").unwrap();
        assert_eq!(record.gps_qualifier, GPSQualifier::DGPS);
        assert_eq!(record.dgps_station_id, Some(b"abc3".to_vec()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            DRecord::parse(s.as_bytes());
        }
    }
}
