use regex::bytes::Regex;

use ::{Error, Result, Time};
use ::utils::coordinates::{parse_latitude_unchecked, parse_longitude_unchecked};
use ::utils::num::parse_int;
use ::utils::additions::*;

#[derive(Debug)]
pub struct BRecord {
    pub time: Time,
    pub latitude: f64,
    pub longitude: f64,
    pub is_valid: bool,
    pub altitude_pressure: i16,
    pub altitude_gps: i16,
    pub additions: AdditionsMap,
}

// B 13 05 10 52 40678 N 007 48278 W A 00779 00769 033011
impl BRecord {
    pub fn parse(line: &[u8]) -> Result<BRecord> {
        Self::parse_with_additions(line, &vec![])
    }

    pub fn parse_with_additions(line: &[u8], addition_defs: &Vec<AdditionDef>) -> Result<BRecord> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^B                     # record typ
                (\d{6})                # UTC time
                (\d{7}[NS])            # latitude
                (\d{8}[EW])            # longitude
                ([AV])                 # validity
                (\d{5}|-\d{4})         # gps altitude
                (\d{5}|-\d{4})         # pressure altitude
                (.*)                   # additions
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let time = Time::parse_unchecked(&cap[1]);
        let latitude = parse_latitude_unchecked(&cap[2]);
        let longitude = parse_longitude_unchecked(&cap[3]);
        let is_valid = &cap[4] == b"A";
        let altitude_pressure = parse_int(&cap[5]).unwrap();
        let altitude_gps = parse_int(&cap[6]).unwrap();

        let additions = addition_defs.parse(&line)?;

        Ok(BRecord {
            time,
            latitude,
            longitude,
            is_valid,
            altitude_gps,
            altitude_pressure,
            additions,
        })
    }

    /// Latitude of the fix using the `latitude` field and the `LAD` addition if
    /// it exists.
    pub fn latitude(&self) -> f64 {
        match self.additional_latitude_decimals() {
            None => return self.latitude,
            Some(value) => if self.latitude.is_sign_negative() {
                self.latitude - value
            } else {
                self.latitude + value
            },
        }
    }

    /// Latitude of the fix using the `longitude` field and the `LOD` addition if
    /// it exists.
    pub fn longitude(&self) -> f64 {
        match self.additional_longitude_decimals() {
            None => self.longitude,
            Some(value) => if self.longitude.is_sign_negative() {
                self.longitude - value
            } else {
                self.longitude + value
            },
        }
    }
}

impl AdditionSupport for BRecord {
    fn get_addition(&self, code: &AdditionCode) -> Option<&[u8]> {
        self.additions.get(code).map(Vec::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let record = BRecord::parse(b"B1414065016925N00953112EA021640228700309").unwrap();
        assert_eq!(record.time, Time::from_hms(14, 14, 06));
        assert_relative_eq!(record.latitude, 50.28208333333333);
        assert_relative_eq!(record.longitude, 9.8852);
        assert_eq!(record.is_valid, true);
        assert_eq!(record.altitude_pressure, 2164);
        assert_eq!(record.altitude_gps, 2287);
        assert_eq!(record.additions.len(), 0);
    }

    #[test]
    fn test_example_1_with_additions() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::FXA, 36, 38),
            AdditionDef::new(AdditionCode::SIU, 39, 40),
        ];
        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA021640228700309", &addition_defs).unwrap();
        assert_eq!(record.time, Time::from_hms(14, 14, 06));
        assert_relative_eq!(record.latitude, 50.28208333333333);
        assert_relative_eq!(record.latitude(), 50.28208333333333);
        assert_relative_eq!(record.longitude, 9.8852);
        assert_relative_eq!(record.longitude(), 9.8852);
        assert_eq!(record.is_valid, true);
        assert_eq!(record.altitude_pressure, 2164);
        assert_eq!(record.altitude_gps, 2287);
        assert_eq!(record.additions.len(), 2);
        assert_eq!(record.additions.get(&AdditionCode::FXA).unwrap(), b"003");
        assert_eq!(record.additions.get(&AdditionCode::SIU).unwrap(), b"09");
        assert_eq!(record.fix_accuracy(), Some(3));
    }

    #[test]
    fn test_lat_lon_precision() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::LAD, 36, 37),
            AdditionDef::new(AdditionCode::LOD, 38, 40),
        ];
        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA021640228712345", &addition_defs).unwrap();
        assert_relative_eq!(record.latitude(), 50. + 16.925_12 / 60.);
        assert_relative_eq!(record.longitude(), 9. + 53.112_345 / 60.);
    }

    #[test]
    fn test_enl() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::ENL, 36, 38),
        ];
        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA0216402287424", &addition_defs).unwrap();
        assert_eq!(record.enl(), Some(424));
    }

    #[test]
    fn test_non_standard_enl() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::ENL, 36, 39),
        ];
        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA02164022874244", &addition_defs).unwrap();
        assert_eq!(record.additions.get(&AdditionCode::ENL).unwrap(), b"4244");
        assert_eq!(record.enl(), None);
    }

    #[test]
    fn test_non_numeric_enl() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::ENL, 36, 38),
        ];
        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA02164022874a4", &addition_defs).unwrap();
        assert_eq!(record.enl(), None);
    }

    #[test]
    fn test_headings() {
        let addition_defs = vec![
            AdditionDef::new(AdditionCode::HDT, 36, 38),
            AdditionDef::new(AdditionCode::HDM, 39, 41),
        ];

        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA0216402287091097", &addition_defs).unwrap();
        assert_eq!(record.heading(), Some(91));
        assert_eq!(record.heading_magnetic(), Some(97));

        let record = BRecord::parse_with_additions(b"B1414065016925N00953112EA0216402287001360", &addition_defs).unwrap();
        assert_eq!(record.heading(), Some(1));
        assert_eq!(record.heading_magnetic(), None);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            BRecord::parse(s.as_bytes());
        }
    }
}
