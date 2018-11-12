use regex::bytes::Regex;

use ::{Error, Result, Time};
use ::utils::num::{buf_to_int, buf_to_uint};
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
                (\d{2})(\d{2})(\d{2})  # UTC time
                (\d{2})(\d{5})([NS])   # latitude
                (\d{3})(\d{5})([EW])   # longitude
                ([AV])                 # validity
                (\d{5}|-\d{4})         # gps altitude
                (\d{5}|-\d{4})         # pressure altitude
                (.*)                   # additions
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let hour = buf_to_uint(&cap[1]);
        let minute = buf_to_uint(&cap[2]);
        let second = buf_to_uint(&cap[3]);
        let time = Time::from_hms(hour, minute, second);

        let abs_latitude = buf_to_uint::<f64>(&cap[4]) + buf_to_uint::<f64>(&cap[5]) / 60000.;
        let latitude = if &cap[6] == b"S" { -abs_latitude } else { abs_latitude };

        let abs_longitude = buf_to_uint::<f64>(&cap[7]) + buf_to_uint::<f64>(&cap[8]) / 60000.;
        let longitude = if &cap[9] == b"W" { -abs_longitude } else { abs_longitude };

        let is_valid = &cap[10] == b"A";
        let altitude_pressure = buf_to_int::<i16>(&cap[11]);
        let altitude_gps = buf_to_int::<i16>(&cap[12]);

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

    /// Fix accuracy in metres
    pub fn fix_accuracy(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::FXA)
    }

    /// Environmental Noise Level
    pub fn enl(&self) -> Option<u16> {
        self.get_three_digit_addition(&AdditionCode::ENL)
    }

    /// Heading True
    pub fn heading(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDT)?;
        if value < 360 { Some(value) } else { None }
    }

    /// Heading Magnetic
    pub fn heading_magnetic(&self) -> Option<u16> {
        let value = self.get_three_digit_addition(&AdditionCode::HDM)?;
        if value < 360 { Some(value) } else { None }
    }

    /// Latitude of the fix using the `latitude` field and the `LAD` addition if
    /// it exists.
    pub fn latitude(&self) -> f64 {
        let addition = match self.latitude_addition() {
            None => return self.latitude,
            Some(addition) => addition,
        };

        if self.latitude.is_sign_negative() {
            self.latitude - addition / 60000.
        } else {
            self.latitude + addition / 60000.
        }
    }

    fn latitude_addition(&self) -> Option<f64> {
        let bytes = self.additions.get(&AdditionCode::LAD)?;
        if !bytes.iter().all(u8::is_ascii_digit) { return None }
        let value: f64 = buf_to_uint(bytes);
        Some(value / f64::from(10).powi(bytes.len() as i32))
    }

    /// Latitude of the fix using the `longitude` field and the `LOD` addition if
    /// it exists.
    pub fn longitude(&self) -> f64 {
        let addition = match self.longitude_addition() {
            None => return self.longitude,
            Some(addition) => addition,
        };

        if self.longitude.is_sign_negative() {
            self.longitude - addition / 60000.
        } else {
            self.longitude + addition / 60000.
        }
    }

    fn longitude_addition(&self) -> Option<f64> {
        let bytes = self.additions.get(&AdditionCode::LOD)?;
        if !bytes.iter().all(u8::is_ascii_digit) { return None }
        let value: f64 = buf_to_uint(bytes);
        Some(value / f64::from(10).powi(bytes.len() as i32))
    }

    fn get_three_digit_addition(&self, code: &AdditionCode) -> Option<u16> {
        let bytes = self.additions.get(code)?;
        if bytes.len() != 3 || !bytes.iter().all(u8::is_ascii_digit) { return None }
        Some(buf_to_uint(bytes))
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
        assert_eq!(record.latitude_addition(), None);
        assert_relative_eq!(record.longitude, 9.8852);
        assert_relative_eq!(record.longitude(), 9.8852);
        assert_eq!(record.longitude_addition(), None);
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
        assert_relative_eq!(record.latitude_addition().unwrap(), 0.12);
        assert_relative_eq!(record.latitude(), 50. + 16.925_12 / 60.);
        assert_relative_eq!(record.longitude_addition().unwrap(), 0.345);
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
