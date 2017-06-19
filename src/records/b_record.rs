use chrono::NaiveTime;

use super::super::coordinate::{parse_coordinate, Point};
use super::super::time::parse_time;
use super::super::ParseError;

pub struct BRecord {
    pub time: NaiveTime,
    pub location: Point,
    pub valid: bool,
    pub pressure_altitude: Option<i32>,
    pub gnss_altitude: Option<i32>,
    pub extra: Vec<u8>,
}

impl BRecord {
    pub fn parse(input: &[u8]) -> Result<Self, ParseError> {
        debug_assert!(input[0] == b'B');

        let len = input.len();
        if len < 35 {
            return Err(ParseError::LineTooShort);
        }

        let _time = parse_time(&input[1..7])?;
        let _coordinate = parse_coordinate(&input[7..24])?;
        let _valid = parse_validity(input[24])?;
        let _pressure_altitude = parse_altitude(&input[25..30])?;
        let _gnss_altitude = parse_altitude(&input[30..35])?;
        let _extra = input[35..].to_vec();

        Ok(BRecord {
            time: _time,
            location: _coordinate,
            valid: _valid,
            pressure_altitude: _pressure_altitude,
            gnss_altitude: _gnss_altitude,
            extra: _extra,
        })
    }
}

fn parse_validity(input: u8) -> Result<bool, ParseError> {
    match input {
        b'A' => Ok(true),
        b'V' => Ok(false),
        _ => Err(ParseError::InvalidValidity(input))
    }
}

fn parse_altitude(input: &[u8]) -> Result<Option<i32>, ParseError> {
    debug_assert!(input.len() == 5);

    return if input == b"00000" {
        Ok(None)
    } else {
        Ok(Some(String::from_utf8(input.to_vec())?.parse::<i32>()?))
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Deg;
    use chrono::NaiveTime;
    use super::{BRecord, parse_altitude, Point};

    #[test]
    fn test_b_record() {
        let record = BRecord::parse(b"B1414065016925N00953112EA021640228700309").unwrap();
        assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
        assert_eq!(record.location, Point::new(Deg(9.8852), Deg(50.28208333333333)));
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
