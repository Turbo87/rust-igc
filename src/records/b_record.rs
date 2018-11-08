use super::super::coordinate::{parse_latitude, parse_longitude};
use super::super::time::{parse_time, Time};
use super::super::{Result, ParseError};

#[derive(Debug)]
pub struct BRecord {
    pub time: Time,
    pub longitude: f64,
    pub latitude: f64,
    pub valid: bool,
    pub pressure_altitude: Option<i32>,
    pub gnss_altitude: Option<i32>,
    pub extra: Vec<u8>,
}

impl BRecord {
    pub(crate) fn parse(input: &str) -> Result<Self> {
        debug_assert_eq!(&input[0..1], "B");

        let len = input.len();
        if len < 35 {
            return Err(ParseError::LineTooShort);
        }

        let _time = parse_time(&input[1..7])?;
        let latitude = parse_latitude(&input[7..15].as_bytes())?;
        let longitude = parse_longitude(&input[15..24].as_bytes())?;
        let _valid = parse_validity(input[24..25].as_bytes()[0])?;
        let _pressure_altitude = parse_altitude(&input[25..30].as_bytes())?;
        let _gnss_altitude = parse_altitude(&input[30..35].as_bytes())?;
        let _extra = input[35..].as_bytes().to_vec();

        Ok(BRecord {
            time: _time,
            latitude,
            longitude,
            valid: _valid,
            pressure_altitude: _pressure_altitude,
            gnss_altitude: _gnss_altitude,
            extra: _extra,
        })
    }
}

fn parse_validity(input: u8) -> Result<bool> {
    match input {
        b'A' => Ok(true),
        b'V' => Ok(false),
        _ => Err(ParseError::InvalidValidity(input))
    }
}

fn parse_altitude(input: &[u8]) -> Result<Option<i32>> {
    debug_assert_eq!(input.len(), 5);

    Ok(if input == b"00000" {
        None
    } else {
        Some(String::from_utf8(input.to_vec())?.parse::<i32>()?)
    })
}

#[cfg(test)]
mod tests {
    use super::{BRecord, parse_altitude, Time};

    #[test]
    fn test_b_record() {
        let record = BRecord::parse("B1414065016925N00953112EA021640228700309").unwrap();
        assert_eq!(record.time, Time::from_hms(14, 14, 06));
        assert_relative_eq!(record.longitude, 9.8852);
        assert_relative_eq!(record.latitude, 50.28208333333333);
        assert_eq!(record.valid, true);
        assert_eq!(record.pressure_altitude, Some(2164));
        assert_eq!(record.gnss_altitude, Some(2287));
        assert_eq!(String::from_utf8(record.extra).unwrap(), "00309");
    }

    #[test]
    fn test_altitude() {
        assert!(parse_altitude(b"abcde").is_err());
        assert!(parse_altitude(b"--000").is_err());
        assert_eq!(parse_altitude(b"00000").unwrap(), None);
        assert_eq!(parse_altitude(b"00001").unwrap(), Some(1));
        assert_eq!(parse_altitude(b"-0001").unwrap(), Some(-1));
        assert_eq!(parse_altitude(b"-0000").unwrap(), Some(0));
        assert_eq!(parse_altitude(b"01234").unwrap(), Some(1234));
        assert_eq!(parse_altitude(b"99999").unwrap(), Some(99999));
        assert_eq!(parse_altitude(b"-9999").unwrap(), Some(-9999));
    }
}
