use cgmath::Deg;
use chrono::NaiveTime;
use geo::Point;

use super::coordinate::coordinate;
use super::helpers::to_string;
use super::numbers::{up_to_99999, down_to_minus_9999};
use super::time::time;

pub struct BRecord {
    pub time: NaiveTime,
    pub location: Point<Deg<f64>>,
    pub valid: bool,
    pub pressure_altitude: Option<i32>,
    pub gnss_altitude: Option<i32>,
    pub extra: String,
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

named!(pub b_record <BRecord>, do_parse!(
    tag!("B") >>
    time: time >>
    location: coordinate >>
    valid: validity >>
    pressure_altitude: altitude >>
    gnss_altitude: altitude >>
    extra: take_until!("\r\n") >>
    take!(2) >>
    (BRecord {
        time: time,
        location: location,
        valid: valid,
        pressure_altitude: pressure_altitude,
        gnss_altitude: gnss_altitude,
        extra: to_string(extra).to_string(),
    })
));

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use cgmath::Deg;
    use chrono::NaiveTime;
    use geo::Point;
    use super::{b_record, altitude};

    #[test]
    fn test_b_record() {
        assert!(b_record(b"C").is_err());

        let result = b_record(b"B1414065016925N00953112EA021640228700309\r\n").unwrap();
        assert_eq!(result.0, b"");

        let record = result.1;
        assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
        assert_eq!(record.location, Point::new(Deg(9.8852), Deg(50.28208333333333)));
        assert_eq!(record.valid, true);
        assert_eq!(record.pressure_altitude, Some(2164));
        assert_eq!(record.gnss_altitude, Some(2287));
        assert_eq!(record.extra, "00309");
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
