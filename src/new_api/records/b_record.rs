use regex::bytes::Regex;

use ::Time;
use ::utils::num::{buf_to_int, buf_to_uint};
use ::new_api::{Error, Result};

#[derive(Debug)]
pub struct BRecord {
    pub time: Time,
    pub latitude: f64,
    pub longitude: f64,
    pub is_valid: bool,
    pub altitude_pressure: i16,
    pub altitude_gps: i16,
    pub additions: Vec<u8>,
}

// B 13 05 10 52 40678 N 007 48278 W A 00779 00769 033011
impl BRecord {
    pub fn parse(line: &[u8]) -> Result<BRecord> {
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

        let additions = cap[13].to_vec();

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let record = BRecord::parse(b"B1414065016925N00953112EA021640228700309").unwrap();
        assert_eq!(record.time, Time::from_hms(14, 14, 06));
        assert_eq!(record.latitude, 50.28208333333333);
        assert_eq!(record.longitude, 9.8852);
        assert_eq!(record.is_valid, true);
        assert_eq!(record.altitude_pressure, 2164);
        assert_eq!(record.altitude_gps, 2287);
        assert_eq!(record.additions, b"00309");
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in r"\PC*") {
            BRecord::parse(s.as_bytes());
        }
    }
}
