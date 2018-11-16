use regex::bytes::Regex;

use ::{Error, Result, Time, Date};
use ::utils::num::parse_int;
use ::utils::text::as_text;
use utils::coordinates::{parse_latitude_unchecked, parse_longitude_unchecked};

#[derive(Debug)]
pub struct CRecordDeclaration {
    date: Date,
    time: Time,
    flight_date: Option<Date>,
    task_number: Option<u16>,
    num_turnpoints: u8,
    text: Option<String>,
}

impl CRecordDeclaration {
    pub fn parse(line: &[u8]) -> Result<CRecordDeclaration> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^C
                (\d{6})    # UTC date
                (\d{6})    # UTC time
                (\d{6})    # flight date
                (\d{4})    # task number
                (\d{2})    # number of turnpoints
                (.*)       # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let date = Date::parse_unchecked(&cap[1]);
        let time = Time::parse_unchecked(&cap[2]);

        let flight_date = match &cap[3] {
            b"000000" => None,
            bytes => Some(Date::parse_unchecked(bytes)),
        };

        let task_number = match &cap[4] {
            b"0000" => None,
            bytes => Some(parse_int::<u16>(bytes).unwrap()),
        };

        let num_turnpoints = parse_int::<u8>(&cap[5]).unwrap();
        let text = if cap[6].is_empty() { None } else { Some(as_text(&cap[6]).ok_or_else(||Error::invalid_record(line))?) };

        Ok(CRecordDeclaration { time, date, flight_date, task_number, num_turnpoints, text })
    }
}

#[derive(Debug)]
pub struct CRecordTurnpoint {
    latitude: f64,
    longitude: f64,
    text: Option<String>,
}

impl CRecordTurnpoint {
    pub fn parse(line: &[u8]) -> Result<CRecordTurnpoint> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x-u)
                ^C
                (\d{7}[NS])    # latitude
                (\d{8}[EW])    # longitude
                (.*)           # text
            ").unwrap();
        }

        let cap = RE.captures(line).ok_or_else(|| Error::invalid_record(line))?;

        let latitude = parse_latitude_unchecked(&cap[1]);
        let longitude = parse_longitude_unchecked(&cap[2]);
        let text = if cap[3].is_empty() { None } else { Some(as_text(&cap[3]).ok_or_else(||Error::invalid_record(line))?) };

        Ok(CRecordTurnpoint { latitude, longitude, text })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_declaration() {
        let decl = CRecordDeclaration::parse(b"C040516072924000000000002Task").unwrap();
        assert_eq!(decl.date, Date::new(16, 5, 4));
        assert_eq!(decl.time, Time::from_hms(7, 29, 24));
        assert_eq!(decl.flight_date, None);
        assert_eq!(decl.task_number, None);
        assert_eq!(decl.num_turnpoints, 2);
        assert_eq!(decl.text, Some("Task".into()));

        let decl = CRecordDeclaration::parse(b"C150709112124150709000114").unwrap();
        assert_eq!(decl.date, Date::new(9, 7, 15));
        assert_eq!(decl.time, Time::from_hms(11, 21, 24));
        assert_eq!(decl.flight_date, Some(Date::new(9, 7, 15)));
        assert_eq!(decl.task_number, Some(1));
        assert_eq!(decl.num_turnpoints, 14);
        assert_eq!(decl.text, None);

        let decl = CRecordDeclaration::parse(b"C210815093841000000000002500K Triangle").unwrap();
        assert_eq!(decl.date, Date::new(15, 8, 21));
        assert_eq!(decl.time, Time::from_hms(9, 38, 41));
        assert_eq!(decl.flight_date, None);
        assert_eq!(decl.task_number, None);
        assert_eq!(decl.num_turnpoints, 2);
        assert_eq!(decl.text, Some("500K Triangle".into()));
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_declaration_with_random_data(s in r"\PC*") {
            CRecordDeclaration::parse(s.as_bytes());
        }
    }

    #[test]
    fn test_parse_turnpoint() {
        let tp = CRecordTurnpoint::parse(b"C5111359N00101899WTAKEOFF Lasham Clubhouse").unwrap();
        assert_relative_eq!(tp.latitude, 51.18931666666667);
        assert_relative_eq!(tp.longitude, -1.03165);
        assert_eq!(tp.text, Some("TAKEOFF Lasham Clubhouse".into()));

        let tp = CRecordTurnpoint::parse(b"C5110179N00102644WSTART Lasham Start S").unwrap();
        assert_relative_eq!(tp.latitude, 51.16965);
        assert_relative_eq!(tp.longitude, -1.0440666666666667);
        assert_eq!(tp.text, Some("START Lasham Start S".into()));

        let tp = CRecordTurnpoint::parse(b"C5209092N00255227WTURN Sarnesfield").unwrap();
        assert_relative_eq!(tp.latitude, 52.15153333333333);
        assert_relative_eq!(tp.longitude, -2.9204499999999998);
        assert_eq!(tp.text, Some("TURN Sarnesfield".into()));

        let tp = CRecordTurnpoint::parse(b"C5230147N00017612WTURN Norman Cross").unwrap();
        assert_relative_eq!(tp.latitude, 52.50245);
        assert_relative_eq!(tp.longitude, -0.2935333333333333);
        assert_eq!(tp.text, Some("TURN Norman Cross".into()));

        let tp = CRecordTurnpoint::parse(b"C5110179N00102644WFINISH Lasham Start S").unwrap();
        assert_relative_eq!(tp.latitude, 51.16965);
        assert_relative_eq!(tp.longitude, -1.0440666666666667);
        assert_eq!(tp.text, Some("FINISH Lasham Start S".into()));

        let tp = CRecordTurnpoint::parse(b"C5111359N00101899WLANDING Lasham Clubhouse").unwrap();
        assert_relative_eq!(tp.latitude, 51.18931666666667);
        assert_relative_eq!(tp.longitude, -1.03165);
        assert_eq!(tp.text, Some("LANDING Lasham Clubhouse".into()));

        let tp = CRecordTurnpoint::parse(b"C8911359S17901899E").unwrap();
        assert_relative_eq!(tp.latitude, -89.18931666666667);
        assert_relative_eq!(tp.longitude, 179.03165);
        assert_eq!(tp.text, None);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_turnpoint_with_random_data(s in r"\PC*") {
            CRecordTurnpoint::parse(s.as_bytes());
        }
    }
}
