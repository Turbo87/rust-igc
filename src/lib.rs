extern crate chrono;

use std::str::FromStr;
use chrono::NaiveTime;

struct BRecord {
    time: NaiveTime,
}

impl FromStr for BRecord {
    type Err = ();
    fn from_str(s: &str) -> Result<BRecord, ()> {
        let time = try!(NaiveTime::from_hms_opt(1, 2, 3).ok_or(()));

        Ok(BRecord {
            time: time,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;
    use BRecord;

    #[test]
    fn it_works() {
        let record = "B1414065016925N00953112EA021640228700309".parse::<BRecord>();
        assert!(record.is_ok());
        assert_eq!(record.unwrap().time, NaiveTime::from_hms(1, 2, 3));
    }
}
