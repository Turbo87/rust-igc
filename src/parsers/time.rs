use nom;
use chrono::NaiveTime;

use super::numbers::{below_24, below_60};

named!(hour <u32>, call!(below_24));
named!(minute <u32>, call!(below_60));
named!(second <u32>, call!(below_60));

named!(pub time <NaiveTime>, do_parse!(
    h: hour >>
    m: minute >>
    s: second >>
    (NaiveTime::from_hms(h, m, s))
));

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use chrono::NaiveTime;
    use super::time;

    #[test]
    fn test_time() {
        assert!(time(b"12345").is_incomplete());
        assert_eq!(time(b"000000"), Done(&[][..], NaiveTime::from_hms(0, 0, 0)));
        assert_eq!(time(b"123456"), Done(&[][..], NaiveTime::from_hms(12, 34, 56)));
        assert_eq!(time(b"235959"), Done(&[][..], NaiveTime::from_hms(23, 59, 59)));
        assert!(time(b"612345").is_err());
    }
}
