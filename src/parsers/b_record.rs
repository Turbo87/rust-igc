use nom::IResult;
use chrono::NaiveTime;

use super::coordinate::{coordinate, Point};
use super::helpers::to_string;
use super::numbers::{up_to_99999, down_to_minus_9999};
use super::time::time;

pub struct BRecord {
    pub time: NaiveTime,
    pub location: Point,
    pub valid: bool,
    pub pressure_altitude: Option<i32>,
    pub gnss_altitude: Option<i32>,
    pub extra: Vec<u8>,
}

named!(validity <bool>, alt!(
    tag!("A") => { |_| true } |
    tag!("V") => { |_| false }
));

named!(altitude <Option<i32>>, alt!(
    tag!("00000") => { |_| None } |
    up_to_99999 => { |value| Some(value as i32) } |
    down_to_minus_9999 => { |value| Some(value as i32) }
));

pub fn b_record(input: &[u8]) -> Result<BRecord, ()> {
    debug_assert!(input[0] == b'B');

    let len = input.len();
    if len < 35 {
        return Err(());
    }

    let _time = time(&input[1..7]).unwrap().1;
    let _coordinate = coordinate(&input[7..24]).unwrap().1;
    let _valid = validity(&input[24..25]).unwrap().1;
    let _pressure_altitude = altitude(&input[25..30]).unwrap().1;
    let _gnss_altitude = altitude(&input[30..35]).unwrap().1;
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

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use cgmath::Deg;
    use chrono::NaiveTime;
    use super::{b_record, altitude, Point};

    #[test]
    fn test_b_record() {
        let record = b_record(b"B1414065016925N00953112EA021640228700309").unwrap();
        assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
        assert_eq!(record.location, Point::new(Deg(9.8852), Deg(50.28208333333333)));
        assert_eq!(record.valid, true);
        assert_eq!(record.pressure_altitude, Some(2164));
        assert_eq!(record.gnss_altitude, Some(2287));
        assert_eq!(String::from_utf8(record.extra).unwrap(), "00309");
    }

    #[test]
    fn test_altitude() {
        assert!(altitude(b"abcde").is_err());
        assert!(altitude(b"--000").is_err());
        assert_eq!(altitude(b"00000"), Done(&[][..], None));
        assert_eq!(altitude(b"00001"), Done(&[][..], Some(1)));
        assert_eq!(altitude(b"-0001"), Done(&[][..], Some(-1)));
        assert_eq!(altitude(b"-0000"), Done(&[][..], Some(0)));
        assert_eq!(altitude(b"01234"), Done(&[][..], Some(1234)));
        assert_eq!(altitude(b"99999"), Done(&[][..], Some(99999)));
        assert_eq!(altitude(b"-9999"), Done(&[][..], Some(-9999)));
    }
}
