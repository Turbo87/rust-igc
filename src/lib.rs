extern crate chrono;
extern crate geo;
extern crate regex;

use std::str::FromStr;
use chrono::NaiveTime;
use geo::Point;
use regex::Regex;

struct BRecord {
    time: NaiveTime,
    location: Point<f64>,
    valid: bool,
    pressure_altitude: i16,
    gps_altitude: i16,
    extra: String,
}

impl FromStr for BRecord {
    type Err = String;
    fn from_str(s: &str) -> Result<BRecord, String> {
        let re = Regex::new(concat!(r"^B",
            r"(\d{2})(\d{2})(\d{2})",
            r"(\d{2})(\d{2})(\d{3})([NS])",
            r"(\d{3})(\d{2})(\d{3})([EW])",
            r"([AV])",
            r"([-\d]\d\d\d\d)",
            r"([-\d]\d\d\d\d)",
            r"([0-9a-zA-Z]*).*$",
        )).unwrap();

        let caps = try!(re.captures(s).ok_or("Could not parse B record"));

        let time = try!(NaiveTime::from_hms_opt(
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        ).ok_or("Invalid time"));

        let latitude = parse_angle(
            caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(6).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(7).unwrap().as_str() == "S",
        );

        let longitude = parse_angle(
            caps.get(8).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(9).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(10).unwrap().as_str().parse::<u32>().unwrap(),
            caps.get(11).unwrap().as_str() == "W",
        );

        let location = Point::new(longitude, latitude);

        let valid = caps.get(12).unwrap().as_str() == "A";
        let pressure_altitude = caps.get(13).unwrap().as_str().parse::<i16>().unwrap();
        let gps_altitude = caps.get(14).unwrap().as_str().parse::<i16>().unwrap();
        let extra = caps.get(15).unwrap().as_str().to_string();

        Ok(BRecord {
            time: time,
            location: location,
            valid: valid,
            pressure_altitude: pressure_altitude,
            gps_altitude: gps_altitude,
            extra: extra,
        })
    }
}

fn parse_angle(deg: u32, min: u32, decmin: u32, negative: bool) -> f64 {
    let value = deg as f64 + (min as f64) / 60. + (decmin as f64) / 60000.;
    if negative { -value } else { value }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;
    use geo::Point;
    use BRecord;

    #[test]
    fn it_works() {
        let record: BRecord = "B1414065016925N00953112EA021640228700309".parse().unwrap();
        assert_eq!(record.time, NaiveTime::from_hms(14, 14, 06));
        assert_eq!(record.location, Point::new(9.8852, 50.28208333333333));
        assert_eq!(record.valid, true);
        assert_eq!(record.pressure_altitude, 2164);
        assert_eq!(record.gps_altitude, 2287);
        assert_eq!(record.extra, "00309");
    }
}
