use nom;
use chrono::NaiveTime;

use super::helpers::buf_to_u32;

named!(lower_hour <u32>, do_parse!(
    f:char_between!('0','1') >>
    s:char_between!('0','9') >>
    (buf_to_u32(f) * 10 + buf_to_u32(s))
));

named!(upper_hour <u32>, do_parse!(
    tag!("2") >>
    s:char_between!('0','3') >>
    (20 + buf_to_u32(s))
));

named!(hour <u32>, alt!(lower_hour | upper_hour));

named!(below_sixty <u32>, do_parse!(
    f:char_between!('0','5') >>
    s:char_between!('0','9') >>
    (buf_to_u32(f) * 10 + buf_to_u32(s))
));

named!(minute <u32>, call!(below_sixty));
named!(second <u32>, call!(below_sixty));

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
    use super::{hour, minute, second, time};

    #[test]
    fn test_hour() {
        assert!(hour(b"-1").is_err());
        assert_eq!(hour(b"00"), Done(&[][..], 0));
        assert_eq!(hour(b"06"), Done(&[][..], 6));
        assert_eq!(hour(b"23"), Done(&[][..], 23));
        assert_eq!(hour(b"23-"), Done(&b"-"[..], 23));
        assert!(hour(b"24").is_err());
    }

    #[test]
    fn test_minute() {
        assert!(minute(b"-1").is_err());
        assert_eq!(minute(b"00"), Done(&[][..], 0));
        assert_eq!(minute(b"06"), Done(&[][..], 6));
        assert_eq!(minute(b"59"), Done(&[][..], 59));
        assert_eq!(minute(b"59-"), Done(&b"-"[..], 59));
        assert!(minute(b"60").is_err());
    }

    #[test]
    fn test_second() {
        assert!(second(b"-1").is_err());
        assert_eq!(second(b"00"), Done(&[][..], 0));
        assert_eq!(second(b"06"), Done(&[][..], 6));
        assert_eq!(second(b"59"), Done(&[][..], 59));
        assert_eq!(second(b"59-"), Done(&b"-"[..], 59));
        assert!(second(b"60").is_err());
    }

    #[test]
    fn test_time() {
        assert!(time(b"12345").is_incomplete());
        assert_eq!(time(b"000000"), Done(&[][..], NaiveTime::from_hms(0, 0, 0)));
        assert_eq!(time(b"123456"), Done(&[][..], NaiveTime::from_hms(12, 34, 56)));
        assert_eq!(time(b"235959"), Done(&[][..], NaiveTime::from_hms(23, 59, 59)));
        assert!(time(b"612345").is_err());
    }
}
