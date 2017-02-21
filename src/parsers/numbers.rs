use nom;

use super::helpers::buf_to_u32;

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
}
