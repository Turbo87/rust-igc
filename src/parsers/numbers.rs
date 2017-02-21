use nom;

use super::helpers::{buf_to_u32, buf_to_i32};

named!(pub below_24 <u32>, alt!(do_parse!(
    f:char_between!('0','1') >>
    s:char_between!('0','9') >>
    (buf_to_u32(f) * 10 + buf_to_u32(s))
) | do_parse!(
    tag!("2") >>
    s:char_between!('0','3') >>
    (20 + buf_to_u32(s))
)));

named!(pub below_60 <u32>, do_parse!(
    f:char_between!('0','5') >>
    s:char_between!('0','9') >>
    (buf_to_u32(f) * 10 + buf_to_u32(s))
));

named!(pub below_90 <u32>, do_parse!(
    f:char_between!('0','8') >>
    s:char_between!('0','9') >>
    (buf_to_u32(f) * 10 + buf_to_u32(s))
));

named!(pub up_to_90 <u32>, alt!(below_90 | map!(tag!("90"), |_| 90)));

named!(pub below_180 <u32>, alt!(do_parse!(
    tag!("0") >>
    s:char_between!('0','9') >>
    t:char_between!('0','9') >>
    (buf_to_u32(s) * 10 + buf_to_u32(t))
) | do_parse!(
    tag!("1") >>
    s:char_between!('0','7') >>
    t:char_between!('0','9') >>
    (100 + buf_to_u32(s) * 10 + buf_to_u32(t))
)));

named!(pub up_to_180 <u32>, alt!(below_180 | map!(tag!("180"), |_| 180)));

named!(pub below_1000 <u32>, do_parse!(
    f:char_between!('0','9') >>
    s:char_between!('0','9') >>
    t:char_between!('0','9') >>
    (buf_to_u32(f) * 100 + buf_to_u32(s) * 10 + buf_to_u32(t))
));

named!(pub up_to_99999 <u32>, do_parse!(
    char_1: char_between!('0','9') >>
    char_2: char_between!('0','9') >>
    char_3: char_between!('0','9') >>
    char_4: char_between!('0','9') >>
    char_5: char_between!('0','9') >>
    (
        buf_to_u32(char_1) * 10000 +
        buf_to_u32(char_2) * 1000 +
        buf_to_u32(char_3) * 100 +
        buf_to_u32(char_4) * 10 +
        buf_to_u32(char_5)
    )
));

named!(pub down_to_minus_9999 <i32>, do_parse!(
    tag!("-") >>
    char_1: char_between!('0','9') >>
    char_2: char_between!('0','9') >>
    char_3: char_between!('0','9') >>
    char_4: char_between!('0','9') >>
    (
        -buf_to_i32(char_1) * 1000 +
        -buf_to_i32(char_2) * 100 +
        -buf_to_i32(char_3) * 10 +
        -buf_to_i32(char_4)
    )
));

#[cfg(test)]
mod tests {
    use nom::IResult::*;
    use chrono::NaiveTime;
    use super::*;

    #[test]
    fn test_below_24() {
        assert!(below_24(b"-1").is_err());
        assert_eq!(below_24(b"00"), Done(&[][..], 0));
        assert_eq!(below_24(b"06"), Done(&[][..], 6));
        assert_eq!(below_24(b"23"), Done(&[][..], 23));
        assert_eq!(below_24(b"23-"), Done(&b"-"[..], 23));
        assert!(below_24(b"24").is_err());
    }

    #[test]
    fn test_below_60() {
        assert!(below_60(b"-1").is_err());
        assert_eq!(below_60(b"00"), Done(&[][..], 0));
        assert_eq!(below_60(b"06"), Done(&[][..], 6));
        assert_eq!(below_60(b"59"), Done(&[][..], 59));
        assert_eq!(below_60(b"59-"), Done(&b"-"[..], 59));
        assert!(below_60(b"60").is_err());
    }

    #[test]
    fn test_below_90() {
        assert!(below_90(b"0").is_incomplete());
        assert!(below_90(b"-1").is_err());
        assert_eq!(below_90(b"00"), Done(&[][..], 0));
        assert_eq!(below_90(b"23"), Done(&[][..], 23));
        assert_eq!(below_90(b"89"), Done(&[][..], 89));
        assert_eq!(below_90(b"89-"), Done(&b"-"[..], 89));
        assert!(below_90(b"90").is_err());
    }

    #[test]
    fn test_up_to_90() {
        assert!(up_to_90(b"0").is_incomplete());
        assert!(up_to_90(b"-1").is_err());
        assert_eq!(up_to_90(b"00"), Done(&[][..], 0));
        assert_eq!(up_to_90(b"23"), Done(&[][..], 23));
        assert_eq!(up_to_90(b"89"), Done(&[][..], 89));
        assert_eq!(up_to_90(b"90"), Done(&[][..], 90));
        assert_eq!(up_to_90(b"89-"), Done(&b"-"[..], 89));
        assert!(up_to_90(b"91").is_err());
    }

    #[test]
    fn test_below_180() {
        assert!(below_180(b"09").is_incomplete());
        assert!(below_180(b"90").is_err());
        assert!(below_180(b"-1").is_err());
        assert_eq!(below_180(b"000"), Done(&[][..], 0));
        assert_eq!(below_180(b"023"), Done(&[][..], 23));
        assert_eq!(below_180(b"123"), Done(&[][..], 123));
        assert_eq!(below_180(b"179"), Done(&[][..], 179));
        assert_eq!(below_180(b"179-"), Done(&b"-"[..], 179));
        assert!(below_180(b"180").is_err());
    }

    #[test]
    fn test_up_to_180() {
        assert!(up_to_180(b"09").is_incomplete());
        assert!(up_to_180(b"90").is_err());
        assert!(up_to_180(b"-1").is_err());
        assert_eq!(up_to_180(b"000"), Done(&[][..], 0));
        assert_eq!(up_to_180(b"023"), Done(&[][..], 23));
        assert_eq!(up_to_180(b"123"), Done(&[][..], 123));
        assert_eq!(up_to_180(b"179"), Done(&[][..], 179));
        assert_eq!(up_to_180(b"180"), Done(&[][..], 180));
        assert_eq!(up_to_180(b"179-"), Done(&b"-"[..], 179));
        assert!(up_to_180(b"181").is_err());
    }

    #[test]
    fn test_below_1000() {
        assert!(below_1000(b"00").is_incomplete());
        assert!(below_1000(b"-10").is_err());
        assert_eq!(below_1000(b"000"), Done(&[][..], 0));
        assert_eq!(below_1000(b"023"), Done(&[][..], 23));
        assert_eq!(below_1000(b"923"), Done(&[][..], 923));
        assert_eq!(below_1000(b"999"), Done(&[][..], 999));
        assert_eq!(below_1000(b"1000"), Done(&b"0"[..], 100));
        assert_eq!(below_1000(b"179-"), Done(&b"-"[..], 179));
    }

    #[test]
    fn test_up_to_99999() {
        assert!(up_to_99999(b"00").is_incomplete());
        assert!(up_to_99999(b"-10").is_err());
        assert!(up_to_99999(b"abcde").is_err());
        assert_eq!(up_to_99999(b"00000"), Done(&[][..], 0));
        assert_eq!(up_to_99999(b"00230"), Done(&[][..], 230));
        assert_eq!(up_to_99999(b"92345"), Done(&[][..], 92345));
        assert_eq!(up_to_99999(b"99999"), Done(&[][..], 99999));
        assert_eq!(up_to_99999(b"100000"), Done(&b"0"[..], 10000));
    }

    #[test]
    fn test_down_to_minus_9999() {
        assert!(down_to_minus_9999(b"-000").is_incomplete());
        assert!(down_to_minus_9999(b"+1000").is_err());
        assert!(down_to_minus_9999(b"abcde").is_err());
        assert_eq!(down_to_minus_9999(b"-0000"), Done(&[][..], 0));
        assert_eq!(down_to_minus_9999(b"-0230"), Done(&[][..], -230));
        assert_eq!(down_to_minus_9999(b"-2345"), Done(&[][..], -2345));
        assert_eq!(down_to_minus_9999(b"-9999"), Done(&[][..], -9999));
        assert_eq!(down_to_minus_9999(b"-10000"), Done(&b"0"[..], -1000));
    }
}
